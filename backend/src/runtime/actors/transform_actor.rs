use regex::{Regex, RegexBuilder};
use usdpl_back::core::serdes::Primitive;

use crate::runtime::primitive_utils;

use caylon_config::{TransformAction, TransformTypeAction, ReplaceTransformAction, ExpandTransformAction, LogTransformAction, LogLevel, PatternConfig};
use super::{SeqAct, ActError};

/// Changes the output or input of an act
pub enum TransformActor {
    Replace(ReplaceTransformActor),
    Expand(ExpandTransformActor),
    Log(LogTransformActor),
}

impl<'a> SeqAct<'a> for TransformActor {
    type BuildParam = ();
    type Config = TransformAction;

    fn build(config: &'a TransformAction, idc: ()) -> Result<Self, ActError> {
        let result = Ok(match &config.transformer {
            TransformTypeAction::Replace(x) =>
                Self::Replace(ReplaceTransformActor::build(x, idc)?),
            TransformTypeAction::Expand(x) =>
                Self::Expand(ExpandTransformActor::build(x, idc)?),
            TransformTypeAction::Log(x) =>
                Self::Log(LogTransformActor::build(x, idc)?),
        });
        result
    }

    fn run(self, parameter: Primitive) -> Primitive {
        match self {
            Self::Replace(x) => x.run(parameter),
            Self::Expand(x) => x.run(parameter),
            Self::Log(x) => x.run(parameter)
        }
    }
}

struct PatternRule {
    pattern: Regex,
    format: String,
}

impl PatternRule {
    #[inline]
    fn from_config(config: &PatternConfig) -> Result<Self, ActError> {
        let re = RegexBuilder::new(&config.pattern)
            .case_insensitive(config.i.unwrap_or(false))
            .multi_line(config.m.unwrap_or(false))
            .dot_matches_new_line(config.s.unwrap_or(false))
            .swap_greed(config.u.unwrap_or(false))
            .ignore_whitespace(config.x.unwrap_or(false))
            .build()
            .map_err(|e| format!("Failed to compile regex `{}`: {}", config.pattern, e))?;
        Ok(Self {
            pattern: re,
            format: config.format.clone(),
        })
    }
}

pub struct ReplaceTransformActor {
    patterns: Vec<PatternRule>,
}

impl<'a> SeqAct<'a> for ReplaceTransformActor {
    type BuildParam = ();
    type Config = ReplaceTransformAction;

    fn build(config: &'a Self::Config, _: ()) -> Result<Self, ActError> {
        let mut patterns_vec: Vec<PatternRule> = Vec::with_capacity(config.patterns.len());
        for pattern in config.patterns.iter() {
            patterns_vec.push(PatternRule::from_config(pattern)?);
        }
        Ok(
            Self {
                patterns: patterns_vec,
            }
        )
    }

    fn run(self, p: Primitive) -> Primitive {
        let mut stringy = primitive_utils::display(p);
        for pattern in self.patterns {
            stringy = pattern.pattern.replace(&stringy, pattern.format).into_owned();
        }
        stringy.into()
    }
}

pub struct ExpandTransformActor {
    format: String,
}


impl<'a> SeqAct<'a> for ExpandTransformActor {
    type BuildParam = ();
    type Config = ExpandTransformAction;

    fn build(config: &'a Self::Config, _: ()) -> Result<Self, ActError> {
        Ok(
            Self {
                format: config.format.clone(),
            }
        )
    }

    fn run(self, p: Primitive) -> Primitive {
        let stringy = primitive_utils::display(p);
        let pattern1 = format!("${}", super::VALUE_VAR);
        let pattern2 = format!("${{{}}}", super::VALUE_VAR);
        self.format.replace(&pattern1, &pattern2).replace(&pattern2, &stringy).into()
    }
}

pub struct LogTransformActor {
    log_level: LogLevel,
}

impl<'a> SeqAct<'a> for LogTransformActor {
    type BuildParam = ();
    type Config = LogTransformAction;

    fn build(config: &'a Self::Config, _: ()) -> Result<Self, ActError> {
        Ok(
            Self {
                log_level: config.level.clone()
            }
        )
    }

    fn run(self, p: Primitive) -> Primitive {
        let stringy = primitive_utils::debug(&p);
        match self.log_level {
            LogLevel::DEBUG => log::debug!("{}", stringy),
            LogLevel::INFO => log::info!("{}", stringy),
            LogLevel::WARN => log::warn!("{}", stringy),
            LogLevel::ERROR => log::error!("{}", stringy),
        }
        p
    }
}
