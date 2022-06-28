use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tests::create_factorial_virtual_machine;

#[inline]
fn factorial(n: u16) -> i64 {
    let mut vm = create_factorial_virtual_machine();
    vm.iinput()[0] = n as i64;
    vm.run(1_000_000);
    vm.ioutput()[0]
}

fn factorial_10(c: &mut Criterion) {
    c.bench_function("factorial 10", |b| b.iter(|| factorial(black_box(10))));
}

fn factorial_20(c: &mut Criterion) {
    c.bench_function("factorial 20", |b| b.iter(|| factorial(black_box(20))));
}

criterion_group!(benches, factorial_10, factorial_20);
criterion_main!(benches);
