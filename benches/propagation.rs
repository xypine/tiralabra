use aaltofunktionromautus::{
    grid::{constant_2d::ConstantSizeGrid2D, dynamic_2d::DynamicSizeGrid2D},
    utils::space::s2d::Location2D,
    wave_function_collapse::interface::WaveFunctionCollapse,
};
use criterion::{Criterion, black_box};
use criterion::{criterion_group, criterion_main};

fn propagate_checkers(size: usize) {
    use aaltofunktionromautus::rules::samples::checkers::STATE_BLACK;
    let rules = aaltofunktionromautus::rules::samples::checkers::rules();

    let mut grid = DynamicSizeGrid2D::new(size, size, rules, black_box(0));
    let _ = grid.collapse(
        black_box(Location2D { x: 0, y: 0 }),
        black_box(Some(STATE_BLACK)),
    );
}

pub fn benchmark_propagation(c: &mut Criterion) {
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
}

criterion_group!(prop, benchmark_propagation);
criterion_main!(prop);
