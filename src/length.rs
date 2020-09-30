use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Nanometer<T>(T);
#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Micrometer<T>(T);
#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Millimeter<T>(T);
#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Centimeter<T>(T);
#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Decimeter<T>(T);
#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Meter<T>(T);
#[derive(Debug,Copy, Clone,PartialOrd, PartialEq)]
pub struct Kilometer<T>(T);

pub trait Length<T : Sized>{
    fn value(self) -> T;
}

impl<T> Length<T> for Nanometer<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Length<T> for Micrometer<T>{
    fn value(self) -> T{
        self.0
    }
}
impl<T> Length<T> for Millimeter<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Length<T> for Centimeter<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Length<T> for Decimeter<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Length<T> for Meter<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Length<T> for Kilometer<T>{
    fn value(self) -> T {
        self.0
    }
}


impl<T : Display> Display for Nanometer<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}nm",self.0)
    }
}
impl<T : Display> Display for Micrometer<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}um",self.0)
    }
}
impl<T : Display> Display for Millimeter<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}mm",self.0)
    }
}
impl<T : Display> Display for Centimeter<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}cm",self.0)
    }
}
impl<T : Display> Display for Decimeter<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}dm",self.0)
    }
}
impl<T : Display> Display for Meter<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}m",self.0)
    }
}
impl<T : Display> Display for Kilometer<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}km",self.0)
    }
}



pub trait ToNanometer<T : From<Self>> : Sized{
    fn nm(self) -> Nanometer<T>{
        Nanometer(self.into())
    }
}
pub trait ToMicrometer<T : From<Self>> : Sized{
    fn um(self) -> Micrometer<T>{
        Micrometer(self.into())
    }
}
pub trait ToMillimeter<T : From<Self>> : Sized{
    fn mm(self) -> Millimeter<T>{
        Millimeter(self.into())
    }
}
pub trait ToCentimeter<T : From<Self>> : Sized{
    fn cm(self) -> Centimeter<T>{
        Centimeter(self.into())
    }
}
pub trait ToDecimeter<T : From<Self>> : Sized{
    fn dm(self) -> Decimeter<T>{
        Decimeter(self.into())
    }
}
pub trait ToMeter<T : From<Self>> : Sized{
    fn m(self) -> Meter<T>{
        Meter(self.into())
    }
}
pub trait ToKilometer<T : From<Self>> : Sized{
    fn km(self) -> Kilometer<T>{
        Kilometer(self.into())
    }
}


macro_rules! to_type_from_number_impl {
    ($name:ident) => {
        impl $name<i8> for i8 {}
        impl $name<i16> for i16 {}
        impl $name<i32> for i32 {}
        impl $name<i64> for i64 {}
        impl $name<i128> for i128 {}
        impl $name<isize> for isize {}
        impl $name<u8> for u8 {}
        impl $name<u16> for u16 {}
        impl $name<u32> for u32 {}
        impl $name<u64> for u64 {}
        impl $name<u128> for u128 {}
        impl $name<usize> for usize {}
        impl $name<f32> for f32 {}
        impl $name<f64> for f64 {}
    };
}

to_type_from_number_impl!(ToNanometer);
to_type_from_number_impl!(ToMicrometer);
to_type_from_number_impl!(ToMillimeter);
to_type_from_number_impl!(ToCentimeter);
to_type_from_number_impl!(ToDecimeter);
to_type_from_number_impl!(ToMeter);
to_type_from_number_impl!(ToKilometer);

impl<T : std::ops::Div<Output = T> + From<f64> + Into<f64>> From<Nanometer<T>> for Meter<T>{
    fn from(v : Nanometer<T>) -> Self {
        Meter((v.0.into() / 1000000000.0_f64).into())
    }
}
impl<T : std::ops::Div<Output = T> + From<f64> + Into<f64>> From<Micrometer<T>> for Meter<T>{
    fn from(v : Micrometer<T>) -> Self {
        Meter((v.0.into() / 1000000.0_f64).into())
    }
}
impl<T : std::ops::Div<Output = T> + From<f64> + Into<f64>> From<Millimeter<T>> for Meter<T>{
    fn from(v : Millimeter<T>) -> Self {
        Meter((v.0.into() / 1000.0_f64).into())
    }
}
impl<T : std::ops::Div<Output = T> + From<f64> + Into<f64>> From<Centimeter<T>> for Meter<T>{
    fn from(v : Centimeter<T>) -> Self {
        Meter((v.0.into() / 100.0_f64).into())
    }
}
impl<T : std::ops::Div<Output = T> + From<f64> + Into<f64>> From<Decimeter<T>> for Meter<T>{
    fn from(v : Decimeter<T>) -> Self {
        Meter((v.0.into() / 10.0_f64).into())
    }
}
impl<T : std::ops::Mul<Output = T> + From<f64> + Into<f64>> From<Kilometer<T>> for Meter<T>{
    fn from(v : Kilometer<T>) -> Self {
        Meter((v.0.into() * 1000.0_f64).into())
    }
}

