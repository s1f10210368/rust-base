#[derive(Debug,PartialEq)]
struct PhantomStruct<X, A> {
    value: A
}

use std::marker::PhantomData;

struct PhantomStruct<X, A> {
    value: A,
    phantom: PhantomData<X>
}

const _phantom1: PhantomStruct<i32, char> = PhantomStruct {
    value: 'P', 
    phantom: PhantomData
};

const _phantom2: PhantomStruct<i64, char> = PhantomStruct {
    value: 'P', 
    phantom: PhantomData
    };