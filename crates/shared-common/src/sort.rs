use std::cmp::{Ord, Ordering};
use std::marker::PhantomData;

pub trait Reverser {
    fn maybe_reverse(ord: Ordering) -> Ordering;
}

pub struct Forward;
impl Reverser for Forward {
    fn maybe_reverse(ord: Ordering) -> Ordering {
        ord
    }
}

pub struct Reverse;
impl Reverser for Reverse {
    fn maybe_reverse(ord: Ordering) -> Ordering {
        ord.reverse()
    }
}

pub struct Sort<F, R>(PhantomData<fn(F)>, std::marker::PhantomData<fn(R)>);

pub trait Field {
    type Type: Ord + ?Sized;
    type OrderingType: ?Sized;

    fn get(p: &Self::OrderingType) -> &Self::Type;
}

impl<F: Field, R: Reverser> Sort<F, R> {
    pub fn sort(x: &F::OrderingType, y: &F::OrderingType) -> Ordering {
        R::maybe_reverse(F::get(x).cmp(F::get(y)))
    }
}
