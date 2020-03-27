#![no_std]
#![no_main]

extern crate panic_halt;

use core::ptr;
use cortex_m::asm;
use cortex_m_rt::entry;

const RCC_APB2ENR: *mut u32 = (0x4002_1000 + 0x18) as *mut u32;
const GPIOA_CRL: *mut u32 = (0x4001_0800 + 0x00) as *mut u32;
const GPIOA_CRH: *mut u32 = (0x4001_0800 + 0x04) as *mut u32;
const GPIOA_BSRR: *mut u32 = (0x4001_0800 + 0x10) as *mut u32;
const GPIOA_IDR: *mut u32 = (0x4001_0800 + 0x08) as *mut u32;

const APB2ENR_IOPAEN: u32 = 2;
const CRL_MODE1: u32 = 4;
const CRH_MODE8: u32 = 0;
const BSRR_BS1: u32 = 1;
const BSRR_BR1: u32 = 1 + 16;
const IDR8: u32 = 8;

enum Status {
    Low,
    High,
}

#[entry]
fn main() -> ! {
    unsafe {
        ptr::write_volatile(RCC_APB2ENR, 1 << APB2ENR_IOPAEN);

        ptr::write_volatile(GPIOA_CRL, 0b0011 << CRL_MODE1);
        ptr::write_volatile(GPIOA_CRH, 0b0100 << CRH_MODE8);

        loop {
            match status(GPIOA_IDR, IDR8) {
                Status::Low => {
                    ptr::write_volatile(GPIOA_BSRR, 1 << BSRR_BR1);
                    delay();
                    ptr::write_volatile(GPIOA_BSRR, 1 << BSRR_BS1);
                    delay();
                },
                Status::High => {
                    ptr::write_volatile(GPIOA_BSRR, 1 << BSRR_BS1);
                }
            }
        }
    }
}

fn delay() {
    for _i in 0..2_000 {
        asm::nop();
    }
}

fn status(gpiox_idr: *mut u32, idrx: u32) -> Status {
    let check = 1 << idrx;
    let data = unsafe {
        ptr::read_volatile(gpiox_idr)
    };
    let result = check & data;

    if result == 0 {
        Status::Low
    } else {
        Status::High
    }
}