use crate::type_num::*;

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
    _cd : std::marker::PhantomData<Cd>
}

impl<T,S,M,Kg,A,K,Mol,Cd>
std::ops::Add<SI<T,S,M,Kg,A,K,Mol,Cd>> for SI<T,S,M,Kg,A,K,Mol,Cd> where
    T : std::ops::Add<Output=T>,
    S : Num,
    M : Num,
    Kg :Num,
    A : Num,
    K : Num,
    Mol : Num,
    Cd : Num{
    type Output = SI<T,S,M,Kg,A,K,Mol,Cd>;

    fn add(self, rhs: SI<T, S, M, Kg, A, K, Mol, Cd>) -> Self::Output {
        SI{
            data : self.data + rhs.data,
            _s: Default::default(),
            _m: Default::default(),
            _kg: Default::default(),
            _a: Default::default(),
            _k: Default::default(),
            _mol: Default::default(),
            _cd: Default::default()
        }
    }
}

impl<T,S,M,Kg,A,K,Mol,Cd>
std::ops::Sub<SI<T,S,M,Kg,A,K,Mol,Cd>> for SI<T,S,M,Kg,A,K,Mol,Cd> where
    T : std::ops::Sub<Output=T>,
    S : Num,
    M : Num,
    Kg :Num,
    A : Num,
    K : Num,
    Mol : Num,
    Cd : Num{
    type Output = SI<T,S,M,Kg,A,K,Mol,Cd>;

    fn sub(self, rhs: SI<T, S, M, Kg, A, K, Mol, Cd>) -> Self::Output {
        SI{
            data : self.data - rhs.data,
            _s: Default::default(),
            _m: Default::default(),
            _kg: Default::default(),
            _a: Default::default(),
            _k: Default::default(),
            _mol: Default::default(),
            _cd: Default::default()
        }
    }
}

impl<T,S1,S2,M1,M2,Kg1,Kg2,A1,A2,K1,K2,Mol1,Mol2,Cd1,Cd2>
    std::ops::Mul<SI<T,S2,M2,Kg2,A2,K2,Mol2,Cd2>> for SI<T,S1,M1,Kg1,A1,K1,Mol1,Cd1> where
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

    fn mul(self, rhs: SI<T, S2, M2, Kg2, A2, K2, Mol2, Cd2>) -> Self::Output {
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
    }
}

impl<T,S1,S2,M1,M2,Kg1,Kg2,A1,A2,K1,K2,Mol1,Mol2,Cd1,Cd2>
std::ops::Div<SI<T,S2,M2,Kg2,A2,K2,Mol2,Cd2>> for SI<T,S1,M1,Kg1,A1,K1,Mol1,Cd1> where
    T : std::ops::Div<Output=T>,
    S1 : Num + Minus<S2> , S2 : Num + Minus<S1>,
    M1 : Num + Minus<M2>, M2 : Num + Minus<M1>,
    Kg1 :Num + Minus<Kg2>, Kg2 : Num + Minus<Kg1>,
    A1 : Num + Minus<A2>, A2 : Num + Minus<A1>,
    K1 : Num + Minus<K2>, K2 : Num + Minus<K1>,
    Mol1 : Num + Minus<Mol2>, Mol2 : Num + Minus<Mol1>,
    Cd1 : Num + Minus<Cd2> , Cd2 : Num + Minus<Cd1>{
    type Output = SI<
        T,
        <S1 as Minus<S2>>::Answer,
        <M1 as Minus<M2>>::Answer,
        <Kg1 as Minus<Kg2>>::Answer,
        <A1 as Minus<A2>>::Answer,
        <K1 as Minus<K2>>::Answer,
        <Mol1 as Minus<Mol2>>::Answer,
        <Cd1 as Minus<Cd2>>::Answer>;

    fn div(self, rhs: SI<T, S2, M2, Kg2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SI{
            data : self.data / rhs.data,
            _s: Default::default(),
            _m: Default::default(),
            _kg: Default::default(),
            _a: Default::default(),
            _k: Default::default(),
            _mol: Default::default(),
            _cd: Default::default()
        }
    }
}

