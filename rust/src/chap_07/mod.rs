pub trait Functor<'a>
where Self :'a {
    type Unpluged : 'a ;
    type Plug<B:'a>: Functor<'a> + 'a;

    fn fmap<F:'a, B:'a>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unpluged) -> B + 'a;
}

impl<'a,R:'a,A:'a> Functor<'a> for Box<dyn Fn(R) -> A + 'a> {
    type Plug<B:'a> = Box<dyn Fn(R)->B + 'a>;
    type Unpluged = A;
    fn fmap<F:'a, B:'a>(self, f: F) -> Self::Plug<B>
        where
            F: Fn(Self::Unpluged) -> B + 'a {
        Box::new(move |r| f(self(r)))
    }
}

pub fn r_fmap<'a,R:'a,A:'a,B:'a>(g:Box<dyn Fn(R)->A + 'a>,f:Box<dyn Fn(A)->B + 'a>) -> Box<dyn Fn(R)->B + 'a>{
    Box::new(move |r| f(g(r)))
}




// Old Implementations 
/*

trait Functor {
    type Unpluged;
    type Plug<B:'static>: Functor;

    fn fmap<F, B>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unpluged) -> B + 'static;
}

impl<A> Functor for Option<A>
{
    type Unpluged = A;
    type Plug<P:'static> = Option<P>;
    fn fmap<F,B>(self, f: F,) -> Option<B> 
    where F: Fn(A)->B{
        match self {
            None => None,
            Some(x) => Some(f(x)),
        }
    }
}

impl<R: 'static,A: 'static> Functor for Box<dyn Fn(R)->A>
{
    type Plug<B:'static> = Box<dyn Fn(R)->B>;
    type Unpluged = A;
    fn fmap<F, B:'static>(self, f: F) -> Self::Plug<B>
        where
            F: Fn(Self::Unpluged) -> B + 'static {
        Box::new(move |r| f(self(r)))
    }
}

impl<A> Functor for Box<A>{
    type Plug<B:'static> = Box<B>;
    type Unpluged = A;
    fn fmap<F, B:'static>(self, f: F) -> Self::Plug<B>
        where
            F: Fn(Self::Unpluged) -> B {
        Box::new(f(*self))
    }
}




 */