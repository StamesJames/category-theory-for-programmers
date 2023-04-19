pub fn option_compose<'a,F,G,A,B,C>(f:F, g:G) -> Box<(dyn Fn(A)->Option<C> + 'a)>
where
F: Fn(A)->Option<B> + 'a,
G: Fn(B)->Option<C> + 'a, 
{
    return Box::new(move |a| {
        match f(a) {
            None => None,
            Some(b) => g(b)
        }
    });
}

pub fn safe_reciprocal(x:f32) -> Option<f32> {
    if x == 0. {None} else {Some(1./x)}
}

pub fn safe_root(x:f32) -> Option<f32>{
    if x == 0. {None} else {Some(f32::sqrt(x))}
}

pub fn safe_root_reciprocal(x:f32) -> Option<f32>{
    (option_compose(safe_reciprocal, safe_root))(x)
}