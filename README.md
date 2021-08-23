# XSparseSet
sparse set is a data-structure with fast iteration and getting data from sparse ID
# sparse set
Sparse set has 2 arrays,the sparse array 'S' and dense array 'D'.
The 'S' and 'D' array must satisfy 2 rules :
* ```S[ID] == index```
* ```D[index] = ID```  
These 2 rules make us get data from ID quickly and we can store all data densely.
# Details
Because we need store data and entity ID. XSparseSet has 3 arrays , "indices" "entities" and "data".
* indices : the sparse array
* entities : the dense array
* data : the dense array
# Examples
```
let mut sparse_set = SparseSet::new();
sparse_set.add(4,'c');
sparse_set.add(7,'a');
assert_eq!(sparse_set.get(4),Some(&'c'));
assert_eq!(sparse_set.get(5),None);
```
