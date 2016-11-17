//! Type definitions for the Cortex-M4 System Control Block registers

use ::volatile_reg32::*;
use super::super::core_cm4_const::*;

/// Structure type to access the System Control Block (SCB).

pub struct ScbRegs {
    /// Offset: 0x000 (R/ )  CPUID Base Register
    pub cpuid   : VolatileReg32,
    /// Offset: 0x004 (R/W)  Interrupt Control and State Register
    pub icsr    : VolatileReg32,
    /// Offset: 0x008 (R/W)  Vector Table Offset Register
    pub vtor    : VolatileReg32,
    /// Offset: 0x00C (R/W)  Application Interrupt and Reset Control Register
    pub aircr   : VolatileReg32,
    /// Offset: 0x010 (R/W)  System Control Register
    pub scr     : VolatileReg32,
    /// Offset: 0x014 (R/W)  Configuration Control Register
    pub ccr     : VolatileReg32,
    /// Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub shp1    : VolatileReg32,
    /// Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub shp2    : VolatileReg32,
    /// Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub shp3    : VolatileReg32,
    /// Offset: 0x024 (R/W)  System Handler Control and State Register
    pub shcsr   : VolatileReg32,
    /// Offset: 0x028 (R/W)  Configurable Fault Status Register
    pub cfsr    : VolatileReg32,
    /// Offset: 0x02C (R/W)  HardFault Status Register
    pub hfsr    : VolatileReg32,
    /// Offset: 0x030 (R/W)  Debug Fault Status Register
    pub dfsr    : VolatileReg32,
    /// Offset: 0x034 (R/W)  MemManage Fault Address Register
    pub mmfar   : VolatileReg32,
    /// Offset: 0x038 (R/W)  BusFault Address Register
    pub bfar    : VolatileReg32,
    /// Offset: 0x03C (R/W)  Auxiliary Fault Status Register
    pub afsr    : VolatileReg32,
    /// Offset: 0x040 (R/ )  Processor Feature Register
    pub pfr0    : VolatileReg32,
    /// Offset: 0x040 (R/ )  Processor Feature Register
    pub pfr1    : VolatileReg32,
    /// Offset: 0x048 (R/ )  Debug Feature Register
    pub dfr     : VolatileReg32,
    /// Offset: 0x04C (R/ )  Auxiliary Feature Register
    pub adr     : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub mmfr0   : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub mmfr1   : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub mmfr2   : VolatileReg32,
    /// Offset: 0x050 (R/ )  Memory Model Feature Register
    pub mmfr3   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub isar0   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub isar1   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub isar2   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub isar3   : VolatileReg32,
    /// Offset: 0x060 (R/ )  Instruction Set Attributes Register
    pub isar4   : VolatileReg32,

    /// Offset: 0x088 (R/W)  Coprocessor Access Control Register
    pub cpacr   : VolatileReg32,
}

impl ScbRegs {
    pub fn init() -> ScbRegs {
        let scb_base: *mut u32 = SCB_BASE as *mut u32;

        let scb = ScbRegs {
            cpuid   : VolatileReg32::new(scb_base),
            icsr    : VolatileReg32::new_offset(scb_base, 1),
            vtor    : VolatileReg32::new_offset(scb_base, 2),
            aircr   : VolatileReg32::new_offset(scb_base, 3),
            scr     : VolatileReg32::new_offset(scb_base, 4),
            ccr     : VolatileReg32::new_offset(scb_base, 5),
            shp1    : VolatileReg32::new_offset(scb_base, 6),
            shp2    : VolatileReg32::new_offset(scb_base, 7),
            shp3    : VolatileReg32::new_offset(scb_base, 8),
            shcsr   : VolatileReg32::new_offset(scb_base, 9),
            cfsr    : VolatileReg32::new_offset(scb_base, 10),
            hfsr    : VolatileReg32::new_offset(scb_base, 11),
            dfsr    : VolatileReg32::new_offset(scb_base, 12),
            mmfar   : VolatileReg32::new_offset(scb_base, 13),
            bfar    : VolatileReg32::new_offset(scb_base, 14),
            afsr    : VolatileReg32::new_offset(scb_base, 15),
            pfr0    : VolatileReg32::new_offset(scb_base, 16),
            pfr1    : VolatileReg32::new_offset(scb_base, 17),
            dfr     : VolatileReg32::new_offset(scb_base, 18),
            adr     : VolatileReg32::new_offset(scb_base, 19),
            mmfr0   : VolatileReg32::new_offset(scb_base, 20),
            mmfr1   : VolatileReg32::new_offset(scb_base, 21),
            mmfr2   : VolatileReg32::new_offset(scb_base, 22),
            mmfr3   : VolatileReg32::new_offset(scb_base, 23),
            isar0   : VolatileReg32::new_offset(scb_base, 24),
            isar1   : VolatileReg32::new_offset(scb_base, 25),
            isar2   : VolatileReg32::new_offset(scb_base, 26),
            isar3   : VolatileReg32::new_offset(scb_base, 27),
            isar4   : VolatileReg32::new_offset(scb_base, 28),

            cpacr   : VolatileReg32::new_offset(scb_base, 34),
        };

        scb
    }
}

/// SCB CPUID Register Definitions
pub const SCB_CPUID_IMPLEMENTER_POS     : u32   = 24;
pub const SCB_CPUID_IMPLEMENTER_MSK     : u32   = 0xFF << SCB_CPUID_IMPLEMENTER_POS;

pub const SCB_CPUID_VARIANT_POS         : u32   = 20;
pub const SCB_CPUID_VARIANT_MSK         : u32   = 0xF << SCB_CPUID_VARIANT_POS;

pub const SCB_CPUID_ARCHITECTURE_POS    : u32   = 16;
pub const SCB_CPUID_ARCHITECTURE_MSK    : u32   = 0xF << SCB_CPUID_ARCHITECTURE_POS;

pub const SCB_CPUID_PARTNO_POS          : u32   = 4;
pub const SCB_CPUID_PARTNO_MSK          : u32   = 0xFFF << SCB_CPUID_PARTNO_POS;

pub const SCB_CPUID_REVISION_POS        : u32   = 0;
pub const SCB_CPUID_REVISION_MSK        : u32   = 0xF << SCB_CPUID_REVISION_POS;

/// SCB Interrupt Control State Register Definitions
pub const SCB_ICSR_NMIPENDSET_POS       : u32   = 31;
pub const SCB_ICSR_NMIPENDSET_MSK       : u32   = 1 << SCB_ICSR_NMIPENDSET_POS;

pub const SCB_ICSR_PENDSVSET_POS        : u32   = 28;
pub const SCB_ICSR_PENDSVSET_MSK        : u32   = 1 << SCB_ICSR_PENDSVSET_POS;

pub const SCB_ICSR_PENDSVCLR_POS        : u32   = 27;
pub const SCB_ICSR_PENDSVCLR_MSK        : u32   = 1 << SCB_ICSR_PENDSVCLR_POS;

pub const SCB_ICSR_PENDSTSET_POS        : u32   = 26;
pub const SCB_ICSR_PENDSTSET_MSK        : u32   = 1 << SCB_ICSR_PENDSTSET_POS;

pub const SCB_ICSR_PENDSTCLR_POS        : u32   = 25;
pub const SCB_ICSR_PENDSTCLR_MSK        : u32   = 1 << SCB_ICSR_PENDSTCLR_POS;

pub const SCB_ICSR_ISRPREEMPT_POS       : u32   = 23;
pub const SCB_ICSR_ISRPREEMPT_MSK       : u32   = 1 << SCB_ICSR_ISRPREEMPT_POS;

pub const SCB_ICSR_ISRPENDING_POS       : u32   = 22;
pub const SCB_ICSR_ISRPENDING_MSK       : u32   = 1 << SCB_ICSR_ISRPENDING_POS;

pub const SCB_ICSR_VECTPENDING_POS      : u32   = 12;
pub const SCB_ICSR_VECTPENDING_MSK      : u32   = 0x1FF << SCB_ICSR_VECTPENDING_POS;

pub const SCB_ICSR_RETTOBASE_POS        : u32   = 11;
pub const SCB_ICSR_RETTOBASE_MSK        : u32   = 1 << SCB_ICSR_RETTOBASE_POS;

pub const SCB_ICSR_VECTACTIVE_POS       : u32   = 0;
pub const SCB_ICSR_VECTACTIVE_MSK       : u32   = 0x1FF << SCB_ICSR_VECTACTIVE_POS;

/// SCB Vector Table Offset Register Definitions
pub const SCB_VTOR_TBLOFF_POS           : u32   = 7;
pub const SCB_VTOR_TBLOFF_MSK           : u32   = 0x1FFFFFF << SCB_VTOR_TBLOFF_POS;

/// SCB Application Interrupt and Reset Control Register Definitions
pub const SCB_AIRCR_VECTKEY_POS         : u32   = 16;
pub const SCB_AIRCR_VECTKEY_MSK         : u32   = 0xFFFF << SCB_AIRCR_VECTKEY_POS;

pub const SCB_AIRCR_VECTKEYSTAT_POS     : u32   = 16;
pub const SCB_AIRCR_VECTKEYSTAT_MSK     : u32   = 0xFFFF << SCB_AIRCR_VECTKEYSTAT_POS;

pub const SCB_AIRCR_ENDIANESS_POS       : u32   = 15;
pub const SCB_AIRCR_ENDIANESS_MSK       : u32   = 1 << SCB_AIRCR_ENDIANESS_POS;

pub const SCB_AIRCR_PRIGROUP_POS        : u32   = 8;
pub const SCB_AIRCR_PRIGROUP_MSK        : u32   = 7 << SCB_AIRCR_PRIGROUP_POS;

pub const SCB_AIRCR_SYSRESETREQ_POS     : u32   = 2;
pub const SCB_AIRCR_SYSRESETREQ_MSK     : u32   = 1 << SCB_AIRCR_SYSRESETREQ_POS;

pub const SCB_AIRCR_VECTCLRACTIVE_POS   : u32   = 1;
pub const SCB_AIRCR_VECTCLRACTIVE_MSK   : u32   = 1 << SCB_AIRCR_VECTCLRACTIVE_POS;

pub const SCB_AIRCR_VECTRESET_POS       : u32   = 0;
pub const SCB_AIRCR_VECTRESET_MSK       : u32   = 1 << SCB_AIRCR_VECTRESET_POS;

/// SCB System Control Register Definitions
pub const SCB_SCR_SEVONPEND_POS         : u32   = 4;
pub const SCB_SCR_SEVONPEND_MSK         : u32   = 1 << SCB_SCR_SEVONPEND_POS;

pub const SCB_SCR_SLEEPDEEP_POS         : u32   = 2;
pub const SCB_SCR_SLEEPDEEP_MSK         : u32   = 1 << SCB_SCR_SLEEPDEEP_POS;

pub const SCB_SCR_SLEEPONEXIT_POS       : u32   = 1;
pub const SCB_SCR_SLEEPONEXIT_MSK       : u32   = 1 << SCB_SCR_SLEEPONEXIT_POS;

/// SCB Configuration Control Register Definitions
pub const SCB_CCR_STKALIGN_POS          : u32   = 9;
pub const SCB_CCR_STKALIGN_MSK          : u32   = 1 << SCB_CCR_STKALIGN_POS;

pub const SCB_CCR_BFHFNMIGN_POS         : u32   = 8;
pub const SCB_CCR_BFHFNMIGN_MSK         : u32   = 1 << SCB_CCR_BFHFNMIGN_POS;

pub const SCB_CCR_DIV_0_TRP_POS         : u32   = 4;
pub const SCB_CCR_DIV_0_TRP_MSK         : u32   = 1 << SCB_CCR_DIV_0_TRP_POS;

pub const SCB_CCR_UNALIGN_TRP_POS       : u32   = 3;
pub const SCB_CCR_UNALIGN_TRP_MSK       : u32   = 1 << SCB_CCR_UNALIGN_TRP_POS;

pub const SCB_CCR_USERSETMPEND_POS      : u32   = 1;
pub const SCB_CCR_USERSETMPEND_MSK      : u32   = 1 << SCB_CCR_USERSETMPEND_POS;

pub const SCB_CCR_NONBASETHRDENA_POS    : u32   = 0;
pub const SCB_CCR_NONBASETHRDENA_MSK    : u32   = 1 << SCB_CCR_NONBASETHRDENA_POS;

/// SCB System Handler Control and State Register Definitions
pub const SCB_SHCSR_USGFAULTENA_POS     : u32   = 18;
pub const SCB_SHCSR_USGFAULTENA_MSK     : u32   = 1 << SCB_SHCSR_USGFAULTENA_POS;

pub const SCB_SHCSR_BUSFAULTENA_POS     : u32   = 17;
pub const SCB_SHCSR_BUSFAULTENA_MSK     : u32   = 1 << SCB_SHCSR_BUSFAULTENA_POS;

pub const SCB_SHCSR_MEMFAULTENA_POS     : u32   = 16;
pub const SCB_SHCSR_MEMFAULTENA_MSK     : u32   = 1 << SCB_SHCSR_MEMFAULTENA_POS;

pub const SCB_SHCSR_SVCALLPENDED_POS    : u32   = 15;
pub const SCB_SHCSR_SVCALLPENDED_MSK    : u32   = 1 << SCB_SHCSR_SVCALLPENDED_POS;

pub const SCB_SHCSR_BUSFAULTPENDED_POS  : u32   = 14;
pub const SCB_SHCSR_BUSFAULTPENDED_MSK  : u32   = 1 << SCB_SHCSR_BUSFAULTPENDED_POS;

pub const SCB_SHCSR_MEMFAULTPENDED_POS  : u32   = 13;
pub const SCB_SHCSR_MEMFAULTPENDED_MSK  : u32   = 1 << SCB_SHCSR_MEMFAULTPENDED_POS;

pub const SCB_SHCSR_USGFAULTPENDED_POS  : u32   = 12;
pub const SCB_SHCSR_USGFAULTPENDED_MSK  : u32   = 1 << SCB_SHCSR_USGFAULTPENDED_POS;

pub const SCB_SHCSR_SYSTICKACT_POS      : u32   = 11;
pub const SCB_SHCSR_SYSTICKACT_MSK      : u32   = 1 << SCB_SHCSR_SYSTICKACT_POS;

pub const SCB_SHCSR_PENDSVACT_POS       : u32   = 10;
pub const SCB_SHCSR_PENDSVACT_MSK       : u32   = 1 << SCB_SHCSR_PENDSVACT_POS;

pub const SCB_SHCSR_MONITORACT_POS      : u32   = 8;
pub const SCB_SHCSR_MONITORACT_MSK      : u32   = 1 << SCB_SHCSR_MONITORACT_POS;

pub const SCB_SHCSR_SVCALLACT_POS       : u32   = 7;
pub const SCB_SHCSR_SVCALLACT_MSK       : u32   = 1 << SCB_SHCSR_SVCALLACT_POS;

pub const SCB_SHCSR_USGFAULTACT_POS     : u32   = 3;
pub const SCB_SHCSR_USGFAULTACT_MSK     : u32   = 1 << SCB_SHCSR_USGFAULTACT_POS;

pub const SCB_SHCSR_BUSFAULTACT_POS     : u32   = 1;
pub const SCB_SHCSR_BUSFAULTACT_MSK     : u32   = 1 << SCB_SHCSR_BUSFAULTACT_POS;

pub const SCB_SHCSR_MEMFAULTACT_POS     : u32   = 0;
pub const SCB_SHCSR_MEMFAULTACT_MSK     : u32   = 1 << SCB_SHCSR_MEMFAULTACT_POS;

/// SCB Configurable Fault Status Registers Definitions
pub const SCB_CFSR_USGFAULTSR_POS       : u32   = 16;
pub const SCB_CFSR_USGFAULTSR_MSK       : u32   = 0xFFFF << SCB_CFSR_USGFAULTSR_POS;

pub const SCB_CFSR_BUSFAULTSR_POS       : u32   = 8;
pub const SCB_CFSR_BUSFAULTSR_MSK       : u32   = 0xFF << SCB_CFSR_BUSFAULTSR_POS;

pub const SCB_CFSR_MEMFAULTSR_POS       : u32   = 0;
pub const SCB_CFSR_MEMFAULTSR_MSK       : u32   = 0xFF << SCB_CFSR_MEMFAULTSR_POS;

/// SCB Hard Fault Status Registers Definitions
pub const SCB_HFSR_DEBUGEVT_POS         : u32   = 31;
pub const SCB_HFSR_DEBUGEVT_MSK         : u32   = 1 << SCB_HFSR_DEBUGEVT_POS;

pub const SCB_HFSR_FORCED_POS           : u32   = 30;
pub const SCB_HFSR_FORCED_MSK           : u32   = 1 << SCB_HFSR_FORCED_POS;

pub const SCB_HFSR_VECTTBL_POS          : u32   = 1;
pub const SCB_HFSR_VECTTBL_MSK          : u32   = 1 << SCB_HFSR_VECTTBL_POS;

/// SCB Debug Fault Status Register Definitions
pub const SCB_DFSR_EXTERNAL_POS         : u32   = 4;
pub const SCB_DFSR_EXTERNAL_MSK         : u32   = 1 << SCB_DFSR_EXTERNAL_POS;

pub const SCB_DFSR_VCATCH_POS           : u32   = 3;
pub const SCB_DFSR_VCATCH_MSK           : u32   = 1 << SCB_DFSR_VCATCH_POS;

pub const SCB_DFSR_DWTTRAP_POS          : u32   = 2;
pub const SCB_DFSR_DWTTRAP_MSK          : u32   = 1 << SCB_DFSR_DWTTRAP_POS;

pub const SCB_DFSR_BKPT_POS             : u32   = 1;
pub const SCB_DFSR_BKPT_MSK             : u32   = 1 << SCB_DFSR_BKPT_POS;

pub const SCB_DFSR_HALTED_POS           : u32   = 0;
pub const SCB_DFSR_HALTED_MSK           : u32   = 1 << SCB_DFSR_HALTED_POS;

