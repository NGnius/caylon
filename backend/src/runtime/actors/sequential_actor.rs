use usdpl_back::core::serdes::Primitive;

use caylon_config::SequenceAction;
use super::{SeqAct, ActError, ActorType};

pub struct SequenceActor {
    steps: Vec<ActorType>
}

impl<'a> SeqAct<'a> for SequenceActor {
    type BuildParam = &'a crate::runtime::RuntimeIO;
    type Config = SequenceAction;

    fn build(config: &'a Self::Config, parameter: Self::BuildParam) -> Result<Self, ActError> {
        let mut actors = Vec::with_capacity(config.steps.len());
        for step in config.steps.iter() {
            actors.push(ActorType::build(step, parameter)?);
        }
        Ok(
            Self {
                steps: actors,
            }
        )
    }

    fn run(self, p: Primitive) -> Primitive {
        let mut output = p;
        for step in self.steps {
            output = step.run(output);
        }
        output
    }
}
