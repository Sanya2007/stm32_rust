#![allow(dead_code)]

//! Type definitions for the Cortex-M Memory Protection Unit (MPU)

use ::volatile_reg32::*;
use super::super::constants::MPU_BASE;

/// Structure type to access the Memory Protection Unit (MPU).
struct MpuRegs {
    /// MPU Type Register
    pub mpu_type    : VolatileReg32,
    /// MPU Control Register
    pub ctrl        : VolatileReg32,
    /// MPU Region RNRber Register
    pub rnr         : VolatileReg32,
    /// MPU Region Base Address Register
    pub rbar        : VolatileReg32,
    /// MPU Region Attribute and Size Register
    pub rasr        : VolatileReg32,
    /// MPU Alias 1 Region Base Address Register
    pub rbar_a1     : VolatileReg32,
    /// MPU Alias 1 Region Attribute and Size Register
    pub rasr_a1     : VolatileReg32,
    /// MPU Alias 2 Region Base Address Register
    pub rbar_a2     : VolatileReg32,
    /// MPU Alias 2 Region Attribute and Size Register
    pub rasr_a2     : VolatileReg32,
    /// MPU Alias 3 Region Base Address Register
    pub rbar_a3     : VolatileReg32,
    /// MPU Alias 3 Region Attribute and Size Register
    pub rasr_a3     : VolatileReg32,
}

impl MpuRegs {
    fn init() -> MpuRegs {
        let mpu_base: *mut u32 = MPU_BASE as *mut u32;

        let mpu_regs = MpuRegs {
            mpu_type    : VolatileReg32::new(mpu_base),
            ctrl        : VolatileReg32::new_offset(mpu_base, 1),
            rnr         : VolatileReg32::new_offset(mpu_base, 2),
            rbar        : VolatileReg32::new_offset(mpu_base, 3),
            rasr        : VolatileReg32::new_offset(mpu_base, 4),
            rbar_a1     : VolatileReg32::new_offset(mpu_base, 5),
            rasr_a1     : VolatileReg32::new_offset(mpu_base, 6),
            rbar_a2     : VolatileReg32::new_offset(mpu_base, 7),
            rasr_a2     : VolatileReg32::new_offset(mpu_base, 8),
            rbar_a3     : VolatileReg32::new_offset(mpu_base, 9),
            rasr_a3     : VolatileReg32::new_offset(mpu_base, 10),
        };

        mpu_regs
    }
}

/// MPU Type Register
pub const MPU_TYPE_IREGION_POS      : u32   = 16;
pub const MPU_TYPE_IREGION_MSK      : u32   = 0xFF << MPU_TYPE_IREGION_POS;

pub const MPU_TYPE_DREGION_POS      : u32   = 8;
pub const MPU_TYPE_DREGION_MSK      : u32   = 0xFF << MPU_TYPE_DREGION_POS;

pub const MPU_TYPE_SEPARATE_POS     : u32   = 0;
pub const MPU_TYPE_SEPARATE_MSK     : u32   = 1 << MPU_TYPE_SEPARATE_POS;

/// MPU Control Register
pub const MPU_CTRL_PRIVDEFENA_POS   : u32   = 2;
pub const MPU_CTRL_PRIVDEFENA_MSK   : u32   = 1 << MPU_CTRL_PRIVDEFENA_POS;

pub const MPU_CTRL_HFNMIENA_POS     : u32   = 1;
pub const MPU_CTRL_HFNMIENA_MSK     : u32   = 1 << MPU_CTRL_HFNMIENA_POS;

pub const MPU_CTRL_ENABLE_POS       : u32   = 0;
pub const MPU_CTRL_ENABLE_MSK       : u32   = 1 << MPU_CTRL_ENABLE_POS;

/// MPU Region Number Register
pub const MPU_RNR_REGION_POS        : u32   = 0;
pub const MPU_RNR_REGION_MSK        : u32   = 0xFF << MPU_RNR_REGION_POS;

/// MPU Region Base Address Register
pub const MPU_RBAR_ADDR_POS         : u32   = 5;
pub const MPU_RBAR_ADDR_MSK         : u32   = 0x7FFFFFF << MPU_RBAR_ADDR_POS;

pub const MPU_RBAR_VALID_POS        : u32   = 4;
pub const MPU_RBAR_VALID_MSK        : u32   = 1 << MPU_RBAR_VALID_POS;

pub const MPU_RBAR_REGION_POS       : u32   = 0;
pub const MPU_RBAR_REGION_MSK       : u32   = 0xF << MPU_RBAR_REGION_POS;

/// MPU Region Attribute and Size Register
pub const MPU_RASR_ATTRS_POS        : u32   = 16;
pub const MPU_RASR_ATTRS_MSK        : u32   = 0xFFFF << MPU_RASR_ATTRS_POS;

pub const MPU_RASR_SRD_POS          : u32   = 8;
pub const MPU_RASR_SRD_MSK          : u32   = 0xFF << MPU_RASR_SRD_POS;

pub const MPU_RASR_SIZE_POS         : u32   = 1;
pub const MPU_RASR_SIZE_MSK         : u32   = 0x1F << MPU_RASR_SIZE_POS;

pub const MPU_RASR_ENABLE_POS       : u32   = 0;
pub const MPU_RASR_ENABLE_MSK       : u32   = 1 << MPU_RASR_ENABLE_POS;

