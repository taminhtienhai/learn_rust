use std::{marker::PhantomData};

use builder::Builder;

#[derive(Builder)]
struct Hello<T> {
    _phamtom_item: PhantomData<T>,
}


struct A {}


fn main() {
    let hello = Hello { _phamtom_item: PhantomData::<A> };

    hello.generated();
}
