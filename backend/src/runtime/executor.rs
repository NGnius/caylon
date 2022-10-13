use std::thread;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};

use usdpl_back::core::serdes::Primitive;

use crate::config::{BaseConfig, ElementConfig};
use crate::api::SteamEvent;
use super::{QueueItem, QueueAction, Act, SeqAct};
use super::{ResultRouter, RouterCommand, JavascriptRouter, JavascriptCommand};

macro_rules! wire_steam_event {
    (
        $func_name: ident,
        $display_name: literal,
        $self: ident,
        $conf: ident,
        $item: ident,
        $index: ident,
        $event: ident,
        $event_json_cache: ident,
    ) => {
        if $conf.event.$func_name() {
            let value = cache_event_maybe(&$event, &mut $event_json_cache);
            match super::Actor::build($item, ($index, &$self.handlers)) {
                Ok(act) => {
                    let respond_to = $self.handlers.result.clone();
                    thread::spawn(move || {
                        let result = act.run(value);
                        match respond_to.send(RouterCommand::HandleResult{$index, result}) {
                            Ok(_) => {},
                            Err(_) => log::warn!("Failed to send DoSteamEvent `{}` response for item #{}", $display_name, $index),
                        }
                    });
                },
                Err(e) => log::error!("Failed to build DoSteamEvent `{}` actor for item #{}: {}", $display_name, $index, e)
            }
        }
    }
}

#[derive(Clone)]
pub struct RuntimeIO {
    pub result: Sender<RouterCommand>,
    pub javascript: Sender<JavascriptCommand>,
}

#[cfg(test)]
impl RuntimeIO {
    pub fn mock() -> (Self, Receiver<RouterCommand>, Receiver<JavascriptCommand>) {
        let (s1, r1) = mpsc::channel();
        let (s2, r2) = mpsc::channel();
        (
            Self {
                result: s1,
                javascript: s2,
            },
            r1,
            r2,
        )
    }
}

pub struct RuntimeExecutor {
    config_data: BaseConfig,
    tasks_receiver: Receiver<QueueItem>,
    config_path: PathBuf,
}

impl RuntimeExecutor {
    pub fn new<P: AsRef<Path>>(conf: BaseConfig, path: P) -> (Self, Sender<QueueItem>) {
        let (tx, rx) = mpsc::channel();
        (Self {
            config_data: conf,
            tasks_receiver: rx,
            config_path: path.as_ref().to_path_buf(),
        }, tx)
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || self.run_loop())
    }

    fn run_loop(self) {
        let (mut state, tasks_receiver) = self.split();
        state.populate_routers();
        for item in tasks_receiver.iter() {
            state.handle_item(item);
        }
    }

    fn split(self) -> (ExecutorState, Receiver<QueueItem>) {
        (
            ExecutorState {
                handlers: ExecutorState::build_routers(self.config_data.items().len()),
                config_data: self.config_data,
                config_path: self.config_path,
            },
            self.tasks_receiver
        )
    }
}

struct ExecutorState {
    handlers: RuntimeIO,
    config_data: BaseConfig,
    config_path: PathBuf,
}

impl ExecutorState {
    fn handle_item(&mut self, item: QueueItem) {
        match item.action {
            QueueAction::GetAbout { respond_to } => {
                // retrieve about information from (in-memory) config file
                respond_to.send(self.config_data.get_about().clone()).unwrap_or(());
            },
            QueueAction::DoUpdate { index, value } => {
                // trigger update event for element
                // e.g. on_click, on_toggle, etc. action
                if let Some(item) = self.config_data.get_item(index) {
                    match super::Actor::build(item, (index, &self.handlers)) {
                        Ok(act) => {
                            let respond_to = self.handlers.result.clone();
                            thread::spawn(move || {
                                let result = act.run(value);
                                match respond_to.send(RouterCommand::HandleResult{index, result}) {
                                    Ok(_) => {},
                                    Err(_) => log::warn!("Failed to send DoUpdate response for item #{}", index),
                                }
                            });
                        },
                        Err(e) => log::error!("Failed to build DoUpdate actor for item #{}: {}", index, e)
                    }
                } else {
                    log::warn!("Received DoUpdate on non-existent item #{} with value `{}`", index, super::primitive_utils::debug(&value))
                }
            },
            QueueAction::DoReload { respond_to } => {
                // reload config file from storage
                self.config_data = BaseConfig::load(&self.config_path);
                self.populate_routers();
                respond_to.send(self.config_data.items().clone()).unwrap_or(());
            },
            QueueAction::SetResultCallback { index, respond_to } => {
                // register a callback with the ResultRouter for an element's action
                // the next time that action is performed, the result will be sent through the callback
                if let Some(elem) = self.config_data.get_item(index) {
                    let display_of = match elem {
                        ElementConfig::ResultDisplay(c) => c.result_of,
                        _ => index,
                    };
                    if let Err(e) = self.handlers.result.send(
                        RouterCommand::AddSender {
                            index: display_of,
                            sender: respond_to,
                    }) {
                        log::warn!("Failed to send to ResultRouter, rebuilding routers");
                        self.handlers = ExecutorState::build_routers(self.config_data.items().len());
                        if let Err(_) = self.handlers.result.send(e.0) {
                            // don't retry if another error occurs
                            log::error!("Failed to send to ResultRouter again, did not SetResultCallback for item #{}", index);
                        }
                    }
                }
            },
            QueueAction::SetJavascriptSubscriber { respond_to } => {
                if let Err(e) = self.handlers.javascript.send(
                    JavascriptCommand::Subscribe {respond_to}) {
                    log::warn!("Failed to send to JavascriptRouter, rebuilding routers");
                    self.handlers = ExecutorState::build_routers(self.config_data.items().len());
                    if let Err(_) = self.handlers.javascript.send(e.0) {
                        // don't retry if another error occurs
                        log::error!("Failed to send to JavascriptRouter again, did not SetJavascriptSubscriber");
                    }
                }
            },
            QueueAction::DoJavascriptResult { value, id } => {
                if let Err(e) = self.handlers.javascript.send(
                    JavascriptCommand::Result {value, id}) {
                    log::warn!("Failed to send to JavascriptRouter, rebuilding routers");
                    self.handlers = ExecutorState::build_routers(self.config_data.items().len());
                    if let Err(_) = self.handlers.javascript.send(e.0) {
                        // don't retry if another error occurs
                        log::error!("Failed to send to JavascriptRouter again, did not SetJavascriptSubscriber");
                    }
                }
            },
            QueueAction::DoSteamEvent { event } => {
                // handle steam event for all elements that may be listening
                let mut event_json_cache: Option<String> = None;
                for (index, item) in self.config_data.items().iter().enumerate() {
                    match item {
                        ElementConfig::EventDisplay(conf) => {
                            match &event {
                                SteamEvent::DownloadItems(_x) => log::error!("Unsupported event"),
                                SteamEvent::DownloadOverview(_x) => log::error!("Unsupported event"),
                                SteamEvent::AchievementNotification(x) => wire_steam_event!{
                                    is_achievement,
                                    "achievement",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                                SteamEvent::BluetoothState(x) => wire_steam_event!{
                                    is_bluetooth,
                                    "bluetooth",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                                SteamEvent::ConnectivityTestChange(_x) => log::error!("Unsupported event"),
                                SteamEvent::NetworkDiagnostic(_x) => log::error!("Unsupported event"),
                                SteamEvent::AudioDeviceAdded(_x) => log::error!("Unsupported event"),
                                SteamEvent::AudioDeviceRemoved(_x) => log::error!("Unsupported event"),
                                SteamEvent::Brightness(x) => wire_steam_event!{
                                    is_brightness,
                                    "brightness",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                                SteamEvent::Airplane(x) => wire_steam_event!{
                                    is_airplane,
                                    "airplane",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                                SteamEvent::Battery(_x) => log::error!("Unsupported event"),
                                SteamEvent::ScreenshotNotification(x) => wire_steam_event!{
                                    is_screenshot,
                                    "screenshot",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                                SteamEvent::ControllerInputMessage(_x) => log::error!("Unsupported event"),
                                SteamEvent::AppLifetimeNotification(x) => wire_steam_event!{
                                    is_game_lifetime,
                                    "game-lifetime",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                                SteamEvent::GameActionStart(x) => wire_steam_event!{
                                    is_game_start,
                                    "game-start",
                                    self,
                                    conf,
                                    item,
                                    index,
                                    x,
                                    event_json_cache,
                                },
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn build_routers(items_len: usize) -> RuntimeIO {
        let js = JavascriptRouter::build(&(), ()).unwrap();
        let js_chan = js.run();
        let router = ResultRouter::build(&(), items_len).unwrap();
        let result_chan = router.run();
        RuntimeIO {
            javascript: js_chan,
            result: result_chan,
        }
    }

    fn populate_routers(&mut self) {
        if let Err(_) = self.handlers.result.send(RouterCommand::Clear{}) {
            return;
        }
        // start reading displays with periodic actions
        for (index, item) in self.config_data.items().iter().enumerate() {
            match item {
                ElementConfig::ReadingDisplay(r) => {
                    if let Ok(actor) = super::PeriodicActor::build(r, (index, &self.handlers)) {
                        actor.run();
                    }
                },
                _ => {}
            }
        }
    }
}

#[inline]
fn cache_event_maybe<T: serde::Serialize>(event: &T, cache: &mut Option<String>) -> Primitive {
    if let Some(cached) = cache {
        Primitive::Json(cached.to_owned())
    } else {
        let dump = serde_json::to_string(event).unwrap();
        *cache = Some(dump.clone());
        Primitive::Json(dump)
    }
}
