use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone)]
struct State {
    data: Vec<i32>,
    stack_data: [i32; 1024],  // Significant stack size
}

#[inline(always)]
pub fn in_place_state_machine<S, I, O, F>(f: F) -> impl FnMut(&mut S, I) -> O
where
    F: Fn(S, I) -> (S, O),
{
    move |state: &mut S, input: I| unsafe {
        let (new_state, output) = f(std::ptr::read(state), input);
        std::ptr::write(state, new_state);
        output
    }
}

// Hand-coded mutable state machine
fn hand_coded_mutable_state_machine(state: &mut State, input: i32) -> i32 {
    state.data.push(input);
    let sum: i32 = state.data.iter().sum();
    state.data.iter_mut().for_each(|x| *x += input);
    state.stack_data.iter_mut().for_each(|x| *x += input);
    sum
}

// Pure functional state machine
fn pure_functional_state_machine(mut state: State, input: i32) -> (State, i32) {
    state.data.push(input);
    let sum: i32 = state.data.iter().sum();
    state.data.iter_mut().for_each(|x| *x += input);
    state.stack_data.iter_mut().for_each(|x| *x += input);
    (state, sum)
}

fn benchmark_in_place_state_machine(c: &mut Criterion) {
    let initial_state = State {
        data: vec![1; 1000],
        stack_data: [1; 1024],
    };
    let mut state = initial_state.clone();
    let mut sm = in_place_state_machine(pure_functional_state_machine);

    c.bench_function("in_place_state_machine", |b| {
        b.iter(|| {
            let input = black_box(5);
            sm(&mut state, input);
        })
    });
}

fn benchmark_hand_coded_mutable_state_machine(c: &mut Criterion) {
    let initial_state = State {
        data: vec![1; 1000],
        stack_data: [1; 1024],
    };
    let mut state = initial_state.clone();

    c.bench_function("hand_coded_mutable_state_machine", |b| {
        b.iter(|| {
            let input = black_box(5);
            hand_coded_mutable_state_machine(&mut state, input);
        })
    });
}

criterion_group!(
    benches,
    benchmark_in_place_state_machine,
    benchmark_hand_coded_mutable_state_machine
);
criterion_main!(benches);
