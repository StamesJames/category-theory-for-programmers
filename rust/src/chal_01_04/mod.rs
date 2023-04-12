

// 1. write the identity Function
pub fn id<T>(a:T)->T{a}


// 2. Write the Composition Function
fn compose<A,B,C>(f: impl Fn(A)->B, g: impl Fn(B)->C) -> impl Fn(A)->C
{
    return move |a:A| g(f(a));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compose_id(){
        // 3. Write a Programm, that tries to test if compose respects identity
        // not entirely possible because Function equality is not decidable here
        let foo = compose(id::<i16>, id::<i16>);
        for i in i16::MIN..i16::MAX {
            assert!(i == foo(i));
        }
        let foo = compose(|x:i16| x + 5, id);
        for i in i16::MIN..i16::MAX-5 {
            assert!(i + 5 == foo(i));
        }
        let foo = compose(id, |x:i16| x + 5);
        for i in i16::MIN..i16::MAX-5 {
            assert!(i + 5 == foo(i));
        }
    }
}