#![no_std]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_intrinsics)]


mod volatile_reg32;
use volatile_reg32::*;

mod stm32f4xx_const;
use stm32f4xx_const::*;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() -> ! {
    loop {}
}


#[no_mangle]
pub extern fn ms_delay(mut ms: u32) {
    while ms > 0 {
        ms -= 1;
        let mut i: u32 = 59710;
        while i > 0 {
            i -= 1;
            unsafe {
                asm!("nop");
            }
        }
    }
}

#[no_mangle]
pub extern fn main() {
    // enable the clock to GPIOD
    let RCC = RCC_Regs::init();
    RCC.AHB1ENR.bit_or(RCC_AHB1ENR_GPIODEN);

    //*AHB1ENR |= RCC_AHB1ENR_GPIODEN;
    // stall instruction pipeline, until instruction completes, as
    // per Errata 2.1.13, "Delay after an RCC peripheral clock enabling"
    unsafe {
        asm!("dsb");
    }

    let GPIOD = GPIO_Regs::init(GPIOPort::PortD);
    GPIOD.MODER.set(1 << 26);

    loop {
        ms_delay(500);
        GPIOD.ODR.bit_xor(1 << 13);
    }

}

