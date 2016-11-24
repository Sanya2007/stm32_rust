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

    pub fn setup(&mut self, mode: PinMode, output_type: PinOutputType,
                speed: PinSpeedType, pull_up_down: PinPullUpDown) {

        self.port.get_clock().enable();

        let gpio = GpioRegs::init(&self.port);
        let mut mask: u32 = 0b11 << (2 * self.pin_num);
        mask = !mask;

        let mut val: u32 = (mode as u32) << (2 * self.pin_num);
        let mut reg = gpio.moder.get();
        self.mode = mode;

        reg &= mask;
        reg |= val;
        gpio.moder.set(reg);

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
pub enum PinSpeedType {
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
