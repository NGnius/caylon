use std::process::Command;

use usdpl_back::core::serdes::Primitive;

use crate::config::CommandAction;
use super::{Act, ActError};

const VALUE_ENV_VAR: &str = "KAYLON_VALUE";

pub struct CommandActor {
    shell: String,
    run: String,
    variable: String,
}

impl CommandActor {
    fn primitive_to_string(obj: Primitive) -> String {
        match obj {
            Primitive::Empty => String::new(),
            Primitive::String(s) => s,
            Primitive::F32(f) => f.to_string(),
            Primitive::F64(f) => f.to_string(),
            Primitive::I32(i) => i.to_string(),
            Primitive::I64(i) => i.to_string(),
            Primitive::U32(u) => u.to_string(),
            Primitive::U64(u) => u.to_string(),
            Primitive::Bool(b) => b.to_string().to_uppercase(),
            Primitive::Json(j) => j,
        }
    }
}

impl Act for CommandActor {
    type Param = Primitive;
    type Config = CommandAction;
    type Return = String;

    fn build(config: &CommandAction, parameter: Primitive) -> Result<Self, ActError> {
        Ok(
            Self {
                shell: "bash".to_owned(),
                run: config.run.clone(),
                variable: Self::primitive_to_string(parameter),
            }
        )
    }

    fn run(self) -> Self::Return {
        let output = Command::new(&self.shell)
            .args(["-c", &self.run])
            .env(VALUE_ENV_VAR, &self.variable)
            .output()
            .expect(&format!("Cannot run `{}`", &self.run));
        if !output.stderr.is_empty() {
            log::error!("Error running `{}`: {}", &self.run, String::from_utf8(output.stderr).unwrap_or_else(|_| "<non utf-8 stderr output>".to_owned()))
        }
        let result = String::from_utf8(output.stdout).expect(&format!("Cannot parse stdout from `{}` as UTF-8", self.run));
        log::debug!("CommandActor ran `{}` (${}=\"{}\") -> `{}`", &self.run, VALUE_ENV_VAR, &self.variable, &result);
        result
    }
}
