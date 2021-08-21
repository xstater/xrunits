use crate::{Unit, CastTo};
use std::fmt::{Display, Formatter};

pub trait Angle : Unit + From<f64> {}

macro_rules! build_type {
    ($type_name : ident,
     $trait_name : ident,
     $short_name : ident,
     $display_name : expr,
     $num : expr,$den : expr) => {
        #[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
        pub struct $type_name(f64);
        impl<T : Into<f64>> From<T> for $type_name {
            fn from(other : T) -> Self {
                $type_name(other.into())
            }
        }
        pub trait $trait_name {
            fn $short_name(self) -> $type_name;
        }
        impl<T : Into<f64>> $trait_name for T {
            fn $short_name(self) -> $type_name {
                $type_name(self.into())
            }
        }
        impl Unit for $type_name {
            const NUM: i32 = $num;
            const DEN: i32 = $den;

            fn value(&self) -> f64 {
                self.0
            }
        }
        impl Angle for $type_name {}
        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,concat!("{}",$display_name),self.0)
            }
        }
        impl<T : Angle> CastTo<T> for $type_name {}
    };
}

build_type!(Radian,BuildRadian,rad,"rad",1,1);
build_type!(Degree,BuildDegree,deg,"deg",31_415_926,1_800_000_000);

#[cfg(test)]
mod tests {
    use crate::CastTo;
    use std::f64::consts::PI;
    use crate::angle::{BuildRadian, Degree};

    #[test]
    fn test() {
        let pi_div_4 = (PI / 4.0).rad();
        let deg : Degree = pi_div_4.cast_to();
        println!("{}={}",pi_div_4,deg)
    }
}
