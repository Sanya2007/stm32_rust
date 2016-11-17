#![allow(dead_code)]

//! Type definitions for the Cortex-M Floating Point Unit (FPU)

use ::volatile_reg32::*;
use super::super::constants::FPU_BASE;

/// Structure type to access the Floating Point Unit (FPU).
struct FpuRegs
{
    /// Floating-Point Context Control Register, Offset: 0x004 (R/W)
    pub fpccr   : VolatileReg32,

    /// Floating-Point Context Address Register, Offset: 0x008 (R/W)
    pub fpcar   : VolatileReg32,

    /// Floating-Point Default Status Control Register, Offset: 0x00C (R/W)
    pub fpdscr  : VolatileReg32,
}

impl FpuRegs {
    /// Use this constructor method to initialize FPU structure before using it
    pub fn init() -> FpuRegs {
        let fpu_base: *mut u32 = FPU_BASE as *mut u32;

        let fpu = FpuRegs {
            fpccr   : VolatileReg32::new_offset(fpu_base, 1),
            fpcar   : VolatileReg32::new_offset(fpu_base, 2),
            fpdscr  : VolatileReg32::new_offset(fpu_base, 3),
        };

        fpu
    }
}

///
/// Floating-Point Context Control Register
///
/// FPCCR: ASPEN bit Position
pub const FPU_FPCCR_ASPEN_POS   : u32   = 31;
/// FPCCR: ASPEN bit Mask
pub const FPU_FPCCR_ASPEN_MSK   : u32   = 1 << FPU_FPCCR_ASPEN_POS;

/// FPCCR: LSPEN Position
pub const FPU_FPCCR_LSPEN_POS   : u32   = 30;
/// FPCCR: LSPEN bit Mask
pub const FPU_FPCCR_LSPEN_MSK   : u32   = 1 << FPU_FPCCR_LSPEN_POS;

/// FPCCR: MONRDY Position
pub const FPU_FPCCR_MONRDY_POS  : u32   = 8;
/// FPCCR: MONRDY bit Mask
pub const FPU_FPCCR_MONRDY_MSK  : u32   = 1 << FPU_FPCCR_MONRDY_POS;

/// FPCCR: BFRDY Position
pub const FPU_FPCCR_BFRDY_POS   : u32   = 6;
/// FPCCR: BFRDY bit Mask
pub const FPU_FPCCR_BFRDY_MSK   : u32   = 1 << FPU_FPCCR_BFRDY_POS;

/// FPCCR: MMRDY Position
pub const FPU_FPCCR_MMRDY_POS   : u32   = 5;
/// FPCCR: MMRDY bit Mask
pub const FPU_FPCCR_MMRDY_MSK   : u32   = 1 << FPU_FPCCR_MMRDY_POS;

/// FPCCR: HFRDY Position
pub const FPU_FPCCR_HFRDY_POS   : u32   = 4;
/// FPCCR: HFRDY bit Mask
pub const FPU_FPCCR_HFRDY_MSK   : u32   = 1 << FPU_FPCCR_HFRDY_POS;

/// FPCCR: processor mode bit Position
pub const FPU_FPCCR_THREAD_POS  : u32   = 3;
/// FPCCR: processor mode active bit Mask
pub const FPU_FPCCR_THREAD_MSK  : u32   = 1 << FPU_FPCCR_THREAD_POS;

/// FPCCR: privilege level bit Position
pub const FPU_FPCCR_USER_POS    : u32   = 1;
/// FPCCR: privilege level bit Mask
pub const FPU_FPCCR_USER_MSK    : u32   = 1 << FPU_FPCCR_USER_POS;

/// FPCCR: Lazy state preservation active bit Position
pub const FPU_FPCCR_LSPACT_POS  : u32   = 0;
/// FPCCR: Lazy state preservation active bit Mask
pub const FPU_FPCCR_LSPACT_MSK  : u32   = 1 << FPU_FPCCR_LSPACT_POS;


/// Floating-Point Context Address Register
/// FPCAR: ADDRESS bit Position
pub const FPU_FPCAR_ADDRESS_POS : u32   = 3;
/// FPCAR: ADDRESS bit Mask
pub const FPU_FPCAR_ADDRESS_MSK : u32   = 0x1FFFFFFF << FPU_FPCAR_ADDRESS_POS;

/// Floating-Point Default Status Control Register
/// FPDSCR: AHP bit Position
pub const FPU_FPDSCR_AHP_POS    : u32   = 26;
/// FPDSCR: AHP bit Mask
pub const FPU_FPDSCR_AHP_MSK    : u32   = 1 << FPU_FPDSCR_AHP_POS;

/// FPDSCR: DN bit Position
pub const FPU_FPDSCR_DN_POS     : u32   = 25;
/// FPDSCR: DN bit Mask
pub const FPU_FPDSCR_DN_MSK     : u32   = 1 << FPU_FPDSCR_DN_POS;

/// FPDSCR: FZ bit Position
pub const FPU_FPDSCR_FZ_POS     : u32   = 24;
/// FPDSCR: FZ bit Mask
pub const FPU_FPDSCR_FZ_MSK     : u32   = 1 << FPU_FPDSCR_FZ_POS;

/// FPDSCR: RMode bit Position
pub const FPU_FPDSCR_RMODE_POS  : u32   = 22;
/// FPDSCR: RMode bit Mask
pub const FPU_FPDSCR_RMODE_MSK  : u32   = 3 << FPU_FPDSCR_RMODE_POS;

