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

pub fn benchmark_addition(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Addition");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let j = rng.random_range(0..vectors.len());

                    let v0 = vectors[i];
                    let v1 = vectors[j];

                    (v0, v1)
                },
                |(v0, v1)| {
                    black_box(scalar::add(v0, v1));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let j = rng.random_range(0..vectors.len());

                    let v0 = vectors[i];
                    let v1 = vectors[j];

                    (v0, v1)
                },
                |(v0, v1)| {
                    black_box(sse2::add(v0, v1));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_component_multiplcation(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Multiplication (Component-Wise)");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let j = rng.random_range(0..vectors.len());

                    let v0 = vectors[i];
                    let v1 = vectors[j];

                    (v0, v1)
                },
                |(v0, v1)| {
                    black_box(scalar::component_mul(v0, v1));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let j = rng.random_range(0..vectors.len());

                    let v0 = vectors[i];
                    let v1 = vectors[j];

                    (v0, v1)
                },
                |(v0, v1)| {
                    black_box(sse2::component_mul(v0, v1));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_scalar_multiplcation(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Multiplication (Scalar)");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    let s: f32 = rng.random();

                    (v0, s)
                },
                |(v0, s)| {
                    black_box(scalar::mul(v0, s));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    let s: f32 = rng.random();

                    (v0, s)
                },
                |(v0, s)| {
                    black_box(sse2::mul(v0, s));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_division_numerator(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Division (V/scalar)");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    let s: f32 = rng.random();

                    (v0, s)
                },
                |(v0, s)| {
                    black_box(scalar::div_with_denominator(v0, s));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    let s: f32 = rng.random();

                    (v0, s)
                },
                |(v0, s)| {
                    black_box(sse2::div_with_denominator(v0, s));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_division_denominator(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Division (scalar/V)");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    let s: f32 = rng.random();

                    (v0, s)
                },
                |(v0, s)| {
                    black_box(scalar::div_with_numerator(s, v0));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    let s: f32 = rng.random();

                    (v0, s)
                },
                |(v0, s)| {
                    black_box(sse2::div_with_denominator(v0, s));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_dot(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Dot Product");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let j = rng.random_range(0..vectors.len());

                    let v0 = vectors[i];
                    let v1 = vectors[j];

                    (v0, v1)
                },
                |(v0, v1)| {
                    black_box(scalar::dot(v0, v1));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let j = rng.random_range(0..vectors.len());

                    let v0 = vectors[i];
                    let v1 = vectors[j];

                    (v0, v1)
                },
                |(v0, v1)| {
                    black_box(sse2::dot(v0, v1));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_magnitude(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Magnitude");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(scalar::magnitude(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(sse2::magnitude(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_magnitude_sqr(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Magnitude (Squared)");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(scalar::magnitude_sqr(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(sse2::magnitude_sqr(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_negate(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Negate");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(scalar::negate(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(sse2::negate(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn benchmark_normalize(c: &mut Criterion) {
    let mut rng = rand::rng();
    let vectors = vector_samples::<f32>(&mut rng);

    let mut group = c.benchmark_group("Vector4 Normalize");
    group.bench_function(
        "Scalar",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(scalar::normalize(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
    group.bench_function(
        "SSE2",
        |b| {
            b.iter_batched(
                || {
                    let i = rng.random_range(0..vectors.len());
                    let v0 = vectors[i];
                    v0
                },
                |v0| {
                    black_box(sse2::normalize(v0));
                },
                BatchSize::SmallInput
            )
        }
    );
}

criterion_group!(
    benches,
        benchmark_addition,
        benchmark_component_multiplcation,
        benchmark_scalar_multiplcation,
        benchmark_dot,
        benchmark_magnitude,
        benchmark_magnitude_sqr,
        benchmark_negate,
        benchmark_normalize
);
criterion_main!(benches);
