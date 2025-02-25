// benches/linked_list_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use true_crab_utils::data_structures::linked_list::LinkedList;

fn bench_linked_list(c: &mut Criterion) {
    c.bench_function("push_front 1000", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for i in 0..1000 {
                list.push_front(i);
            }
            black_box(list);
        });
    });

    c.bench_function("push_back 1000", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for i in 0..1000 {
                list.push_back(i);
            }
            black_box(list);
        });
    });
}

criterion_group!(benches, bench_linked_list);
criterion_main!(benches);
