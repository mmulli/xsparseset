use std::num::NonZeroUsize;

#[derive(Debug,Clone)]
pub struct SparseSet<E,T>
    where E : Copy + Into<usize>,
          T : Sized{
    pub (in crate) indices : Vec<Option<NonZeroUsize>>,
    pub (in crate) entities :  Vec<E>,
    pub (in crate) data : Vec<T>
}

impl<E,T> SparseSet<E,T>
    where E : Copy + Into<usize>,
          T : Sized {

    pub fn new() -> Self {
        SparseSet{
            indices: vec![],
            entities: vec![],
            data: vec![]
        }
    }

    pub fn add(&mut self,entity : E,data : T) {
        let entity_ : usize = entity.into();
        //enlarge sparse
        while self.indices.len() <= entity_ {
            self.indices.push(None);
        }
        if let Some(index) = self.indices[entity_] {
            //already exists
            //overwrite
            self.data[index.get() - 1] = data;
        }else{
            //have not yet
            self.indices[entity_] = NonZeroUsize::new(self.entities.len() + 1);
            self.entities.push(entity);
            self.data.push(data);
        }
    }

    pub fn remove(&mut self,entity : E) -> Option<T> {
        let entity : usize = entity.into();
        if self.indices.len() < entity {
            return None;
        }
        if let Some(index) = self.indices[entity] {
            let index = index.get() - 1;
            self.indices.swap(self.entities[index].into(),(*self.entities.last().unwrap()).into());
            self.indices[entity] = None;
            self.entities.swap_remove(index);
            return Some(self.data.swap_remove(index));
        }
        None
    }

    pub fn swap_by_index(&mut self,index_a : usize,index_b : usize) {
        if index_a == index_b { return; }
        if index_a >= self.len() {
            panic!("index_a={} is out of range",index_a);
        }
        if index_b >= self.len() {
            panic!("index_b={} is out of range",index_b);
        }
        let entity_a : usize = self.entities[index_a].into();
        let entity_b : usize = self.entities[index_b].into();
        self.indices.swap(entity_a,entity_b);
        self.entities.swap(index_a,index_b);
        self.data.swap(index_a,index_b);
    }

    pub fn swap_by_entity(&mut self,entity_a : E,entity_b : E) {
        if !self.exist(entity_a) {
            panic!("entity_a is not exist in sparse set");
        }
        if !self.exist(entity_b) {
            panic!("entity_b is not exist in sparse set");
        }
        let entity_a : usize = entity_a.into();
        let entity_b : usize = entity_b.into();
        if entity_a == entity_b { return; }
        let index_a = self.indices[entity_a].unwrap().get() - 1;
        let index_b = self.indices[entity_b].unwrap().get() - 1;
        self.indices.swap(entity_a,entity_b);
        self.entities.swap(index_a,index_b);
        self.data.swap(index_a,index_b);
    }

    /// Build a group in the longer one's ```index``` position
    /// it return the length of group
    /// Ai: 0 1 2 3 4 5 6 7 8 9
    /// A : x x x x a b c d x x
    /// B :         a b c d x
    /// Bi:         0 1 2 3 4
    /// ## Panics:
    /// Panic if index is out of the bound of the longer one
    pub fn make_group_in<U : Sized>(&mut self,other : &mut SparseSet<E,U>,index : usize) -> usize{
        let mut len = 0;
        let mut index = index;
        if self.len() > other.len() {
            for index_b in 0..other.len() {
                let entity_id = unsafe { other.entities.get_unchecked(index_b) };
                if let Some(index_a) = self.get_index(*entity_id) {
                    self.swap_by_index(index,index_a);
                    other.swap_by_index(index_b,len);
                    len += 1;
                    index += 1;
                }
            }
        }else{
            for index_a in 0..self.len() {
                let entity_id = unsafe { self.entities.get_unchecked(index_a) };
                if let Some(index_b) = other.get_index(*entity_id) {
                    self.swap_by_index(len,index_a);
                    other.swap_by_index(index_b,index);
                    len += 1;
                    index += 1;
                }
            }
        }
        len
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn exist(&self,entity : E) -> bool {
        let entity : usize = entity.into();
        if entity < self.indices.len()  {
            self.indices[entity].is_some()
        }else{
            false
        }
    }

    pub fn get(&self,entity : E) -> Option<&T> {
        let entity : usize = entity.into();
        if entity< self.indices.len() {
            if let Some(index) = self.indices[entity] {
                let index = index.get() - 1;
                return Some(&self.data[index])
            }
        }
        None
    }
    pub fn get_mut(&mut self,entity : E) -> Option<&mut T> {
        let entity : usize = entity.into();
        if entity < self.indices.len() {
            if let Some(index) = self.indices[entity] {
                let index = index.get() - 1;
                return Some(&mut self.data[index])
            }
        }
        None
    }

    pub fn get_index(&self,entity : E) -> Option<usize> {
        let entity : usize = entity.into();
        if entity < self.indices.len() {
            if let Some(index) = self.indices[entity] {
                return Some(index.get() - 1);
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.entities.len() == 0
    }

    pub fn indices(&self) -> &[Option<NonZeroUsize>] {
        self.indices.as_slice()
    }

    pub fn entities(&self) -> &[E] {
        self.entities.as_slice()
    }

    pub fn data(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    pub fn entity_iter(&self) -> impl Iterator<Item=(E,&T)> {
        self.entities
            .iter()
            .map(|x|*x)
            .zip(self.data
                .iter())
    }
    pub fn entity_iter_mut(&mut self) -> impl Iterator<Item=(E,&mut T)> {
        self.entities
            .iter()
            .map(|x|*x)
            .zip(self.data
                .iter_mut())
    }
}

#[cfg(test)]
mod tests{
    use crate::SparseSet;

    #[test]
    fn basic_test(){
        let mut s1 = SparseSet::new();
        s1.add(5usize,'a');
        s1.add(3,'b');
        assert_eq!(s1.entities(),&[5,3]);
        assert_eq!(s1.data(),&['a','b']);
        println!("{:?}",s1);

        s1.add(3,'c');
        s1.add(1,'d');
        assert_eq!(s1.entities(),&[5,3,1]);
        assert_eq!(s1.data(),&['a','c','d']);
        println!("{:?}",s1);

        assert_eq!(s1.get(4),None);
        assert_eq!(s1.get(1),Some(&'d'));
        *s1.get_mut(1).unwrap() = 'f';
        assert_eq!(s1.get(1),Some(&'f'));
        assert_eq!(s1.get_index(3),Some(1));
        println!("{:?}",s1);
        *s1.get_mut(1).unwrap() = 'd';

        assert_eq!(s1.remove(2),None);
        assert_eq!(s1.remove(5),Some('a'));
        assert_eq!(s1.entities(),&[1,3]);
        assert_eq!(s1.data(),&['d','c']);
        println!("{:?}",s1);
        assert_eq!(s1.remove(1),Some('d'));
        assert_eq!(s1.remove(3),Some('c'));
        println!("{:?}",s1);
        assert!(s1.is_empty());
    }

    #[test]
    fn swap_test(){
        let mut s1 = SparseSet::new();
        s1.add(3usize,'a');
        s1.add(5,'b');
        s1.add(6,'c');
        s1.add(2,'d');
        assert_eq!(s1.entities(),&[3,5,6,2]);
        assert_eq!(s1.data(),&['a','b','c','d']);
        println!("{:?}",s1);

        s1.swap_by_index(1,2);
        assert_eq!(s1.entities(),&[3,6,5,2]);
        assert_eq!(s1.data(),&['a','c','b','d']);
        println!("{:?}",s1);

        s1.swap_by_entity(2,3);
        assert_eq!(s1.entities(),&[2,6,5,3]);
        assert_eq!(s1.data(),&['d','c','b','a']);
        println!("{:?}",s1);
    }
    #[test]
    fn iter_test(){
        let mut ss = SparseSet::new();
        ss.add(3usize,'a');
        ss.add(4,'b');
        ss.add(7,'c');
        ss.add(1,'d');
        ss.add(2,'e');

        {
            let mut itr = ss.entity_iter();
            assert_eq!(itr.next(), Some((3, &'a')));
            assert_eq!(itr.next(), Some((4, &'b')));
            assert_eq!(itr.next(), Some((7, &'c')));
            assert_eq!(itr.next(), Some((1, &'d')));
            assert_eq!(itr.next(), Some((2, &'e')));
            assert_eq!(itr.next(), None);
        }
        {
            let mut itr = ss.entity_iter_mut();
            if let Some((_, ch)) = itr.next() {
                *ch = '1';
            } else {
                panic!();
            }
        }
        assert_eq!(ss.data().first().unwrap(),&'1');
    }

    #[test]
    fn group_test(){
        let mut ss1 = SparseSet::new();
        ss1.add(3usize,'a');
        ss1.add(4,'b');
        ss1.add(7,'c');
        ss1.add(1,'d');
        ss1.add(2,'e');

        let mut ss2 = SparseSet::new();
        ss2.add(2usize,1);
        ss2.add(1,3);
        ss2.add(3,5);
        ss2.add(5,7);

        let len = ss1.make_group_in(&mut ss2,1);
        println!("{:?}",ss1.entities());
        println!("   {:?}",ss2.entities());
        println!("{}",len);
        assert_eq!(ss1.entities(),[7,2,1,3,4]);
        assert_eq!(ss2.entities(),[2,1,3,5]);
        assert_eq!(len,3);
    }
}
