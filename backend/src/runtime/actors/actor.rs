use usdpl_back::core::serdes::Primitive;

use crate::config::{ElementConfig, ActionConfig, TopLevelActionConfig};

pub type ActError = String;

/// Something capable of performing an action.
pub trait Act<'a>: Sized + 'a {
    type Param;
    type Config: ?Sized + 'a;
    type Return;
    fn build(config: &'a Self::Config, parameter: Self::Param) -> Result<Self, ActError>;
    fn run(self) -> Self::Return;
}

/// Action performer for a regular element
pub struct Actor {
    actor_type: TopLevelActorType,
    index: usize,
}

impl<'a> SeqAct<'a> for Actor {
    type BuildParam = (usize, &'a crate::runtime::RuntimeIO);
    type Config = ElementConfig;

    fn build(config: &'a ElementConfig, parameter: Self::BuildParam) -> Result<Self, ActError> {
        let i = parameter.0;
        let a_type = match config {
            ElementConfig::Button(b) => TopLevelActorType::build(&b.on_click, parameter.1),
            ElementConfig::Toggle(t) => TopLevelActorType::build(&t.on_toggle, parameter.1),
            ElementConfig::Slider(s) => TopLevelActorType::build(&s.on_set, parameter.1),
            ElementConfig::ReadingDisplay(r) => TopLevelActorType::build(&r.on_period, parameter.1),
            ElementConfig::ResultDisplay(_) => Err(format!("Item #{} is a ResultDisplay, which can't act", i)),
        }?;
        Ok(Self {
            actor_type: a_type,
            index: i,
        })
    }

    fn run(self, parameter: Primitive) -> Primitive {
        log::info!("Running act for item {}", self.index);
        let result = self.actor_type.run(parameter);
        log::info!("Completed act for item {}", self.index);
        result
    }
}

pub enum TopLevelActorType {
    Command(super::CommandActor),
    Transform(super::TransformActor),
    Mirror,
    Sequence(super::SequenceActor),
    Javascript(super::JavascriptActor),
    Json(super::JsonActor),
}

impl<'a> SeqAct<'a> for TopLevelActorType {
    type BuildParam = &'a crate::runtime::RuntimeIO;
    type Config = TopLevelActionConfig;

    fn build(config: &'a Self::Config, parameter: Self::BuildParam) -> Result<Self, ActError> {
        Ok(match config {
            TopLevelActionConfig::Sequence(s) =>
                Self::Sequence(super::SequenceActor::build(s, parameter)?),
            TopLevelActionConfig::Command(c) =>
                Self::Command(super::CommandActor::build(c, ())?),
            TopLevelActionConfig::Transform(t) =>
                Self::Transform(super::TransformActor::build(t, ())?),
            TopLevelActionConfig::Mirror(_) =>
                Self::Mirror,
            TopLevelActionConfig::Javascript(j) =>
                Self::Javascript(super::JavascriptActor::build(j, parameter)?),
            TopLevelActionConfig::Json(j) =>
                Self::Json(super::JsonActor::build(j, ())?),
        })
    }

    fn run(self, p: Primitive) -> Primitive {
        match self {
            Self::Command(c) => c.run(p),
            Self::Transform(t) => t.run(p),
            Self::Mirror => p,
            Self::Sequence(s) => s.run(p),
            Self::Javascript(j) => j.run(p),
            Self::Json(j) => j.run(p),
        }
    }
}

pub enum ActorType {
    Command(super::CommandActor),
    Transform(super::TransformActor),
    Javascript(super::JavascriptActor),
    Json(super::JsonActor),
}

impl<'a> SeqAct<'a> for ActorType {
    type BuildParam = &'a crate::runtime::RuntimeIO;
    type Config = ActionConfig;

    fn build(config: &'a Self::Config, parameter: Self::BuildParam) -> Result<Self, ActError> {
        Ok(match config {
            ActionConfig::Command(c) =>
                Self::Command(super::CommandActor::build(c, ())?),
            ActionConfig::Transform(t) =>
                Self::Transform(super::TransformActor::build(t, ())?),
            ActionConfig::Javascript(j) =>
                Self::Javascript(super::JavascriptActor::build(j, parameter)?),
            ActionConfig::Json(j) =>
                Self::Json(super::JsonActor::build(j, ())?),
        })
    }

    fn run(self, p: Primitive) -> Primitive {
        match self {
            Self::Command(c) => c.run(p),
            Self::Transform(t) => t.run(p),
            Self::Javascript(j) => j.run(p),
            Self::Json(j) => j.run(p),
        }
    }
}

/// Something capable of performing an action in a sequence.
pub trait SeqAct<'a>: Sized + 'a {
    type BuildParam;
    type Config: ?Sized + 'a;
    fn build(config: &'a Self::Config, bp: Self::BuildParam) -> Result<Self, ActError>;
    fn run(self, parameter: Primitive) -> Primitive;
}

pub struct SeqActor<'a, X: SeqAct<'a, BuildParam=()>> {
    param: Primitive,
    seq_act: X,
    _l: std::marker::PhantomData<&'a ()>,
}

impl<'a, X: SeqAct<'a, BuildParam=()>> Act<'a> for SeqActor<'a, X> {
    type Param = Primitive;
    type Config = <X as SeqAct<'a>>::Config;
    type Return = Primitive;

    fn build(config: &'a Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        Ok(
            Self {
                param: parameter,
                seq_act: X::build(config, ())?,
                _l: Default::default()
            }
        )
    }

    fn run(self) -> Self::Return {
        self.seq_act.run(self.param)
    }
}
