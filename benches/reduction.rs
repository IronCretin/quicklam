use criterion::{black_box, criterion_group, criterion_main, Criterion};

use quicklam::parse;

fn church(b: &mut Criterion) {
    let exp = parse(r"(\f x. (f (f x))) (\f x. f (f (f x))) S Z").unwrap();
    b.bench_function("3^2", |b| {
        b.iter(|| {
            let mut exp = exp.clone();
            exp.reduce_all();
            black_box(exp)
        })
    });

    let exp = parse(r"(\f x. f (f (f (f x)))) (\f x. f (f (f (f x)))) S Z").unwrap();
    b.bench_function("4^4", |b| {
        b.iter(|| {
            let mut exp = exp.clone();
            exp.reduce_all();
            black_box(exp)
        })
    });

    let exp = parse(r"(\S K. S K K x) (\x y z. x z (y z)) (\x y. x)").unwrap();
    b.bench_function("SKI", |b| {
        b.iter(|| {
            let mut exp = exp.clone();
            exp.reduce_all();
            black_box(exp)
        })
    });
}

criterion_group!(benches, church);
criterion_main!(benches);
