use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::config::{BaseConfig, ElementConfig};
use super::{QueueItem, QueueAction, Act};
use super::{ResultRouter, RouterCommand};

pub struct RuntimeExecutor {
    config_data: BaseConfig,
    tasks_receiver: Receiver<QueueItem>
}

impl RuntimeExecutor {
    pub fn new(conf: BaseConfig) -> (Self, Sender<QueueItem>) {
        let (tx, rx) = mpsc::channel();
        (Self {
            config_data: conf,
            tasks_receiver: rx,
        }, tx)
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || self.run_loop())
    }

    fn run_loop(self) {
        let (mut state, tasks_receiver) = self.split();
        state.populate_router();
        for item in tasks_receiver.iter() {
            state.handle_item(item);
        }
    }

    fn split(self) -> (ExecutorState, Receiver<QueueItem>) {
        (
            ExecutorState {
                result_handler: ExecutorState::build_router(self.config_data.items().len()),
                config_data: self.config_data,
            },
            self.tasks_receiver
        )
    }
}

struct ExecutorState {
    config_data: BaseConfig,
    result_handler: Sender<RouterCommand>,
}

impl ExecutorState {
    fn handle_item(&mut self, item: QueueItem) {
        match item.action {
            QueueAction::GetAbout { respond_to } => {
                respond_to.send(self.config_data.get_about().clone()).unwrap_or(());
            },
            QueueAction::DoUpdate { index, value } => {
                if let Some(item) = self.config_data.get_item(index) {
                    match super::Actor::build(item, (index, value)) {
                        Ok(act) => {
                            let respond_to = self.result_handler.clone();
                            thread::spawn(move || {
                                let result = act.run();
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
                self.config_data = BaseConfig::load(crate::consts::FILEPATH);
                self.populate_router();
                respond_to.send(self.config_data.items().clone()).unwrap_or(());
            },
            QueueAction::SetCallback { index, respond_to } => {
                if let Some(elem) = self.config_data.get_item(index) {
                    let display_of = match elem {
                        ElementConfig::ResultDisplay(c) => c.result_of,
                        _ => index,
                    };
                    if let Err(_) = self.result_handler.send(
                        RouterCommand::AddSender {
                            index: display_of,
                            sender: respond_to,
                    }) {
                        log::warn!("Failed to send to ResultRouter, rebuilding router");
                        self.result_handler = ExecutorState::build_router(self.config_data.items().len());
                    }
                }
            }
        }
    }

    fn build_router(items_len: usize) -> Sender<RouterCommand> {
        let router = ResultRouter::build(&(), items_len).unwrap();
        let result = router.run();
        result
    }

    fn populate_router(&mut self) {
        if let Err(_) = self.result_handler.send(RouterCommand::Clear{}) {
            return;
        }
        // start reading displays with periodic actions
        for (index, item) in self.config_data.items().iter().enumerate() {
            match item {
                ElementConfig::ReadingDisplay(r) => {
                    if let Ok(actor) = super::PeriodicActor::build(r, (index, self.result_handler.clone())) {
                        actor.run();
                    }
                },
                _ => {}
            }
        }
    }
}