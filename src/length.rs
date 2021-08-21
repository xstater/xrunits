use crate::{Unit, CastTo};
use std::fmt::{Display, Formatter};

pub trait Length : Unit + From<f64> {}

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
        impl Length for $type_name {}
        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,concat!("{}",$display_name),self.0)
            }
        }
        impl<T : Length> CastTo<T> for $type_name {}
    };
}
build_type!(Kilometer,BuildKilometer,km,"km",1000,1);
build_type!(Meter,BuildMeter,m,"m",1,1);
build_type!(Decimeter,BuildDecimeter,dm,"dm",1,10);
build_type!(Centimeter,BuildCentimeter,cm,"cm",1,100);
build_type!(Millimeter,BuildMillimeter,mm,"mm",1,1000);
build_type!(Micrometer,BuildMicrometer,um,"um",1,1000_000);
build_type!(Nanometer,BuildNanometer,nm,"nm",1,1000_000_000);

#[cfg(test)]
mod tests {
    use crate::length::{BuildMeter, Centimeter, Millimeter};
    use crate::CastTo;

    #[test]
    fn test() {
        let m = 1.4444.m();
        let cm : Centimeter = m.cast_to();
        let mm : Millimeter = cm.cast_to();
        println!("{}={}={}",m,cm,mm);
    }
}