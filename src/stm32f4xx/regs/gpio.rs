
//! General Purpose I/O registers

use ::volatile_reg32::*;
use super::constants::{ GPIOA_BASE,
                        GPIOB_BASE,
                        GPIOC_BASE,
                        GPIOD_BASE,
                        GPIOE_BASE,
                        GPIOF_BASE,
                        GPIOG_BASE,
                        GPIOH_BASE,
                        GPIOI_BASE,
                        };

pub struct GPIO_Regs {
    /// GPIO port mode register,
    pub MODER   : VolatileReg32,

    /// GPIO port output type register,
    pub OTYPER  : VolatileReg32,

    /// GPIO port output speed register,
    pub OSPEEDR : VolatileReg32,

    /// GPIO port pull-up/pull-down register,
    pub PUPDR   : VolatileReg32,

    /// GPIO port input data register,
    pub IDR     : VolatileReg32,

    /// GPIO port output data register,
    pub ODR     : VolatileReg32,

    /// GPIO port bit set/reset low register,
    pub BSRRL   : VolatileReg32,

    /// GPIO port bit set/reset high register,
    pub BSRRH   : VolatileReg32,

    /// GPIO port configuration lock register,
    pub LCKR    : VolatileReg32,

    /// GPIO alternate function registers,
    pub AFR0    : VolatileReg32,

    /// GPIO alternate function registers,
    pub AFR1    : VolatileReg32,
}

pub enum GPIOPort {
    PortA,
    PortB,
    PortC,
    PortD,
    PortE,
    PortF,
    PortG,
    PortH,
    PortI,
}

impl GPIO_Regs {
    pub fn init(port: GPIOPort) -> GPIO_Regs {
        let gpio_base: *mut u32 = match port {
            GPIOPort::PortA => GPIOA_BASE,
            GPIOPort::PortB => GPIOB_BASE,
            GPIOPort::PortC => GPIOC_BASE,
            GPIOPort::PortD => GPIOD_BASE,
            GPIOPort::PortE => GPIOE_BASE,
            GPIOPort::PortF => GPIOF_BASE,
            GPIOPort::PortG => GPIOG_BASE,
            GPIOPort::PortH => GPIOH_BASE,
            GPIOPort::PortI => GPIOI_BASE,
        } as *mut u32;

        let gpio = GPIO_Regs {
            MODER   : VolatileReg32::new(gpio_base),
            OTYPER  : VolatileReg32::new_offset(gpio_base, 1),
            OSPEEDR : VolatileReg32::new_offset(gpio_base, 2),
            PUPDR   : VolatileReg32::new_offset(gpio_base, 3),
            IDR     : VolatileReg32::new_offset(gpio_base, 4),
            ODR     : VolatileReg32::new_offset(gpio_base, 5),
            BSRRL   : VolatileReg32::new_offset(gpio_base, 6),
            BSRRH   : VolatileReg32::new_offset(gpio_base, 7),
            LCKR    : VolatileReg32::new_offset(gpio_base, 8),
            AFR0    : VolatileReg32::new_offset(gpio_base, 9),
            AFR1    : VolatileReg32::new_offset(gpio_base, 10),
        };

        gpio
    }
}

// Bits definition for GPIO_MODER register
pub const GPIO_MODER_MODER0         : u32   = 0x00000003;
pub const GPIO_MODER_MODER0_0       : u32   = 0x00000001;
pub const GPIO_MODER_MODER0_1       : u32   = 0x00000002;

pub const GPIO_MODER_MODER1         : u32   = 0x0000000C;
pub const GPIO_MODER_MODER1_0       : u32   = 0x00000004;
pub const GPIO_MODER_MODER1_1       : u32   = 0x00000008;

pub const GPIO_MODER_MODER2         : u32   = 0x00000030;
pub const GPIO_MODER_MODER2_0       : u32   = 0x00000010;
pub const GPIO_MODER_MODER2_1       : u32   = 0x00000020;

pub const GPIO_MODER_MODER3         : u32   = 0x000000C0;
pub const GPIO_MODER_MODER3_0       : u32   = 0x00000040;
pub const GPIO_MODER_MODER3_1       : u32   = 0x00000080;

pub const GPIO_MODER_MODER4         : u32   = 0x00000300;
pub const GPIO_MODER_MODER4_0       : u32   = 0x00000100;
pub const GPIO_MODER_MODER4_1       : u32   = 0x00000200;

pub const GPIO_MODER_MODER5         : u32   = 0x00000C00;
pub const GPIO_MODER_MODER5_0       : u32   = 0x00000400;
pub const GPIO_MODER_MODER5_1       : u32   = 0x00000800;

pub const GPIO_MODER_MODER6         : u32   = 0x00003000;
pub const GPIO_MODER_MODER6_0       : u32   = 0x00001000;
pub const GPIO_MODER_MODER6_1       : u32   = 0x00002000;

pub const GPIO_MODER_MODER7         : u32   = 0x0000C000;
pub const GPIO_MODER_MODER7_0       : u32   = 0x00004000;
pub const GPIO_MODER_MODER7_1       : u32   = 0x00008000;

pub const GPIO_MODER_MODER8         : u32   = 0x00030000;
pub const GPIO_MODER_MODER8_0       : u32   = 0x00010000;
pub const GPIO_MODER_MODER8_1       : u32   = 0x00020000;

pub const GPIO_MODER_MODER9         : u32   = 0x000C0000;
pub const GPIO_MODER_MODER9_0       : u32   = 0x00040000;
pub const GPIO_MODER_MODER9_1       : u32   = 0x00080000;

pub const GPIO_MODER_MODER10        : u32   = 0x00300000;
pub const GPIO_MODER_MODER10_0      : u32   = 0x00100000;
pub const GPIO_MODER_MODER10_1      : u32   = 0x00200000;

pub const GPIO_MODER_MODER11        : u32   = 0x00C00000;
pub const GPIO_MODER_MODER11_0      : u32   = 0x00400000;
pub const GPIO_MODER_MODER11_1      : u32   = 0x00800000;

pub const GPIO_MODER_MODER12        : u32   = 0x03000000;
pub const GPIO_MODER_MODER12_0      : u32   = 0x01000000;
pub const GPIO_MODER_MODER12_1      : u32   = 0x02000000;

pub const GPIO_MODER_MODER13        : u32   = 0x0C000000;
pub const GPIO_MODER_MODER13_0      : u32   = 0x04000000;
pub const GPIO_MODER_MODER13_1      : u32   = 0x08000000;

pub const GPIO_MODER_MODER14        : u32   = 0x30000000;
pub const GPIO_MODER_MODER14_0      : u32   = 0x10000000;
pub const GPIO_MODER_MODER14_1      : u32   = 0x20000000;

pub const GPIO_MODER_MODER15        : u32   = 0xC0000000;
pub const GPIO_MODER_MODER15_0      : u32   = 0x40000000;
pub const GPIO_MODER_MODER15_1      : u32   = 0x80000000;

// Bits definition for GPIO_OTYPER register
pub const GPIO_OTYPER_OT_0          : u32   = 0x00000001;
pub const GPIO_OTYPER_OT_1          : u32   = 0x00000002;
pub const GPIO_OTYPER_OT_2          : u32   = 0x00000004;
pub const GPIO_OTYPER_OT_3          : u32   = 0x00000008;
pub const GPIO_OTYPER_OT_4          : u32   = 0x00000010;
pub const GPIO_OTYPER_OT_5          : u32   = 0x00000020;
pub const GPIO_OTYPER_OT_6          : u32   = 0x00000040;
pub const GPIO_OTYPER_OT_7          : u32   = 0x00000080;
pub const GPIO_OTYPER_OT_8          : u32   = 0x00000100;
pub const GPIO_OTYPER_OT_9          : u32   = 0x00000200;
pub const GPIO_OTYPER_OT_10         : u32   = 0x00000400;
pub const GPIO_OTYPER_OT_11         : u32   = 0x00000800;
pub const GPIO_OTYPER_OT_12         : u32   = 0x00001000;
pub const GPIO_OTYPER_OT_13         : u32   = 0x00002000;
pub const GPIO_OTYPER_OT_14         : u32   = 0x00004000;
pub const GPIO_OTYPER_OT_15         : u32   = 0x00008000;

// Bits definition for GPIO_OSPEEDR register
pub const GPIO_OSPEEDER_OSPEEDR0    : u32   = 0x00000003;
pub const GPIO_OSPEEDER_OSPEEDR0_0  : u32   = 0x00000001;
pub const GPIO_OSPEEDER_OSPEEDR0_1  : u32   = 0x00000002;

pub const GPIO_OSPEEDER_OSPEEDR1    : u32   = 0x0000000C;
pub const GPIO_OSPEEDER_OSPEEDR1_0  : u32   = 0x00000004;
pub const GPIO_OSPEEDER_OSPEEDR1_1  : u32   = 0x00000008;

pub const GPIO_OSPEEDER_OSPEEDR2    : u32   = 0x00000030;
pub const GPIO_OSPEEDER_OSPEEDR2_0  : u32   = 0x00000010;
pub const GPIO_OSPEEDER_OSPEEDR2_1  : u32   = 0x00000020;

pub const GPIO_OSPEEDER_OSPEEDR3    : u32   = 0x000000C0;
pub const GPIO_OSPEEDER_OSPEEDR3_0  : u32   = 0x00000040;
pub const GPIO_OSPEEDER_OSPEEDR3_1  : u32   = 0x00000080;

pub const GPIO_OSPEEDER_OSPEEDR4    : u32   = 0x00000300;
pub const GPIO_OSPEEDER_OSPEEDR4_0  : u32   = 0x00000100;
pub const GPIO_OSPEEDER_OSPEEDR4_1  : u32   = 0x00000200;

pub const GPIO_OSPEEDER_OSPEEDR5    : u32   = 0x00000C00;
pub const GPIO_OSPEEDER_OSPEEDR5_0  : u32   = 0x00000400;
pub const GPIO_OSPEEDER_OSPEEDR5_1  : u32   = 0x00000800;

pub const GPIO_OSPEEDER_OSPEEDR6    : u32   = 0x00003000;
pub const GPIO_OSPEEDER_OSPEEDR6_0  : u32   = 0x00001000;
pub const GPIO_OSPEEDER_OSPEEDR6_1  : u32   = 0x00002000;

pub const GPIO_OSPEEDER_OSPEEDR7    : u32   = 0x0000C000;
pub const GPIO_OSPEEDER_OSPEEDR7_0  : u32   = 0x00004000;
pub const GPIO_OSPEEDER_OSPEEDR7_1  : u32   = 0x00008000;

pub const GPIO_OSPEEDER_OSPEEDR8    : u32   = 0x00030000;
pub const GPIO_OSPEEDER_OSPEEDR8_0  : u32   = 0x00010000;
pub const GPIO_OSPEEDER_OSPEEDR8_1  : u32   = 0x00020000;

pub const GPIO_OSPEEDER_OSPEEDR9    : u32   = 0x000C0000;
pub const GPIO_OSPEEDER_OSPEEDR9_0  : u32   = 0x00040000;
pub const GPIO_OSPEEDER_OSPEEDR9_1  : u32   = 0x00080000;

pub const GPIO_OSPEEDER_OSPEEDR10   : u32   = 0x00300000;
pub const GPIO_OSPEEDER_OSPEEDR10_0 : u32   = 0x00100000;
pub const GPIO_OSPEEDER_OSPEEDR10_1 : u32   = 0x00200000;

pub const GPIO_OSPEEDER_OSPEEDR11   : u32   = 0x00C00000;
pub const GPIO_OSPEEDER_OSPEEDR11_0 : u32   = 0x00400000;
pub const GPIO_OSPEEDER_OSPEEDR11_1 : u32   = 0x00800000;

pub const GPIO_OSPEEDER_OSPEEDR12   : u32   = 0x03000000;
pub const GPIO_OSPEEDER_OSPEEDR12_0 : u32   = 0x01000000;
pub const GPIO_OSPEEDER_OSPEEDR12_1 : u32   = 0x02000000;

pub const GPIO_OSPEEDER_OSPEEDR13   : u32   = 0x0C000000;
pub const GPIO_OSPEEDER_OSPEEDR13_0 : u32   = 0x04000000;
pub const GPIO_OSPEEDER_OSPEEDR13_1 : u32   = 0x08000000;

pub const GPIO_OSPEEDER_OSPEEDR14   : u32   = 0x30000000;
pub const GPIO_OSPEEDER_OSPEEDR14_0 : u32   = 0x10000000;
pub const GPIO_OSPEEDER_OSPEEDR14_1 : u32   = 0x20000000;

pub const GPIO_OSPEEDER_OSPEEDR15   : u32   = 0xC0000000;
pub const GPIO_OSPEEDER_OSPEEDR15_0 : u32   = 0x40000000;
pub const GPIO_OSPEEDER_OSPEEDR15_1 : u32   = 0x80000000;

// Bits definition for GPIO_PUPDR register
pub const GPIO_PUPDR_PUPDR0         : u32   = 0x00000003;
pub const GPIO_PUPDR_PUPDR0_0       : u32   = 0x00000001;
pub const GPIO_PUPDR_PUPDR0_1       : u32   = 0x00000002;

pub const GPIO_PUPDR_PUPDR1         : u32   = 0x0000000C;
pub const GPIO_PUPDR_PUPDR1_0       : u32   = 0x00000004;
pub const GPIO_PUPDR_PUPDR1_1       : u32   = 0x00000008;

pub const GPIO_PUPDR_PUPDR2         : u32   = 0x00000030;
pub const GPIO_PUPDR_PUPDR2_0       : u32   = 0x00000010;
pub const GPIO_PUPDR_PUPDR2_1       : u32   = 0x00000020;

pub const GPIO_PUPDR_PUPDR3         : u32   = 0x000000C0;
pub const GPIO_PUPDR_PUPDR3_0       : u32   = 0x00000040;
pub const GPIO_PUPDR_PUPDR3_1       : u32   = 0x00000080;

pub const GPIO_PUPDR_PUPDR4         : u32   = 0x00000300;
pub const GPIO_PUPDR_PUPDR4_0       : u32   = 0x00000100;
pub const GPIO_PUPDR_PUPDR4_1       : u32   = 0x00000200;

pub const GPIO_PUPDR_PUPDR5         : u32   = 0x00000C00;
pub const GPIO_PUPDR_PUPDR5_0       : u32   = 0x00000400;
pub const GPIO_PUPDR_PUPDR5_1       : u32   = 0x00000800;

pub const GPIO_PUPDR_PUPDR6         : u32   = 0x00003000;
pub const GPIO_PUPDR_PUPDR6_0       : u32   = 0x00001000;
pub const GPIO_PUPDR_PUPDR6_1       : u32   = 0x00002000;

pub const GPIO_PUPDR_PUPDR7         : u32   = 0x0000C000;
pub const GPIO_PUPDR_PUPDR7_0       : u32   = 0x00004000;
pub const GPIO_PUPDR_PUPDR7_1       : u32   = 0x00008000;

pub const GPIO_PUPDR_PUPDR8         : u32   = 0x00030000;
pub const GPIO_PUPDR_PUPDR8_0       : u32   = 0x00010000;
pub const GPIO_PUPDR_PUPDR8_1       : u32   = 0x00020000;

pub const GPIO_PUPDR_PUPDR9         : u32   = 0x000C0000;
pub const GPIO_PUPDR_PUPDR9_0       : u32   = 0x00040000;
pub const GPIO_PUPDR_PUPDR9_1       : u32   = 0x00080000;

pub const GPIO_PUPDR_PUPDR10        : u32   = 0x00300000;
pub const GPIO_PUPDR_PUPDR10_0      : u32   = 0x00100000;
pub const GPIO_PUPDR_PUPDR10_1      : u32   = 0x00200000;

pub const GPIO_PUPDR_PUPDR11        : u32   = 0x00C00000;
pub const GPIO_PUPDR_PUPDR11_0      : u32   = 0x00400000;
pub const GPIO_PUPDR_PUPDR11_1      : u32   = 0x00800000;

pub const GPIO_PUPDR_PUPDR12        : u32   = 0x03000000;
pub const GPIO_PUPDR_PUPDR12_0      : u32   = 0x01000000;
pub const GPIO_PUPDR_PUPDR12_1      : u32   = 0x02000000;

pub const GPIO_PUPDR_PUPDR13        : u32   = 0x0C000000;
pub const GPIO_PUPDR_PUPDR13_0      : u32   = 0x04000000;
pub const GPIO_PUPDR_PUPDR13_1      : u32   = 0x08000000;

pub const GPIO_PUPDR_PUPDR14        : u32   = 0x30000000;
pub const GPIO_PUPDR_PUPDR14_0      : u32   = 0x10000000;
pub const GPIO_PUPDR_PUPDR14_1      : u32   = 0x20000000;

pub const GPIO_PUPDR_PUPDR15        : u32   = 0xC0000000;
pub const GPIO_PUPDR_PUPDR15_0      : u32   = 0x40000000;
pub const GPIO_PUPDR_PUPDR15_1      : u32   = 0x80000000;

// Bits definition for GPIO_IDR register
pub const GPIO_IDR_IDR_0            : u32   = 0x00000001;
pub const GPIO_IDR_IDR_1            : u32   = 0x00000002;
pub const GPIO_IDR_IDR_2            : u32   = 0x00000004;
pub const GPIO_IDR_IDR_3            : u32   = 0x00000008;
pub const GPIO_IDR_IDR_4            : u32   = 0x00000010;
pub const GPIO_IDR_IDR_5            : u32   = 0x00000020;
pub const GPIO_IDR_IDR_6            : u32   = 0x00000040;
pub const GPIO_IDR_IDR_7            : u32   = 0x00000080;
pub const GPIO_IDR_IDR_8            : u32   = 0x00000100;
pub const GPIO_IDR_IDR_9            : u32   = 0x00000200;
pub const GPIO_IDR_IDR_10           : u32   = 0x00000400;
pub const GPIO_IDR_IDR_11           : u32   = 0x00000800;
pub const GPIO_IDR_IDR_12           : u32   = 0x00001000;
pub const GPIO_IDR_IDR_13           : u32   = 0x00002000;
pub const GPIO_IDR_IDR_14           : u32   = 0x00004000;
pub const GPIO_IDR_IDR_15           : u32   = 0x00008000;

// Bits definition for GPIO_ODR register
pub const GPIO_ODR_ODR_0            : u32   = 0x00000001;
pub const GPIO_ODR_ODR_1            : u32   = 0x00000002;
pub const GPIO_ODR_ODR_2            : u32   = 0x00000004;
pub const GPIO_ODR_ODR_3            : u32   = 0x00000008;
pub const GPIO_ODR_ODR_4            : u32   = 0x00000010;
pub const GPIO_ODR_ODR_5            : u32   = 0x00000020;
pub const GPIO_ODR_ODR_6            : u32   = 0x00000040;
pub const GPIO_ODR_ODR_7            : u32   = 0x00000080;
pub const GPIO_ODR_ODR_8            : u32   = 0x00000100;
pub const GPIO_ODR_ODR_9            : u32   = 0x00000200;
pub const GPIO_ODR_ODR_10           : u32   = 0x00000400;
pub const GPIO_ODR_ODR_11           : u32   = 0x00000800;
pub const GPIO_ODR_ODR_12           : u32   = 0x00001000;
pub const GPIO_ODR_ODR_13           : u32   = 0x00002000;
pub const GPIO_ODR_ODR_14           : u32   = 0x00004000;
pub const GPIO_ODR_ODR_15           : u32   = 0x00008000;

// Bits definition for GPIO_BSRR register
pub const GPIO_BSRR_BS_0            : u32   = 0x00000001;
pub const GPIO_BSRR_BS_1            : u32   = 0x00000002;
pub const GPIO_BSRR_BS_2            : u32   = 0x00000004;
pub const GPIO_BSRR_BS_3            : u32   = 0x00000008;
pub const GPIO_BSRR_BS_4            : u32   = 0x00000010;
pub const GPIO_BSRR_BS_5            : u32   = 0x00000020;
pub const GPIO_BSRR_BS_6            : u32   = 0x00000040;
pub const GPIO_BSRR_BS_7            : u32   = 0x00000080;
pub const GPIO_BSRR_BS_8            : u32   = 0x00000100;
pub const GPIO_BSRR_BS_9            : u32   = 0x00000200;
pub const GPIO_BSRR_BS_10           : u32   = 0x00000400;
pub const GPIO_BSRR_BS_11           : u32   = 0x00000800;
pub const GPIO_BSRR_BS_12           : u32   = 0x00001000;
pub const GPIO_BSRR_BS_13           : u32   = 0x00002000;
pub const GPIO_BSRR_BS_14           : u32   = 0x00004000;
pub const GPIO_BSRR_BS_15           : u32   = 0x00008000;
pub const GPIO_BSRR_BR_0            : u32   = 0x00010000;
pub const GPIO_BSRR_BR_1            : u32   = 0x00020000;
pub const GPIO_BSRR_BR_2            : u32   = 0x00040000;
pub const GPIO_BSRR_BR_3            : u32   = 0x00080000;
pub const GPIO_BSRR_BR_4            : u32   = 0x00100000;
pub const GPIO_BSRR_BR_5            : u32   = 0x00200000;
pub const GPIO_BSRR_BR_6            : u32   = 0x00400000;
pub const GPIO_BSRR_BR_7            : u32   = 0x00800000;
pub const GPIO_BSRR_BR_8            : u32   = 0x01000000;
pub const GPIO_BSRR_BR_9            : u32   = 0x02000000;
pub const GPIO_BSRR_BR_10           : u32   = 0x04000000;
pub const GPIO_BSRR_BR_11           : u32   = 0x08000000;
pub const GPIO_BSRR_BR_12           : u32   = 0x10000000;
pub const GPIO_BSRR_BR_13           : u32   = 0x20000000;
pub const GPIO_BSRR_BR_14           : u32   = 0x40000000;
pub const GPIO_BSRR_BR_15           : u32   = 0x80000000;

