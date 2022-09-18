use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::{VecDeque, HashMap};

use usdpl_back::core::serdes::Primitive;

//use crate::config::ElementConfig;
use super::{Act, ActError};
use crate::api::JavascriptData;

pub enum Javascript {
    Raw(String),
}

impl Javascript {
    fn raw(self) -> String {
        match self {
            Self::Raw(s) => s,
        }
    }
}

pub enum JavascriptCommand {
    Subscribe { respond_to: Sender<JavascriptData>},
    Run { js: Javascript, respond_to: Sender<Primitive>},
    Result { value: Primitive, id: usize },
}

pub struct JavascriptRouter {
    /// command listener
    comm: Receiver<JavascriptCommand>,
    /// cache of sender, for Act paradigm
    comm_tx: Option<Sender<JavascriptCommand>>,
    /// cache of javascript to be run, in case no sender is subscribed
    cache_js: VecDeque<JavascriptData>,
    /// javascript subscriber
    js_handler: Option<Sender<JavascriptData>>,
    /// result subscriber
    result_handlers: HashMap<usize, Sender<Primitive>>,
    /// internal JS run id
    next_id: usize,
}

impl<'a> Act<'a> for JavascriptRouter {
    type Param = ();
    type Config = ();
    type Return = Sender<JavascriptCommand>;

    fn build(_config: &'a Self::Config, _parameter: Self::Param) -> Result<Self, ActError> {
        let (tx, rx) = mpsc::channel();
        Ok(Self {
            comm: rx,
            comm_tx: Some(tx),
            cache_js: VecDeque::with_capacity(8),
            js_handler: None,
            result_handlers: HashMap::with_capacity(8),
            next_id: 42,
        })
    }

    fn run(mut self) -> Self::Return {
        let result = self.comm_tx.take().unwrap();
        std::thread::spawn(move || {
            log::debug!("ResultRouter starting");
            for command in self.comm.iter() {
                match command {
                    JavascriptCommand::Subscribe{ respond_to } => {
                        log::debug!("Handling JS Subscribe");
                        self.js_handler = Some(respond_to);
                        if !self.cache_js.is_empty() {
                            let respond_to = self.js_handler.as_ref().unwrap();
                            'inner: while let Some(data) = self.cache_js.pop_front() {
                                if let Err(e) = respond_to.send(data) {
                                    log::warn!("Unable to send on new JS subscriber, cannot empty cache");
                                    self.cache_js.push_front(e.0); // re-cache
                                    self.js_handler = None;
                                    break 'inner;
                                }
                            }
                        }
                    },
                    JavascriptCommand::Run { js, respond_to } => {
                        log::debug!("Handling JS Run");
                        let current_id = self.next_id;
                        self.next_id += 1;
                        self.result_handlers.insert(current_id, respond_to);
                        let data = JavascriptData::from_string(js.raw(), current_id);
                        if let Some(handler) = &self.js_handler {
                            if let Err(e) = handler.send(data) {
                                log::debug!("Removing JS Run handler, caching JS data #{} for next subscriber", current_id);
                                self.js_handler = None;
                                self.cache_js.push_back(e.0);
                            } else {
                                log::debug!("Successfully published JS data #{} to run", current_id);
                            }
                        } else {
                            self.cache_js.push_back(data);
                            log::debug!("Cached JS Run data #{}", current_id);
                        }
                    },
                    JavascriptCommand::Result { value, id } => {
                        log::debug!("Handling JS Result for id #{}", id);
                        if let Some(respond_to) = self.result_handlers.remove(&id) {
                            if let Err(_) = respond_to.send(value) {
                                log::warn!("Unable to send to result handler #{}, ignoring result", id);
                            } else {
                                log::debug!("Successfully handled JS result for #{}", id);
                            }
                        } else {
                            log::warn!("No JS result handler for #{}", id);
                        }
                    }
                }
            }
        });
        result
    }
}
