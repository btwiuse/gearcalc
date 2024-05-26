#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<Ops, Ops>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Ops {
    Add(i128, i128),
    Sub(i128, i128),
    Mult(i128, i128),
    Div(i128, i128),
    Pow(i128, u32),
    Modulus(i128, i128),
    Neg(i128),
    Int128(i128),
}
