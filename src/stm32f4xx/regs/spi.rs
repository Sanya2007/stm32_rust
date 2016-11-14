
//! Serial Peripheral Interface registers

use ::volatile_reg32::*;
use super::constants::{ I2S2ext_BASE,
                        I2S3ext_BASE,
                        SPI1_BASE,
                        SPI2_BASE,
                        SPI3_BASE,
                        };


struct SPI_Regs
{
    /// SPI control register 1 (not used in I2S mode)
    pub CR1     : VolatileReg32,

    /// SPI control register 2
    pub CR2     : VolatileReg32,

    /// SPI status register
    pub SR      : VolatileReg32,

    /// SPI data register
    pub DR      : VolatileReg32,

    /// SPI CRC polynomial register (not used in I2S mode)
    pub CRCPR   : VolatileReg32,

    /// SPI RX CRC register (not used in I2S mode)
    pub RXCRCR  : VolatileReg32,

    /// SPI TX CRC register (not used in I2S mode)
    pub TXCRCR  : VolatileReg32,

    /// SPI_I2S configuration register
    pub I2SCFGR : VolatileReg32,

    /// SPI_I2S prescaler register
    pub I2SPR   : VolatileReg32,
}

pub enum SPIInst {
    I2S2ext,
    I2S3ext,
    SPI1,
    SPI2,
    SPI3,
}

impl SPI_Regs {
    pub fn init(inst: SPIInst) -> SPI_Regs {
        let spi_base: *mut u32 = match inst {
            SPIInst::I2S2ext    => I2S2ext_BASE,
            SPIInst::I2S3ext    => I2S3ext_BASE,
            SPIInst::SPI1       => SPI1_BASE,
            SPIInst::SPI2       => SPI2_BASE,
            SPIInst::SPI3       => SPI3_BASE,
        } as *mut u32;

        let spi = SPI_Regs {
            CR1     : VolatileReg32::new(spi_base),
            CR2     : VolatileReg32::new_offset(spi_base, 1),
            SR      : VolatileReg32::new_offset(spi_base, 2),
            DR      : VolatileReg32::new_offset(spi_base, 3),
            CRCPR   : VolatileReg32::new_offset(spi_base, 4),
            RXCRCR  : VolatileReg32::new_offset(spi_base, 5),
            TXCRCR  : VolatileReg32::new_offset(spi_base, 6),
            I2SCFGR : VolatileReg32::new_offset(spi_base, 7),
            I2SPR   : VolatileReg32::new_offset(spi_base, 8),
        };

        spi

    }
}

// Bit definition for SPI_CR1 register
pub const SPI_CR1_CPHA              : u32   = 0x00000001;       // Clock Phase
pub const SPI_CR1_CPOL              : u32   = 0x00000002;       // Clock Polarity
pub const SPI_CR1_MSTR              : u32   = 0x00000004;       // Master Selection

pub const SPI_CR1_BR                : u32   = 0x00000038;       // BR[2:0] bits (Baud Rate Control)
pub const SPI_CR1_BR_0              : u32   = 0x00000008;       // Bit 0
pub const SPI_CR1_BR_1              : u32   = 0x00000010;       // Bit 1
pub const SPI_CR1_BR_2              : u32   = 0x00000020;       // Bit 2

pub const SPI_CR1_SPE               : u32   = 0x00000040;       // SPI Enable
pub const SPI_CR1_LSBFIRST          : u32   = 0x00000080;       // Frame Format
pub const SPI_CR1_SSI               : u32   = 0x00000100;       // Internal slave select
pub const SPI_CR1_SSM               : u32   = 0x00000200;       // Software slave management
pub const SPI_CR1_RXONLY            : u32   = 0x00000400;       // Receive only
pub const SPI_CR1_DFF               : u32   = 0x00000800;       // Data Frame Format
pub const SPI_CR1_CRCNEXT           : u32   = 0x00001000;       // Transmit CRC next
pub const SPI_CR1_CRCEN             : u32   = 0x00002000;       // Hardware CRC calculation enable
pub const SPI_CR1_BIDIOE            : u32   = 0x00004000;       // Output enable in bidirectional mode
pub const SPI_CR1_BIDIMODE          : u32   = 0x00008000;       // Bidirectional data mode enable

// Bit definition for SPI_CR2 register
pub const SPI_CR2_RXDMAEN           : u32   = 0x00000001;       // Rx Buffer DMA Enable
pub const SPI_CR2_TXDMAEN           : u32   = 0x00000002;       // Tx Buffer DMA Enable
pub const SPI_CR2_SSOE              : u32   = 0x00000004;       // SS Output Enable
pub const SPI_CR2_ERRIE             : u32   = 0x00000020;       // Error Interrupt Enable
pub const SPI_CR2_RXNEIE            : u32   = 0x00000040;       // RX buffer Not Empty Interrupt Enable
pub const SPI_CR2_TXEIE             : u32   = 0x00000080;       // Tx buffer Empty Interrupt Enable

// Bit definition for SPI_SR register
pub const SPI_SR_RXNE               : u32   = 0x00000001;       // Receive buffer Not Empty
pub const SPI_SR_TXE                : u32   = 0x00000002;       // Transmit buffer Empty
pub const SPI_SR_CHSIDE             : u32   = 0x00000004;       // Channel side
pub const SPI_SR_UDR                : u32   = 0x00000008;       // Underrun flag
pub const SPI_SR_CRCERR             : u32   = 0x00000010;       // CRC Error flag
pub const SPI_SR_MODF               : u32   = 0x00000020;       // Mode fault
pub const SPI_SR_OVR                : u32   = 0x00000040;       // Overrun flag
pub const SPI_SR_BSY                : u32   = 0x00000080;       // Busy flag

// Bit definition for SPI_DR register
pub const SPI_DR_DR                 : u32   = 0x0000FFFF;       // Data Register

// Bit definition for SPI_CRCPR register
pub const SPI_CRCPR_CRCPOLY         : u32   = 0x0000FFFF;       // CRC polynomial register

// Bit definition for SPI_RXCRCR register
pub const SPI_RXCRCR_RXCRC          : u32   = 0x0000FFFF;       // Rx CRC Register

// Bit definition for SPI_TXCRCR register
pub const SPI_TXCRCR_TXCRC          : u32   = 0x0000FFFF;       // Tx CRC Register

// Bit definition for SPI_I2SCFGR register
pub const SPI_I2SCFGR_CHLEN         : u32   = 0x00000001;   // Channel length (number of bits per audio channel)

pub const SPI_I2SCFGR_DATLEN        : u32   = 0x00000006;   // DATLEN[1:0] bits (Data length to be transferred)
pub const SPI_I2SCFGR_DATLEN_0      : u32   = 0x00000002;   // Bit 0
pub const SPI_I2SCFGR_DATLEN_1      : u32   = 0x00000004;   // Bit 1

pub const SPI_I2SCFGR_CKPOL         : u32   = 0x00000008;   // steady state clock polarity

pub const SPI_I2SCFGR_I2SSTD        : u32   = 0x00000030;   // I2SSTD[1:0] bits (I2S standard selection)
pub const SPI_I2SCFGR_I2SSTD_0      : u32   = 0x00000010;   // Bit 0
pub const SPI_I2SCFGR_I2SSTD_1      : u32   = 0x00000020;   // Bit 1

pub const SPI_I2SCFGR_PCMSYNC       : u32   = 0x00000080;   // PCM frame synchronization

pub const SPI_I2SCFGR_I2SCFG        : u32   = 0x00000300;   // I2SCFG[1:0] bits (I2S configuration mode)
pub const SPI_I2SCFGR_I2SCFG_0      : u32   = 0x00000100;   // Bit 0
pub const SPI_I2SCFGR_I2SCFG_1      : u32   = 0x00000200;   // Bit 1

pub const SPI_I2SCFGR_I2SE          : u32   = 0x00000400;   // I2S Enable
pub const SPI_I2SCFGR_I2SMOD        : u32   = 0x00000800;   // I2S mode selection

// Bit definition for SPI_I2SPR register
pub const SPI_I2SPR_I2SDIV          : u32   = 0x000000FF;   // I2S Linear prescaler
pub const SPI_I2SPR_ODD             : u32   = 0x00000100;   // Odd factor for the prescaler
pub const SPI_I2SPR_MCKOE           : u32   = 0x00000200;   // Master Clock Output Enable

