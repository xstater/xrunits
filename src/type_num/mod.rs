pub mod num;
pub mod inc;
pub mod dec;
pub mod add;
pub mod minus;

pub use num::*;
pub use inc::Inc;
pub use dec::Dec;
pub use add::Add;
pub use minus::Minus;

// pub struct SI<T,Time,Length,Mass,Current,Temperature,Amount,Luminous>;
//
// pub type Meter<T> = SI<T,Zero,One,Zero,Zero,Zero,Zero,Zero>;
// pub type Second<T> = SI<T,One,Zero,Zero,Zero,Zero,Zero,Zero>;
