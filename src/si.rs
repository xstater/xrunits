use crate::type_num::*;
use std::ops::Mul;

#[derive(Default)]
pub struct SI<
    T,
    S : Num,
    M : Num,
    Kg : Num,
    A : Num,
    K : Num,
    Mol : Num,
    Cd : Num>{
    data : T,
    _s : std::marker::PhantomData<S>,
    _m : std::marker::PhantomData<M>,
    _kg : std::marker::PhantomData<Kg>,
    _a : std::marker::PhantomData<A>,
    _k : std::marker::PhantomData<K>,
    _mol : std::marker::PhantomData<Mol>,
    _cd : std::marker::PhantomData<Cd>}

pub type Meter<T> = SI<T,Zero,P1,Zero,Zero,Zero,Zero,Zero>;
pub type Second<T> = SI<T,P1,Zero,Zero,Zero,Zero,Zero,Zero>;

impl Meter<i32> {
    pub fn m(v : i32) -> Meter<i32> {
        let mut t : Meter<i32> = Default::default();
        t.data = v;
        t
    }
}

impl Second<i32>{
    pub fn s(v : i32) -> Second<i32>{
        let mut t : Second<i32> = Default::default();
        t.data = v;
        t
    }
}


impl<T,S1,S2,M1,M2,Kg1,Kg2,A1,A2,K1,K2,Mol1,Mol2,Cd1,Cd2>
    Mul<SI<T,S1,M1,Kg1,A1,K1,Mol1,Cd1>> for SI<T,S2,M2,Kg2,A2,K2,Mol2,Cd2> where
    T : std::ops::Mul<Output=T>,
    S1 : Num + Add<S2> , S2 : Num + Add<S1>,
    M1 : Num + Add<M2>, M2 : Num + Add<M1>,
    Kg1 :Num + Add<Kg2>, Kg2 : Num + Add<Kg1>,
    A1 : Num + Add<A2>, A2 : Num + Add<A1>,
    K1 : Num + Add<K2>, K2 : Num + Add<K1>,
    Mol1 : Num + Add<Mol2>, Mol2 : Num + Add<Mol1>,
    Cd1 : Num + Add<Cd2> , Cd2 : Num + Add<Cd1>{
    type Output = SI<
        T,
        <S1 as Add<S2>>::Answer,
        <M1 as Add<M2>>::Answer,
        <Kg1 as Add<Kg2>>::Answer,
        <A1 as Add<A2>>::Answer,
        <K1 as Add<K2>>::Answer,
        <Mol1 as Add<Mol2>>::Answer,
        <Cd1 as Add<Cd2>>::Answer>;

    fn mul(self, rhs: SI<T, S1, M1, Kg1, A1, K1, Mol1, Cd1>) -> Self::Output {
        SI{
            data : self.data * rhs.data,
            _s: Default::default(),
            _m: Default::default(),
            _kg: Default::default(),
            _a: Default::default(),
            _k: Default::default(),
            _mol: Default::default(),
            _cd: Default::default()
        }
        // unimplemented!()
    }
}
