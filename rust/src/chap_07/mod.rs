use std::rc::Rc;

fn fmap_reader<'a,A,B,C,G,H>(f: impl Fn(B)->C + 'a) -> impl Fn(G)->H + 'a
where 
A: 'a,
B: 'a,
C: 'a,
G: Fn(A) -> B + 'a,
H: Fn(A) -> C + 'a,
{
    let f = Rc::new(f);
    return move |g:G|->H {
        let f = f.clone();
        return move |a:A|->C {
            return f(g(a));
        }
    }
}