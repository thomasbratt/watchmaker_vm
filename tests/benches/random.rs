use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tests::create_random_virtual_machine;

#[inline]
fn random(n: usize) {
    let mut vm = create_random_virtual_machine(n, 4);
    vm.run(100 * n);
}

fn random_4_096(c: &mut Criterion) {
    c.bench_function("random 4_096", |b| b.iter(|| random(black_box(4_096))));
}

fn random_256(c: &mut Criterion) {
    c.bench_function("random 256", |b| b.iter(|| random(black_box(256))));
}

criterion_group!(benches, random_256, random_4_096);
criterion_main!(benches);
