#[derive(Default,Debug)]
pub struct Zero;

#[derive(Default,Debug)]
pub struct P1;
#[derive(Default,Debug)]
pub struct P2;
#[derive(Default,Debug)]
pub struct P3;
#[derive(Default,Debug)]
pub struct P4;
#[derive(Default,Debug)]
pub struct P5;

#[derive(Default,Debug)]
pub struct N1;
#[derive(Default,Debug)]
pub struct N2;
#[derive(Default,Debug)]
pub struct N3;
#[derive(Default,Debug)]
pub struct N4;
#[derive(Default,Debug)]
pub struct N5;

pub trait Num{
    const NUMBER : i32;
    fn number(&self) -> i32 {
        Self::NUMBER
    }
}

impl Num for Zero{ const NUMBER : i32 = 0; }
impl Num for P1{ const NUMBER: i32 = 1; }
impl Num for P2{ const NUMBER: i32 = 2; }
impl Num for P3{ const NUMBER: i32 = 3; }
impl Num for P4{ const NUMBER: i32 = 4; }
impl Num for P5{ const NUMBER: i32 = 5; }
impl Num for N1{ const NUMBER: i32 = -1; }
impl Num for N2{ const NUMBER: i32 = -2; }
impl Num for N3{ const NUMBER: i32 = -3; }
impl Num for N4{ const NUMBER: i32 = -4; }
impl Num for N5{ const NUMBER: i32 = -5; }
