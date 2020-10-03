use crate::type_num::num::*;

pub trait Inc : Num{ type Answer; }

impl Inc for N5 { type Answer = N4; }
impl Inc for N4 { type Answer = N3; }
impl Inc for N3 { type Answer = N2; }
impl Inc for N2 { type Answer = N1; }
impl Inc for N1 { type Answer = Zero; }
impl Inc for Zero { type Answer = P1; }
impl Inc for P1 { type Answer = P2; }
impl Inc for P2 { type Answer = P3; }
impl Inc for P3 { type Answer = P4; }
impl Inc for P4 { type Answer = P5; }
