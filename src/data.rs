use std::fmt;
use std::fmt::{Display,Formatter};
use std::ops::Mul;

#[derive(Debug,Copy, Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Byte<T>(T);
#[derive(Debug,Copy, Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Kibibyte<T>(T);
#[derive(Debug,Copy, Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Mebibyte<T>(T);
#[derive(Debug,Copy, Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Gibibyte<T>(T);
#[derive(Debug,Copy, Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Tebibyte<T>(T);
#[derive(Debug,Copy, Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Pebibyte<T>(T);

pub trait Data<T>{
    fn value(self) -> T;
}

impl<T> Data<T> for Byte<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Data<T> for Kibibyte<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Data<T> for Mebibyte<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Data<T> for Gibibyte<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Data<T> for Tebibyte<T>{
    fn value(self) -> T {
        self.0
    }
}
impl<T> Data<T> for Pebibyte<T>{
    fn value(self) -> T {
        self.0
    }
}

impl<T : Display> Display for Byte<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}bytes",self.0)
    }
}
impl<T : Display> Display for Kibibyte<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}KiB",self.0)
    }
}
impl<T : Display> Display for Mebibyte<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}MiB",self.0)
    }
}
impl<T : Display> Display for Gibibyte<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}GiB",self.0)
    }
}
impl<T : Display> Display for Tebibyte<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}TiB",self.0)
    }
}
impl<T : Display> Display for Pebibyte<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}PiB",self.0)
    }
}


pub trait ToByte<T : From<Self>> : Sized {
    fn bytes(self) -> Byte<T> {
        Byte(self.into())
    }
}
pub trait ToKibibyte<T : From<Self>> : Sized {
    #[allow(non_snake_case)]
    fn KiB(self) -> Kibibyte<T> {
        Kibibyte(self.into())
    }
}
pub trait ToMebibyte<T : From<Self>> : Sized{
    #[allow(non_snake_case)]
    fn MiB(self) -> Mebibyte<T> {
        Mebibyte(self.into())
    }
}
pub trait ToGibibyte<T : From<Self>> : Sized {
    #[allow(non_snake_case)]
    fn GiB(self) -> Gibibyte<T> {
        Gibibyte(self.into())
    }
}
pub trait ToTebibyte<T : From<Self>> : Sized {
    #[allow(non_snake_case)]
    fn TiB(self) -> Tebibyte<T> {
        Tebibyte(self.into())
    }
}
pub trait ToPebibyte<T : From<Self>> : Sized{
    #[allow(non_snake_case)]
    fn PiB(self) -> Pebibyte<T> {
        Pebibyte(self.into())
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
to_type_from_number_impl!(ToByte);
to_type_from_number_impl!(ToKibibyte);
to_type_from_number_impl!(ToMebibyte);
to_type_from_number_impl!(ToGibibyte);
to_type_from_number_impl!(ToTebibyte);
to_type_from_number_impl!(ToPebibyte);

impl<T : Mul<Output = T> + From<f64> + Into<f64>> From<Kibibyte<T>> for Byte<T>{
    fn from(v : Kibibyte<T>) -> Self {
        Byte((v.0.into() * 1024_f64).into())
    }
}
impl<T : Mul<Output = T> + From<f64> + Into<f64>> From<Mebibyte<T>> for Byte<T>{
    fn from(v : Mebibyte<T>) -> Self {
        Byte((v.0.into() * 131072_f64).into())
    }
}
impl<T : Mul<Output = T> + From<f64> + Into<f64>> From<Gibibyte<T>> for Byte<T>{
    fn from(v : Gibibyte<T>) -> Self {
        Byte((v.0.into() * 1073741824_f64).into())
    }
}
impl<T : Mul<Output = T> + From<f64> + Into<f64>> From<Tebibyte<T>> for Byte<T>{
    fn from(v : Tebibyte<T>) -> Self {
        Byte((v.0.into() * 1099511627776_f64).into())
    }
}
impl<T : Mul<Output = T> + From<f64> + Into<f64>> From<Pebibyte<T>> for Byte<T>{
    fn from(v : Pebibyte<T>) -> Self {
        Byte((v.0.into() * 1.125900e15_f64).into())
    }
}
