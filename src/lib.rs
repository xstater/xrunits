//! # XRUnits
//! XRUnits is a unit library
//! # **My english is NOT very good !!**
//! ## Motivation
//! We usually use some real-world unit.
//! If we need to develop a program to deal with these units,
//! like creating a downloader which need to calculate the
//! data transfer speed or set the frequency of MCU,we need
//! do lots of duplicate works to convert these unit, so I want
//! to develop a unit library with STRONG TYPE and Easily using
//! to deal with these situations.
//! ## Details
//! To avoid duplicate code,I use lots of macro to automatically
//! generate code.
//! ## Examples
//! To create 2Kg
//! ```
//! # use xrunits::mass::BuildKilogram;
//! let mass = 2.kg();
//! ```
//! Convert it to ton and gram
//! ```
//! # use xrunits::mass::{Ton, BuildKilogram, Gram};
//! # use xrunits::CastTo;
//! let mass : Ton = 2.kg().cast_to();
//! let mass : Gram = mass.cast_to();
//! ```
//! Print to screen
//! ```
//! # use xrunits::time::BuildSecond;
//! println!("time:{}",3.sec()); //time:3s
//! ```
//! Calculate the Frequency from Period
//! ```
//! # use xrunits::time::{BuildMicrosecond, IntoFrequency};
//! # use xrunits::frequency::Megahertz;
//! let us = 1.us();
//! let mhz : Megahertz = us.into_frequency();
//! ```

/// the length Unit
pub mod length;
/// the Time Unit
pub mod time;
/// the Data Unit
pub mod data;
/// the Frequency Unit
pub mod frequency;
/// the Mass Unit
pub mod mass;
/// the Angle Unit
pub mod angle;

/// Core Trait that represent a Unit
pub trait Unit : Sized{
    /// The numerator of factor
    const NUM : i32;
    /// The denominator of factor
    const DEN : i32;

    /// Get the value
    fn value(&self) -> f64;

    /// Get the value of base unit
    /// ## Base unit
    /// Base unit is a unit with NUM=DEN=1,like Meter Gram or Byte
    fn base_value(&self) -> f64{
        self.value() * (Self::NUM as f64) / (Self::DEN as f64)
    }
}

/// # Cast a unit to another unit
pub trait CastTo<T : Unit + From<f64>> : Unit {
    /// Execute the casting
    fn cast_to(self) -> T {
        <T as From<f64>>::from(
            self.base_value() *
                (<T as Unit>::DEN as f64) /
                (<T as Unit>::NUM as f64)
        )
    }
}
