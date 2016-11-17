#![allow(dead_code)]

use ::volatile_reg32::*;
use super::super::constants::SYS_TICK_BASE;

/// Structure type to access the System Timer (SysTick).
struct SysTickRegs {
    /// SysTick Control and Status Register
    pub ctrl    : VolatileReg32,
    /// SysTick Reload Value Register
    pub load    : VolatileReg32,
    /// SysTick Current Value Register
    pub val     : VolatileReg32,
    /// SysTick Calibration Register
    pub calib   : VolatileReg32,
}

impl SysTickRegs {
    pub fn init() -> SysTickRegs {
        let sys_tick_base: *mut u32 = SYS_TICK_BASE as *mut u32;

        sys_tick_regs = SysTickRegs {
            ctrl    : VolatileReg32::new(sys_tick_base),
            load    : VolatileReg32::new_offset(sys_tick_base, 1),
            val     : VolatileReg32::new_offset(sys_tick_base, 2),
            calib   : VolatileReg32::new_offset(sys_tick_base, 3),
        };

        sys_tick_regs
    }
}

/// SysTick Control / Status Register Definitions
pub const SYS_TICK_CTRL_COUNTFLAG_POS   : u32   = 16;
pub const SYS_TICK_CTRL_COUNTFLAG_MSK   : u32   = 1 << SYS_TICK_CTRL_COUNTFLAG_POS;

pub const SYS_TICK_CTRL_CLKSOURCE_POS   : u32   = 2;
pub const SYS_TICK_CTRL_CLKSOURCE_MSK   : u32   = 1 << SYS_TICK_CTRL_CLKSOURCE_POS;

pub const SYS_TICK_CTRL_TICKINT_POS     : u32   = 1;
pub const SYS_TICK_CTRL_TICKINT_MSK     : u32   = 1 << SYS_TICK_CTRL_TICKINT_POS;

pub const SYS_TICK_CTRL_ENABLE_POS      : u32   = 0;
pub const SYS_TICK_CTRL_ENABLE_MSK      : u32   = 1 << SYS_TICK_CTRL_ENABLE_POS;

/// SysTick Reload Register Definitions
pub const SYS_TICK_LOAD_RELOAD_POS      : u32   = 0;
pub const SYS_TICK_LOAD_RELOAD_MSK      : u32   = 0xFFFFFF << SYS_TICK_LOAD_RELOAD_POS;

/// SysTick Current Register Definitions
pub const SYS_TICK_VAL_CURRENT_POS      : u32   = 0;
pub const SYS_TICK_VAL_CURRENT_MSK      : u32   = 0xFFFFFF << SYS_TICK_VAL_CURRENT_POS;

/// SysTick Calibration Register Definitions
pub const SYS_TICK_CALIB_NOREF_POS      : u32   = 31;
pub const SYS_TICK_CALIB_NOREF_MSK      : u32   = 1 << SYS_TICK_CALIB_NOREF_POS;

pub const SYS_TICK_CALIB_SKEW_POS       : u32   = 30;
pub const SYS_TICK_CALIB_SKEW_MSK       : u32   = 1 << SYS_TICK_CALIB_SKEW_POS;

pub const SYS_TICK_CALIB_TENMS_POS      : u32   = 0;
pub const SYS_TICK_CALIB_TENMS_MSK      : u32   = 0xFFFFFF << SYS_TICK_VAL_CURRENT_POS;

