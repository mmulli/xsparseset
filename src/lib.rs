//! # XSparseSet
//! Sparse is a data-structure that can get data by dispersed ID and cache-friendly
mod sparse_storage;

use std::num::NonZeroUsize;

pub use sparse_storage::{SparseStorage, VecWrapper, VecStorage};

#[derive(Debug, Clone)]
pub struct SparseSet<E, T, S> {
    sparse: S,
    dense: Vec<E>,
    data: Vec<T>,
}

impl<E, T, S> Default for SparseSet<E, T, S>
where
    E: Copy,
    S: SparseStorage<EntityId = E> + Default,
{
    fn default() -> Self {
        SparseSet {
            sparse: S::default(),
            dense: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl<E, T, S> SparseSet<E, T, S>
where
    E: Copy,
    S: SparseStorage<EntityId = E>,
{
    /// Create sparse set with sparse storage
    pub fn with_storage(sparse_storage: S) -> Self {
        SparseSet {
            sparse: sparse_storage,
            dense: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Clear the sparse set
    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
        self.data.clear();
    }

    /// Insert the `dat` with `id` into sparse set
    /// # return
    /// It returns Some(T) if sparse set has this id ,
    /// otherwise returns None
    pub fn insert(&mut self, id: E, dat: T) -> Option<T> {
        if let Some(index) = self.sparse.get_index(id) {
            let index: usize = index.get() - 1;
            // Safety
            // The index stored in sparse is always in range
            let data_ref = unsafe { self.data.get_unchecked_mut(index) };
            Some(std::mem::replace(data_ref, dat))
        } else {
            let new_index = NonZeroUsize::new(self.dense.len() + 1);
            self.sparse.set_index(id, new_index);
            self.dense.push(id);
            self.data.push(dat);
            None
        }
    }

    /// Insert a lot of data
    /// # Panics
    /// * `ids.len() != data.len()`
    pub fn insert_batch(&mut self,ids: &mut Vec<E>,data: &mut Vec<T>) {
        if ids.len() != data.len() {
            panic!("ids.len() != dat.len()")
        }
        let start_index = self.data.len() + 1;
        // # Safety
        // * the index stored in sparse is start from 1
        let start_index  = unsafe {
            NonZeroUsize::new_unchecked(start_index)
        };
        self.sparse.set_indices(&ids, start_index);
        self.dense.append(ids);
        self.data.append(data);
    }

    /// Remove from sparse set
    /// # return
    /// It returns Some(T) if sparse set has this id ,
    /// otherwise returns None
    pub fn remove(&mut self, id: E) -> Option<T> {
        // swap this and the last entity to ensure the `dense` is dense
        if let Some(last_id) = self.dense.last().copied() {
            self.swap_by_entity_id(id, last_id);
            self.sparse.set_index(id, None);
            self.dense.remove(self.dense.len() - 1);
            Some(self.data.remove(self.data.len() - 1))
        } else {
            None
        }
    }

    /// swap 2 entites in sparse set by entity id
    /// # Details
    /// Do nothing if `id_a` or `id_b` is NOT in sparse set
    pub fn swap_by_entity_id(&mut self, id_a: E, id_b: E) {
        let index_a = self.sparse.get_index(id_a);
        let index_b = self.sparse.get_index(id_b);
        if index_a.is_none() || index_b.is_none() {
            return;
        }
        let index_a = index_a.unwrap().get() - 1;
        let index_b = index_b.unwrap().get() - 1;

        // Safety
        // The index stored in sparse is always in range
        unsafe {
            self.swap_by_index_unchecked(index_a, index_b);
        }
    }

    /// swap 2 entites in sparse set by index
    /// # Panics
    /// Panic if index is out of range
    pub fn swap_by_index(&mut self, index_a: usize, index_b: usize) {
        if index_a >= self.len() {
            panic!("index_a={} is out of range", index_a);
        }
        if index_b >= self.len() {
            panic!("index_b={} is out of range", index_b);
        }

        unsafe { self.swap_by_index_unchecked(index_a, index_b) }
    }

    /// swap 2 entites in sparse set by index with out any check
    /// # Safety
    /// Safe only `index_a` and `index_b` is less than `self.len()`
    pub unsafe fn swap_by_index_unchecked(&mut self, index_a: usize, index_b: usize) {
        if index_a == index_b {
            return;
        }
        let id_a = *self.dense.get_unchecked(index_a);
        let id_b = *self.dense.get_unchecked(index_b);

        self.sparse.swap(id_a, id_b);
        self.dense.swap(index_a, index_b);
        self.data.swap(index_a, index_b);
    }

    /// Get the count of entities in sparse set
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    /// Check sparse set is empty
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    /// Check if the sparse set has id
    pub fn contains(&self, id: E) -> bool {
        self.sparse.get_index(id).is_some()
    }

    /// Get the reference of data by given `id`
    /// # Returns
    /// Return None if sparse set doesn't contain this `id`
    pub fn get(&self, id: E) -> Option<&T> {
        let index = self.sparse.get_index(id)?.get() - 1;
        // Safety
        // The index stored in sparse is always in range
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    /// Get the MUTABLE reference by data by given `id`
    /// # Returns
    /// Return None if sparse set doesn't contain this `id`
    pub fn get_mut(&mut self, id: E) -> Option<&mut T> {
        let index = self.get_index(id)?;
        // Safety
        // The index stored in sparse is always in range
        unsafe { Some(self.data.get_unchecked_mut(index)) }
    }

    /// Get the index of the entity was given by `id` in sparse set
    /// # Returns
    /// Return None if sparse set doesn't contain this `id`
    pub fn get_index(&self, id: E) -> Option<usize> {
        self.sparse.get_index(id).map(|x| x.get() - 1)
    }

    /// Get the slice of data
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Get the slice of data
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Get the slice of ID , or the dense array
    /// # Details
    /// There is NO any `fn ids_mut(&self)` in this lib.  
    /// Because the mapping relations between the sparse and the dense is ensured by this lib.  
    /// Leaking the mutable slice of dense is unsafe and will cause some unexpected error
    pub fn ids(&self) -> &[E] {
        &self.dense
    }
}

#[cfg(test)]
mod tests {
    use std::{num::NonZeroUsize, collections::BTreeSet};

    use rand::{thread_rng, Rng};

    use crate::{sparse_storage::VecStorage, SparseSet};

    type EntityId = NonZeroUsize;

    #[test]
    fn interface_test() {
        let mut sparse_set: SparseSet<EntityId, char, VecStorage<EntityId>> = SparseSet::default();

        assert_eq!(sparse_set.len(), 0);
        assert!(sparse_set.is_empty());
        assert!(sparse_set.data().is_empty());
        assert!(sparse_set.ids().is_empty());

        let id = NonZeroUsize::new(124).unwrap();

        assert_eq!(sparse_set.remove(id), None);
        assert!(!sparse_set.contains(id));
        assert_eq!(sparse_set.len(), 0);
        assert!(sparse_set.is_empty());
        assert!(sparse_set.data().is_empty());
        assert!(sparse_set.ids().is_empty());

        // insert
        assert_eq!(sparse_set.insert(id, 'c'), None);

        assert_eq!(sparse_set.len(), 1);
        assert!(!sparse_set.is_empty());
        assert_eq!(sparse_set.get(id).copied(), Some('c'));
        assert!(sparse_set.contains(id));
        assert_eq!(sparse_set.data(), &['c']);
        assert_eq!(sparse_set.ids(), &[id]);

        // insert again to change the value
        assert_eq!(sparse_set.insert(id, 'b'), Some('c'));

        assert_eq!(sparse_set.len(), 1);
        assert!(!sparse_set.is_empty());
        assert_eq!(sparse_set.get(id).copied(), Some('b'));
        assert!(sparse_set.contains(id));
        assert_eq!(sparse_set.data(), &['b']);
        assert_eq!(sparse_set.ids(), &[id]);

        // remove this one
        assert_eq!(sparse_set.remove(id), Some('b'));

        assert!(!sparse_set.contains(id));
        assert_eq!(sparse_set.len(), 0);
        assert!(sparse_set.is_empty());
        assert!(sparse_set.data().is_empty());
        assert!(sparse_set.ids().is_empty());

        // remove twice
        assert_eq!(sparse_set.remove(id), None);

        assert!(!sparse_set.contains(id));
        assert_eq!(sparse_set.len(), 0);
        assert!(sparse_set.is_empty());
        assert!(sparse_set.data().is_empty());
        assert!(sparse_set.ids().is_empty());

        // generate a lot of ids'
        let mut rng = thread_rng();
        let count = 100000;
        // generate unique id
        let ids = std::iter::from_fn(move || {
            Some((rng.gen_range(1000..100000), rng.gen_range('a'..='z')))
        })
        .map(|(x, c)| (NonZeroUsize::new(x).unwrap(), c))
        .take(count);

        for (id, c) in ids {
            sparse_set.insert(id, c);
            assert!(sparse_set.contains(id));
            assert_eq!(sparse_set.get(id).copied(), Some(c));
        }

    }

    #[test]
    fn batch_test() {
        let mut rng = rand::thread_rng();
        let mut sparse_set : SparseSet<EntityId, char, VecStorage<EntityId>> = SparseSet::default();
        let mut set = BTreeSet::new();

        let mut ids = Vec::new();
        let mut data = Vec::new();
        
        let count = 100_000;
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

        let mut ids_in = ids.clone();
        let mut data_in = data.clone();
        sparse_set.insert_batch(&mut ids_in, &mut data_in);

        assert_eq!(data.len(), sparse_set.len());
        assert_eq!(&data,sparse_set.data());
        
        for (id,data) in ids.iter().zip(data.iter()) {
            let ch = sparse_set.get(id.clone());
            assert!(ch.is_some());
            assert_eq!(data.clone(),ch.copied().unwrap());
        }
    }
}
