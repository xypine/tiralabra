use aaltofunktionromautus::{
    backtracking::{gradual_reset::BacktrackerByGradualReset, reset::BacktrackerByReset},
    grid::{constant_2d::ConstantSizeGrid2D, dynamic_2d::DynamicSizeGrid2D},
    utils::space::s2d::Location2D,
    wave_function_collapse::interface::WaveFunctionCollapse,
};
use criterion::{Criterion, black_box};
use criterion::{criterion_group, criterion_main};
use rand::{Rng, thread_rng};

fn eval_terrain_simple(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::terrain_simple::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules, black_box(0));
    let _ = grid.run::<BacktrackerByReset>(size, None);
}

fn eval_terrain(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::terrain::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules, black_box(0));
    let _ = grid.run::<BacktrackerByReset>(size * size, None);
}

fn eval_flowers(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::flowers_singlepixel::rules();

    let mut rng = thread_rng();
    let mut grid = DynamicSizeGrid2D::new(size, size, rules, black_box(rng.random()));
    let _ = grid.run::<BacktrackerByReset>(size * size, None);
}

fn eval_flowers_reset(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::flowers_singlepixel::rules();

    let mut rng = thread_rng();
    let mut grid = DynamicSizeGrid2D::new(size, size, rules, black_box(rng.random()));
    let mut b = BacktrackerByGradualReset::new(1);
    let _ = grid.run(size * size, Some(b));
}

fn eval_flowers_reset_gradual(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::flowers_singlepixel::rules();

    let mut rng = thread_rng();
    let mut grid = DynamicSizeGrid2D::new(size, size, rules, black_box(rng.random()));
    let mut b = BacktrackerByReset {};
    let _ = grid.run(size * size, Some(b));
}

pub fn benchmark_execution_terrain_simple(c: &mut Criterion) {
    c.bench_function("eval_terrain_simple 10", |b| {
        b.iter(|| {
            eval_terrain_simple(black_box(10));
        })
    });
    c.bench_function("eval_terrain_simple 20", |b| {
        b.iter(|| {
            eval_terrain_simple(black_box(20));
        })
    });
    c.bench_function("eval_terrain_simple 50", |b| {
        b.iter(|| {
            eval_terrain_simple(black_box(50));
        })
    });
    c.bench_function("eval_terrain_simple 75", |b| {
        b.iter(|| {
            eval_terrain_simple(black_box(75));
        })
    });
    c.bench_function("eval_terrain_simple 100", |b| {
        b.iter(|| {
            eval_terrain_simple(black_box(100));
        })
    });
    c.bench_function("eval_terrain_simple 120", |b| {
        b.iter(|| {
            eval_terrain_simple(black_box(120));
        })
    });
}

pub fn benchmark_execution_terrainx(c: &mut Criterion) {
    c.bench_function("eval_terrainx 10", |b| {
        b.iter(|| {
            eval_terrain(black_box(10));
        })
    });
    c.bench_function("eval_terrainx 20", |b| {
        b.iter(|| {
            eval_terrain(black_box(20));
        })
    });
    c.bench_function("eval_terrainx 50", |b| {
        b.iter(|| {
            eval_terrain(black_box(50));
        })
    });
    c.bench_function("eval_terrainx 75", |b| {
        b.iter(|| {
            eval_terrain(black_box(75));
        })
    });
    c.bench_function("eval_terrainx 100", |b| {
        b.iter(|| {
            eval_terrain(black_box(100));
        })
    });
    c.bench_function("eval_terrainx 120", |b| {
        b.iter(|| {
            eval_terrain(black_box(120));
        })
    });
}

pub fn benchmark_execution_flowers(c: &mut Criterion) {
    c.bench_function("eval_flowers 10", |b| {
        b.iter(|| {
            eval_flowers(black_box(10));
        })
    });
    c.bench_function("eval_flowers 20", |b| {
        b.iter(|| {
            eval_flowers(black_box(20));
        })
    });
    c.bench_function("eval_flowers 50", |b| {
        b.iter(|| {
            eval_flowers(black_box(50));
        })
    });
    c.bench_function("eval_flowers 75", |b| {
        b.iter(|| {
            eval_flowers(black_box(75));
        })
    });
    c.bench_function("eval_flowers 100", |b| {
        b.iter(|| {
            eval_flowers(black_box(100));
        })
    });
    c.bench_function("eval_flowers 120", |b| {
        b.iter(|| {
            eval_flowers(black_box(120));
        })
    });
}

pub fn benchmark_execution_flowers_reset(c: &mut Criterion) {
    c.bench_function("eval_flowers_reset 10", |b| {
        b.iter(|| {
            eval_flowers_reset(black_box(10));
        })
    });
    c.bench_function("eval_flowers_reset 20", |b| {
        b.iter(|| {
            eval_flowers_reset(black_box(20));
        })
    });
    c.bench_function("eval_flowers_reset 50", |b| {
        b.iter(|| {
            eval_flowers_reset(black_box(50));
        })
    });
    c.bench_function("eval_flowers_reset 75", |b| {
        b.iter(|| {
            eval_flowers_reset(black_box(75));
        })
    });
    c.bench_function("eval_flowers_reset 100", |b| {
        b.iter(|| {
            eval_flowers_reset(black_box(100));
        })
    });
    c.bench_function("eval_flowers_reset 120", |b| {
        b.iter(|| {
            eval_flowers_reset(black_box(120));
        })
    });
}

pub fn benchmark_execution_flowers_reset_gradual(c: &mut Criterion) {
    c.bench_function("eval_flowers_reset_gradual 10", |b| {
        b.iter(|| {
            eval_flowers_reset_gradual(black_box(10));
        })
    });
    c.bench_function("eval_flowers_reset_gradual 20", |b| {
        b.iter(|| {
            eval_flowers_reset_gradual(black_box(20));
        })
    });
    c.bench_function("eval_flowers_reset_gradual 50", |b| {
        b.iter(|| {
            eval_flowers_reset_gradual(black_box(50));
        })
    });
    c.bench_function("eval_flowers_reset_gradual 75", |b| {
        b.iter(|| {
            eval_flowers_reset_gradual(black_box(75));
        })
    });
    c.bench_function("eval_flowers_reset_gradual 100", |b| {
        b.iter(|| {
            eval_flowers_reset_gradual(black_box(100));
        })
    });
    c.bench_function("eval_flowers_reset_gradual 120", |b| {
        b.iter(|| {
            eval_flowers_reset_gradual(black_box(120));
        })
    });
}

criterion_group!(
    eval,
    benchmark_execution_terrain_simple,
    benchmark_execution_terrainx,
    benchmark_execution_flowers,
    benchmark_execution_flowers_reset,
    benchmark_execution_flowers_reset_gradual,
);
criterion_main!(eval);
