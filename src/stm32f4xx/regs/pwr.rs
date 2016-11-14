
//! Power Control registers

use ::volatile_reg32::*;
use super::constants::PWR_BASE;


pub struct PWR_Regs
{
    /// PWR power control register
    pub CR  : VolatileReg32,

    /// PWR power control/status register
    pub CSR : VolatileReg32,
}

impl PWR_Regs {

    pub fn init() -> PWR_Regs {
        let pwr_base: *mut u32 = PWR_BASE as *mut u32;

        let pwr: PWR_Regs = PWR_Regs {
            CR  : VolatileReg32::new(pwr_base),
            CSR : VolatileReg32::new_offset(pwr_base, 1),
        };

        pwr
    }
}


// Bit definition for PWR_CR register
pub const PWR_CR_LPDS       : u32   = 0x00000001;   // Low-Power Deepsleep
pub const PWR_CR_PDDS       : u32   = 0x00000002;   // Power Down Deepsleep
pub const PWR_CR_CWUF       : u32   = 0x00000004;   // Clear Wakeup Flag
pub const PWR_CR_CSBF       : u32   = 0x00000008;   // Clear Standby Flag
pub const PWR_CR_PVDE       : u32   = 0x00000010;   // Power Voltage Detector Enable

pub const PWR_CR_PLS        : u32   = 0x000000E0;   // PLS[2:0] bits (PVD Level Selection)
pub const PWR_CR_PLS_0      : u32   = 0x00000020;   // Bit 0
pub const PWR_CR_PLS_1      : u32   = 0x00000040;   // Bit 1
pub const PWR_CR_PLS_2      : u32   = 0x00000080;   // Bit 2


// PVD level configuration
pub const PWR_CR_PLS_LEV0   : u32   = 0x00000000;   // PVD level 0
pub const PWR_CR_PLS_LEV1   : u32   = 0x00000020;   // PVD level 1
pub const PWR_CR_PLS_LEV2   : u32   = 0x00000040;   // PVD level 2
pub const PWR_CR_PLS_LEV3   : u32   = 0x00000060;   // PVD level 3
pub const PWR_CR_PLS_LEV4   : u32   = 0x00000080;   // PVD level 4
pub const PWR_CR_PLS_LEV5   : u32   = 0x000000A0;   // PVD level 5
pub const PWR_CR_PLS_LEV6   : u32   = 0x000000C0;   // PVD level 6
pub const PWR_CR_PLS_LEV7   : u32   = 0x000000E0;   // PVD level 7

pub const PWR_CR_DBP        : u32   = 0x00000100;   // Disable Backup Domain write protection
pub const PWR_CR_FPDS       : u32   = 0x00000200;   // Flash power down in Stop mode
pub const PWR_CR_VOS        : u32   = 0x00004000;   // Regulator voltage scaling output selection

// Bit definition for PWR_CSR register
pub const PWR_CSR_WUF       : u32   = 0x00000001;   // Wakeup Flag
pub const PWR_CSR_SBF       : u32   = 0x00000002;   // Standby Flag
pub const PWR_CSR_PVDO      : u32   = 0x00000004;   // PVD Output
pub const PWR_CSR_BRR       : u32   = 0x00000008;   // Backup regulator ready
pub const PWR_CSR_EWUP      : u32   = 0x00000100;   // Enable WKUP pin
pub const PWR_CSR_BRE       : u32   = 0x00000200;   // Backup regulator enable
pub const PWR_CSR_VOSRDY    : u32   = 0x00004000;   // Regulator voltage scaling output selection ready
