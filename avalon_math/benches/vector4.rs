use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use rand::prelude::*;
use avalon_math::Vector4;
use avalon_math::scalar::vector4 as scalar;
use avalon_math::sse2::vector4 as sse2;

fn vector_samples<T>(rng: &mut impl rand::Rng) -> Vec<Vector4<T>> where
    rand::distr::StandardUniform: Distribution<T> {
    static VECTOR_N: usize = 10_000_000;
    let mut vectors = vec![];
    vectors.resize_with(VECTOR_N, || {
        Vector4 {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
            w: rng.random(),
        }
    });
    vectors
}

macro_rules! pick_vector {
    ( $rng:expr, $vector_batch:expr, v0 ) => {
        || {
            let i = $rng.random_range(0..$vector_batch.len());
            let v0 = $vector_batch[i];
            v0
        }
    };
    ( $rng:expr, $vector_batch:expr, v0, s ) => {
        || {
            let i = $rng.random_range(0..$vector_batch.len());
            let v0 = $vector_batch[i];
            let s: f32 = $rng.random();
            (v0, s)
        }
    };
    ( $rng:expr, $vector_batch:expr, v0, v1 ) => {
        || {
            let i = $rng.random_range(0..$vector_batch.len());
            let j = $rng.random_range(0..$vector_batch.len());
            let v0 = $vector_batch[i];
            let v1 = $vector_batch[j];
            (v0, v1)
        }
    };
}

macro_rules! operate {
    ( $name:ident, $group:tt, $bench_group:tt, $rng:expr, $vectors:expr, $operator:ident, v0 ) => {
        $bench_group.bench_function(
            stringify!($name),
            |b| {
                b.iter_batched(
                    pick_vector!($rng, $vectors, v0),
                    |v0| {
                        black_box(unsafe { $group::$operator(v0) });
                    },
                    BatchSize::SmallInput
                )
            }
        );
    };
    ( $name:ident, $group:tt, $bench_group:tt, $rng:expr, $vectors:expr, $operator:tt, v0, v1 ) => {
        $bench_group.bench_function(
            stringify!($name),
            |b| {
                b.iter_batched(
                    pick_vector!($rng, $vectors, v0, v1),
                    |(v0, v1)| {
                        black_box(unsafe { $group::$operator(v0, v1) });
                    },
                    BatchSize::SmallInput
                )
            }
        );
    };
    ( $name:ident, $group:tt, $bench_group:tt, $rng:expr, $vectors:expr, $operator:tt, v0, s ) => {
        $bench_group.bench_function(
            stringify!($name),
            |b| {
                b.iter_batched(
                    pick_vector!($rng, $vectors, v0, s),
                    |(v0, s)| {
                        black_box(unsafe { $group::$operator(v0, s) });
                    },
                    BatchSize::SmallInput
                )
            }
        );
    };
    ( $name:ident, $group:tt, $bench_group:tt, $rng:expr, $vectors:expr, $operator:tt, s, v0 ) => {
        $bench_group.bench_function(
            stringify!($name),
            |b| {
                b.iter_batched(
                    pick_vector!($rng, $vectors, v0, s),
                    |(v0, s)| {
                        black_box(unsafe { $group::$operator(s, v0) });
                    },
                    BatchSize::SmallInput
                )
            }
        );
    };
}

macro_rules! benchmark {
    ( $function:tt, $operator:tt, v0 ) => {
        fn $function(c: &mut Criterion) {
            let mut rng = rand::rng();
            let vectors = vector_samples::<f32>(&mut rng);

            let mut group = c.benchmark_group(format!("Vector4 ({})", stringify!($operator)));
            operate!(Scalar, scalar, group, rng, vectors, $operator, v0);
            operate!(SSE2, sse2, group, rng, vectors, $operator, v0);
        }
    };
    ( $function:tt, $operator:tt, s, v0 ) => {
        fn $function(c: &mut Criterion) {
            let mut rng = rand::rng();
            let vectors = vector_samples::<f32>(&mut rng);

            let mut group = c.benchmark_group(format!("Vector4 ({})", stringify!($operator)));
            operate!(Scalar, scalar, group, rng, vectors, $operator, s, v0);
            operate!(SSE2, sse2, group, rng, vectors, $operator, s, v0);
        }
    };
    ( $function:tt, $operator:tt, v0, s ) => {
        fn $function(c: &mut Criterion) {
            let mut rng = rand::rng();
            let vectors = vector_samples::<f32>(&mut rng);

            let mut group = c.benchmark_group(format!("Vector4 ({})", stringify!($operator)));
            operate!(Scalar, scalar, group, rng, vectors, $operator, v0, s);
            operate!(SSE2, sse2, group, rng, vectors, $operator, v0, s);
        }
    };
    ( $function:tt, $operator:tt, v0, v1 ) => {
        fn $function(c: &mut Criterion) {
            let mut rng = rand::rng();
            let vectors = vector_samples::<f32>(&mut rng);

            let mut group = c.benchmark_group(format!("Vector4 ({})", stringify!($operator)));
            operate!(Scalar, scalar, group, rng, vectors, $operator, v0, v1);
            operate!(SSE2, sse2, group, rng, vectors, $operator, v0, v1);
        }
    };
}

benchmark!(benchmark_addition, add, v0, v1);
benchmark!(benchmark_component_multiplcation, component_mul, v0, v1);
benchmark!(benchmark_scalar_multiplcation, mul, v0, s);
benchmark!(benchmark_division_numerator, div_with_numerator, s, v0);
benchmark!(benchmark_division_denominator, div_with_denominator, v0, s);
benchmark!(benchmark_dot, dot, v0, v1);
benchmark!(benchmark_magnitude, magnitude, v0);
benchmark!(benchmark_magnitude_sqr, magnitude_sqr, v0);
benchmark!(benchmark_negate, negate, v0);
benchmark!(benchmark_normalize, normalize, v0);
benchmark!(benchmark_project, project, v0, v1);

criterion_group!(
    benches,
        benchmark_project,
        benchmark_normalize,
        benchmark_negate,
        benchmark_magnitude_sqr,
        benchmark_magnitude,
        benchmark_dot,
        benchmark_division_denominator,
        benchmark_division_numerator,
        benchmark_scalar_multiplcation,
        benchmark_component_multiplcation,
        benchmark_addition,
);
criterion_main!(benches);
