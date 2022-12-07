#![no_std]

use gstd::prelude::*;

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
