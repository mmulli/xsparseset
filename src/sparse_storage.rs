use std::collections::HashMap;
use std::collections::btree_map::Range;
use std::hash::Hash;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

/// A trait that can represent the storage of the Sparse Data
pub trait SparseStorage {
    /// the type of Entity ID
    type EntityId : Copy;

    /// Get index from the given entity id
    fn get_index(&self, entity_id: Self::EntityId) -> Option<NonZeroUsize>;

    /// set the entity mapping to index
    fn set_index(&mut self, entity_id: Self::EntityId, index : Option<NonZeroUsize>);

    /// set a batch of indices
    /// # Remarks
    /// * The index must be continuous and start from `start_index`
    fn set_indices(&mut self,entity_ids: &[Self::EntityId], start_index: NonZeroUsize) {
        let mut index = start_index.get();
        for id in entity_ids {
            self.set_index(*id, NonZeroUsize::new(index));
            index += 1;
        }
    }

    /// Clear itself
    fn clear(&mut self);

    /// swap 2 entitis
    fn swap(&mut self,entity_id_1: Self::EntityId,entity_id_2: Self::EntityId) {
        let index_1 = self.get_index(entity_id_1);
        let index_2 = self.get_index(entity_id_2);

        self.set_index(entity_id_1, index_2);
        self.set_index(entity_id_2, index_1);
    }
}

impl<E> SparseStorage for HashMap<E,NonZeroUsize> 
where E : Hash + Eq + Copy{
    type EntityId = E;

    fn get_index(&self, entity_id: Self::EntityId) -> Option<NonZeroUsize> {
        self.get(&entity_id).copied()
    }

    fn set_index(&mut self, entity_id: Self::EntityId, index : Option<NonZeroUsize>) {
        if let Some(index) = index {
            self.insert(entity_id, index);
        } else {
            self.remove(&entity_id);
        }
    }

    fn clear(&mut self){
        self.clear();
    }
}

/// To make the Vec Rank up and avoid the warning
/// VecWrapeer :: T -> U -> VecWrapper
#[derive(Debug,Clone)]
pub struct VecWrapper<T,E>(Vec<T>,PhantomData<E>);

impl<T,E> Default for VecWrapper<T,E> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

pub type VecStorage<E> = VecWrapper<Option<NonZeroUsize>,E>;

impl<E> SparseStorage for VecWrapper<Option<NonZeroUsize>,E>
where E : Into<usize> + Copy {
    type EntityId = E;

    fn get_index(&self, entity_id: Self::EntityId) -> Option<NonZeroUsize> {
        let entity_id : usize = entity_id.into();
        if entity_id < self.0.len() {
            unsafe {
                *self.0.get_unchecked(entity_id)
            }
        } else {
            None
        }
    }

    fn set_index(&mut self, entity_id: Self::EntityId, index : Option<NonZeroUsize>) {
        let entity_id : usize = entity_id.into();
        if entity_id >= self.0.len() {
            self.0.resize(entity_id + 1, None);
        }
        *unsafe { self.0.get_unchecked_mut(entity_id) } = index;
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

