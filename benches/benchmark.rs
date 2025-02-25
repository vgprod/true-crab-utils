use criterion::{black_box, criterion_group, criterion_main, Criterion};
use true_crab_utils::data_structures::array;

fn bench_array_operations(c: &mut Criterion) {
    let mut arr = black_box(vec![2, 4, 10]);

    c.bench_function("reverse", |b| b.iter(|| array::reverse(&mut arr)));
    c.bench_function("insert", |b| b.iter(|| array::insert(&mut arr, 100, 1)));
    c.bench_function("merge", |b| {
        let arr1 = black_box(vec![2, 4]);
        let arr2 = black_box(vec![3, 1]);
        b.iter(|| array::merge_and_sort(&arr1, &arr2));
    });
}

criterion_group!(benches, bench_array_operations);
criterion_main!(benches);
