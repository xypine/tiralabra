use aaltofunktionromautus::{
    grid::{constant_2d::ConstantSizeGrid2D, dynamic_2d::DynamicSizeGrid2D},
    interface::WaveFunctionCollapse,
    utils::space::Location2D,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn propagate_checkers(size: usize) {
    use aaltofunktionromautus::rules::samples::checkers::STATE_BLACK;
    let rules = aaltofunktionromautus::rules::samples::checkers::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules);
    let _ = grid.collapse(Location2D { x: 0, y: 0 }, Some(STATE_BLACK));
}

fn execute_terrain_simple(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::terrain_simple::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules);
    let _ = grid.run(size * size);
}

fn execute_terrain(size: usize) {
    let rules = aaltofunktionromautus::rules::samples::terrain::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules);
    let _ = grid.run(size * size);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("propagate 10", |b| {
        b.iter(|| {
            propagate_checkers(black_box(10));
        })
    });
    c.bench_function("propagate 20", |b| {
        b.iter(|| {
            propagate_checkers(black_box(20));
        })
    });
    c.bench_function("propagate 50", |b| {
        b.iter(|| {
            propagate_checkers(black_box(50));
        })
    });
    c.bench_function("propagate 75", |b| {
        b.iter(|| {
            propagate_checkers(black_box(75));
        })
    });
    c.bench_function("propagate 100", |b| {
        b.iter(|| {
            propagate_checkers(black_box(100));
        })
    });
    c.bench_function("propagate 120", |b| {
        b.iter(|| {
            propagate_checkers(black_box(120));
        })
    });

    c.bench_function("execute_terrain_simple 10", |b| {
        b.iter(|| {
            execute_terrain_simple(black_box(10));
        })
    });
    c.bench_function("execute_terrain_simple 20", |b| {
        b.iter(|| {
            execute_terrain_simple(black_box(20));
        })
    });
    c.bench_function("execute_terrain_simple 50", |b| {
        b.iter(|| {
            execute_terrain_simple(black_box(50));
        })
    });
    c.bench_function("execute_terrain_simple 75", |b| {
        b.iter(|| {
            execute_terrain_simple(black_box(75));
        })
    });
    c.bench_function("execute_terrain_simple 100", |b| {
        b.iter(|| {
            execute_terrain_simple(black_box(100));
        })
    });
    c.bench_function("execute_terrain_simple 120", |b| {
        b.iter(|| {
            execute_terrain_simple(black_box(120));
        })
    });

    c.bench_function("execute_terrainx 10", |b| {
        b.iter(|| {
            execute_terrain(black_box(10));
        })
    });
    c.bench_function("execute_terrainx 20", |b| {
        b.iter(|| {
            execute_terrain(black_box(20));
        })
    });
    c.bench_function("execute_terrainx 50", |b| {
        b.iter(|| {
            execute_terrain(black_box(50));
        })
    });
    c.bench_function("execute_terrainx 75", |b| {
        b.iter(|| {
            execute_terrain(black_box(75));
        })
    });
    c.bench_function("execute_terrainx 100", |b| {
        b.iter(|| {
            execute_terrain(black_box(100));
        })
    });
    c.bench_function("execute_terrainx 120", |b| {
        b.iter(|| {
            execute_terrain(black_box(120));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
