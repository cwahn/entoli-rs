use crate::base::misc::in_place;

pub trait State: Sized {
    type S;
    type Output;

    fn run_state(self, s: Self::S) -> (Self::Output, Self::S);

    fn exec_state(self, s: Self::S) -> Self::S {
        self.run_state(s).1
    }

    fn eval_state(self, s: Self::S) -> Self::Output {
        self.run_state(s).0
    }

    fn map<B, F>(self, f: F) -> StateMap<Self, F>
    where
        F: FnOnce(Self::Output) -> B,
    {
        StateMap { state: self, f }
    }

    fn pure(output: Self::Output) -> StatePure<Self::S, Self::Output> {
        StatePure {
            output,
            _phantom: std::marker::PhantomData,
        }
    }

    fn and_then<Mb, F>(self, f: F) -> StateBind<Self, F>
    where
        Mb: State<S = Self::S>,
        F: FnOnce(Self::Output) -> Mb,
    {
        StateBind { state: self, f }
    }

    fn then<Mb>(self, mb: Mb) -> StateBind<Self, impl FnOnce(Self::Output) -> Mb>
    where
        Mb: State<S = Self::S>,
    {
        self.and_then(|_| mb)
    }
}

pub struct StateMap<St, F> {
    state: St,
    f: F,
}

impl<St, B, F> State for StateMap<St, F>
where
    St: State,
    F: FnOnce(St::Output) -> B,
{
    type S = St::S;
    type Output = B;

    fn run_state(self, s: Self::S) -> (Self::Output, Self::S) {
        let (output, new_s) = self.state.run_state(s);
        ((self.f)(output), new_s)
    }
}

pub struct StatePure<S, O> {
    output: O,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, O> State for StatePure<S, O> {
    type S = S;
    type Output = O;

    fn run_state(self, s: Self::S) -> (Self::Output, Self::S) {
        (self.output, s)
    }
}

pub struct StateBind<St, F> {
    state: St,
    f: F,
}

impl<St, Mb, F> State for StateBind<St, F>
where
    St: State,
    Mb: State<S = St::S>,
    F: FnOnce(St::Output) -> Mb,
{
    type S = St::S;
    type Output = Mb::Output;

    fn run_state(self, s: Self::S) -> (Self::Output, Self::S) {
        let (output, new_s) = self.state.run_state(s);
        let new_state = (self.f)(output);
        new_state.run_state(new_s)
    }
}

#[allow(non_camel_case_types)]
pub struct get<S: Clone> {
    _phantom: std::marker::PhantomData<S>,
}

impl<S> State for get<S>
where
    S: Clone,
{
    type S = S;
    type Output = S;

    fn run_state(self, s: Self::S) -> (Self::Output, Self::S) {
        (s.clone(), s)
    }
}

pub struct StatePut<S> {
    new_state: S,
}

impl<S> State for StatePut<S> {
    type S = S;
    type Output = ();

    fn run_state(self, _: Self::S) -> (Self::Output, Self::S) {
        ((), self.new_state)
    }
}

pub struct StateModify<S, F> {
    f: F,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, F> State for StateModify<S, F>
where
    F: FnOnce(S) -> S,
{
    type S = S;
    type Output = ();

    fn run_state(self, mut s: Self::S) -> (Self::Output, Self::S) {
        unsafe {
            std::ptr::write(&mut s, (self.f)(std::ptr::read(&s)));
        }
        ((), s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_pure() {
        let state = StatePure::<i32, i32>::pure(5);
        assert_eq!(state.run_state(0), (5, 0));
    }

    #[test]
    fn test_state_map() {
        let state = StatePure::<i32, i32>::pure(5).map(|x| x * 2);
        assert_eq!(state.run_state(0), (10, 0));
    }

    #[test]
    fn test_state_and_then() {
        let state = StatePure::<i32, i32>::pure(5).and_then(|x| StatePure::pure(x * 2));
        assert_eq!(state.run_state(0), (10, 0));
    }

    #[test]
    fn test_state_then() {
        let state = StatePure::<i32, i32>::pure(5).then(StatePure::pure(10));
        assert_eq!(state.run_state(0), (10, 0));
    }

    #[test]
    fn test_state_get() {
        let state = get::<i32> {
            _phantom: std::marker::PhantomData,
        };
        assert_eq!(state.run_state(5), (5, 5));
    }

    #[test]
    fn test_state_put() {
        let state = StatePut { new_state: 10 };
        assert_eq!(state.run_state(5), ((), 10));
    }

    #[test]
    fn test_state_modify() {
        let state = StateModify {
            f: |x| x * 2,
            _phantom: std::marker::PhantomData,
        };
        assert_eq!(state.run_state(5), ((), 10));
    }
}
