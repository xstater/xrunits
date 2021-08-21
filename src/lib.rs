pub mod length;
pub mod time;
pub mod data;
pub mod frequency;
pub mod mass;

pub trait Unit : Sized{
    const NUM : i32;
    const DEN : i32;

    fn value(&self) -> f64;

    fn base_value(&self) -> f64{
        self.value() * (Self::NUM as f64) / (Self::DEN as f64)
    }
}

pub trait CastTo<T : Unit + From<f64>> : Unit {
    fn cast_to(self) -> T {
        <T as From<f64>>::from(
            self.base_value() *
                (<T as Unit>::DEN as f64) /
                (<T as Unit>::NUM as f64)
        )
    }
}
