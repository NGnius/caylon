use std::sync::mpsc::{self, Receiver, Sender};

use usdpl_back::core::serdes::Primitive;

//use crate::config::ElementConfig;
use super::{Act, ActError};

const MAX_HANDLERS_PER_ITEM: usize = 8;

pub enum RouterCommand {
    AddSender {
        index: usize,
        sender: Sender<Primitive>,
    },
    HandleResult {
        index: usize,
        result: Primitive,
    },
    Clear{}
}

pub struct ResultRouter {
    comm: Receiver<RouterCommand>,
    senders: Vec<[Option<Sender<Primitive>>; MAX_HANDLERS_PER_ITEM]>,
    comm_tx: Option<Sender<RouterCommand>>,
    cache: Vec<Option<Primitive>>,
}

impl ResultRouter {
    fn all_senders_none(senders: &[Option<Sender<Primitive>>]) -> bool {
        let mut all_none = true;
        for s in senders.iter() {
            all_none &= s.is_none();
        }
        all_none
    }
}

impl Act for ResultRouter {
    type Param = usize;
    type Config = ();
    type Return = Sender<RouterCommand>;

    fn build(_config: &Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        let (tx, rx) = mpsc::channel();
        let mut cache_vec = Vec::with_capacity(parameter);
        for _ in 0..parameter {
            cache_vec.push(None);
        }
        Ok(Self {
            comm: rx,
            senders: vec![[(); MAX_HANDLERS_PER_ITEM].map(|_| None); parameter],
            comm_tx: Some(tx),
            cache: cache_vec,
        })
    }

    fn run(mut self) -> Self::Return {
        let result = self.comm_tx.take().unwrap();
        std::thread::spawn(move || {
            log::debug!("ResultRouter starting");
            for command in self.comm.iter() {
                match command {
                    RouterCommand::AddSender { index, sender } => {
                        log::debug!("Handling AddSender for item #{}", index);
                        if let Some(senders) = self.senders.get_mut(index) {
                            // send cached value, if available
                            if self.cache[index].is_some() {
                                log::debug!("Routing cached result for item #{}", index);
                                let result = self.cache[index].take().unwrap();
                                match sender.send(result) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        self.cache[index] = Some(e.0);
                                        log::debug!("ResultRouter ignoring AddSender since sending cached value failed");
                                        continue;
                                    },
                                }
                            }
                            // save sender for future results
                            let mut was_set = false;
                            'inner_loop: for sender_opt in senders {
                                if sender_opt.is_none() {
                                    *sender_opt = Some(sender);
                                    was_set = true;
                                    break 'inner_loop;
                                }
                            }
                            if !was_set {
                                log::warn!("ResultRouter could not add another sender for index {}", index);
                            }
                        } else {
                            log::warn!("ResultRouter got AddSender command for invalid index {} (max: {})", index, self.senders.len());
                        }
                    }
                    RouterCommand::HandleResult {index, result} => {
                        log::debug!("Handling HandleResult for item #{}", index);
                        if let Some(senders) = self.senders.get_mut(index) {
                            if Self::all_senders_none(senders) {
                                self.cache[index] = Some(result);
                                log::debug!("Cached result for item #{}", index);
                            } else {
                                for (i, sender_opt) in senders.iter_mut().enumerate() {
                                    if let Some(sender) = sender_opt {
                                        match sender.send(super::primitive_utils::clone(&result)) {
                                            Ok(_) => {},
                                            Err(_) => {
                                                log::debug!("Removing sender {} because it seems closed", i);
                                                *sender_opt = None;
                                            }
                                        }
                                    }
                                }
                                log::debug!("Routed result for item #{}", index);
                            }
                        } else {
                            log::warn!("ResultRouter got AddSender command for invalid index {} (max: {})", index, self.senders.len());
                        }
                    },
                    RouterCommand::Clear {} => {
                        log::debug!("Handling Clear");
                        for i in 0..self.senders.len() {
                            self.senders[i] = [(); MAX_HANDLERS_PER_ITEM].map(|_| None);
                            self.cache[i] = None;
                        }
                    }
                }
            }
            log::warn!("ResultRouter completed");
        });
        result
    }
}
