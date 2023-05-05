pub trait Bifunctor<A, B>{
    fn bimap<NA, NB>(f: impl Fn(A)-> NA, g: impl Fn(B)-> NB, x:impl Bifunctor<A,B>)
        -> Self 
        where Self : Sized;
}