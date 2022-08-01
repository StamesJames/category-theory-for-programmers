


pub fn my_id<T>(a:T)->T{a}

pub fn compose<'a,F,G,A,B,C>(f:F, g:G) -> Box<(dyn Fn(A)->C + 'a)>
where
    F: Fn(A)->B + 'a,
    G: Fn(B)->C + 'a, 
{
    return Box::new(move |a| g(f(a)) );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id(){
        assert_eq!(1,my_id(1));
    }

    #[test]
    fn test_compose(){
        assert_eq!((|a| a + 5)(1), compose(|a| a+2, |b| b+3)(1));
    }
}