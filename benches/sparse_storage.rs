use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    num::NonZeroUsize,
};

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use xsparseset::{SparseSet, VecStorage};

type EntityId = NonZeroUsize;

fn insert_batch(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut set = BTreeSet::new();

    let mut ids = Vec::new();
    let mut data = Vec::new();

    let count = 1_000;
    for _ in 0..count {
        'gen_data: loop {
            let id = rng.gen_range(1..100_000_000);
            if !set.contains(&id) {
                set.insert(id);
                let id = EntityId::new(id).unwrap();
                let d = rng.gen_range('a'..='z');

                ids.push(id);
                data.push(d);
                break 'gen_data;
            }
        }
    }

    criterion.bench_function("InsertBatch:vec_wrapper", |b| {
        b.iter(|| {
            let mut ids_in = ids.clone();
            let mut data_in = data.clone();
            let mut sparse_set: SparseSet<EntityId, char, VecStorage<EntityId>> =
                SparseSet::default();
            sparse_set.insert_batch(&mut ids_in, &mut data_in);
        })
    });
    criterion.bench_function("InsertBatch:BTreeMap", |b| {
        b.iter(|| {
            let mut ids_in = ids.clone();
            let mut data_in = data.clone();
            let mut sparse_set: SparseSet<EntityId, char, BTreeMap<EntityId, NonZeroUsize>> =
                SparseSet::default();
            sparse_set.insert_batch(&mut ids_in, &mut data_in);
        })
    });
    criterion.bench_function("InsertBatch:HashMap", |b| {
        b.iter(|| {
            let mut ids_in = ids.clone();
            let mut data_in = data.clone();
            let mut sparse_set: SparseSet<EntityId, char, HashMap<EntityId, NonZeroUsize>> =
                SparseSet::default();
            sparse_set.insert_batch(&mut ids_in, &mut data_in);
        })
    });
}

fn insert(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let count = 1_000;
    let data_gen_iter =
        std::iter::from_fn(move || Some((rng.gen_range(1000..100000), rng.gen_range('a'..='z'))))
            .map(|(x, c)| (NonZeroUsize::new(x).unwrap(), c));

    criterion.bench_function("Insert:vec_wrapper", |b| {
        b.iter(|| {
            let mut sparse_set: SparseSet<EntityId, char, VecStorage<EntityId>> = SparseSet::default();
            for (id, ch) in data_gen_iter.clone().take(count) {
                sparse_set.insert(id, ch);
            }
        });
    });
    criterion.bench_function("Insert:BTreeMap", |b| {
        b.iter(|| {
            let mut sparse_set: SparseSet<EntityId, char, BTreeMap<EntityId,NonZeroUsize>> = SparseSet::default();
            for (id, ch) in data_gen_iter.clone().take(count) {
                sparse_set.insert(id, ch);
            }
        });
    });
    criterion.bench_function("Insert:HashMap", |b| {
        b.iter(|| {
            let mut sparse_set: SparseSet<EntityId, char, HashMap<EntityId,NonZeroUsize>> = SparseSet::default();
            for (id, ch) in data_gen_iter.clone().take(count) {
                sparse_set.insert(id, ch);
            }
        });
    });
}

criterion_group!(benches, insert_batch, insert);
criterion_main!(benches);
