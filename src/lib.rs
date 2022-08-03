use std::collections::HashMap;
use std::hash::Hash;

pub struct SparseSet<E, T, S> {
    sparse: S,
    indices: Vec<E>,
    data: Vec<T>,
}

/// A trait that can be used to store the Sparse Data
pub trait SparseStorage {
    type Entity;

    fn get_index(&self, entity_id: Self::Entity) -> Option<usize>;
    fn add_index(&mut self, entity_id: Self::Entity, index : usize);
}

impl<E> SparseStorage for HashMap<E, usize>
where
    E: Eq + Hash,
{
    type Entity = E;

    fn get_index(&self, entity_id: Self::Entity) -> Option<usize> {
        self.get(&entity_id).copied()
    }

    fn add_index(&mut self, entity_id: Self::Entity, index : usize) {
        self.insert(entity_id, index);
    }
}

impl SparseStorage for Vec<usize> {
    type Entity = usize;

    fn get_index(&self, entity_id: Self::Entity) -> Option<usize> {
        self.get(entity_id).copied()
    }

    fn add_index(&mut self, entity_id: Self::Entity, index : usize) {
        if entity_id < self.len() {
            self.resize(entity_id - self.len(), 0);
        }

        unsafe { *self.get_unchecked_mut(entity_id) = index };
    }
}

impl<E, T, S> SparseSet<E, T, S>
where
    S: SparseStorage<Entity = E> + Default,
{
    pub fn new() -> Self {
        SparseSet {
            sparse: S::default(),
            indices: vec![],
            data: vec![],
        }
    }
}
