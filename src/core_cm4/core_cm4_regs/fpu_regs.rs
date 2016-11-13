//! Type definitions for the Cortex-M Floating Point Unit (FPU)

use ::volatile_reg32::*;
use super::super::core_cm4_const::*;

/// Structure type to access the Floating Point Unit (FPU).
struct FPU_Regs
{
    /// Floating-Point Context Control Register, Offset: 0x004 (R/W)
    pub FPCCR   : VolatileReg32,

    /// Floating-Point Context Address Register, Offset: 0x008 (R/W)
    pub FPCAR   : VolatileReg32,

    /// Floating-Point Default Status Control Register, Offset: 0x00C (R/W)
    pub FPDSCR  : VolatileReg32,
}

impl FPU_Regs {
    /// Use this constructor method to initialize FPU structure before using it
    pub fn init() -> FPU_Regs {
        let fpu_base: *mut u32 = FPU_BASE as *mut u32;

        let fpu = FPU_Regs {
            FPCCR   : VolatileReg32::new_offset(fpu_base, 1),
            FPCAR   : VolatileReg32::new_offset(fpu_base, 2),
            FPDSCR  : VolatileReg32::new_offset(fpu_base, 3),
        };

        fpu
    }
}

///
/// Floating-Point Context Control Register
///
/// FPCCR: ASPEN bit Position
pub const FPU_FPCCR_ASPEN_Pos   : u32   = 31;
/// FPCCR: ASPEN bit Mask
pub const FPU_FPCCR_ASPEN_Msk   : u32   = 1 << FPU_FPCCR_ASPEN_Pos;

/// FPCCR: LSPEN Position
pub const FPU_FPCCR_LSPEN_Pos   : u32   = 30;
/// FPCCR: LSPEN bit Mask
pub const FPU_FPCCR_LSPEN_Msk   : u32   = 1 << FPU_FPCCR_LSPEN_Pos;

/// FPCCR: MONRDY Position
pub const FPU_FPCCR_MONRDY_Pos  : u32   = 8;
/// FPCCR: MONRDY bit Mask
pub const FPU_FPCCR_MONRDY_Msk  : u32   = 1 << FPU_FPCCR_MONRDY_Pos;

/// FPCCR: BFRDY Position
pub const FPU_FPCCR_BFRDY_Pos   : u32   = 6;
/// FPCCR: BFRDY bit Mask
pub const FPU_FPCCR_BFRDY_Msk   : u32   = 1 << FPU_FPCCR_BFRDY_Pos;

/// FPCCR: MMRDY Position
pub const FPU_FPCCR_MMRDY_Pos   : u32   = 5;
/// FPCCR: MMRDY bit Mask
pub const FPU_FPCCR_MMRDY_Msk   : u32   = 1 << FPU_FPCCR_MMRDY_Pos;

/// FPCCR: HFRDY Position
pub const FPU_FPCCR_HFRDY_Pos   : u32   = 4;
/// FPCCR: HFRDY bit Mask
pub const FPU_FPCCR_HFRDY_Msk   : u32   = 1 << FPU_FPCCR_HFRDY_Pos;

/// FPCCR: processor mode bit Position
pub const FPU_FPCCR_THREAD_Pos  : u32   = 3;
/// FPCCR: processor mode active bit Mask
pub const FPU_FPCCR_THREAD_Msk  : u32   = 1 << FPU_FPCCR_THREAD_Pos;

/// FPCCR: privilege level bit Position
pub const FPU_FPCCR_USER_Pos    : u32   = 1;
/// FPCCR: privilege level bit Mask
pub const FPU_FPCCR_USER_Msk    : u32   = 1 << FPU_FPCCR_USER_Pos;

/// FPCCR: Lazy state preservation active bit Position
pub const FPU_FPCCR_LSPACT_Pos  : u32   = 0;
/// FPCCR: Lazy state preservation active bit Mask
pub const FPU_FPCCR_LSPACT_Msk  : u32   = 1 << FPU_FPCCR_LSPACT_Pos;


/// Floating-Point Context Address Register
/// FPCAR: ADDRESS bit Position
pub const FPU_FPCAR_ADDRESS_Pos : u32   = 3;
/// FPCAR: ADDRESS bit Mask
pub const FPU_FPCAR_ADDRESS_Msk : u32   = 0x1FFFFFFF << FPU_FPCAR_ADDRESS_Pos;

/// Floating-Point Default Status Control Register
/// FPDSCR: AHP bit Position
pub const FPU_FPDSCR_AHP_Pos    : u32   = 26;
/// FPDSCR: AHP bit Mask
pub const FPU_FPDSCR_AHP_Msk    : u32   = 1 << FPU_FPDSCR_AHP_Pos;

/// FPDSCR: DN bit Position
pub const FPU_FPDSCR_DN_Pos     : u32   = 25;
/// FPDSCR: DN bit Mask
pub const FPU_FPDSCR_DN_Msk     : u32   = 1 << FPU_FPDSCR_DN_Pos;

/// FPDSCR: FZ bit Position
pub const FPU_FPDSCR_FZ_Pos     : u32   = 24;
/// FPDSCR: FZ bit Mask
pub const FPU_FPDSCR_FZ_Msk     : u32   = 1 << FPU_FPDSCR_FZ_Pos;

/// FPDSCR: RMode bit Position
pub const FPU_FPDSCR_RMode_Pos  : u32   = 22;
/// FPDSCR: RMode bit Mask
pub const FPU_FPDSCR_RMode_Msk  : u32   = 3 << FPU_FPDSCR_RMode_Pos;

