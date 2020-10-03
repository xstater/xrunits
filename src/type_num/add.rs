use crate::type_num::num::*;

pub trait Add<T> : Num { type Answer : Num; }

macro_rules! expand_impl {
    ($lhs : ident,$rhs : ident,$ans : ident) => {
        impl Add<$rhs> for $lhs { type Answer = $ans; }
    };
}

expand_impl!(N5,Zero,N5);
expand_impl!(N5,P1,N4);
expand_impl!(N5,P2,N3);
expand_impl!(N5,P3,N2);
expand_impl!(N5,P4,N1);
expand_impl!(N5,P5,Zero);

expand_impl!(N4,N1,N5);
expand_impl!(N4,Zero,N4);
expand_impl!(N4,P1,N3);
expand_impl!(N4,P2,N2);
expand_impl!(N4,P3,N1);
expand_impl!(N4,P4,Zero);
expand_impl!(N4,P5,P1);

expand_impl!(N3,N2,N5);
expand_impl!(N3,N1,N4);
expand_impl!(N3,Zero,N3);
expand_impl!(N3,P1,N2);
expand_impl!(N3,P2,N1);
expand_impl!(N3,P3,Zero);
expand_impl!(N3,P4,P1);
expand_impl!(N3,P5,P2);

expand_impl!(N2,N3,N5);
expand_impl!(N2,N2,N4);
expand_impl!(N2,N1,N3);
expand_impl!(N2,Zero,N2);
expand_impl!(N2,P1,N1);
expand_impl!(N2,P2,Zero);
expand_impl!(N2,P3,P1);
expand_impl!(N2,P4,P2);
expand_impl!(N2,P5,P3);

expand_impl!(N1,N4,N5);
expand_impl!(N1,N3,N4);
expand_impl!(N1,N2,N3);
expand_impl!(N1,N1,N2);
expand_impl!(N1,Zero,N1);
expand_impl!(N1,P1,Zero);
expand_impl!(N1,P2,P1);
expand_impl!(N1,P3,P2);
expand_impl!(N1,P4,P3);
expand_impl!(N1,P5,P4);

expand_impl!(Zero,N5,N5);
expand_impl!(Zero,N4,N4);
expand_impl!(Zero,N3,N3);
expand_impl!(Zero,N2,N2);
expand_impl!(Zero,N1,N1);
expand_impl!(Zero,Zero,Zero);
expand_impl!(Zero,P1,P1);
expand_impl!(Zero,P2,P2);
expand_impl!(Zero,P3,P3);
expand_impl!(Zero,P4,P4);
expand_impl!(Zero,P5,P5);

expand_impl!(P1,N5,N4);
expand_impl!(P1,N4,N3);
expand_impl!(P1,N3,N2);
expand_impl!(P1,N2,N1);
expand_impl!(P1,N1,Zero);
expand_impl!(P1,Zero,P1);
expand_impl!(P1,P1,P2);
expand_impl!(P1,P2,P3);
expand_impl!(P1,P3,P4);
expand_impl!(P1,P4,P5);

expand_impl!(P2,N5,N3);
expand_impl!(P2,N4,N2);
expand_impl!(P2,N3,N1);
expand_impl!(P2,N2,Zero);
expand_impl!(P2,N1,P1);
expand_impl!(P2,Zero,P2);
expand_impl!(P2,P1,P3);
expand_impl!(P2,P2,P4);
expand_impl!(P2,P3,P5);

expand_impl!(P3,N5,N2);
expand_impl!(P3,N4,N1);
expand_impl!(P3,N3,Zero);
expand_impl!(P3,N2,P1);
expand_impl!(P3,N1,P2);
expand_impl!(P3,Zero,P3);
expand_impl!(P3,P1,P4);
expand_impl!(P3,P2,P5);

expand_impl!(P4,N5,N1);
expand_impl!(P4,N4,Zero);
expand_impl!(P4,N3,P1);
expand_impl!(P4,N2,P2);
expand_impl!(P4,N1,P3);
expand_impl!(P4,Zero,P4);
expand_impl!(P4,P1,P5);

expand_impl!(P5,N5,Zero);
expand_impl!(P5,N4,P1);
expand_impl!(P5,N3,P2);
expand_impl!(P5,N2,P3);
expand_impl!(P5,N1,P4);
expand_impl!(P5,Zero,P5);
