extern crate xrunits;

use xrunits::type_num::*;
use xrunits::si::{Meter, Second};

#[test]
fn basic_test(){
    assert_eq!(P3.number(), 3);
    assert_eq!(N4.number(),-4);
    assert_eq!(<P3 as Inc>::Answer::NUMBER,4);
    assert_eq!(<Zero as Dec>::Answer::NUMBER,-1);
    assert_eq!(<<Zero as Inc>::Answer as Inc>::Answer::NUMBER,2);

    assert_eq!(<P2 as Add<P3>>::Answer::NUMBER,5);
    assert_eq!(<Zero as Minus<N3>>::Answer::NUMBER,3);
    assert_eq!(<P4 as Minus<P3>>::Answer::NUMBER,1);

    let m = Meter::m(3);
    let s : Second<i32> = Default::default();
    #[allow(unused)]
    let ms = m * s;
}