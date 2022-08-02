use std::{collections::HashMap, hash::Hash};

struct Memoized<F,A,B>
where
    F: Fn(&A)->B,
    A: Clone + Hash + Eq,
    B: Clone
{
    function : F,
    memory: HashMap<A,B>
}

impl<F,A,B> Memoized<F,A,B>
where
    F: Fn(&A)->B,
    A: Clone + Hash + Eq,
    B: Clone
{
    fn new(f:F) -> Memoized<F,A,B>{
        Memoized {
            function: f,
            memory: HashMap::new(),
        }
    }

    fn apply(&mut self, arg: &A)->B {
        if self.memory.contains_key(&arg) {
            return self.memory.get(&arg).unwrap().clone();
        } else {
            let result = (self.function)(arg);
            self.memory.insert(arg.clone(), result.clone());
            return result;
        }
    
    }
}