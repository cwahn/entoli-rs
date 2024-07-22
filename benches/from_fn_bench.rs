use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn from_fn_bench(c: &mut Criterion) {
    c.bench_function("repeat_with + take", |b| {
        b.iter(|| {
            let mut counter = 0;
            let iter = std::iter::repeat_with(move || {
                counter += 1;
                counter * 2
            })
            .take(10_000);

            for value in iter {
                black_box(value);
            }
        });
    });

    c.bench_function("from_fn", |b| {
        b.iter(|| {
            let mut counter = 0;
            let iter = std::iter::from_fn(move || {
                counter += 1;
                if counter < 10_000 {
                    Some(counter * 2)
                } else {
                    None
                }
            });

            for value in iter {
                black_box(value);
            }
        });
    });
}

// These two seems to have the same effect

criterion_group!(benches, from_fn_bench);
criterion_main!(benches);
