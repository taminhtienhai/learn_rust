trait ComposeFunc<TStart, TEnd> {
    fn apply(&self, init: TStart) -> TEnd;
}


impl <T1, T2, T3, A, B> ComposeFunc<T1,T3> for (A, B)
where
    A: Fn(T1) -> T2,
    B: Fn(T2) -> T3 {

    fn apply(&self, init: T1) -> T3 {
        (self.1)((self.0)(init))
    }
}

fn compose<In,Out>(fns: impl ComposeFunc<In,Out>, init: In) -> Out {
    fns.apply(init)
}

fn sum(a: usize) -> impl Fn(usize) -> usize {
    move |b: usize| a + b
}

fn mul(a: usize) -> impl Fn(usize) -> usize {
    move |b: usize| a * b
}

fn main() {
    // let sum = |a| move |b| a + b;
    // let mul = |a| move |b| a * b;

    let result = compose((sum(2), mul(3)), 1);

    let rs2 = (sum(3), mul(1)).apply(2);

    println!("{result:?}");

    println!("{rs2:?}");
}