use criterion::{black_box, criterion_group, criterion_main, Criterion};
use entoli::base::misc::in_place;

fn in_place_bench(c: &mut Criterion) {
    // Generate a large vector
    let large_vec: Vec<i32> = (0..10_000).collect();

    // Heavier computation for allocation-related case
    c.bench_function("manual_in_place (allocation)", |b| {
        let f = |v: &mut Vec<i32>| {
            v.iter_mut().for_each(|x| *x = *x * 2 + 3 - 1 * 5 / 2);
        };
        let mut data = large_vec.clone();

        b.iter(|| {
            f(&mut data);
            black_box(&data);
        });
    });

    c.bench_function("in_place (allocation)", |b| {
        let f =
            |v: Vec<i32>| -> Vec<i32> { v.into_iter().map(|x| x * 2 + 3 - 1 * 5 / 2).collect() };
        let mut data = large_vec.clone();
        let in_place_f = in_place(f);

        b.iter(|| {
            in_place_f(&mut data);
            black_box(&data);
        });
    });

    c.bench_function("non_in_place (allocation)", |b| {
        let f =
            |v: Vec<i32>| -> Vec<i32> { v.into_iter().map(|x| x * 2 + 3 - 1 * 5 / 2).collect() };
        let data = large_vec.clone();

        b.iter(|| {
            let new_data = f(data.clone());
            black_box(&new_data);
        });
    });

    // Heavier computation for non-allocation-related case
    c.bench_function("manual_in_place (non-allocation)", |b| {
        let f = |v: &mut i32| {
            *v = *v * 2 + 3 - 1 * 5 / 2;
        };
        let mut data = 5;

        b.iter(|| {
            f(&mut data);
            black_box(&data);
        });
    });

    c.bench_function("in_place (non-allocation)", |b| {
        let f = |v: i32| -> i32 { v * 2 + 3 - 1 * 5 / 2 };
        let mut data = 5;
        let in_place_f = in_place(f);

        b.iter(|| {
            in_place_f(&mut data);
            black_box(&data);
        });
    });

    c.bench_function("non_in_place (non-allocation)", |b| {
        let f = |v: i32| -> i32 { v * 2 + 3 - 1 * 5 / 2 };
        let data = 5;

        b.iter(|| {
            let new_data = f(data);
            black_box(&new_data);
        });
    });
}

criterion_group!(benches, in_place_bench);
criterion_main!(benches);
