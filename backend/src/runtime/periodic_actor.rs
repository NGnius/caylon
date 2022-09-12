use std::sync::mpsc::Sender;
use std::time::Duration;

use usdpl_back::core::serdes::Primitive;

use crate::config::ReadingConfig;
use super::{Act, ActError, ActorType, RouterCommand};

pub struct PeriodicActor {
    config: ReadingConfig,
    result_handler: Sender<RouterCommand>,
    index: usize,
}

impl Act for PeriodicActor {
    type Param = (usize, Sender<RouterCommand>);
    type Config = ReadingConfig;
    type Return = ();

    fn build(config: &Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        ActorType::build(&config.on_period, Primitive::Empty)?;
        Ok(
            Self {
                config: config.clone(),
                result_handler: parameter.1,
                index: parameter.0,
            }
        )
    }

    fn run(self) -> Self::Return {
        std::thread::spawn(move || {
            let sleep_duration = Duration::from_millis(self.config.period_ms);
            loop {
                let actor = match ActorType::build(&self.config.on_period, Primitive::Empty) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("PeriodicActor failed to build for item #{}: {}", self.index, e);
                        break;
                    }
                };
                let result = actor.run();
                match self.result_handler.send(RouterCommand::HandleResult {
                    index: self.index, result
                }) {
                    Ok(_) => {},
                    Err(_e) => {
                        log::warn!("PeriodicActor failed to handle result for item #{}", self.index);
                        break;
                    }
                }
                std::thread::sleep(sleep_duration);
            }
            log::info!("PeriodicActor completed for #{}", self.index);
        });
    }
}
