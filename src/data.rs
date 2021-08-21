use crate::{Unit,CastTo};
use std::fmt::{Display, Formatter};

pub trait Data : Unit + From<f64> {}

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
        impl Data for $type_name {}
        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,concat!("{}",$display_name),self.0)
            }
        }
        impl<T : Data> CastTo<T> for $type_name {}
    };
}
// too large
// build_type!(Tebibyte,BuildTebibyte,tib,"TiB",137_438_953_472,1);
build_type!(Gibibyte,BuildGibibyte,gib,"GiB",1_073_741_824,1);
build_type!(Mebibyte,BuildMebibyte,mib,"MiB",1_048_576,1);
build_type!(Kibibyte,BuildKibibyte,kib,"KiB",1024,1);
build_type!(Byte,BuildByte,byte,"Byte",1,1);
build_type!(Bit,BuildBit,bit,"Bit",1,8);

#[cfg(test)]
mod tests{
    use crate::CastTo;
    use crate::data::{BuildGibibyte, Mebibyte};

    #[test]
    fn test() {
        let gb2 = 2.gib();
        let mb : Mebibyte = gb2.cast_to();
        println!("{}={}",gb2,mb);
    }
}
