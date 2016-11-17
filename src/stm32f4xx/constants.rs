#![allow(dead_code)]

//! stm32f4xx_const module
//! This module contans main constants for stm32f4xx series MCUs

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
pub const I2S2_EXT_BASE         : u32   = APB1PERIPH_BASE + 0x3400;
pub const SPI2_BASE             : u32   = APB1PERIPH_BASE + 0x3800;
pub const SPI3_BASE             : u32   = APB1PERIPH_BASE + 0x3C00;
pub const I2S3_EXT_BASE         : u32   = APB1PERIPH_BASE + 0x4000;
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
pub const DMA1_STREAM0_BASE     : u32   = DMA1_BASE + 0x010;
pub const DMA1_STREAM1_BASE     : u32   = DMA1_BASE + 0x028;
pub const DMA1_STREAM2_BASE     : u32   = DMA1_BASE + 0x040;
pub const DMA1_STREAM3_BASE     : u32   = DMA1_BASE + 0x058;
pub const DMA1_STREAM4_BASE     : u32   = DMA1_BASE + 0x070;
pub const DMA1_STREAM5_BASE     : u32   = DMA1_BASE + 0x088;
pub const DMA1_STREAM6_BASE     : u32   = DMA1_BASE + 0x0A0;
pub const DMA1_STREAM7_BASE     : u32   = DMA1_BASE + 0x0B8;
pub const DMA2_BASE             : u32   = AHB1PERIPH_BASE + 0x6400;
pub const DMA2_STREAM0_BASE     : u32   = DMA2_BASE + 0x010;
pub const DMA2_STREAM1_BASE     : u32   = DMA2_BASE + 0x028;
pub const DMA2_STREAM2_BASE     : u32   = DMA2_BASE + 0x040;
pub const DMA2_STREAM3_BASE     : u32   = DMA2_BASE + 0x058;
pub const DMA2_STREAM4_BASE     : u32   = DMA2_BASE + 0x070;
pub const DMA2_STREAM5_BASE     : u32   = DMA2_BASE + 0x088;
pub const DMA2_STREAM6_BASE     : u32   = DMA2_BASE + 0x0A0;
pub const DMA2_STREAM7_BASE     : u32   = DMA2_BASE + 0x0B8;
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
pub const FSMC_BANK1_R_BASE     : u32   = FSMC_R_BASE + 0x0000;
pub const FSMC_BANK1E_R_BASE    : u32   = FSMC_R_BASE + 0x0104;
pub const FSMC_BANK2_R_BASE     : u32   = FSMC_R_BASE + 0x0060;
pub const FSMC_BANK3_R_BASE     : u32   = FSMC_R_BASE + 0x0080;
pub const FSMC_BANK4_R_BASE     : u32   = FSMC_R_BASE + 0x00A0;

/// Debug MCU registers base address
pub const DBGMCU_BASE           : u32   = 0xE0042000;

