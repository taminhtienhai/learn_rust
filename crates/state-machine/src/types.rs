pub(crate) trait NextState {
    type State;

    fn next(&self, state: Self::State);
}

pub(crate) trait NextStateContext {
    type State;
    type Context: Default;

    fn next(&self, state: Self::State, context: Self::Context);
}