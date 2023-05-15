
pub trait BiFunctor<'a> 
where Self : 'a {
    type Unpluged1: 'a;
    type Unpluged2: 'a;
    type Plug<B1:'a,B2:'a>: BiFunctor<'a> + 'a;
    fn bimap<F1:'a, F2:'a, B1:'a, B2:'a>(self, f1:F1, f2:F2) -> Self::Plug<B1,B2>
    where
        F1: Fn(Self::Unpluged1) -> B1 + 'a,
        F2: Fn(Self::Unpluged2) -> B2 + 'a;
}

impl<'a,A1:'a,A2:'a> BiFunctor<'a> for (A1,A2) {
    type Plug<B1:'a,B2:'a> = (B1,B2);
    type Unpluged1 = A1;
    type Unpluged2 = A2;
    fn bimap<F1:'a, F2:'a, B1:'a, B2:'a>(self, f1:F1, f2:F2) -> Self::Plug<B1,B2>
        where
            F1: Fn(Self::Unpluged1) -> B1 + 'a,
            F2: Fn(Self::Unpluged2) -> B2 + 'a {
        (f1(self.0), f2(self.1))
    }
}

pub trait Functor<'a>
where Self :'a {
    type Unpluged : 'a ;
    type Plug<B:'a>: Functor<'a> + 'a;

    fn fmap<F:'a, B:'a>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unpluged) -> B + 'a;
}