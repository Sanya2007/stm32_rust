#![no_std]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_intrinsics)]


mod volatile_reg32;
mod stm32f4xx;
mod core_cm4;
mod hal;

use core_cm4::core_cm4_regs::scb::*;

use stm32f4xx::regs::flash::*;
use stm32f4xx::regs::gpio::*;
use stm32f4xx::regs::pwr::*;
use stm32f4xx::regs::rcc::*;
use stm32f4xx::regs::tim::*;
use hal::gpio;
use hal::peripheral_clock;


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
pub extern fn main() {

    // Green LED
    let mut pin_d12 = gpio::Pin::init(Port::GpioD, 12);
    pin_d12.enable_clock();
    pin_d12.setup_mode(gpio::PinMode::Output);

    // Orange LED
    let mut pin_d13 = gpio::Pin::init(Port::GpioD, 13);
    pin_d13.setup_mode(gpio::PinMode::Output);

    // Red LED
    let mut pin_d14 = gpio::Pin::init(Port::GpioD, 14);
    pin_d14.setup_mode(gpio::PinMode::Output);

    // Blue LED
    let mut pin_d15 = gpio::Pin::init(Port::GpioD, 15);
    pin_d15.setup_mode(gpio::PinMode::Output);

    // Enable clock for TIMER7
    {
        let clock_tim7 = peripheral_clock::PeripheralClock::Tim7;
        clock_tim7.enable();
    }
    let tim7_regs = TimRegs::init(TimInst::TIM7);
    tim7_regs.cr1.bit_or(TIM_CR1_OPM | TIM_CR1_URS);
    tim7_regs.psc.set(42_000);
    tim7_regs.arr.set(2);

    let wait_ms = |time: u32| {
        tim7_regs.arr.set(2 * time);
        tim7_regs.cr1.bit_or(TIM_CR1_CEN);
        while tim7_regs.sr.get() == 0 {

        }
        tim7_regs.sr.set(0);
    };

/*
    // Setup pins for SPI
    let mut reset = gpio::Pin::init(Port::GpioB, 10);
    reset.enable_clock();
    reset.setup_mode(gpio::PinMode::Output);
    reset.setup_speed(gpio::PinSpeed::High);

    let mut dc = gpio::Pin::init(Port::GpioB, 11);
    reset.setup_mode(gpio::PinMode::Output);
    reset.setup_speed(gpio::PinSpeed::High);

    {
        let mut cs = gpio::Pin::init(Port::GpioB, 12);
        cs.setup_alt_func(5);
        cs.setup_speed(gpio::PinSpeed::High);
        cs.setup_mode(gpio::PinMode::AltFunc);

        let mut sck = gpio::Pin::init(Port::GpioB, 13);
        sck.setup_alt_func(5);
        sck.setup_speed(gpio::PinSpeed::High);
        sck.setup_mode(gpio::PinMode::AltFunc);

        let mut miso = gpio::Pin::init(Port::GpioB, 14);
        miso.setup_alt_func(5);
        miso.setup_speed(gpio::PinSpeed::High);
        miso.setup_mode(gpio::PinMode::AltFunc);

        let mut mosi = gpio::Pin::init(Port::GpioB, 15);
        mosi.setup_alt_func(5);
        mosi.setup_speed(gpio::PinSpeed::High);
        mosi.setup_mode(gpio::PinMode::AltFunc);
    }
*/


    loop {
        wait_ms(500);
        pin_d12.toggle();

        wait_ms(500);
        pin_d13.toggle();

        wait_ms(500);
        pin_d14.toggle();

        wait_ms(500);
        pin_d15.toggle();
    }
}

#[no_mangle]
pub extern fn system_init() {

    let rcc = RccRegs::init();
    let scb = ScbRegs::init();

    // /* FPU settings ------------------------------------------------------------*/
    // #if (__FPU_PRESENT == 1) && (__FPU_USED == 1)
    //     SCB->CPACR |= ((3UL << 10*2)|(3UL << 11*2));  /* set CP10 and CP11 Full Access */
    // #endif
    scb.cpacr.bit_or((3 << 10 * 2) | (3 << 11 * 2));


    // Reset the RCC clock configuration to the default reset state
    // Set HSION bit
    // RCC->CR |= (uint32_t)0x00000001;
    rcc.cr.bit_or(0x00000001);

    // Reset CFGR register
    // RCC->CFGR = 0x00000000;
    rcc.cfgr.set(0x00000000);

    // Reset HSEON, CSSON and PLLON bits
    // RCC->CR &= (uint32_t)0xFEF6FFFF;
    rcc.cr.bit_and(0xFEF6FFFF);

    // Reset PLLCFGR register
    // RCC->PLLCFGR = 0x24003010;
    rcc.pllcfgr.set(0x24003010);

    // /* Reset HSEBYP bit */
    // RCC->CR &= (uint32_t)0xFFFBFFFF;
    rcc.cr.bit_and(0xFFFBFFFF);

    // Disable all interrupts
    // RCC->CIR = 0x00000000;
    rcc.cir.set(0x00000000);

    // #ifdef DATA_IN_ExtSRAM
    //     SystemInit_ExtMemCtl();
    // #endif /* DATA_IN_ExtSRAM */

    // /* Configure the System clock source, PLL Multiplier and Divider factors,
    //    AHB/APBx prescalers and Flash settings ----------------------------------*/
    // set_sys_clock();
    set_sys_clock();

    // /* Configure the Vector Table location add offset address ------------------*/
    // #ifdef VECT_TAB_SRAM
    //     SCB->VTOR = SRAM_BASE | VECT_TAB_OFFSET; /* Vector Table Relocation in Internal SRAM */
    // #else
    //     SCB->VTOR = FLASH_BASE | VECT_TAB_OFFSET; /* Vector Table Relocation in Internal FLASH */
    // #endif
    scb.vtor.set(stm32f4xx::constants::FLASH_BASE);
}


#[no_mangle]
pub extern fn set_sys_clock() {
/******************************************************************************/
/*            PLL (clocked by HSE) used as System clock source                */
/******************************************************************************/
    const PLL_M: u32 = 8;
    const PLL_N: u32 = 336;
    const PLL_P: u32 = 2;
    const PLL_Q: u32 = 7;

    let mut start_up_counter: u32 = 0;
    let hse_status: bool;

    let rcc = RccRegs::init();
    let pwr = PwrRegs::init();
    let pllcfgr: u32;
    let flash = FlashRegs::init();

    // Enable HSE
    // RCC->CR |= ((uint32_t)RCC_CR_HSEON);
    rcc.cr.bit_or(RCC_CR_HSEON);

    // Wait till HSE is ready and if Time out is reached exit

    while rcc.cr.get() & RCC_CR_HSERDY == 0 && start_up_counter < 0x500 {
        start_up_counter += 1;
    }

    if rcc.cr.get() & RCC_CR_HSERDY != 0 {
        hse_status = true;
    } else {
        hse_status = false;
    }

    if hse_status {
        // Select regulator voltage output Scale 1 mode, System frequency up to 168 MHz
        rcc.apb1enr.bit_or(RCC_APB1ENR_PWREN);
        pwr.cr.bit_or(PWR_CR_VOS);

        // HCLK = SYSCLK / 1
        rcc.cfgr.bit_or(RCC_CFGR_HPRE_DIV1);

        // PCLK2 = HCLK / 2
        rcc.cfgr.bit_or(RCC_CFGR_PPRE2_DIV2);

        // PCLK1 = HCLK / 4
        rcc.cfgr.bit_or(RCC_CFGR_PPRE1_DIV4);

        // Configure the main PLL
        pllcfgr = PLL_M | (PLL_N << 6) | (((PLL_P >> 1) -1) << 16) | (RCC_PLLCFGR_PLLSRC_HSE) | (PLL_Q << 24);
        rcc.pllcfgr.set(pllcfgr);

        // Enable the main PLL
        rcc.cr.bit_or(RCC_CR_PLLON);

        // Wait till the main PLL is ready
        while rcc.cr.get() & RCC_CR_PLLRDY == 0 {
        }

        // Configure Flash prefetch, Instruction cache, Data cache and wait state
        flash.acr.set(FLASH_ACR_ICEN | FLASH_ACR_DCEN | FLASH_ACR_LATENCY_5WS);

        // Select the main PLL as system clock source
        rcc.cfgr.bit_and(!RCC_CFGR_SW);
        rcc.cfgr.bit_or(RCC_CFGR_SW_PLL);

        // Wait till the main PLL is used as system clock source
        while rcc.cfgr.get() & RCC_CFGR_SWS != RCC_CFGR_SWS_PLL {
        }
    } else { /* If HSE fails to start-up, the application will have wrong clock
           configuration. User can add here some code to deal with this error */
    }

}

