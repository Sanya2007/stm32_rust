#![allow(dead_code)]

//! Reset and Clock Control registers

use ::volatile_reg32::*;
use super::constants::RCC_BASE;


pub struct RccRegs {
    /// RCC clock control register,
    pub cr              : VolatileReg32,

    /// RCC PLL configuration register,
    pub pllcfgr         : VolatileReg32,

    /// RCC clock configuration register,
    pub cfgr            : VolatileReg32,

    /// RCC clock interrupt register,
    pub cir             : VolatileReg32,

    /// RCC AHB1 peripheral reset register,
    pub ahb1rstr        : VolatileReg32,

    /// RCC AHB2 peripheral reset register,
    pub ahb2rstr        : VolatileReg32,

    /// RCC AHB3 peripheral reset register,
    pub ahb3rstr        : VolatileReg32,

    /// RCC APB1 peripheral reset register,
    pub apb1rstr        : VolatileReg32,

    /// RCC APB2 peripheral reset register,
    pub apb2rstr        : VolatileReg32,

    /// RCC AHB1 peripheral clock register,
    pub ahb1enr         : VolatileReg32,

    /// RCC AHB2 peripheral clock register,
    pub ahb2enr         : VolatileReg32,

    /// RCC AHB3 peripheral clock register,
    pub ahb3enr         : VolatileReg32,

    /// RCC APB1 peripheral clock enable register,
    pub apb1enr         : VolatileReg32,

    /// RCC APB2 peripheral clock enable register,
    pub apb2enr         : VolatileReg32,

    /// RCC AHB1 peripheral clock enable in low power mode register,
    pub ahb1lpenr       : VolatileReg32,

    /// RCC AHB2 peripheral clock enable in low power mode register,
    pub ahb2lpenr       : VolatileReg32,

    /// RCC AHB3 peripheral clock enable in low power mode register,
    pub ahb3lpenr       : VolatileReg32,

    /// RCC APB1 peripheral clock enable in low power mode register,
    pub apb1lpenr       : VolatileReg32,

    /// RCC APB2 peripheral clock enable in low power mode register,
    pub apb2lpenr       : VolatileReg32,

    /// RCC Backup domain control register,
    pub bdcr            : VolatileReg32,

    /// RCC clock control & status register,
    pub csr             : VolatileReg32,

    /// RCC spread spectrum clock generation register,
    pub sscgr           : VolatileReg32,

    /// RCC PLLI2S configuration register,
    pub plli2scfgr      : VolatileReg32,
}

impl RccRegs {

    pub fn init() -> RccRegs {

        let rcc_base: *mut u32 = RCC_BASE as *mut u32;

        let rcc = RccRegs {
            cr              : VolatileReg32::new(rcc_base),
            pllcfgr         : VolatileReg32::new_offset(rcc_base, 1),
            cfgr            : VolatileReg32::new_offset(rcc_base, 2),
            cir             : VolatileReg32::new_offset(rcc_base, 3),
            ahb1rstr        : VolatileReg32::new_offset(rcc_base, 4),
            ahb2rstr        : VolatileReg32::new_offset(rcc_base, 5),
            ahb3rstr        : VolatileReg32::new_offset(rcc_base, 6),
            // reserved0    :                                     7
            apb1rstr        : VolatileReg32::new_offset(rcc_base, 8),
            apb2rstr        : VolatileReg32::new_offset(rcc_base, 9),
            // reserved1[0] :                                     10
            // reserved1[1] :                                     11
            ahb1enr         : VolatileReg32::new_offset(rcc_base, 12),
            ahb2enr         : VolatileReg32::new_offset(rcc_base, 13),
            ahb3enr         : VolatileReg32::new_offset(rcc_base, 14),
            // reserved2    :                                     15
            apb1enr         : VolatileReg32::new_offset(rcc_base, 16),
            apb2enr         : VolatileReg32::new_offset(rcc_base, 17),
            // reserved3[0] :                                     18
            // reserved3[1] :                                     19
            ahb1lpenr       : VolatileReg32::new_offset(rcc_base, 20),
            ahb2lpenr       : VolatileReg32::new_offset(rcc_base, 21),
            ahb3lpenr       : VolatileReg32::new_offset(rcc_base, 22),
            // reserved4    :                                     23
            apb1lpenr       : VolatileReg32::new_offset(rcc_base, 24),
            apb2lpenr       : VolatileReg32::new_offset(rcc_base, 25),
            // reserved5[0] :                                     26
            // reserved5[1] :                                     27
            bdcr            : VolatileReg32::new_offset(rcc_base, 28),
            csr             : VolatileReg32::new_offset(rcc_base, 29),
            // reserved6[0] :                                     30
            // reserved6[1] :                                     31
            sscgr           : VolatileReg32::new_offset(rcc_base, 32),
            plli2scfgr      : VolatileReg32::new_offset(rcc_base, 33),
        };

        rcc
    }
}

// Bit definition for RCC_CR register
pub const RCC_CR_HSION                  : u32   = 0x00000001;
pub const RCC_CR_HSIRDY                 : u32   = 0x00000002;

pub const RCC_CR_HSITRIM                : u32   = 0x000000F8;
pub const RCC_CR_HSITRIM_0              : u32   = 0x00000008;
pub const RCC_CR_HSITRIM_1              : u32   = 0x00000010;
pub const RCC_CR_HSITRIM_2              : u32   = 0x00000020;
pub const RCC_CR_HSITRIM_3              : u32   = 0x00000040;
pub const RCC_CR_HSITRIM_4              : u32   = 0x00000080;

pub const RCC_CR_HSICAL                 : u32   = 0x0000FF00;
pub const RCC_CR_HSICAL_0               : u32   = 0x00000100;
pub const RCC_CR_HSICAL_1               : u32   = 0x00000200;
pub const RCC_CR_HSICAL_2               : u32   = 0x00000400;
pub const RCC_CR_HSICAL_3               : u32   = 0x00000800;
pub const RCC_CR_HSICAL_4               : u32   = 0x00001000;
pub const RCC_CR_HSICAL_5               : u32   = 0x00002000;
pub const RCC_CR_HSICAL_6               : u32   = 0x00004000;
pub const RCC_CR_HSICAL_7               : u32   = 0x00008000;

pub const RCC_CR_HSEON                  : u32   = 0x00010000;
pub const RCC_CR_HSERDY                 : u32   = 0x00020000;
pub const RCC_CR_HSEBYP                 : u32   = 0x00040000;
pub const RCC_CR_CSSON                  : u32   = 0x00080000;
pub const RCC_CR_PLLON                  : u32   = 0x01000000;
pub const RCC_CR_PLLRDY                 : u32   = 0x02000000;
pub const RCC_CR_PLLI2SON               : u32   = 0x04000000;
pub const RCC_CR_PLLI2SRDY              : u32   = 0x08000000;

// Bit definition for RCC_PLLCFGR register
pub const RCC_PLLCFGR_PLLM              : u32   = 0x0000003F;
pub const RCC_PLLCFGR_PLLM_0            : u32   = 0x00000001;
pub const RCC_PLLCFGR_PLLM_1            : u32   = 0x00000002;
pub const RCC_PLLCFGR_PLLM_2            : u32   = 0x00000004;
pub const RCC_PLLCFGR_PLLM_3            : u32   = 0x00000008;
pub const RCC_PLLCFGR_PLLM_4            : u32   = 0x00000010;
pub const RCC_PLLCFGR_PLLM_5            : u32   = 0x00000020;

pub const RCC_PLLCFGR_PLLN              : u32   = 0x00007FC0;
pub const RCC_PLLCFGR_PLLN_0            : u32   = 0x00000040;
pub const RCC_PLLCFGR_PLLN_1            : u32   = 0x00000080;
pub const RCC_PLLCFGR_PLLN_2            : u32   = 0x00000100;
pub const RCC_PLLCFGR_PLLN_3            : u32   = 0x00000200;
pub const RCC_PLLCFGR_PLLN_4            : u32   = 0x00000400;
pub const RCC_PLLCFGR_PLLN_5            : u32   = 0x00000800;
pub const RCC_PLLCFGR_PLLN_6            : u32   = 0x00001000;
pub const RCC_PLLCFGR_PLLN_7            : u32   = 0x00002000;
pub const RCC_PLLCFGR_PLLN_8            : u32   = 0x00004000;

pub const RCC_PLLCFGR_PLLP              : u32   = 0x00030000;
pub const RCC_PLLCFGR_PLLP_0            : u32   = 0x00010000;
pub const RCC_PLLCFGR_PLLP_1            : u32   = 0x00020000;

pub const RCC_PLLCFGR_PLLSRC            : u32   = 0x00400000;
pub const RCC_PLLCFGR_PLLSRC_HSE        : u32   = 0x00400000;
pub const RCC_PLLCFGR_PLLSRC_HSI        : u32   = 0x00000000;

pub const RCC_PLLCFGR_PLLQ              : u32   = 0x0F000000;
pub const RCC_PLLCFGR_PLLQ_0            : u32   = 0x01000000;
pub const RCC_PLLCFGR_PLLQ_1            : u32   = 0x02000000;
pub const RCC_PLLCFGR_PLLQ_2            : u32   = 0x04000000;
pub const RCC_PLLCFGR_PLLQ_3            : u32   = 0x08000000;

//  Bit definition for RCC_CFGR register
// SW configuration
pub const RCC_CFGR_SW                   : u32   = 0x00000003;   // SW[1:0] bits (System clock Switch)
pub const RCC_CFGR_SW_0                 : u32   = 0x00000001;   // Bit 0
pub const RCC_CFGR_SW_1                 : u32   = 0x00000002;   // Bit 1

pub const RCC_CFGR_SW_HSI               : u32   = 0x00000000;   // HSI selected as system clock
pub const RCC_CFGR_SW_HSE               : u32   = 0x00000001;   // HSE selected as system clock
pub const RCC_CFGR_SW_PLL               : u32   = 0x00000002;   // PLL selected as system clock

// SWS configuration
pub const RCC_CFGR_SWS                  : u32   = 0x0000000C;   // SWS[1:0] bits (System Clock Switch Status)
pub const RCC_CFGR_SWS_0                : u32   = 0x00000004;   // Bit 0
pub const RCC_CFGR_SWS_1                : u32   = 0x00000008;   // Bit 1

pub const RCC_CFGR_SWS_HSI              : u32   = 0x00000000;   // HSI oscillator used as system clock
pub const RCC_CFGR_SWS_HSE              : u32   = 0x00000004;   // HSE oscillator used as system clock
pub const RCC_CFGR_SWS_PLL              : u32   = 0x00000008;   // PLL used as system clock

// HPRE configuration
pub const RCC_CFGR_HPRE                 : u32   = 0x000000F0;   // HPRE[3:0] bits (AHB prescaler)
pub const RCC_CFGR_HPRE_0               : u32   = 0x00000010;   // Bit 0
pub const RCC_CFGR_HPRE_1               : u32   = 0x00000020;   // Bit 1
pub const RCC_CFGR_HPRE_2               : u32   = 0x00000040;   // Bit 2
pub const RCC_CFGR_HPRE_3               : u32   = 0x00000080;   // Bit 3

pub const RCC_CFGR_HPRE_DIV1            : u32   = 0x00000000;   // SYSCLK not divided
pub const RCC_CFGR_HPRE_DIV2            : u32   = 0x00000080;   // SYSCLK divided by 2
pub const RCC_CFGR_HPRE_DIV4            : u32   = 0x00000090;   // SYSCLK divided by 4
pub const RCC_CFGR_HPRE_DIV8            : u32   = 0x000000A0;   // SYSCLK divided by 8
pub const RCC_CFGR_HPRE_DIV16           : u32   = 0x000000B0;   // SYSCLK divided by 16
pub const RCC_CFGR_HPRE_DIV64           : u32   = 0x000000C0;   // SYSCLK divided by 64
pub const RCC_CFGR_HPRE_DIV128          : u32   = 0x000000D0;   // SYSCLK divided by 128
pub const RCC_CFGR_HPRE_DIV256          : u32   = 0x000000E0;   // SYSCLK divided by 256
pub const RCC_CFGR_HPRE_DIV512          : u32   = 0x000000F0;   // SYSCLK divided by 512

// PPRE1 configuration
pub const RCC_CFGR_PPRE1                : u32   = 0x00001C00;   // PRE1[2:0] bits (APB1 prescaler)
pub const RCC_CFGR_PPRE1_0              : u32   = 0x00000400;   // Bit 0
pub const RCC_CFGR_PPRE1_1              : u32   = 0x00000800;   // Bit 1
pub const RCC_CFGR_PPRE1_2              : u32   = 0x00001000;   // Bit 2

pub const RCC_CFGR_PPRE1_DIV1           : u32   = 0x00000000;   // HCLK not divided
pub const RCC_CFGR_PPRE1_DIV2           : u32   = 0x00001000;   // HCLK divided by 2
pub const RCC_CFGR_PPRE1_DIV4           : u32   = 0x00001400;   // HCLK divided by 4
pub const RCC_CFGR_PPRE1_DIV8           : u32   = 0x00001800;   // HCLK divided by 8
pub const RCC_CFGR_PPRE1_DIV16          : u32   = 0x00001C00;   // HCLK divided by 16

// PPRE2 configuration
pub const RCC_CFGR_PPRE2                : u32   = 0x0000E000;   // PRE2[2:0] bits (APB2 prescaler)
pub const RCC_CFGR_PPRE2_0              : u32   = 0x00002000;   // Bit 0
pub const RCC_CFGR_PPRE2_1              : u32   = 0x00004000;   // Bit 1
pub const RCC_CFGR_PPRE2_2              : u32   = 0x00008000;   // Bit 2

pub const RCC_CFGR_PPRE2_DIV1           : u32   = 0x00000000;   // HCLK not divided
pub const RCC_CFGR_PPRE2_DIV2           : u32   = 0x00008000;   // HCLK divided by 2
pub const RCC_CFGR_PPRE2_DIV4           : u32   = 0x0000A000;   // HCLK divided by 4
pub const RCC_CFGR_PPRE2_DIV8           : u32   = 0x0000C000;   // HCLK divided by 8
pub const RCC_CFGR_PPRE2_DIV16          : u32   = 0x0000E000;   // HCLK divided by 16

// RTCPRE configuration
pub const RCC_CFGR_RTCPRE               : u32   = 0x001F0000;
pub const RCC_CFGR_RTCPRE_0             : u32   = 0x00010000;
pub const RCC_CFGR_RTCPRE_1             : u32   = 0x00020000;
pub const RCC_CFGR_RTCPRE_2             : u32   = 0x00040000;
pub const RCC_CFGR_RTCPRE_3             : u32   = 0x00080000;
pub const RCC_CFGR_RTCPRE_4             : u32   = 0x00100000;

// MCO1 configuration
pub const RCC_CFGR_MCO1                 : u32   = 0x00600000;
pub const RCC_CFGR_MCO1_0               : u32   = 0x00200000;
pub const RCC_CFGR_MCO1_1               : u32   = 0x00400000;

pub const RCC_CFGR_I2SSRC               : u32   = 0x00800000;

pub const RCC_CFGR_MCO1PRE              : u32   = 0x07000000;
pub const RCC_CFGR_MCO1PRE_0            : u32   = 0x01000000;
pub const RCC_CFGR_MCO1PRE_1            : u32   = 0x02000000;
pub const RCC_CFGR_MCO1PRE_2            : u32   = 0x04000000;

pub const RCC_CFGR_MCO2PRE              : u32   = 0x38000000;
pub const RCC_CFGR_MCO2PRE_0            : u32   = 0x08000000;
pub const RCC_CFGR_MCO2PRE_1            : u32   = 0x10000000;
pub const RCC_CFGR_MCO2PRE_2            : u32   = 0x20000000;

pub const RCC_CFGR_MCO2                 : u32   = 0xC0000000;
pub const RCC_CFGR_MCO2_0               : u32   = 0x40000000;
pub const RCC_CFGR_MCO2_1               : u32   = 0x80000000;

// Bit definition for RCC_CIR register
pub const RCC_CIR_LSIRDYF               : u32   = 0x00000001;
pub const RCC_CIR_LSERDYF               : u32   = 0x00000002;
pub const RCC_CIR_HSIRDYF               : u32   = 0x00000004;
pub const RCC_CIR_HSERDYF               : u32   = 0x00000008;
pub const RCC_CIR_PLLRDYF               : u32   = 0x00000010;
pub const RCC_CIR_PLLI2SRDYF            : u32   = 0x00000020;
pub const RCC_CIR_CSSF                  : u32   = 0x00000080;
pub const RCC_CIR_LSIRDYIE              : u32   = 0x00000100;
pub const RCC_CIR_LSERDYIE              : u32   = 0x00000200;
pub const RCC_CIR_HSIRDYIE              : u32   = 0x00000400;
pub const RCC_CIR_HSERDYIE              : u32   = 0x00000800;
pub const RCC_CIR_PLLRDYIE              : u32   = 0x00001000;
pub const RCC_CIR_PLLI2SRDYIE           : u32   = 0x00002000;
pub const RCC_CIR_LSIRDYC               : u32   = 0x00010000;
pub const RCC_CIR_LSERDYC               : u32   = 0x00020000;
pub const RCC_CIR_HSIRDYC               : u32   = 0x00040000;
pub const RCC_CIR_HSERDYC               : u32   = 0x00080000;
pub const RCC_CIR_PLLRDYC               : u32   = 0x00100000;
pub const RCC_CIR_PLLI2SRDYC            : u32   = 0x00200000;
pub const RCC_CIR_CSSC                  : u32   = 0x00800000;

// Bit definition for RCC_AHB1RSTR register
pub const RCC_AHB1RSTR_GPIOARST         : u32   = 0x00000001;
pub const RCC_AHB1RSTR_GPIOBRST         : u32   = 0x00000002;
pub const RCC_AHB1RSTR_GPIOCRST         : u32   = 0x00000004;
pub const RCC_AHB1RSTR_GPIODRST         : u32   = 0x00000008;
pub const RCC_AHB1RSTR_GPIOERST         : u32   = 0x00000010;
pub const RCC_AHB1RSTR_GPIOFRST         : u32   = 0x00000020;
pub const RCC_AHB1RSTR_GPIOGRST         : u32   = 0x00000040;
pub const RCC_AHB1RSTR_GPIOHRST         : u32   = 0x00000080;
pub const RCC_AHB1RSTR_GPIOIRST         : u32   = 0x00000100;
pub const RCC_AHB1RSTR_CRCRST           : u32   = 0x00001000;
pub const RCC_AHB1RSTR_DMA1RST          : u32   = 0x00200000;
pub const RCC_AHB1RSTR_DMA2RST          : u32   = 0x00400000;
pub const RCC_AHB1RSTR_ETHMACRST        : u32   = 0x02000000;
pub const RCC_AHB1RSTR_OTGHRST          : u32   = 0x10000000;

// Bit definition for RCC_AHB2RSTR register
pub const RCC_AHB2RSTR_DCMIRST          : u32   = 0x00000001;
pub const RCC_AHB2RSTR_CRYPRST          : u32   = 0x00000010;
pub const RCC_AHB2RSTR_HSAHRST          : u32   = 0x00000020;
pub const RCC_AHB2RSTR_RNGRST           : u32   = 0x00000040;
pub const RCC_AHB2RSTR_OTGFSRST         : u32   = 0x00000080;

// Bit definition for RCC_AHB3RSTR register
pub const RCC_AHB3RSTR_FSMCRST          : u32   = 0x00000001;

// Bit definition for RCC_APB1RSTR register
pub const RCC_APB1RSTR_TIM2RST          : u32   = 0x00000001;
pub const RCC_APB1RSTR_TIM3RST          : u32   = 0x00000002;
pub const RCC_APB1RSTR_TIM4RST          : u32   = 0x00000004;
pub const RCC_APB1RSTR_TIM5RST          : u32   = 0x00000008;
pub const RCC_APB1RSTR_TIM6RST          : u32   = 0x00000010;
pub const RCC_APB1RSTR_TIM7RST          : u32   = 0x00000020;
pub const RCC_APB1RSTR_TIM12RST         : u32   = 0x00000040;
pub const RCC_APB1RSTR_TIM13RST         : u32   = 0x00000080;
pub const RCC_APB1RSTR_TIM14RST         : u32   = 0x00000100;
pub const RCC_APB1RSTR_WWDGEN           : u32   = 0x00000800;
pub const RCC_APB1RSTR_SPI2RST          : u32   = 0x00008000;
pub const RCC_APB1RSTR_SPI3RST          : u32   = 0x00010000;
pub const RCC_APB1RSTR_USART2RST        : u32   = 0x00020000;
pub const RCC_APB1RSTR_USART3RST        : u32   = 0x00040000;
pub const RCC_APB1RSTR_UART4RST         : u32   = 0x00080000;
pub const RCC_APB1RSTR_UART5RST         : u32   = 0x00100000;
pub const RCC_APB1RSTR_I2C1RST          : u32   = 0x00200000;
pub const RCC_APB1RSTR_I2C2RST          : u32   = 0x00400000;
pub const RCC_APB1RSTR_I2C3RST          : u32   = 0x00800000;
pub const RCC_APB1RSTR_CAN1RST          : u32   = 0x02000000;
pub const RCC_APB1RSTR_CAN2RST          : u32   = 0x04000000;
pub const RCC_APB1RSTR_PWRRST           : u32   = 0x10000000;
pub const RCC_APB1RSTR_DACRST           : u32   = 0x20000000;

// Bit definition for RCC_APB2RSTR register
pub const RCC_APB2RSTR_TIM1RST          : u32   = 0x00000001;
pub const RCC_APB2RSTR_TIM8RST          : u32   = 0x00000002;
pub const RCC_APB2RSTR_USART1RST        : u32   = 0x00000010;
pub const RCC_APB2RSTR_USART6RST        : u32   = 0x00000020;
pub const RCC_APB2RSTR_ADCRST           : u32   = 0x00000100;
pub const RCC_APB2RSTR_SDIORST          : u32   = 0x00000800;
pub const RCC_APB2RSTR_SPI1RST          : u32   = 0x00001000;
pub const RCC_APB2RSTR_SYSCFGRST        : u32   = 0x00004000;
pub const RCC_APB2RSTR_TIM9RST          : u32   = 0x00010000;
pub const RCC_APB2RSTR_TIM10RST         : u32   = 0x00020000;
pub const RCC_APB2RSTR_TIM11RST         : u32   = 0x00040000;

// Bit definition for RCC_AHB1ENR register
pub const RCC_AHB1ENR_GPIOAEN           : u32   = 0x00000001;
pub const RCC_AHB1ENR_GPIOBEN           : u32   = 0x00000002;
pub const RCC_AHB1ENR_GPIOCEN           : u32   = 0x00000004;
pub const RCC_AHB1ENR_GPIODEN           : u32   = 0x00000008;
pub const RCC_AHB1ENR_GPIOEEN           : u32   = 0x00000010;
pub const RCC_AHB1ENR_GPIOFEN           : u32   = 0x00000020;
pub const RCC_AHB1ENR_GPIOGEN           : u32   = 0x00000040;
pub const RCC_AHB1ENR_GPIOHEN           : u32   = 0x00000080;
pub const RCC_AHB1ENR_GPIOIEN           : u32   = 0x00000100;
pub const RCC_AHB1ENR_CRCEN             : u32   = 0x00001000;
pub const RCC_AHB1ENR_BKPSRAMEN         : u32   = 0x00040000;
pub const RCC_AHB1ENR_CCMDATARAMEN      : u32   = 0x00100000;
pub const RCC_AHB1ENR_DMA1EN            : u32   = 0x00200000;
pub const RCC_AHB1ENR_DMA2EN            : u32   = 0x00400000;
pub const RCC_AHB1ENR_ETHMACEN          : u32   = 0x02000000;
pub const RCC_AHB1ENR_ETHMACTXEN        : u32   = 0x04000000;
pub const RCC_AHB1ENR_ETHMACRXEN        : u32   = 0x08000000;
pub const RCC_AHB1ENR_ETHMACPTPEN       : u32   = 0x10000000;
pub const RCC_AHB1ENR_OTGHSEN           : u32   = 0x20000000;
pub const RCC_AHB1ENR_OTGHSULPIEN       : u32   = 0x40000000;

// Bit definition for RCC_AHB2ENR register
pub const RCC_AHB2ENR_DCMIEN            : u32   = 0x00000001;
pub const RCC_AHB2ENR_CRYPEN            : u32   = 0x00000010;
pub const RCC_AHB2ENR_HASHEN            : u32   = 0x00000020;
pub const RCC_AHB2ENR_RNGEN             : u32   = 0x00000040;
pub const RCC_AHB2ENR_OTGFSEN           : u32   = 0x00000080;

// Bit definition for RCC_AHB3ENR register
pub const RCC_AHB3ENR_FSMCEN            : u32   = 0x00000001;

// Bit definition for RCC_APB1ENR register
pub const RCC_APB1ENR_TIM2EN            : u32   = 0x00000001;
pub const RCC_APB1ENR_TIM3EN            : u32   = 0x00000002;
pub const RCC_APB1ENR_TIM4EN            : u32   = 0x00000004;
pub const RCC_APB1ENR_TIM5EN            : u32   = 0x00000008;
pub const RCC_APB1ENR_TIM6EN            : u32   = 0x00000010;
pub const RCC_APB1ENR_TIM7EN            : u32   = 0x00000020;
pub const RCC_APB1ENR_TIM12EN           : u32   = 0x00000040;
pub const RCC_APB1ENR_TIM13EN           : u32   = 0x00000080;
pub const RCC_APB1ENR_TIM14EN           : u32   = 0x00000100;
pub const RCC_APB1ENR_WWDGEN            : u32   = 0x00000800;
pub const RCC_APB1ENR_SPI2EN            : u32   = 0x00004000;
pub const RCC_APB1ENR_SPI3EN            : u32   = 0x00008000;
pub const RCC_APB1ENR_USART2EN          : u32   = 0x00020000;
pub const RCC_APB1ENR_USART3EN          : u32   = 0x00040000;
pub const RCC_APB1ENR_UART4EN           : u32   = 0x00080000;
pub const RCC_APB1ENR_UART5EN           : u32   = 0x00100000;
pub const RCC_APB1ENR_I2C1EN            : u32   = 0x00200000;
pub const RCC_APB1ENR_I2C2EN            : u32   = 0x00400000;
pub const RCC_APB1ENR_I2C3EN            : u32   = 0x00800000;
pub const RCC_APB1ENR_CAN1EN            : u32   = 0x02000000;
pub const RCC_APB1ENR_CAN2EN            : u32   = 0x04000000;
pub const RCC_APB1ENR_PWREN             : u32   = 0x10000000;
pub const RCC_APB1ENR_DACEN             : u32   = 0x20000000;

// Bit definition for RCC_APB2ENR register
pub const RCC_APB2ENR_TIM1EN            : u32   = 0x00000001;
pub const RCC_APB2ENR_TIM8EN            : u32   = 0x00000002;
pub const RCC_APB2ENR_USART1EN          : u32   = 0x00000010;
pub const RCC_APB2ENR_USART6EN          : u32   = 0x00000020;
pub const RCC_APB2ENR_ADC1EN            : u32   = 0x00000100;
pub const RCC_APB2ENR_ADC2EN            : u32   = 0x00000200;
pub const RCC_APB2ENR_ADC3EN            : u32   = 0x00000400;
pub const RCC_APB2ENR_SDIOEN            : u32   = 0x00000800;
pub const RCC_APB2ENR_SPI1EN            : u32   = 0x00001000;
pub const RCC_APB2ENR_SYSCFGEN          : u32   = 0x00004000;
pub const RCC_APB2ENR_TIM11EN           : u32   = 0x00040000;
pub const RCC_APB2ENR_TIM10EN           : u32   = 0x00020000;
pub const RCC_APB2ENR_TIM9EN            : u32   = 0x00010000;

// Bit definition for RCC_AHB1LPENR register
pub const RCC_AHB1LPENR_GPIOALPEN       : u32   = 0x00000001;
pub const RCC_AHB1LPENR_GPIOBLPEN       : u32   = 0x00000002;
pub const RCC_AHB1LPENR_GPIOCLPEN       : u32   = 0x00000004;
pub const RCC_AHB1LPENR_GPIODLPEN       : u32   = 0x00000008;
pub const RCC_AHB1LPENR_GPIOELPEN       : u32   = 0x00000010;
pub const RCC_AHB1LPENR_GPIOFLPEN       : u32   = 0x00000020;
pub const RCC_AHB1LPENR_GPIOGLPEN       : u32   = 0x00000040;
pub const RCC_AHB1LPENR_GPIOHLPEN       : u32   = 0x00000080;
pub const RCC_AHB1LPENR_GPIOILPEN       : u32   = 0x00000100;
pub const RCC_AHB1LPENR_CRCLPEN         : u32   = 0x00001000;
pub const RCC_AHB1LPENR_FLITFLPEN       : u32   = 0x00008000;
pub const RCC_AHB1LPENR_SRAM1LPEN       : u32   = 0x00010000;
pub const RCC_AHB1LPENR_SRAM2LPEN       : u32   = 0x00020000;
pub const RCC_AHB1LPENR_BKPSRAMLPEN     : u32   = 0x00040000;
pub const RCC_AHB1LPENR_DMA1LPEN        : u32   = 0x00200000;
pub const RCC_AHB1LPENR_DMA2LPEN        : u32   = 0x00400000;
pub const RCC_AHB1LPENR_ETHMACLPEN      : u32   = 0x02000000;
pub const RCC_AHB1LPENR_ETHMACTXLPEN    : u32   = 0x04000000;
pub const RCC_AHB1LPENR_ETHMACRXLPEN    : u32   = 0x08000000;
pub const RCC_AHB1LPENR_ETHMACPTPLPEN   : u32   = 0x10000000;
pub const RCC_AHB1LPENR_OTGHSLPEN       : u32   = 0x20000000;
pub const RCC_AHB1LPENR_OTGHSULPILPEN   : u32   = 0x40000000;

// Bit definition for RCC_AHB2LPENR register
pub const RCC_AHB2LPENR_DCMILPEN        : u32   = 0x00000001;
pub const RCC_AHB2LPENR_CRYPLPEN        : u32   = 0x00000010;
pub const RCC_AHB2LPENR_HASHLPEN        : u32   = 0x00000020;
pub const RCC_AHB2LPENR_RNGLPEN         : u32   = 0x00000040;
pub const RCC_AHB2LPENR_OTGFSLPEN       : u32   = 0x00000080;

// Bit definition for RCC_AHB3LPENR register
pub const RCC_AHB3LPENR_FSMCLPEN        : u32   = 0x00000001;

// Bit definition for RCC_APB1LPENR register
pub const RCC_APB1LPENR_TIM2LPEN        : u32   = 0x00000001;
pub const RCC_APB1LPENR_TIM3LPEN        : u32   = 0x00000002;
pub const RCC_APB1LPENR_TIM4LPEN        : u32   = 0x00000004;
pub const RCC_APB1LPENR_TIM5LPEN        : u32   = 0x00000008;
pub const RCC_APB1LPENR_TIM6LPEN        : u32   = 0x00000010;
pub const RCC_APB1LPENR_TIM7LPEN        : u32   = 0x00000020;
pub const RCC_APB1LPENR_TIM12LPEN       : u32   = 0x00000040;
pub const RCC_APB1LPENR_TIM13LPEN       : u32   = 0x00000080;
pub const RCC_APB1LPENR_TIM14LPEN       : u32   = 0x00000100;
pub const RCC_APB1LPENR_WWDGLPEN        : u32   = 0x00000800;
pub const RCC_APB1LPENR_SPI2LPEN        : u32   = 0x00004000;
pub const RCC_APB1LPENR_SPI3LPEN        : u32   = 0x00008000;
pub const RCC_APB1LPENR_USART2LPEN      : u32   = 0x00020000;
pub const RCC_APB1LPENR_USART3LPEN      : u32   = 0x00040000;
pub const RCC_APB1LPENR_UART4LPEN       : u32   = 0x00080000;
pub const RCC_APB1LPENR_UART5LPEN       : u32   = 0x00100000;
pub const RCC_APB1LPENR_I2C1LPEN        : u32   = 0x00200000;
pub const RCC_APB1LPENR_I2C2LPEN        : u32   = 0x00400000;
pub const RCC_APB1LPENR_I2C3LPEN        : u32   = 0x00800000;
pub const RCC_APB1LPENR_CAN1LPEN        : u32   = 0x02000000;
pub const RCC_APB1LPENR_CAN2LPEN        : u32   = 0x04000000;
pub const RCC_APB1LPENR_PWRLPEN         : u32   = 0x10000000;
pub const RCC_APB1LPENR_DACLPEN         : u32   = 0x20000000;

// Bit definition for RCC_APB2LPENR register
pub const RCC_APB2LPENR_TIM1LPEN        : u32   = 0x00000001;
pub const RCC_APB2LPENR_TIM8LPEN        : u32   = 0x00000002;
pub const RCC_APB2LPENR_USART1LPEN      : u32   = 0x00000010;
pub const RCC_APB2LPENR_USART6LPEN      : u32   = 0x00000020;
pub const RCC_APB2LPENR_ADC1LPEN        : u32   = 0x00000100;
pub const RCC_APB2LPENR_ADC2PEN         : u32   = 0x00000200;
pub const RCC_APB2LPENR_ADC3LPEN        : u32   = 0x00000400;
pub const RCC_APB2LPENR_SDIOLPEN        : u32   = 0x00000800;
pub const RCC_APB2LPENR_SPI1LPEN        : u32   = 0x00001000;
pub const RCC_APB2LPENR_SYSCFGLPEN      : u32   = 0x00004000;
pub const RCC_APB2LPENR_TIM9LPEN        : u32   = 0x00010000;
pub const RCC_APB2LPENR_TIM10LPEN       : u32   = 0x00020000;
pub const RCC_APB2LPENR_TIM11LPEN       : u32   = 0x00040000;

// Bit definition for RCC_BDCR register
pub const RCC_BDCR_LSEON                : u32   = 0x00000001;
pub const RCC_BDCR_LSERDY               : u32   = 0x00000002;
pub const RCC_BDCR_LSEBYP               : u32   = 0x00000004;

pub const RCC_BDCR_RTCSEL               : u32   = 0x00000300;
pub const RCC_BDCR_RTCSEL_0             : u32   = 0x00000100;
pub const RCC_BDCR_RTCSEL_1             : u32   = 0x00000200;

pub const RCC_BDCR_RTCEN                : u32   = 0x00008000;
pub const RCC_BDCR_BDRST                : u32   = 0x00010000;

// Bit definition for RCC_CSR register
pub const RCC_CSR_LSION                 : u32   = 0x00000001;
pub const RCC_CSR_LSIRDY                : u32   = 0x00000002;
pub const RCC_CSR_RMVF                  : u32   = 0x01000000;
pub const RCC_CSR_BORRSTF               : u32   = 0x02000000;
pub const RCC_CSR_PADRSTF               : u32   = 0x04000000;
pub const RCC_CSR_PORRSTF               : u32   = 0x08000000;
pub const RCC_CSR_SFTRSTF               : u32   = 0x10000000;
pub const RCC_CSR_WDGRSTF               : u32   = 0x20000000;
pub const RCC_CSR_WWDGRSTF              : u32   = 0x40000000;
pub const RCC_CSR_LPWRRSTF              : u32   = 0x80000000;

// Bit definition for RCC_SSCGR register
pub const RCC_SSCGR_MODPER              : u32   = 0x00001FFF;
pub const RCC_SSCGR_INCSTEP             : u32   = 0x0FFFE000;
pub const RCC_SSCGR_SPREADSEL           : u32   = 0x40000000;
pub const RCC_SSCGR_SSCGEN              : u32   = 0x80000000;

// Bit definition for RCC_PLLI2SCFGR register
pub const RCC_PLLI2SCFGR_PLLI2SN        : u32   = 0x00007FC0;
pub const RCC_PLLI2SCFGR_PLLI2SR        : u32   = 0x70000000;
