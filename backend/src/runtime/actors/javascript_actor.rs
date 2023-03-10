use std::sync::mpsc::{Sender, self};

use usdpl_back::core::serdes::Primitive;

use caylon_config::JavascriptAction;
use super::{SeqAct, ActError};
use crate::runtime::{RuntimeIO, JavascriptCommand, Javascript};

pub struct JavascriptActor {
    command_chan: Sender<JavascriptCommand>,
    js_script: String,
}

impl JavascriptActor {
    fn primitive_to_js(primitive: Primitive) -> String {
        match primitive {
            Primitive::Empty => "null".to_owned(),
            Primitive::String(s) => format!("\"{}\"", s),
            Primitive::F32(f) => f.to_string(),
            Primitive::F64(f) => f.to_string(),
            Primitive::U32(u) => u.to_string(),
            Primitive::U64(u) => u.to_string(),
            Primitive::I32(i) => i.to_string(),
            Primitive::I64(i) => i.to_string(),
            Primitive::Bool(b) => b.to_string(),
            Primitive::Json(j) => j.to_string(),
        }
    }
}

impl<'a> SeqAct<'a> for JavascriptActor {
    type BuildParam = &'a RuntimeIO;
    type Config = JavascriptAction;

    fn build(config: &'a Self::Config, param: Self::BuildParam) -> Result<Self, ActError> {
        Ok(
            Self {
                command_chan: param.javascript.clone(),
                js_script: config.run.clone(),
            }
        )
    }

    fn run(self, parameter: Primitive) -> Primitive {
        let (rx, tx) = mpsc::channel();
        let js_script = Javascript::Raw(
            format!("const {} = {};\n{}",
                super::VALUE_VAR,
                Self::primitive_to_js(parameter),
                self.js_script
            )
        );
        if let Err(_) = self.command_chan.send(
            JavascriptCommand::Run { js: js_script, respond_to: rx}
        ) {
            log::warn!("Failed to send JavascriptCommand::Run for JavascriptActor");
            Primitive::Empty
        } else {
            match tx.recv() {
                Ok(result) => result,
                Err(_) => {
                    log::warn!("Failed to receive Javascript result for JavascriptActor");
                    Primitive::Empty
                }
            }
        }

    }
}
