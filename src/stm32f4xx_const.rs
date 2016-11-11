#![allow(dead_code)]

//! stm32f4xx_const module
//! This module contans main constants for stm32f4xx series MCUs

use ::volatile_reg32::*;

/// FLASH(up to 1 MB) base address in the alias region
pub const FLASH_BASE            : u32   = 0x08000000;

/// CCM(core coupled memory) data RAM(64 KB) base address in the alias region
pub const CCMDATARAM_BASE       : u32   = 0x10000000;

/// SRAM1(112 KB) base address in the alias region
pub const SRAM1_BASE            : u32   = 0x20000000;

/// SRAM2(16 KB) base address in the alias region
pub const SRAM2_BASE            : u32	= 0x2001C000;

/// Peripheral base address in the alias region
pub const PERIPH_BASE           : u32	= 0x40000000;

/// Backup SRAM(4 KB) base address in the alias region
pub const BKPSRAM_BASE          : u32	= 0x40024000;

/// FSMC registers base address
pub const FSMC_R_BASE           : u32	= 0xA0000000;

/// CCM(core coupled memory) data RAM(64 KB) base address in the bit-band region
pub const CCMDATARAM_BB_BASE    : u32	= 0x12000000;

/// SRAM1(112 KB) base address in the bit-band region
pub const SRAM1_BB_BASE         : u32	= 0x22000000;

/// SRAM2(16 KB) base address in the bit-band region
pub const SRAM2_BB_BASE         : u32	= 0x2201C000;

/// Peripheral base address in the bit-band region
pub const PERIPH_BB_BASE        : u32	= 0x42000000;

/// Backup SRAM(4 KB) base address in the bit-band region
pub const BKPSRAM_BB_BASE       : u32	= 0x42024000;

/// Peripheral memory map
pub const APB1PERIPH_BASE       : u32   = PERIPH_BASE;
pub const APB2PERIPH_BASE       : u32   = PERIPH_BASE + 0x00010000;
pub const AHB1PERIPH_BASE       : u32   = PERIPH_BASE + 0x00020000;
pub const AHB2PERIPH_BASE       : u32   = PERIPH_BASE + 0x10000000;

/// APB1 peripherals
pub const TIM2_BASE             : u32   = APB1PERIPH_BASE + 0x0000;
pub const TIM3_BASE             : u32   = APB1PERIPH_BASE + 0x0400;
pub const TIM4_BASE             : u32   = APB1PERIPH_BASE + 0x0800;
pub const TIM5_BASE             : u32   = APB1PERIPH_BASE + 0x0C00;
pub const TIM6_BASE             : u32   = APB1PERIPH_BASE + 0x1000;
pub const TIM7_BASE             : u32   = APB1PERIPH_BASE + 0x1400;
pub const TIM12_BASE            : u32   = APB1PERIPH_BASE + 0x1800;
pub const TIM13_BASE            : u32   = APB1PERIPH_BASE + 0x1C00;
pub const TIM14_BASE            : u32   = APB1PERIPH_BASE + 0x2000;
pub const RTC_BASE              : u32   = APB1PERIPH_BASE + 0x2800;
pub const WWDG_BASE             : u32   = APB1PERIPH_BASE + 0x2C00;
pub const IWDG_BASE             : u32   = APB1PERIPH_BASE + 0x3000;
pub const I2S2ext_BASE          : u32   = APB1PERIPH_BASE + 0x3400;
pub const SPI2_BASE             : u32   = APB1PERIPH_BASE + 0x3800;
pub const SPI3_BASE             : u32   = APB1PERIPH_BASE + 0x3C00;
pub const I2S3ext_BASE          : u32   = APB1PERIPH_BASE + 0x4000;
pub const USART2_BASE           : u32   = APB1PERIPH_BASE + 0x4400;
pub const USART3_BASE           : u32   = APB1PERIPH_BASE + 0x4800;
pub const UART4_BASE            : u32   = APB1PERIPH_BASE + 0x4C00;
pub const UART5_BASE            : u32   = APB1PERIPH_BASE + 0x5000;
pub const I2C1_BASE             : u32   = APB1PERIPH_BASE + 0x5400;
pub const I2C2_BASE             : u32   = APB1PERIPH_BASE + 0x5800;
pub const I2C3_BASE             : u32   = APB1PERIPH_BASE + 0x5C00;
pub const CAN1_BASE             : u32   = APB1PERIPH_BASE + 0x6400;
pub const CAN2_BASE             : u32   = APB1PERIPH_BASE + 0x6800;
pub const PWR_BASE              : u32   = APB1PERIPH_BASE + 0x7000;
pub const DAC_BASE              : u32   = APB1PERIPH_BASE + 0x7400;

/// APB2 peripherals
pub const TIM1_BASE             : u32   = APB2PERIPH_BASE + 0x0000;
pub const TIM8_BASE             : u32   = APB2PERIPH_BASE + 0x0400;
pub const USART1_BASE           : u32   = APB2PERIPH_BASE + 0x1000;
pub const USART6_BASE           : u32   = APB2PERIPH_BASE + 0x1400;
pub const ADC1_BASE             : u32   = APB2PERIPH_BASE + 0x2000;
pub const ADC2_BASE             : u32   = APB2PERIPH_BASE + 0x2100;
pub const ADC3_BASE             : u32   = APB2PERIPH_BASE + 0x2200;
pub const ADC_BASE              : u32   = APB2PERIPH_BASE + 0x2300;
pub const SDIO_BASE             : u32   = APB2PERIPH_BASE + 0x2C00;
pub const SPI1_BASE             : u32   = APB2PERIPH_BASE + 0x3000;
pub const SYSCFG_BASE           : u32   = APB2PERIPH_BASE + 0x3800;
pub const EXTI_BASE             : u32   = APB2PERIPH_BASE + 0x3C00;
pub const TIM9_BASE             : u32   = APB2PERIPH_BASE + 0x4000;
pub const TIM10_BASE            : u32   = APB2PERIPH_BASE + 0x4400;
pub const TIM11_BASE            : u32   = APB2PERIPH_BASE + 0x4800;

/// AHB1 peripherals
pub const GPIOA_BASE            : u32   = AHB1PERIPH_BASE + 0x0000;
pub const GPIOB_BASE            : u32   = AHB1PERIPH_BASE + 0x0400;
pub const GPIOC_BASE            : u32   = AHB1PERIPH_BASE + 0x0800;
pub const GPIOD_BASE            : u32   = AHB1PERIPH_BASE + 0x0C00;
pub const GPIOE_BASE            : u32   = AHB1PERIPH_BASE + 0x1000;
pub const GPIOF_BASE            : u32   = AHB1PERIPH_BASE + 0x1400;
pub const GPIOG_BASE            : u32   = AHB1PERIPH_BASE + 0x1800;
pub const GPIOH_BASE            : u32   = AHB1PERIPH_BASE + 0x1C00;
pub const GPIOI_BASE            : u32   = AHB1PERIPH_BASE + 0x2000;
pub const CRC_BASE              : u32   = AHB1PERIPH_BASE + 0x3000;
pub const RCC_BASE              : u32   = AHB1PERIPH_BASE + 0x3800;
pub const FLASH_R_BASE          : u32   = AHB1PERIPH_BASE + 0x3C00;
pub const DMA1_BASE             : u32   = AHB1PERIPH_BASE + 0x6000;
pub const DMA1_Stream0_BASE     : u32   = DMA1_BASE + 0x010;
pub const DMA1_Stream1_BASE     : u32   = DMA1_BASE + 0x028;
pub const DMA1_Stream2_BASE     : u32   = DMA1_BASE + 0x040;
pub const DMA1_Stream3_BASE     : u32   = DMA1_BASE + 0x058;
pub const DMA1_Stream4_BASE     : u32   = DMA1_BASE + 0x070;
pub const DMA1_Stream5_BASE     : u32   = DMA1_BASE + 0x088;
pub const DMA1_Stream6_BASE     : u32   = DMA1_BASE + 0x0A0;
pub const DMA1_Stream7_BASE     : u32   = DMA1_BASE + 0x0B8;
pub const DMA2_BASE             : u32   = AHB1PERIPH_BASE + 0x6400;
pub const DMA2_Stream0_BASE     : u32   = DMA2_BASE + 0x010;
pub const DMA2_Stream1_BASE     : u32   = DMA2_BASE + 0x028;
pub const DMA2_Stream2_BASE     : u32   = DMA2_BASE + 0x040;
pub const DMA2_Stream3_BASE     : u32   = DMA2_BASE + 0x058;
pub const DMA2_Stream4_BASE     : u32   = DMA2_BASE + 0x070;
pub const DMA2_Stream5_BASE     : u32   = DMA2_BASE + 0x088;
pub const DMA2_Stream6_BASE     : u32   = DMA2_BASE + 0x0A0;
pub const DMA2_Stream7_BASE     : u32   = DMA2_BASE + 0x0B8;
pub const ETH_BASE              : u32   = AHB1PERIPH_BASE + 0x8000;
pub const ETH_MAC_BASE          : u32   = ETH_BASE;
pub const ETH_MMC_BASE          : u32   = ETH_BASE + 0x0100;
pub const ETH_PTP_BASE          : u32   = ETH_BASE + 0x0700;
pub const ETH_DMA_BASE          : u32   = ETH_BASE + 0x1000;

/// AHB2 peripherals
pub const DCMI_BASE             : u32   = AHB2PERIPH_BASE + 0x50000;
pub const CRYP_BASE             : u32   = AHB2PERIPH_BASE + 0x60000;
pub const HASH_BASE             : u32   = AHB2PERIPH_BASE + 0x60400;
pub const RNG_BASE              : u32   = AHB2PERIPH_BASE + 0x60800;

/// FSMC Bankx registers base address
pub const FSMC_Bank1_R_BASE     : u32   = FSMC_R_BASE + 0x0000;
pub const FSMC_Bank1E_R_BASE    : u32   = FSMC_R_BASE + 0x0104;
pub const FSMC_Bank2_R_BASE     : u32   = FSMC_R_BASE + 0x0060;
pub const FSMC_Bank3_R_BASE     : u32   = FSMC_R_BASE + 0x0080;
pub const FSMC_Bank4_R_BASE     : u32   = FSMC_R_BASE + 0x00A0;

/// Debug MCU registers base address
pub const DBGMCU_BASE           : u32   = 0xE0042000;


pub struct RCC_Regs {
    /// RCC clock control register,
    /// Address offset: 0x00
    pub CR              : VolatileReg32,

    /// RCC PLL configuration register,
    /// Address offset: 0x04
    pub PLLCFGR         : VolatileReg32,

    /// RCC clock configuration register,
    /// Address offset: 0x08
    pub CFGR            : VolatileReg32,

    /// RCC clock interrupt register,
    /// Address offset: 0x0C
    pub CIR             : VolatileReg32,

    /// RCC AHB1 peripheral reset register,
    /// Address offset: 0x10
    pub AHB1RSTR        : VolatileReg32,

    /// RCC AHB2 peripheral reset register,
    /// Address offset: 0x14
    pub AHB2RSTR        : VolatileReg32,

    /// RCC AHB3 peripheral reset register,
    /// Address offset: 0x18
    pub AHB3RSTR        : VolatileReg32,

    // Reserved, 0x1C
    // RESERVED0           : (),

    /// RCC APB1 peripheral reset register,
    /// Address offset: 0x20
    pub APB1RSTR        : VolatileReg32,

    /// RCC APB2 peripheral reset register,
    /// Address offset: 0x24
    pub APB2RSTR        : VolatileReg32,

    // Reserved, 0x28-0x2C
    // RESERVED1           : (),

    /// RCC AHB1 peripheral clock register,
    /// Address offset: 0x30
    pub AHB1ENR         : VolatileReg32,

    /// RCC AHB2 peripheral clock register,
    /// Address offset: 0x34
    pub AHB2ENR         : VolatileReg32,

    /// RCC AHB3 peripheral clock register,
    /// Address offset: 0x38
    pub AHB3ENR         : VolatileReg32,

    // Reserved, 0x3C
    // RESERVED2           : (),

    /// RCC APB1 peripheral clock enable register,
    /// Address offset: 0x40
    pub APB1ENR         : VolatileReg32,

    /// RCC APB2 peripheral clock enable register,
    /// Address offset: 0x44
    pub APB2ENR         : VolatileReg32,

    // Reserved, 0x48-0x4C
    // RESERVED3           : (),

    /// RCC AHB1 peripheral clock enable in low power mode register,
    /// Address offset: 0x50
    pub AHB1LPENR       : VolatileReg32,

    /// RCC AHB2 peripheral clock enable in low power mode register,
    /// Address offset: 0x54
    pub AHB2LPENR       : VolatileReg32,

    /// RCC AHB3 peripheral clock enable in low power mode register,
    /// Address offset: 0x58
    pub AHB3LPENR       : VolatileReg32,

    // Reserved, 0x5C
    // RESERVED4           : (),

    /// RCC APB1 peripheral clock enable in low power mode register,
    /// Address offset: 0x60
    pub APB1LPENR       : VolatileReg32,

    /// RCC APB2 peripheral clock enable in low power mode register,
    /// Address offset: 0x64
    pub APB2LPENR       : VolatileReg32,

    // Reserved, 0x68-0x6C
    // RESERVED5           : (),

    /// RCC Backup domain control register,
    /// Address offset: 0x70
    pub BDCR            : VolatileReg32,

    /// RCC clock control & status register,
    /// Address offset: 0x74
    pub CSR             : VolatileReg32,

    // Reserved, 0x78-0x7C
    // RESERVED6           : (),

    /// RCC spread spectrum clock generation register,
    /// Address offset: 0x80
    pub SSCGR           : VolatileReg32,

    /// RCC PLLI2S configuration register,
    /// Address offset: 0x84
    pub PLLI2SCFGR      : VolatileReg32,
}

impl RCC_Regs {
    pub fn init() -> RCC_Regs {

        let rcc_base: *mut u32 = RCC_BASE as *mut u32;

        let rcc: RCC_Regs = RCC_Regs {
            CR              : VolatileReg32::new(rcc_base),
            PLLCFGR         : VolatileReg32::new_offset(rcc_base, 1),
            CFGR            : VolatileReg32::new_offset(rcc_base, 2),
            CIR             : VolatileReg32::new_offset(rcc_base, 3),
            AHB1RSTR        : VolatileReg32::new_offset(rcc_base, 4),
            AHB2RSTR        : VolatileReg32::new_offset(rcc_base, 5),
            AHB3RSTR        : VolatileReg32::new_offset(rcc_base, 6),
            // RESERVED0    :                                     7
            APB1RSTR        : VolatileReg32::new_offset(rcc_base, 8),
            APB2RSTR        : VolatileReg32::new_offset(rcc_base, 9),
            // RESERVED1[0] :                                     10
            // RESERVED1[1] :                                     11
            AHB1ENR         : VolatileReg32::new_offset(rcc_base, 12),
            AHB2ENR         : VolatileReg32::new_offset(rcc_base, 13),
            AHB3ENR         : VolatileReg32::new_offset(rcc_base, 14),
            // RESERVED2    :                                     15
            APB1ENR         : VolatileReg32::new_offset(rcc_base, 16),
            APB2ENR         : VolatileReg32::new_offset(rcc_base, 17),
            // RESERVED3[0] :                                     18
            // RESERVED3[1] :                                     19
            AHB1LPENR       : VolatileReg32::new_offset(rcc_base, 20),
            AHB2LPENR       : VolatileReg32::new_offset(rcc_base, 21),
            AHB3LPENR       : VolatileReg32::new_offset(rcc_base, 22),
            // RESERVED4    :                                     23
            APB1LPENR       : VolatileReg32::new_offset(rcc_base, 24),
            APB2LPENR       : VolatileReg32::new_offset(rcc_base, 25),
            // RESERVED5[0] :                                     26
            // RESERVED5[1] :                                     27
            BDCR            : VolatileReg32::new_offset(rcc_base, 28),
            CSR             : VolatileReg32::new_offset(rcc_base, 29),
            // RESERVED6[0] :                                     30
            // RESERVED6[1] :                                     31
            SSCGR           : VolatileReg32::new_offset(rcc_base, 32),
            PLLI2SCFGR      : VolatileReg32::new_offset(rcc_base, 33),
        };

        rcc
    }
}

pub struct GPIO_Regs {
    /// GPIO port mode register,
    /// Address offset: 0x00
    pub MODER   : VolatileReg32,

    /// GPIO port output type register,
    /// Address offset: 0x04
    pub OTYPER  : VolatileReg32,

    /// GPIO port output speed register,
    /// Address offset: 0x08
    pub OSPEEDR : VolatileReg32,

    /// GPIO port pull-up/pull-down register,
    /// Address offset: 0x0C
    pub PUPDR   : VolatileReg32,

    /// GPIO port input data register,
    /// Address offset: 0x10
    pub IDR     : VolatileReg32,

    /// GPIO port output data register,
    /// Address offset: 0x14
    pub ODR     : VolatileReg32,

    /// GPIO port bit set/reset low register,
    /// Address offset: 0x18
    pub BSRRL   : VolatileReg32,

    /// GPIO port bit set/reset high register,
    /// Address offset: 0x1A
    pub BSRRH   : VolatileReg32,

    /// GPIO port configuration lock register,
    /// Address offset: 0x1C
    pub LCKR    : VolatileReg32,

    /// GPIO alternate function registers,
    /// Address offset: 0x20-0x24
    pub AFR0    : VolatileReg32,
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

// const       AHB1ENR_TEMP:   usize       = RCC_BASE_TEMP + 0x30;
// pub const   AHB1ENR:        *mut u32    = AHB1ENR_TEMP as *mut u32;

pub const RCC_AHB1ENR_GPIODEN:  u32 = 0x00000008;

