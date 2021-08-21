use crate::{Unit,CastTo};
use std::fmt::{Display, Formatter};
use crate::time::{Time};

pub trait Frequency : Unit + From<f64> {}
pub trait IntoPeriod<T : Time> : Frequency {
    fn into_period(self) -> T;
}

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
        impl Frequency for $type_name {}
        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,concat!("{}",$display_name),self.0)
            }
        }
        impl<T : Frequency> CastTo<T> for $type_name {}
        impl<T : Time> IntoPeriod<T> for $type_name {
            fn into_period(self) -> T {
                let hz : Hertz = self.cast_to();
                <T as From<f64>>::from(
                    (1.0 / hz.value()) *
                    (<T as Unit>::DEN as f64) /
                    (<T as Unit>::NUM as f64)
                )
            }
        }
    };
}

// build_type!(Terahertz,BuildTerahertz,thz,"THz",1_000_000_000_000,1);
build_type!(Gigahertz,BuildGigahertz,ghz,"GHz",1_000_000_000,1);
build_type!(Megahertz,BuildMegahertz,mhz,"MHz",1_000_000,1);
build_type!(Kilohertz,BuildKilohertz,khz,"KHz",1000,1);
build_type!(Hertz,BuildHertz,hz,"Hz",1,1);


#[cfg(test)]
mod tests{
    use crate::CastTo;
    use crate::frequency::{BuildKilohertz, Hertz, Megahertz, IntoPeriod};
    use crate::time::Millisecond;

    #[test]
    fn test() {
        let khz = 1234.khz();
        let hz : Hertz = khz.cast_to();
        let mhz : Megahertz = hz.cast_to();
        println!("{}={}={}",khz,hz,mhz);
    }

    #[test]
    fn period_test() {
        let khz = 1.khz();
        let period : Millisecond = khz.into_period();
        println!("{}={}",khz,period);
    }
}
