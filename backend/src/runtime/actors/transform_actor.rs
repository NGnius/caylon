use regex::{Regex, RegexBuilder};
use usdpl_back::core::serdes::Primitive;

use crate::runtime::primitive_utils;

use crate::config::{TransformAction, ActionConfig, TransformTypeAction, ReplaceTransformAction, ExpandTransformAction, LogTransformAction, LogLevel, PatternConfig};
use super::{Act, ActError, ActorType};

/// Changes the output or input of an act
pub enum TransformActor {
    PreReplace(PreReplaceTransformActor),
    PostReplace(PostReplaceTransformActor),
    PreExpand(PreExpandTransformActor),
    PostExpand(PostExpandTransformActor),
    Log(LogTransformActor),
}

impl<'a> Act<'a> for TransformActor {
    type Param = Primitive;
    type Config = TransformAction;
    type Return = Primitive;

    fn build(config: &'a TransformAction, parameter: Primitive) -> Result<Self, ActError> {
        let result = Ok(match &config.transformer {
            TransformTypeAction::PreReplace(x) =>
                Self::PreReplace(PreReplaceTransformActor::build(&(x, &config.target), parameter)?),
            TransformTypeAction::PostReplace(x) =>
                Self::PostReplace(PostReplaceTransformActor::build(&(x, &config.target), parameter)?),
            TransformTypeAction::PreExpand(x) =>
                Self::PreExpand(PreExpandTransformActor::build(&(x, &config.target), parameter)?),
            TransformTypeAction::PostExpand(x) =>
                Self::PostExpand(PostExpandTransformActor::build(&(x, &config.target), parameter)?),
            TransformTypeAction::Log(x) =>
                Self::Log(LogTransformActor::build(&(x, &config.target), parameter)?),
        });
        result
    }

    fn run(self) -> Self::Return {
        match self {
            Self::PreReplace(x) => x.run(),
            Self::PostReplace(x) => x.run(),
            Self::PreExpand(x) => x.run(),
            Self::PostExpand(x) => x.run(),
            Self::Log(x) => x.run()
        }
    }
}

pub(super) struct TransformPostActor {
    op_fn: Box<dyn (FnOnce(Primitive) -> Primitive) + Send>,
    actor: Box<ActorType>,
}

impl<'a> Act<'a> for TransformPostActor {
    type Param = (Primitive, Box<dyn (FnOnce(Primitive) -> Primitive) + Send>);
    type Config = ActionConfig;
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        Ok(
            Self {
                op_fn: parameter.1,
                actor: Box::new(ActorType::build(config, parameter.0)?),
            }
        )
    }

    fn run(self) -> Self::Return {
        (self.op_fn)(self.actor.run())
    }
}

/// executes op_fn and ActionConfig::build in Actor::build()
/// this blocks the main execution thread,
/// but an Err() from ActionConfig::build will be be propogated correctly
pub(super) struct TransformEagerPreActor {
    actor: Box<ActorType>,
}

impl<'a> Act<'a> for TransformEagerPreActor {
    type Param = (Primitive, Box<dyn (FnOnce(Primitive) -> Primitive) + Send>);
    type Config = ActionConfig;
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        let primitive = (parameter.1)(parameter.0);
        Ok(
            Self {
                actor: Box::new(ActorType::build(config, primitive)?),
            }
        )
    }

    fn run(self) -> Self::Return {
        self.actor.run()
    }
}

/// executes op_fn and ActionConfig::build in Actor.run()
/// this doesn't block the main execution thread,
/// but an Err() from ActionConfig::build will produce an empty result
pub(super) struct TransformLazyPreActor {
    op_fn: Box<dyn (FnOnce(Primitive) -> Primitive) + Send>,
    action: ActionConfig,
    primitive: Primitive,
}

impl<'a> Act<'a> for TransformLazyPreActor {
    type Param = (Primitive, Box<dyn (FnOnce(Primitive) -> Primitive) + Send>);
    type Config = ActionConfig;
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        Ok(
            Self {
                op_fn: parameter.1,
                action: config.to_owned(),
                primitive: parameter.0,
            }
        )
    }

    fn run(self) -> Self::Return {
        let primitive = (self.op_fn)(self.primitive);
        match ActorType::build(&self.action, primitive) {
            Ok(action) => action.run(),
            Err(e) => {
                log::error!("Failed to lazily build action for pre-transformer: {}", e);
                Primitive::Empty
            }
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

fn replace_fn(config: &ReplaceTransformAction) -> Result<impl FnOnce(Primitive) -> Primitive, ActError> {
    let mut patterns: Vec<PatternRule> = Vec::with_capacity(config.patterns.len());
    for pattern in config.patterns.iter() {
        patterns.push(PatternRule::from_config(pattern)?);
    }

    Ok(move |p| {
        let mut stringy = primitive_utils::display(p);
        for pattern in patterns {
            stringy = pattern.pattern.replace(&stringy, pattern.format).into_owned();
        }
        stringy.into()
    })
}

pub struct PreReplaceTransformActor {
    transformer: TransformEagerPreActor,
}

impl<'a> Act<'a> for PreReplaceTransformActor {
    type Param = Primitive;
    type Config = (&'a ReplaceTransformAction, &'a ActionConfig);
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Primitive) -> Result<Self, ActError> {
        Ok(
            Self {
                transformer: TransformEagerPreActor::build(
                    config.1,
                    (parameter,
                    Box::new(replace_fn(config.0)?)
                    ))?,
            }
        )
    }

    fn run(self) -> Self::Return {
        self.transformer.run()
    }
}

pub struct PostReplaceTransformActor {
    transformer: TransformPostActor,
}

impl<'a> Act<'a> for PostReplaceTransformActor {
    type Param = Primitive;
    type Config = (&'a ReplaceTransformAction, &'a ActionConfig);
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Primitive) -> Result<Self, ActError> {

        Ok(
            Self {
                transformer: TransformPostActor::build(
                    config.1,
                    (parameter,
                    Box::new(replace_fn(config.0)?)
                    ))?,
            }
        )
    }

    fn run(self) -> Self::Return {
        self.transformer.run()
    }
}

fn expand_fn(config: &ExpandTransformAction) -> Result<impl FnOnce(Primitive) -> Primitive, ActError> {
    let format = config.format.clone();
    Ok(move |p| {
        let stringy = primitive_utils::display(p);
        let pattern1 = format!("${}", super::VALUE_VAR);
        let pattern2 = format!("${{{}}}", super::VALUE_VAR);
        format.replace(&pattern1, &pattern2).replace(&pattern2, &stringy).into()
    })
}

pub struct PreExpandTransformActor {
    transformer: TransformLazyPreActor,
}


impl<'a> Act<'a> for PreExpandTransformActor {
    type Param = Primitive;
    type Config = (&'a ExpandTransformAction, &'a ActionConfig);
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Primitive) -> Result<Self, ActError> {
        Ok(
            Self {
                transformer: TransformLazyPreActor::build(
                    config.1,
                    (parameter,
                    Box::new(expand_fn(config.0)?)
                    ))?,
            }
        )
    }

    fn run(self) -> Self::Return {
        self.transformer.run()
    }
}

pub struct PostExpandTransformActor {
    transformer: TransformPostActor,
}


impl<'a> Act<'a> for PostExpandTransformActor {
    type Param = Primitive;
    type Config = (&'a ExpandTransformAction, &'a ActionConfig);
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Primitive) -> Result<Self, ActError> {
        Ok(
            Self {
                transformer: TransformPostActor::build(
                    config.1,
                    (parameter,
                    Box::new(expand_fn(config.0)?)
                    ))?,
            }
        )
    }

    fn run(self) -> Self::Return {
        self.transformer.run()
    }
}

pub struct LogTransformActor {
    generic: TransformPostActor,
}

impl<'a> Act<'a> for LogTransformActor {
    type Param = Primitive;
    type Config = (&'a LogTransformAction, &'a ActionConfig);
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Primitive) -> Result<Self, ActError> {
        Ok(
            Self {
                generic: TransformPostActor::build(
                    config.1,
                    (parameter,
                        match config.0.level {
                            LogLevel::DEBUG => Box::new(|p| {log::debug!("{}", primitive_utils::debug(&p));p}),
                            LogLevel::INFO => Box::new(|p| {log::info!("{}", primitive_utils::debug(&p));p}),
                            LogLevel::WARN => Box::new(|p| {log::warn!("{}", primitive_utils::debug(&p));p}),
                            LogLevel::ERROR => Box::new(|p| {log::error!("{}", primitive_utils::debug(&p));p}),
                        }
                    ))?,
            }
        )
    }

    fn run(self) -> Self::Return {
        self.generic.run()
    }
}
