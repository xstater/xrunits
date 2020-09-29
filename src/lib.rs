pub trait Num{
    fn number(self) -> Self{
        self
    }
}

impl Num for u8;
impl Num for u16;
impl Num for u32;
