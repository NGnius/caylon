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

#[cfg(test)]
pub struct SeqActTestHarness<'a, ParamT, ConfigT, ActorT, FnT>
where
    ConfigT: 'a,
    ActorT: SeqAct<'a, BuildParam=ParamT, Config=ConfigT>,
    FnT: Fn(&'a ConfigT, ParamT) -> Result<ActorT, ActError>,
{
    actor_factory: FnT,
    inputs: Vec<(&'a ConfigT, ParamT, Primitive)>,
    outputs: std::collections::VecDeque<Expected>,
}

pub enum Expected {
    Output(Primitive),
    BuildErr(ActError),
    OutputIdc,
}

#[cfg(test)]
impl<'a, ParamT, ConfigT, ActorT, FnT> SeqActTestHarness<'a, ParamT, ConfigT, ActorT, FnT>
where
    ConfigT: 'a,
    ActorT: SeqAct<'a, BuildParam=ParamT, Config=ConfigT>,
    FnT: Fn(&'a ConfigT, ParamT) -> Result<ActorT, ActError>,
{
    #[allow(dead_code)]
    pub fn new(factory: FnT, inputs_vars: Vec<(&'a ConfigT, ParamT, Primitive)>, output_vars: std::collections::VecDeque<Expected>) -> Self {
        Self {
            actor_factory: factory,
            inputs: inputs_vars,
            outputs: output_vars,
        }
    }

    pub fn builder(factory: FnT) -> Self {
        Self {
            actor_factory: factory,
            inputs: Vec::new(),
            outputs: std::collections::VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_input(mut self, conf: &'a ConfigT, build_param: ParamT, run_param: Primitive) -> Self {
        self.inputs.push((conf, build_param, run_param));
        self
    }

    #[allow(dead_code)]
    pub fn expect(mut self, expected: Expected) -> Self {
        self.outputs.push_back(expected);
        self
    }

    pub fn with_io(mut self, input: (&'a ConfigT, ParamT, Primitive), output: Expected) -> Self {
        self.inputs.push(input);
        self.outputs.push_back(output);
        self
    }

    pub fn run(mut self) {
        for input in self.inputs {
            let expected = self.outputs.pop_front().expect("Not enough outputs for available inputs");
            let actor_result = (self.actor_factory)(input.0, input.1);
            match expected {
                Expected::Output(primitive) => {
                    let debug_prim = crate::runtime::primitive_utils::debug(&input.2);
                    let actual = actor_result.expect("Expected Ok actor").run(input.2);
                    let debug_actual = crate::runtime::primitive_utils::debug(&actual);
                    match primitive {
                        Primitive::Empty => if let Primitive::Empty = actual {
                            // good!
                        } else {
                            panic!("Expected SeqAct.run({}) to output Empty, got `{}`", debug_prim, debug_actual);
                        },
                        Primitive::String(s) => if let Primitive::String(actual) = actual {
                            assert_eq!(actual, s, "Expected SeqAct.run({}) to output Primitive::String(`{}`)", debug_prim, s);
                        } else {
                            panic!("Expected SeqAct.run({}) to output Primitive::String(`{}`), got `{}`", debug_prim, s, debug_actual);
                        },
                        Primitive::Json(j) => if let Primitive::Json(actual) = actual {
                            assert_eq!(actual, j, "Expected SeqAct.run({}) to output Primitive::Json(`{}`)", debug_prim, j);
                        } else {
                            panic!("Expected SeqAct.run({}) to output Primitive::Json(`{}`), got `{}`", debug_prim, j, debug_actual);
                        },
                        _ => todo!("NGNIUS!!! Complete your damn test harness!!"),
                    }
                },
                Expected::BuildErr(err) => {
                    if let Err(actual) = actor_result {
                        assert_eq!(actual, err, "Expected and actual build error do not match");
                    } else {
                        panic!("Expected build error `{}`, but result was ok", err);
                    }
                },
                Expected::OutputIdc => {
                    actor_result.expect("Expected Ok actor").run(input.2);
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::*;

    #[test]
    fn full_actor_test() {
        let (runtime_io, _result_rx, _js_rx) = crate::runtime::RuntimeIO::mock();
        SeqActTestHarness::builder(Actor::build)
            .with_io(
                (&ElementConfig::Button(ButtonConfig {
                    title: "Test Button".into(),
                    on_click: TopLevelActionConfig::Mirror(MirrorAction)
                }),
                (0, &runtime_io),
                Primitive::Empty),

                Expected::Output(Primitive::Empty))
            .run();
    }

    #[test]
    fn top_level_actor_test() {
        let (runtime_io, _result_rx, _js_rx) = crate::runtime::RuntimeIO::mock();
        SeqActTestHarness::builder(TopLevelActorType::build)
            .with_io(
                (&TopLevelActionConfig::Mirror(MirrorAction), &runtime_io, Primitive::Empty), Expected::Output(Primitive::Empty))
            .run();
    }

    #[test]
    fn std_actor_test() {
        let (runtime_io, _result_rx, _js_rx) = crate::runtime::RuntimeIO::mock();
        SeqActTestHarness::builder(ActorType::build)
            .with_io(
                (&ActionConfig::Transform(TransformAction{
                    transformer: TransformTypeAction::Log(LogTransformAction{level: LogLevel::DEBUG})}),
                &runtime_io,
                Primitive::String("Test log output".into())),

                Expected::OutputIdc)
            .run();
    }
}
