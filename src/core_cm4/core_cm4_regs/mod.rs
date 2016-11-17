#![allow(dead_code)]

use ::volatile_reg32::*;
use super::core_cm4_const::*;

pub mod fpu;
pub mod scb;

// #define SCnSCB              ((SCnSCB_Type    *)     SCS_BASE      )   /*!< System control Register not in SCB
// #define SCB                 ((SCB_Type       *)     SCB_BASE      )   /*!< SCB configuration struct
// #define SysTick             ((SysTick_Type   *)     SysTick_BASE  )   /*!< SysTick configuration struct
// #define NVIC                ((NVIC_Type      *)     NVIC_BASE     )   /*!< NVIC configuration struct
// #define ITM                 ((ITM_Type       *)     ITM_BASE      )   /*!< ITM configuration struct
// #define CoreDebug           ((CoreDebug_Type *)     CoreDebug_BASE)   /*!< Core Debug configuration struct

/// CMSIS_core_register



/// Structure type to access the System Control and ID Register not in the SCB.
pub struct ScNScbRegs {
    /// Offset: 0x004 (R/ )  Interrupt Controller Type Register
    pub ictr    : VolatileReg32,
    /// Offset: 0x008 (R/W)  Auxiliary Control Register
    pub actlr   : VolatileReg32,
}

impl ScNScbRegs {
    pub fn init() -> ScNScbRegs {
        let scnscb_base: *mut u32 = SCS_BASE as *mut u32;

        let scnscb = ScNScbRegs {
            ictr    : VolatileReg32::new_offset(scnscb_base, 1),
            actlr   : VolatileReg32::new_offset(scnscb_base, 2),
        };

        scnscb
    }
}

/// Interrupt Controller Type Register Definitions
pub const SC_N_SCB_ICTR_INTLINESNUM_POS : u32   = 0;
pub const SC_N_SCB_ICTR_INTLINESNUM_MSK : u32   = 0xF << SC_N_SCB_ICTR_INTLINESNUM_POS;

/// Auxiliary Control Register Definitions
pub const SC_N_SCB_ACTLR_DISOOFP_POS    : u32   = 9;
pub const SC_N_SCB_ACTLR_DISOOFP_MSK    : u32   = 1 << SC_N_SCB_ACTLR_DISOOFP_POS;

pub const SC_N_SCB_ACTLR_DISFPCA_POS    : u32   = 8;
pub const SC_N_SCB_ACTLR_DISFPCA_MSK    : u32   = 1 << SC_N_SCB_ACTLR_DISFPCA_POS;

pub const SC_N_SCB_ACTLR_DISFOLD_POS    : u32   = 2;
pub const SC_N_SCB_ACTLR_DISFOLD_MSK    : u32   = 1 << SC_N_SCB_ACTLR_DISFOLD_POS;

pub const SC_N_SCB_ACTLR_DISDEFWBUF_POS : u32   = 1;
pub const SC_N_SCB_ACTLR_DISDEFWBUF_MSK : u32   = 1 << SC_N_SCB_ACTLR_DISDEFWBUF_POS;

pub const SC_N_SCB_ACTLR_DISMCYCINT_POS : u32   = 0;
pub const SC_N_SCB_ACTLR_DISMCYCINT_MSK : u32   = 1 << SC_N_SCB_ACTLR_DISMCYCINT_POS;

