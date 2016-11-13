use ::volatile_reg32::*;
use super::core_cm4_const::*;

pub mod fpu_regs;

// #define SCnSCB              ((SCnSCB_Type    *)     SCS_BASE      )   /*!< System control Register not in SCB
// #define SCB                 ((SCB_Type       *)     SCB_BASE      )   /*!< SCB configuration struct
// #define SysTick             ((SysTick_Type   *)     SysTick_BASE  )   /*!< SysTick configuration struct
// #define NVIC                ((NVIC_Type      *)     NVIC_BASE     )   /*!< NVIC configuration struct
// #define ITM                 ((ITM_Type       *)     ITM_BASE      )   /*!< ITM configuration struct
// #define CoreDebug           ((CoreDebug_Type *)     CoreDebug_BASE)   /*!< Core Debug configuration struct

/// CMSIS_core_register

/// Structure type to access the System Control Block (SCB).

pub struct SCB_Regs {
    /// Offset: 0x000 (R/ )  CPUID Base Register
    pub CPUID   : VolatileReg32,
    /// Offset: 0x004 (R/W)  Interrupt Control and State Register
    pub ICSR    : VolatileReg32,
    /// Offset: 0x008 (R/W)  Vector Table Offset Register
    pub VTOR    : VolatileReg32,
    /// Offset: 0x00C (R/W)  Application Interrupt and Reset Control Register
    pub AIRCR   : VolatileReg32,
    /// Offset: 0x010 (R/W)  System Control Register
    pub SCR     : VolatileReg32,
    /// Offset: 0x014 (R/W)  Configuration Control Register
    pub CCR     : VolatileReg32,
    /// Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub SHP1    : VolatileReg32,
    /// Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub SHP2    : VolatileReg32,
    /// Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub SHP3    : VolatileReg32,
    /// Offset: 0x024 (R/W)  System Handler Control and State Register
    pub SHCSR   : VolatileReg32,
    /// Offset: 0x028 (R/W)  Configurable Fault Status Register
    pub CFSR    : VolatileReg32,
    /// Offset: 0x02C (R/W)  HardFault Status Register
    pub HFSR    : VolatileReg32,
    /// Offset: 0x030 (R/W)  Debug Fault Status Register
    pub DFSR    : VolatileReg32,
    /// Offset: 0x034 (R/W)  MemManage Fault Address Register
    pub MMFAR   : VolatileReg32,
    /// Offset: 0x038 (R/W)  BusFault Address Register
    pub BFAR    : VolatileReg32,
    /// Offset: 0x03C (R/W)  Auxiliary Fault Status Register
    pub AFSR    : VolatileReg32,
    /// Offset: 0x040 (R/ )  Processor Feature Register
    pub PFR0    : VolatileReg32,
    /// Offset: 0x040 (R/ )  Processor Feature Register
    pub PFR1    : VolatileReg32,
    /// Offset: 0x048 (R/ )  Debug Feature Register
    pub DFR     : VolatileReg32,
    /// Offset: 0x04C (R/ )  Auxiliary Feature Register
    pub ADR     : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub MMFR0   : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub MMFR1   : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub MMFR2   : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub MMFR3   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub ISAR0   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub ISAR1   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub ISAR2   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub ISAR3   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub ISAR4   : VolatileReg32,

    /// Offset: 0x088 (R/W)  Coprocessor Access Control Register
    pub CPACR   : VolatileReg32,
}

impl SCB_Regs {
    pub fn init() -> SCB_Regs {
        let scb_base: *mut u32 = SCB_BASE as *mut u32;

        let scb = SCB_Regs {
            CPUID   : VolatileReg32::new(scb_base),
            ICSR    : VolatileReg32::new_offset(scb_base, 1),
            VTOR    : VolatileReg32::new_offset(scb_base, 2),
            AIRCR   : VolatileReg32::new_offset(scb_base, 3),
            SCR     : VolatileReg32::new_offset(scb_base, 4),
            CCR     : VolatileReg32::new_offset(scb_base, 5),
            SHP1    : VolatileReg32::new_offset(scb_base, 6),
            SHP2    : VolatileReg32::new_offset(scb_base, 7),
            SHP3    : VolatileReg32::new_offset(scb_base, 8),
            SHCSR   : VolatileReg32::new_offset(scb_base, 9),
            CFSR    : VolatileReg32::new_offset(scb_base, 10),
            HFSR    : VolatileReg32::new_offset(scb_base, 11),
            DFSR    : VolatileReg32::new_offset(scb_base, 12),
            MMFAR   : VolatileReg32::new_offset(scb_base, 13),
            BFAR    : VolatileReg32::new_offset(scb_base, 14),
            AFSR    : VolatileReg32::new_offset(scb_base, 15),
            PFR0    : VolatileReg32::new_offset(scb_base, 16),
            PFR1    : VolatileReg32::new_offset(scb_base, 17),
            DFR     : VolatileReg32::new_offset(scb_base, 18),
            ADR     : VolatileReg32::new_offset(scb_base, 19),
            MMFR0   : VolatileReg32::new_offset(scb_base, 20),
            MMFR1   : VolatileReg32::new_offset(scb_base, 21),
            MMFR2   : VolatileReg32::new_offset(scb_base, 22),
            MMFR3   : VolatileReg32::new_offset(scb_base, 23),
            ISAR0   : VolatileReg32::new_offset(scb_base, 24),
            ISAR1   : VolatileReg32::new_offset(scb_base, 25),
            ISAR2   : VolatileReg32::new_offset(scb_base, 26),
            ISAR3   : VolatileReg32::new_offset(scb_base, 27),
            ISAR4   : VolatileReg32::new_offset(scb_base, 28),

            CPACR   : VolatileReg32::new_offset(scb_base, 34),
        };

        scb
    }
}

/// SCB CPUID Register Definitions
pub const SCB_CPUID_IMPLEMENTER_Pos     : u32   = 24;
pub const SCB_CPUID_IMPLEMENTER_Msk     : u32   = 0xFF << SCB_CPUID_IMPLEMENTER_Pos;

pub const SCB_CPUID_VARIANT_Pos         : u32   = 20;
pub const SCB_CPUID_VARIANT_Msk         : u32   = 0xF << SCB_CPUID_VARIANT_Pos;

pub const SCB_CPUID_ARCHITECTURE_Pos    : u32   = 16;
pub const SCB_CPUID_ARCHITECTURE_Msk    : u32   = 0xF << SCB_CPUID_ARCHITECTURE_Pos;

pub const SCB_CPUID_PARTNO_Pos          : u32   = 4;
pub const SCB_CPUID_PARTNO_Msk          : u32   = 0xFFF << SCB_CPUID_PARTNO_Pos;

pub const SCB_CPUID_REVISION_Pos        : u32   = 0;
pub const SCB_CPUID_REVISION_Msk        : u32   = 0xF << SCB_CPUID_REVISION_Pos;

/// SCB Interrupt Control State Register Definitions
pub const SCB_ICSR_NMIPENDSET_Pos       : u32   = 31;
pub const SCB_ICSR_NMIPENDSET_Msk       : u32   = 1 << SCB_ICSR_NMIPENDSET_Pos;

pub const SCB_ICSR_PENDSVSET_Pos        : u32   = 28;
pub const SCB_ICSR_PENDSVSET_Msk        : u32   = 1 << SCB_ICSR_PENDSVSET_Pos;

pub const SCB_ICSR_PENDSVCLR_Pos        : u32   = 27;
pub const SCB_ICSR_PENDSVCLR_Msk        : u32   = 1 << SCB_ICSR_PENDSVCLR_Pos;

pub const SCB_ICSR_PENDSTSET_Pos        : u32   = 26;
pub const SCB_ICSR_PENDSTSET_Msk        : u32   = 1 << SCB_ICSR_PENDSTSET_Pos;

pub const SCB_ICSR_PENDSTCLR_Pos        : u32   = 25;
pub const SCB_ICSR_PENDSTCLR_Msk        : u32   = 1 << SCB_ICSR_PENDSTCLR_Pos;

pub const SCB_ICSR_ISRPREEMPT_Pos       : u32   = 23;
pub const SCB_ICSR_ISRPREEMPT_Msk       : u32   = 1 << SCB_ICSR_ISRPREEMPT_Pos;

pub const SCB_ICSR_ISRPENDING_Pos       : u32   = 22;
pub const SCB_ICSR_ISRPENDING_Msk       : u32   = 1 << SCB_ICSR_ISRPENDING_Pos;

pub const SCB_ICSR_VECTPENDING_Pos      : u32   = 12;
pub const SCB_ICSR_VECTPENDING_Msk      : u32   = 0x1FF << SCB_ICSR_VECTPENDING_Pos;

pub const SCB_ICSR_RETTOBASE_Pos        : u32   = 11;
pub const SCB_ICSR_RETTOBASE_Msk        : u32   = 1 << SCB_ICSR_RETTOBASE_Pos;

pub const SCB_ICSR_VECTACTIVE_Pos       : u32   = 0;
pub const SCB_ICSR_VECTACTIVE_Msk       : u32   = 0x1FF << SCB_ICSR_VECTACTIVE_Pos;

/// SCB Vector Table Offset Register Definitions
pub const SCB_VTOR_TBLOFF_Pos           : u32   = 7;
pub const SCB_VTOR_TBLOFF_Msk           : u32   = 0x1FFFFFF << SCB_VTOR_TBLOFF_Pos;

/// SCB Application Interrupt and Reset Control Register Definitions
pub const SCB_AIRCR_VECTKEY_Pos         : u32   = 16;
pub const SCB_AIRCR_VECTKEY_Msk         : u32   = 0xFFFF << SCB_AIRCR_VECTKEY_Pos;

pub const SCB_AIRCR_VECTKEYSTAT_Pos     : u32   = 16;
pub const SCB_AIRCR_VECTKEYSTAT_Msk     : u32   = 0xFFFF << SCB_AIRCR_VECTKEYSTAT_Pos;

pub const SCB_AIRCR_ENDIANESS_Pos       : u32   = 15;
pub const SCB_AIRCR_ENDIANESS_Msk       : u32   = 1 << SCB_AIRCR_ENDIANESS_Pos;

pub const SCB_AIRCR_PRIGROUP_Pos        : u32   = 8;
pub const SCB_AIRCR_PRIGROUP_Msk        : u32   = 7 << SCB_AIRCR_PRIGROUP_Pos;

pub const SCB_AIRCR_SYSRESETREQ_Pos     : u32   = 2;
pub const SCB_AIRCR_SYSRESETREQ_Msk     : u32   = 1 << SCB_AIRCR_SYSRESETREQ_Pos;

pub const SCB_AIRCR_VECTCLRACTIVE_Pos   : u32   = 1;
pub const SCB_AIRCR_VECTCLRACTIVE_Msk   : u32   = 1 << SCB_AIRCR_VECTCLRACTIVE_Pos;

pub const SCB_AIRCR_VECTRESET_Pos       : u32   = 0;
pub const SCB_AIRCR_VECTRESET_Msk       : u32   = 1 << SCB_AIRCR_VECTRESET_Pos;

/// SCB System Control Register Definitions
pub const SCB_SCR_SEVONPEND_Pos         : u32   = 4;
pub const SCB_SCR_SEVONPEND_Msk         : u32   = 1 << SCB_SCR_SEVONPEND_Pos;

pub const SCB_SCR_SLEEPDEEP_Pos         : u32   = 2;
pub const SCB_SCR_SLEEPDEEP_Msk         : u32   = 1 << SCB_SCR_SLEEPDEEP_Pos;

pub const SCB_SCR_SLEEPONEXIT_Pos       : u32   = 1;
pub const SCB_SCR_SLEEPONEXIT_Msk       : u32   = 1 << SCB_SCR_SLEEPONEXIT_Pos;

/// SCB Configuration Control Register Definitions
pub const SCB_CCR_STKALIGN_Pos          : u32   = 9;
pub const SCB_CCR_STKALIGN_Msk          : u32   = 1 << SCB_CCR_STKALIGN_Pos;

pub const SCB_CCR_BFHFNMIGN_Pos         : u32   = 8;
pub const SCB_CCR_BFHFNMIGN_Msk         : u32   = 1 << SCB_CCR_BFHFNMIGN_Pos;

pub const SCB_CCR_DIV_0_TRP_Pos         : u32   = 4;
pub const SCB_CCR_DIV_0_TRP_Msk         : u32   = 1 << SCB_CCR_DIV_0_TRP_Pos;

pub const SCB_CCR_UNALIGN_TRP_Pos       : u32   = 3;
pub const SCB_CCR_UNALIGN_TRP_Msk       : u32   = 1 << SCB_CCR_UNALIGN_TRP_Pos;

pub const SCB_CCR_USERSETMPEND_Pos      : u32   = 1;
pub const SCB_CCR_USERSETMPEND_Msk      : u32   = 1 << SCB_CCR_USERSETMPEND_Pos;

pub const SCB_CCR_NONBASETHRDENA_Pos    : u32   = 0;
pub const SCB_CCR_NONBASETHRDENA_Msk    : u32   = 1 << SCB_CCR_NONBASETHRDENA_Pos;

/// SCB System Handler Control and State Register Definitions
pub const SCB_SHCSR_USGFAULTENA_Pos     : u32   = 18;
pub const SCB_SHCSR_USGFAULTENA_Msk     : u32   = 1 << SCB_SHCSR_USGFAULTENA_Pos;

pub const SCB_SHCSR_BUSFAULTENA_Pos     : u32   = 17;
pub const SCB_SHCSR_BUSFAULTENA_Msk     : u32   = 1 << SCB_SHCSR_BUSFAULTENA_Pos;

pub const SCB_SHCSR_MEMFAULTENA_Pos     : u32   = 16;
pub const SCB_SHCSR_MEMFAULTENA_Msk     : u32   = 1 << SCB_SHCSR_MEMFAULTENA_Pos;

pub const SCB_SHCSR_SVCALLPENDED_Pos    : u32   = 15;
pub const SCB_SHCSR_SVCALLPENDED_Msk    : u32   = 1 << SCB_SHCSR_SVCALLPENDED_Pos;

pub const SCB_SHCSR_BUSFAULTPENDED_Pos  : u32   = 14;
pub const SCB_SHCSR_BUSFAULTPENDED_Msk  : u32   = 1 << SCB_SHCSR_BUSFAULTPENDED_Pos;

pub const SCB_SHCSR_MEMFAULTPENDED_Pos  : u32   = 13;
pub const SCB_SHCSR_MEMFAULTPENDED_Msk  : u32   = 1 << SCB_SHCSR_MEMFAULTPENDED_Pos;

pub const SCB_SHCSR_USGFAULTPENDED_Pos  : u32   = 12;
pub const SCB_SHCSR_USGFAULTPENDED_Msk  : u32   = 1 << SCB_SHCSR_USGFAULTPENDED_Pos;

pub const SCB_SHCSR_SYSTICKACT_Pos      : u32   = 11;
pub const SCB_SHCSR_SYSTICKACT_Msk      : u32   = 1 << SCB_SHCSR_SYSTICKACT_Pos;

pub const SCB_SHCSR_PENDSVACT_Pos       : u32   = 10;
pub const SCB_SHCSR_PENDSVACT_Msk       : u32   = 1 << SCB_SHCSR_PENDSVACT_Pos;

pub const SCB_SHCSR_MONITORACT_Pos      : u32   = 8;
pub const SCB_SHCSR_MONITORACT_Msk      : u32   = 1 << SCB_SHCSR_MONITORACT_Pos;

pub const SCB_SHCSR_SVCALLACT_Pos       : u32   = 7;
pub const SCB_SHCSR_SVCALLACT_Msk       : u32   = 1 << SCB_SHCSR_SVCALLACT_Pos;

pub const SCB_SHCSR_USGFAULTACT_Pos     : u32   = 3;
pub const SCB_SHCSR_USGFAULTACT_Msk     : u32   = 1 << SCB_SHCSR_USGFAULTACT_Pos;

pub const SCB_SHCSR_BUSFAULTACT_Pos     : u32   = 1;
pub const SCB_SHCSR_BUSFAULTACT_Msk     : u32   = 1 << SCB_SHCSR_BUSFAULTACT_Pos;

pub const SCB_SHCSR_MEMFAULTACT_Pos     : u32   = 0;
pub const SCB_SHCSR_MEMFAULTACT_Msk     : u32   = 1 << SCB_SHCSR_MEMFAULTACT_Pos;

/// SCB Configurable Fault Status Registers Definitions
pub const SCB_CFSR_USGFAULTSR_Pos       : u32   = 16;
pub const SCB_CFSR_USGFAULTSR_Msk       : u32   = 0xFFFF << SCB_CFSR_USGFAULTSR_Pos;

pub const SCB_CFSR_BUSFAULTSR_Pos       : u32   = 8;
pub const SCB_CFSR_BUSFAULTSR_Msk       : u32   = 0xFF << SCB_CFSR_BUSFAULTSR_Pos;

pub const SCB_CFSR_MEMFAULTSR_Pos       : u32   = 0;
pub const SCB_CFSR_MEMFAULTSR_Msk       : u32   = 0xFF << SCB_CFSR_MEMFAULTSR_Pos;

/// SCB Hard Fault Status Registers Definitions
pub const SCB_HFSR_DEBUGEVT_Pos         : u32   = 31;
pub const SCB_HFSR_DEBUGEVT_Msk         : u32   = 1 << SCB_HFSR_DEBUGEVT_Pos;

pub const SCB_HFSR_FORCED_Pos           : u32   = 30;
pub const SCB_HFSR_FORCED_Msk           : u32   = 1 << SCB_HFSR_FORCED_Pos;

pub const SCB_HFSR_VECTTBL_Pos          : u32   = 1;
pub const SCB_HFSR_VECTTBL_Msk          : u32   = 1 << SCB_HFSR_VECTTBL_Pos;

/// SCB Debug Fault Status Register Definitions
pub const SCB_DFSR_EXTERNAL_Pos         : u32   = 4;
pub const SCB_DFSR_EXTERNAL_Msk         : u32   = 1 << SCB_DFSR_EXTERNAL_Pos;

pub const SCB_DFSR_VCATCH_Pos           : u32   = 3;
pub const SCB_DFSR_VCATCH_Msk           : u32   = 1 << SCB_DFSR_VCATCH_Pos;

pub const SCB_DFSR_DWTTRAP_Pos          : u32   = 2;
pub const SCB_DFSR_DWTTRAP_Msk          : u32   = 1 << SCB_DFSR_DWTTRAP_Pos;

pub const SCB_DFSR_BKPT_Pos             : u32   = 1;
pub const SCB_DFSR_BKPT_Msk             : u32   = 1 << SCB_DFSR_BKPT_Pos;

pub const SCB_DFSR_HALTED_Pos           : u32   = 0;
pub const SCB_DFSR_HALTED_Msk           : u32   = 1 << SCB_DFSR_HALTED_Pos;


/// Structure type to access the System Control and ID Register not in the SCB.
pub struct SCnSCB_Regs {
    /// Offset: 0x004 (R/ )  Interrupt Controller Type Register
    pub ICTR    : VolatileReg32,
    /// Offset: 0x008 (R/W)  Auxiliary Control Register
    pub ACTLR   : VolatileReg32,
}

impl SCnSCB_Regs {
    pub fn init() -> SCnSCB_Regs {
        let scnscb_base: *mut u32 = SCS_BASE as *mut u32;

        let scnscb = SCnSCB_Regs {
            ICTR    : VolatileReg32::new_offset(scnscb_base, 1),
            ACTLR   : VolatileReg32::new_offset(scnscb_base, 2),
        };

        scnscb
    }
}

/// Interrupt Controller Type Register Definitions
pub const SCnSCB_ICTR_INTLINESNUM_Pos   : u32   = 0;
pub const SCnSCB_ICTR_INTLINESNUM_Msk   : u32   = 0xF << SCnSCB_ICTR_INTLINESNUM_Pos;

/// Auxiliary Control Register Definitions
pub const SCnSCB_ACTLR_DISOOFP_Pos      : u32   = 9;
pub const SCnSCB_ACTLR_DISOOFP_Msk      : u32   = 1 << SCnSCB_ACTLR_DISOOFP_Pos;

pub const SCnSCB_ACTLR_DISFPCA_Pos      : u32   = 8;
pub const SCnSCB_ACTLR_DISFPCA_Msk      : u32   = 1 << SCnSCB_ACTLR_DISFPCA_Pos;

pub const SCnSCB_ACTLR_DISFOLD_Pos      : u32   = 2;
pub const SCnSCB_ACTLR_DISFOLD_Msk      : u32   = 1 << SCnSCB_ACTLR_DISFOLD_Pos;

pub const SCnSCB_ACTLR_DISDEFWBUF_Pos   : u32   = 1;
pub const SCnSCB_ACTLR_DISDEFWBUF_Msk   : u32   = 1 << SCnSCB_ACTLR_DISDEFWBUF_Pos;

pub const SCnSCB_ACTLR_DISMCYCINT_Pos   : u32   = 0;
pub const SCnSCB_ACTLR_DISMCYCINT_Msk   : u32   = 1 << SCnSCB_ACTLR_DISMCYCINT_Pos;

