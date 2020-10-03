use crate::type_num::num::*;

pub trait Dec : Num { type Answer; }

impl Dec for N4 { type Answer = N5; }
impl Dec for N3 { type Answer = N4; }
impl Dec for N2 { type Answer = N3; }
impl Dec for N1 { type Answer = N2; }
impl Dec for Zero { type Answer = N1; }
impl Dec for P1 { type Answer = Zero; }
impl Dec for P2 { type Answer = P1; }
impl Dec for P3 { type Answer = P2; }
impl Dec for P4 { type Answer = P3; }
impl Dec for P5 { type Answer = P4; }
