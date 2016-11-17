#![allow(dead_code)]

use ::volatile_reg32::*;
use super::super::constants::NVIC_BASE;

/// Structure type to access the Nested Vectored Interrupt Controller (NVIC).
struct NvicRegs {
    /// Interrupt Set Enable Register
    pub iser    : [VolatileReg32; 3],
    /// Interrupt Clear Enable Register
    pub icer    : [VolatileReg32; 3],
    /// Interrupt Set Pending Register
    pub ispr    : [VolatileReg32; 3],
    /// Interrupt Clear Pending Register
    pub icpr    : [VolatileReg32; 3],
    /// Interrupt Active bit Register
    pub iabr    : [VolatileReg32; 3],
    /// Interrupt Priority Register (8Bit wide)
    pub ipr     : [VolatileReg32; 21],
    /// Software Trigger Interrupt Register
    pub stir    : VolatileReg32,
}

impl NvicRegs {
    pub fn init() -> NvicRegs {
        let iser_base:  *mut u32 = NVIC_BASE as *mut u32;
        let icer_base:  *mut u32 = (NVIC_BASE + 0x20) as *mut u32;
        let ispr_base:  *mut u32 = (NVIC_BASE + 0x40) as *mut u32;
        let icpr_base:  *mut u32 = (NVIC_BASE + 0x60) as *mut u32;
        let iabr_base:  *mut u32 = (NVIC_BASE + 0x80) as *mut u32;
        let ipr_base:   *mut u32 = (NVIC_BASE + 0xC0) as *mut u32;
        let stir_base:  *mut u32 = (NVIC_BASE + 0x0380) as *mut u32;

        let nvic_regs = NvicRegs {
            iser:   [
                        VolatileReg32::new(iser_base),
                        VolatileReg32::new_offset(iser_base, 1),
                        VolatileReg32::new_offset(iser_base, 2),
                    ],
            icer:   [
                        VolatileReg32::new(icer_base),
                        VolatileReg32::new_offset(icer_base, 1),
                        VolatileReg32::new_offset(icer_base, 2),
                    ],
            ispr:   [
                        VolatileReg32::new(ispr_base),
                        VolatileReg32::new_offset(ispr_base, 1),
                        VolatileReg32::new_offset(ispr_base, 2),
                    ],
            icpr:   [
                        VolatileReg32::new(icpr_base),
                        VolatileReg32::new_offset(icpr_base, 1),
                        VolatileReg32::new_offset(icpr_base, 2),
                    ],
            iabr:   [
                        VolatileReg32::new(iabr_base),
                        VolatileReg32::new_offset(iabr_base, 1),
                        VolatileReg32::new_offset(iabr_base, 2),
                    ],
            ipr:    [
                        VolatileReg32::new(ipr_base),
                        VolatileReg32::new_offset(ipr_base, 1),
                        VolatileReg32::new_offset(ipr_base, 2),
                        VolatileReg32::new_offset(ipr_base, 3),
                        VolatileReg32::new_offset(ipr_base, 4),
                        VolatileReg32::new_offset(ipr_base, 5),
                        VolatileReg32::new_offset(ipr_base, 6),
                        VolatileReg32::new_offset(ipr_base, 7),
                        VolatileReg32::new_offset(ipr_base, 8),
                        VolatileReg32::new_offset(ipr_base, 9),
                        VolatileReg32::new_offset(ipr_base, 10),
                        VolatileReg32::new_offset(ipr_base, 11),
                        VolatileReg32::new_offset(ipr_base, 12),
                        VolatileReg32::new_offset(ipr_base, 13),
                        VolatileReg32::new_offset(ipr_base, 14),
                        VolatileReg32::new_offset(ipr_base, 15),
                        VolatileReg32::new_offset(ipr_base, 16),
                        VolatileReg32::new_offset(ipr_base, 17),
                        VolatileReg32::new_offset(ipr_base, 18),
                        VolatileReg32::new_offset(ipr_base, 19),
                        VolatileReg32::new_offset(ipr_base, 20),
                    ],
            stir:   VolatileReg32::new(stir_base),
        };

        nvic_regs
    }
}

/// Software Triggered Interrupt Register Definitions
pub const NVIC_STIR_INTID_POS   : u32   = 0;
pub const NVIC_STIR_INTID_MSK   : u32   = 0x1FF << NVIC_STIR_INTID_POS;

