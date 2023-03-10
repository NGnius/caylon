use std::process::Command;

use usdpl_back::core::serdes::Primitive;

use caylon_config::CommandAction;
use super::{SeqAct, ActError};

/// Runs a CLI command in Bash
pub struct CommandActor {
    shell: String,
    run: String,
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

impl<'a> SeqAct<'a> for CommandActor {
    type BuildParam = ();
    type Config = CommandAction;

    fn build(config: &'a CommandAction, _: ()) -> Result<Self, ActError> {
        Ok(
            Self {
                shell: "bash".to_owned(),
                run: config.run.clone(),
            }
        )
    }

    fn run(self, parameter: Primitive) -> Primitive {
        let variable = Self::primitive_to_string(parameter);
        let output = Command::new(&self.shell)
            .args(["-c", &self.run])
            .env(super::VALUE_VAR, &variable)
            .output()
            .expect(&format!("Cannot run `{}`", &self.run));
        if !output.stderr.is_empty() {
            log::error!("Error running `{}`: {}", &self.run, String::from_utf8(output.stderr).unwrap_or_else(|_| "<non utf-8 stderr output>".to_owned()))
        }
        let result = String::from_utf8(output.stdout).expect(&format!("Cannot parse stdout from `{}` as UTF-8", self.run));
        log::debug!("CommandActor ran `{}` (${}=\"{}\") -> `{}`", &self.run, super::VALUE_VAR, &variable, &result);
        Primitive::String(result)
    }
}
