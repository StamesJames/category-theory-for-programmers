


pub fn id<T>(a:T)->T{a}

fn compose<A,B,C>(f: impl Fn(A)->B, g: impl Fn(B)->C) -> impl Fn(A)->C
{
    return move |a:A| g(f(a));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id(){
        assert_eq!(1,id(1));
    }

    #[test]
    fn test_compose(){
        assert_eq!((|a| a + 5)(1), compose(|a| a+2, |b| b+3)(1));
    }
}