use crate::{Unit,CastTo};
use std::fmt::{Display, Formatter};

pub trait Mass : Unit + From<f64> {}

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
        impl Mass for $type_name {}
        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,concat!("{}",$display_name),self.0)
            }
        }
        impl<T : Mass> CastTo<T> for $type_name {}
    };
}

build_type!(Ton,BuildTon,t,"t",1000_000,1);
build_type!(Kilogram,BuildKilogram,kg,"kg",1000,1);
build_type!(Gram,BuildGram,g,"g",1,1);
build_type!(Carat,BuildCarat,ct,"ct",1,5);
build_type!(Milligram,BuildMilligram,mg,"mg",1,1_000);
build_type!(Microgram,BuildMicrogram,ug,"ug",1,1_000_000);

#[cfg(test)]
mod tests{
    use crate::CastTo;
    use crate::mass::{BuildKilogram, Gram, Carat};

    #[test]
    fn test() {
        let kg = 2.kg();
        let gram : Gram = kg.cast_to();
        let carat : Carat = gram.cast_to();
        println!("{}={}={}",kg,gram,carat);
    }
}
