use aaltofunktionromautus::{
    grid::{constant_2d::ConstantSizeGrid2D, dynamic_2d::DynamicSizeGrid2D},
    interface::WaveFunctionCollapse,
    utils::space::Location2D,
};
use criterion::{Criterion, black_box};
use criterion::{criterion_group, criterion_main};

fn eval_terrain_simple(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::terrain_simple::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules);
    let _ = grid.run(size * size);
}

fn eval_terrain(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::terrain::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules);
    let _ = grid.run(size * size);
}

fn eval_flowers(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::flowers_singlepixel::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules);
    let _ = grid.collapse(
        Location2D { x: 0, y: size - 1 },
        Some(aaltofunktionromautus::rules::samples::flowers_singlepixel::STATE_GROUND),
    );
    let _ = grid.run(size * size);
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

criterion_group!(
    eval,
    benchmark_execution_terrain_simple,
    benchmark_execution_terrainx,
    benchmark_execution_flowers
);
criterion_main!(eval);
