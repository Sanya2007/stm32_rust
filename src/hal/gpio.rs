#![allow(dead_code)]

use ::stm32f4xx::regs::gpio::*;
use ::hal::peripheral_clock::PeripheralClock;

impl Port {
    pub fn get_clock(&self) -> PeripheralClock {
        match self {
            &Port::GpioA => PeripheralClock::GpioA,
            &Port::GpioB => PeripheralClock::GpioB,
            &Port::GpioC => PeripheralClock::GpioC,
            &Port::GpioD => PeripheralClock::GpioD,
            &Port::GpioE => PeripheralClock::GpioE,
            &Port::GpioF => PeripheralClock::GpioF,
            &Port::GpioG => PeripheralClock::GpioG,
            &Port::GpioH => PeripheralClock::GpioH,
            &Port::GpioI => PeripheralClock::GpioI,
        }
    }
}

pub struct Pin {
    pub port:       Port,
    pub pin_num:    u8,
    pub mode:       PinMode,
}

impl Pin {
    pub fn init(port: Port, pin_num: u8) -> Pin {
        Pin {
            port:       port,
            pin_num:    pin_num,
            mode:       PinMode::Input,
        }
    }

    pub fn enable_clock(&self) {
        self.port.get_clock().enable();
    }

    pub fn setup_mode(&mut self, mode: PinMode) {
        let gpio = GpioRegs::init(&self.port);
        let mut mask: u32 = 0b11 << (2 * self.pin_num);
        mask = !mask;

        let val: u32 = (mode as u32) << (2 * self.pin_num);
        let mut mode_reg = gpio.moder.get();

        mode_reg &= mask;
        mode_reg |= val;
        gpio.moder.set(mode_reg);

        self.mode = mode;

    }
    pub fn setup_output_type(&self, output_type: PinOutputType) {
        let gpio = GpioRegs::init(&self.port);

        let val = (1 as u32) << self.pin_num;

        if output_type == PinOutputType::OpenDrain {
            gpio.otyper.bit_or(val);
        } else if output_type == PinOutputType::PushPull{
            gpio.otyper.bit_and(!val);
        }
    }

    pub fn setup_speed(&self, speed: PinSpeed) {
        let gpio = GpioRegs::init(&self.port);
        gpio.ospeedr.bit_or((speed as u32) << (2 * self.pin_num));
    }

    pub fn setup_pull_up_down(&self, pull_up_down: PinPullUpDown) {
        let gpio = GpioRegs::init(&self.port);
        gpio.pupdr.bit_or((pull_up_down as u32) << (2 * self.pin_num));
    }

    pub fn setup_alt_func(&self, alt_func: u32) {
        let gpio = GpioRegs::init(&self.port);
        if self.pin_num < 8 {
            gpio.afrl.bit_or(alt_func << (self.pin_num * 4));
        } else {
            gpio.afrh.bit_or(alt_func << ((self.pin_num - 8) * 4));
        }
    }

    /// Set the value of the pin
    pub fn set(&self, val: bool) {
        let regs = GpioRegs::init(&self.port);
        if val {
            regs.bsrr.set(1 << self.pin_num);
        } else {
            regs.bsrr.set(1 << (self.pin_num + 16));
        }
    }

    /// Returns the value of the pin
    pub fn get(&self) -> u32 {
        let regs = GpioRegs::init(&self.port);
        let mut val: u32;

        val = regs.idr.get();
        val >>= self.pin_num;
        val &= 1;

        val
    }

    /// Changes the current value of the pin, and returns the new value
    pub fn toggle(&self) -> u32 {
        if self.get() == 1 {
            self.set(false);
            0
        } else {
            self.set(true);
            1
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PinMode {
    Input   = 0x00,
    Output  = 0x01,
    AltFunc = 0x02,
    Analog  = 0x03,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PinOutputType {
    PushPull    = 0x00,
    OpenDrain   = 0x01,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PinSpeed {
    Low         = 0x00,
    Medium      = 0x01,
    High        = 0x02,
    VeryHigh    = 0x03,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PinPullUpDown {
    NoPullUpDown    = 0x00,
    PullUp          = 0x01,
    PullDown        = 0x02,
}
