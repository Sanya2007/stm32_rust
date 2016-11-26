#![allow(dead_code)]

use ::stm32f4xx::regs::rcc::*;

#[derive(Clone, Copy)]
pub enum PeripheralClock {
    // AHB1
    GpioA,
    GpioB,
    GpioC,
    GpioD,
    GpioE,
    GpioF,
    GpioG,
    GpioH,
    GpioI,
    Crc,
    BkpSram,
    CcmDataRam,
    Dma1,
    Dma2,
    EthMac,
    EthMacTx,
    EthMacRx,
    EthMacPtp,
    OtgHs,
    OtgHsulpi,

    // AHB2
    Dcmi,
    Cryp,
    Hash,
    Rng,
    OtgFs,

    // AHB3
    Fsmc,

    // APB1
    Tim2,
    Tim3,
    Tim4,
    Tim5,
    Tim6,
    Tim7,
    Tim12,
    Tim13,
    Tim14,
    WWdg,
    Spi2,
    Spi3,
    Usart2,
    Usart3,
    Uart4,
    Uart5,
    I2c1,
    I2c2,
    I2c3,
    Can1,
    Can2,
    Pwr,
    Dac,

    // APB2
    Tim1,
    Tim8,
    Usart1,
    Usart6,
    Adc1,
    Adc2,
    Adc3,
    Sdio,
    Spi1,
    SysCfg,
    Tim9,
    Tim10,
    Tim11,
}

impl PeripheralClock {
    pub fn enable(&self) {
        self.set_bit(true);
    }
    pub fn disable(&self) {
        self.set_bit(false);
    }

    fn set_bit(self, set: bool) {
        use self::PeripheralClock::*;

        let bit_pos: u32 = self.get_bit_pos();
        let bit_msk: u32 = !bit_pos;

        let rcc = RccRegs::init();

        let reg = match self {
            GpioA | GpioB | GpioC | GpioD | GpioE | GpioF | GpioG | GpioH |
            GpioI | Crc | BkpSram | CcmDataRam | Dma1 | Dma2 | EthMac |
            EthMacTx | EthMacRx | EthMacPtp | OtgHs | OtgHsulpi =>
                rcc.ahb1enr,

            Dcmi | Cryp | Hash | Rng | OtgFs =>
                rcc.ahb2enr,

            Fsmc =>
                rcc.ahb3enr,

            Tim2 | Tim3 | Tim4 | Tim5 | Tim6 | Tim7 | Tim12 | Tim13 | Tim14 |
            WWdg | Spi2 | Spi3 | Usart2 | Usart3 | Uart4 | Uart5 | I2c1 |
            I2c2 | I2c3 | Can1 | Can2 | Pwr | Dac =>
                rcc.apb1enr,

            Tim1 | Tim8 | Usart1 | Usart6 | Adc1 | Adc2 | Adc3 | Sdio |
            Spi1 | SysCfg | Tim9 | Tim10 | Tim11 =>
                rcc.apb2enr,
        };

        if set {
            reg.bit_or(bit_pos);

            // stall instruction pipeline, until instruction completes, as
            // per Errata 2.1.13, "Delay after an RCC peripheral clock enabling"
            unsafe { asm!("dsb"); }
        } else {
            reg.bit_and(bit_msk);
        }


    }

    fn get_bit_pos(self) -> u32 {
        use self::PeripheralClock::*;
        match self {
            GpioA       => RCC_AHB1ENR_GPIOAEN      ,
            GpioB       => RCC_AHB1ENR_GPIOBEN      ,
            GpioC       => RCC_AHB1ENR_GPIOCEN      ,
            GpioD       => RCC_AHB1ENR_GPIODEN      ,
            GpioE       => RCC_AHB1ENR_GPIOEEN      ,
            GpioF       => RCC_AHB1ENR_GPIOFEN      ,
            GpioG       => RCC_AHB1ENR_GPIOGEN      ,
            GpioH       => RCC_AHB1ENR_GPIOHEN      ,
            GpioI       => RCC_AHB1ENR_GPIOIEN      ,
            Crc         => RCC_AHB1ENR_CRCEN        ,
            BkpSram     => RCC_AHB1ENR_BKPSRAMEN    ,
            CcmDataRam  => RCC_AHB1ENR_CCMDATARAMEN ,
            Dma1        => RCC_AHB1ENR_DMA1EN       ,
            Dma2        => RCC_AHB1ENR_DMA2EN       ,
            EthMac      => RCC_AHB1ENR_ETHMACEN     ,
            EthMacTx    => RCC_AHB1ENR_ETHMACTXEN   ,
            EthMacRx    => RCC_AHB1ENR_ETHMACRXEN   ,
            EthMacPtp   => RCC_AHB1ENR_ETHMACPTPEN  ,
            OtgHs       => RCC_AHB1ENR_OTGHSEN      ,
            OtgHsulpi   => RCC_AHB1ENR_OTGHSULPIEN  ,

            Dcmi        => RCC_AHB2ENR_DCMIEN       ,
            Cryp        => RCC_AHB2ENR_CRYPEN       ,
            Hash        => RCC_AHB2ENR_HASHEN       ,
            Rng         => RCC_AHB2ENR_RNGEN        ,
            OtgFs       => RCC_AHB2ENR_OTGFSEN      ,

            Fsmc        => RCC_AHB3ENR_FSMCEN       ,

            Tim2        => RCC_APB1ENR_TIM2EN       ,
            Tim3        => RCC_APB1ENR_TIM3EN       ,
            Tim4        => RCC_APB1ENR_TIM4EN       ,
            Tim5        => RCC_APB1ENR_TIM5EN       ,
            Tim6        => RCC_APB1ENR_TIM6EN       ,
            Tim7        => RCC_APB1ENR_TIM7EN       ,
            Tim12       => RCC_APB1ENR_TIM12EN      ,
            Tim13       => RCC_APB1ENR_TIM13EN      ,
            Tim14       => RCC_APB1ENR_TIM14EN      ,
            WWdg        => RCC_APB1ENR_WWDGEN       ,
            Spi2        => RCC_APB1ENR_SPI2EN       ,
            Spi3        => RCC_APB1ENR_SPI3EN       ,
            Usart2      => RCC_APB1ENR_USART2EN     ,
            Usart3      => RCC_APB1ENR_USART3EN     ,
            Uart4       => RCC_APB1ENR_UART4EN      ,
            Uart5       => RCC_APB1ENR_UART5EN      ,
            I2c1        => RCC_APB1ENR_I2C1EN       ,
            I2c2        => RCC_APB1ENR_I2C2EN       ,
            I2c3        => RCC_APB1ENR_I2C3EN       ,
            Can1        => RCC_APB1ENR_CAN1EN       ,
            Can2        => RCC_APB1ENR_CAN2EN       ,
            Pwr         => RCC_APB1ENR_PWREN        ,
            Dac         => RCC_APB1ENR_DACEN        ,

            Tim1        => RCC_APB2ENR_TIM1EN       ,
            Tim8        => RCC_APB2ENR_TIM8EN       ,
            Usart1      => RCC_APB2ENR_USART1EN     ,
            Usart6      => RCC_APB2ENR_USART6EN     ,
            Adc1        => RCC_APB2ENR_ADC1EN       ,
            Adc2        => RCC_APB2ENR_ADC2EN       ,
            Adc3        => RCC_APB2ENR_ADC3EN       ,
            Sdio        => RCC_APB2ENR_SDIOEN       ,
            Spi1        => RCC_APB2ENR_SPI1EN       ,
            SysCfg      => RCC_APB2ENR_SYSCFGEN     ,
            Tim9        => RCC_APB2ENR_TIM11EN      ,
            Tim10       => RCC_APB2ENR_TIM10EN      ,
            Tim11       => RCC_APB2ENR_TIM9EN       ,
        }
    }
}
