#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![feature(try_from)]
#![no_std]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]

extern crate bare_metal;
extern crate cortex_m;
extern crate vcell;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

pub mod stm32f7x7;
