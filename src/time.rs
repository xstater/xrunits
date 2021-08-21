use crate::{Unit,CastTo};
use std::fmt::{Display, Formatter};
use crate::frequency::Frequency;

pub trait Time : Unit + From<f64> {}
pub trait IntoFrequency<T : Frequency> : Time {
    fn into_frequency(self) -> T;
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
        impl Time for $type_name {}
        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,concat!("{}",$display_name),self.0)
            }
        }
        impl<T : Time> CastTo<T> for $type_name {}
        impl<T : Frequency> IntoFrequency<T> for $type_name {
            fn into_frequency(self) -> T {
                let sec : Second = self.cast_to();
                <T as From<f64>>::from(
                    (1.0 / sec.value()) *
                    (<T as Unit>::DEN as f64) /
                    (<T as Unit>::NUM as f64)
                )
            }
        }
    };
}
build_type!(Year,BuildYear,year,"year",31536000,1);
build_type!(Month,BuildMonth,mon,"mon",2592000,1);
build_type!(Week,BuildWeek,week,"week",604800,1);
build_type!(Day,BuildDay,day,"day",86400,1);
build_type!(Hour,BuildHour,hr,"hr",3600,1);
build_type!(Minute,BuildMinute,min,"min",60,1);
build_type!(Second,BuildSecond,sec,"sec",1,1);
build_type!(Millisecond,BuildMillisecond,ms,"ms",1,1000);
build_type!(Microsecond,BuildMicrosecond,us,"us",1,1000_000);
build_type!(Nanosecond,BuildNanosecond,ns,"ns",1,1000_000_000);

#[cfg(test)]
mod tests{
    use crate::time::{BuildDay, Week, Hour, BuildMicrosecond, IntoFrequency};
    use crate::CastTo;
    use crate::frequency::{Kilohertz};

    #[test]
    fn test() {
        let day2 = 2.day();
        let week : Week = day2.cast_to();
        let hour : Hour = day2.cast_to();
        println!("{}={}={}",day2,week,hour);
    }

    #[test]
    fn freq_test() {
        let time = 2.us();
        let freq : Kilohertz = time.into_frequency();
        println!("{}={}",time,freq);
    }
}