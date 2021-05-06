use std::num::NonZeroUsize;

#[derive(Debug,Clone)]
pub struct SparseSet<E,T>
    where E : Copy + Into<usize>,
          T : Sized{
    indices : Vec<Option<NonZeroUsize>>,
    entities :  Vec<E>,
    data : Vec<T>
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

    pub fn indices(&self) -> &[Option<NonZeroUsize>] {
        self.indices.as_slice()
    }

    pub fn entities(&self) -> &[E] {
        self.entities.as_slice()
    }

    pub fn data(&self) -> &[T] {
        self.data.as_slice()
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

        assert_eq!(s1.remove(2),None);
        assert_eq!(s1.remove(5),Some('a'));
        assert_eq!(s1.entities(),&[1,3]);
        assert_eq!(s1.data(),&['d','c']);
        println!("{:?}",s1);
        assert_eq!(s1.remove(1),Some('d'));
        assert_eq!(s1.remove(3),Some('c'));
        println!("{:?}",s1);
    }
}
