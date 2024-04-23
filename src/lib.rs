#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod win1;
mod win2;
mod win3;

#[skyline::main(name = "ike_victoryfix")]
pub fn main() {
    win1::install();
    win2::install();
    win3::install();
}