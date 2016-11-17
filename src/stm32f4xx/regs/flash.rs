#![allow(dead_code)]

//! FLASH registers

use ::volatile_reg32::*;
use super::constants::FLASH_R_BASE;


pub struct FlashRegs{

    /// FLASH access control register
    pub acr     : VolatileReg32,

    /// FLASH key register
    pub keyr    : VolatileReg32,

    /// FLASH option key register
    pub optkeyr : VolatileReg32,

    /// FLASH status register
    pub sr      : VolatileReg32,

    /// FLASH control register
    pub cr      : VolatileReg32,

    /// FLASH option control register
    pub optcr   : VolatileReg32,
}

impl FlashRegs {

    pub fn init() -> FlashRegs {
        let flash_base: *mut u32 = FLASH_R_BASE as *mut u32;

        let flash = FlashRegs {
            acr     : VolatileReg32::new(flash_base),
            keyr    : VolatileReg32::new_offset(flash_base, 1),
            optkeyr : VolatileReg32::new_offset(flash_base, 2),
            sr      : VolatileReg32::new_offset(flash_base, 3),
            cr      : VolatileReg32::new_offset(flash_base, 4),
            optcr   : VolatileReg32::new_offset(flash_base, 5),
        };

        flash
    }
}

// Bits definition for FLASH_ACR register
pub const FLASH_ACR_LATENCY         : u32   = 0x00000007;
pub const FLASH_ACR_LATENCY_0WS     : u32   = 0x00000000;
pub const FLASH_ACR_LATENCY_1WS     : u32   = 0x00000001;
pub const FLASH_ACR_LATENCY_2WS     : u32   = 0x00000002;
pub const FLASH_ACR_LATENCY_3WS     : u32   = 0x00000003;
pub const FLASH_ACR_LATENCY_4WS     : u32   = 0x00000004;
pub const FLASH_ACR_LATENCY_5WS     : u32   = 0x00000005;
pub const FLASH_ACR_LATENCY_6WS     : u32   = 0x00000006;
pub const FLASH_ACR_LATENCY_7WS     : u32   = 0x00000007;

pub const FLASH_ACR_PRFTEN          : u32   = 0x00000100;
pub const FLASH_ACR_ICEN            : u32   = 0x00000200;
pub const FLASH_ACR_DCEN            : u32   = 0x00000400;
pub const FLASH_ACR_ICRST           : u32   = 0x00000800;
pub const FLASH_ACR_DCRST           : u32   = 0x00001000;
pub const FLASH_ACR_BYTE0_ADDRESS   : u32   = 0x40023C00;
pub const FLASH_ACR_BYTE2_ADDRESS   : u32   = 0x40023C03;

// Bits definition for FLASH_SR register
pub const FLASH_SR_EOP              : u32   = 0x00000001;
pub const FLASH_SR_SOP              : u32   = 0x00000002;
pub const FLASH_SR_WRPERR           : u32   = 0x00000010;
pub const FLASH_SR_PGAERR           : u32   = 0x00000020;
pub const FLASH_SR_PGPERR           : u32   = 0x00000040;
pub const FLASH_SR_PGSERR           : u32   = 0x00000080;
pub const FLASH_SR_BSY              : u32   = 0x00010000;

// Bits definition for FLASH_CR register
pub const FLASH_CR_PG               : u32   = 0x00000001;
pub const FLASH_CR_SER              : u32   = 0x00000002;
pub const FLASH_CR_MER              : u32   = 0x00000004;
pub const FLASH_CR_SNB_0            : u32   = 0x00000008;
pub const FLASH_CR_SNB_1            : u32   = 0x00000010;
pub const FLASH_CR_SNB_2            : u32   = 0x00000020;
pub const FLASH_CR_SNB_3            : u32   = 0x00000040;
pub const FLASH_CR_PSIZE_0          : u32   = 0x00000100;
pub const FLASH_CR_PSIZE_1          : u32   = 0x00000200;
pub const FLASH_CR_STRT             : u32   = 0x00010000;
pub const FLASH_CR_EOPIE            : u32   = 0x01000000;
pub const FLASH_CR_LOCK             : u32   = 0x80000000;

// Bits definition for FLASH_OPTCR register
pub const FLASH_OPTCR_OPTLOCK       : u32   = 0x00000001;
pub const FLASH_OPTCR_OPTSTRT       : u32   = 0x00000002;
pub const FLASH_OPTCR_BOR_LEV_0     : u32   = 0x00000004;
pub const FLASH_OPTCR_BOR_LEV_1     : u32   = 0x00000008;
pub const FLASH_OPTCR_BOR_LEV       : u32   = 0x0000000C;
pub const FLASH_OPTCR_WDG_SW        : u32   = 0x00000020;
pub const FLASH_OPTCR_N_RST_STOP    : u32   = 0x00000040;
pub const FLASH_OPTCR_N_RST_STDBY   : u32   = 0x00000080;
pub const FLASH_OPTCR_RDP_0         : u32   = 0x00000100;
pub const FLASH_OPTCR_RDP_1         : u32   = 0x00000200;
pub const FLASH_OPTCR_RDP_2         : u32   = 0x00000400;
pub const FLASH_OPTCR_RDP_3         : u32   = 0x00000800;
pub const FLASH_OPTCR_RDP_4         : u32   = 0x00001000;
pub const FLASH_OPTCR_RDP_5         : u32   = 0x00002000;
pub const FLASH_OPTCR_RDP_6         : u32   = 0x00004000;
pub const FLASH_OPTCR_RDP_7         : u32   = 0x00008000;
pub const FLASH_OPTCR_N_WRP_0       : u32   = 0x00010000;
pub const FLASH_OPTCR_N_WRP_1       : u32   = 0x00020000;
pub const FLASH_OPTCR_N_WRP_2       : u32   = 0x00040000;
pub const FLASH_OPTCR_N_WRP_3       : u32   = 0x00080000;
pub const FLASH_OPTCR_N_WRP_4       : u32   = 0x00100000;
pub const FLASH_OPTCR_N_WRP_5       : u32   = 0x00200000;
pub const FLASH_OPTCR_N_WRP_6       : u32   = 0x00400000;
pub const FLASH_OPTCR_N_WRP_7       : u32   = 0x00800000;
pub const FLASH_OPTCR_N_WRP_8       : u32   = 0x01000000;
pub const FLASH_OPTCR_N_WRP_9       : u32   = 0x02000000;
pub const FLASH_OPTCR_N_WRP_10      : u32   = 0x04000000;
pub const FLASH_OPTCR_N_WRP_11      : u32   = 0x08000000;
