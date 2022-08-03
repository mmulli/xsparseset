mod sparse_storage;

use std::num::NonZeroUsize;

pub use sparse_storage::SparseStorage;

#[derive(Debug, Clone)]
pub struct SparseSet<E, T, S> {
    sparse: S,
    dense: Vec<E>,
    data: Vec<T>,
}

impl<E, T, S> SparseSet<E, T, S>
where
    E: Copy,
    S: SparseStorage<EntityId = E> + Default,
{
    /// Creata an empty SparseSet
    pub fn new() -> Self {
        SparseSet {
            sparse: S::default(),
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

    /// Insert the dat with id to
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

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    /// Check if the sparse set has id
    pub fn contains(&self,id: E) -> bool {
        self.sparse.get_index(id).is_some()
    }

    /// Get the reference of data by given `id`
    /// # Returns
    /// Return None if sparse set doesn't contain this `id`
    pub fn get(&self,id: E) -> Option<&T> {
        let index = self.sparse.get_index(id)?.get() - 1;
        // Safety
        // The index stored in sparse is always in range
        unsafe {
            Some(self.data.get_unchecked(index))
        }
    }

    /// Get the MUTABLE reference by data by given `id`
    /// # Returns
    /// Return None if sparse set doesn't contain this `id`
    pub fn get_mut(&mut self,id: E) -> Option<&mut T> {
        let index = self.get_index(id)?;
        // Safety
        // The index stored in sparse is always in range
        unsafe {
            Some(self.data.get_unchecked_mut(index))
        }
    }

    /// Get the index of the entity was given by `id` in sparse set
    /// # Returns
    /// Return None if sparse set doesn't contain this `id`
    pub fn get_index(&mut self,id : E) -> Option<usize> {
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
    #[test]
    fn basic_test() {
        assert!(2 == 2);
    }
}
