use std::time::Duration;

use usdpl_back::core::serdes::Primitive;

use caylon_config::ReadingConfig;
use super::{Act, SeqAct, ActError, TopLevelActorType};
use crate::runtime::{RouterCommand, RuntimeIO};

/// Runs an action periodically
pub struct PeriodicActor {
    config: ReadingConfig,
    io: RuntimeIO,
    index: usize,
}

impl<'a> Act<'a> for PeriodicActor {
    type Param = (usize, &'a RuntimeIO);
    type Config = ReadingConfig;
    type Return = ();

    fn build(config: &'a Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        TopLevelActorType::build(&config.on_period, parameter.1)?;
        Ok(
            Self {
                config: config.clone(),
                io: parameter.1.clone(),
                index: parameter.0,
            }
        )
    }

    fn run(self) -> Self::Return {
        std::thread::spawn(move || {
            let sleep_duration = self.config.period_ms.map(|x| Duration::from_millis(x));
            loop {
                let actor = match TopLevelActorType::build(&self.config.on_period, &self.io) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("PeriodicActor failed to build for item #{}: {}", self.index, e);
                        break;
                    }
                };
                let result = actor.run(Primitive::Empty);
                match self.io.result.send(RouterCommand::HandleResult {
                    index: self.index, result
                }) {
                    Ok(_) => {},
                    Err(_e) => {
                        log::warn!("PeriodicActor failed to handle result for item #{}", self.index);
                        break;
                    }
                }
                if let Some(dur) = sleep_duration {
                    std::thread::sleep(dur);
                } else {
                    break;
                }
            }
            log::info!("PeriodicActor completed for #{}", self.index);
        });
    }
}
