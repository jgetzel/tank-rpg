pub trait StateMachine {
    type State: Copy + PartialEq;
    type Params<'a>: 'a;

    fn update<'a>(&mut self, params: Self::Params<'a>);
}

pub trait StateBehavior<S: StateMachine>: Send + Sync {
    fn update<'a>(&self, params: S::Params<'a>) -> Option<S::State>;
}
