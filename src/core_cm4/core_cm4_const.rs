//! Memory mapping of Cortex-M4 Hardware

/// System Control Space Base Address
pub const SCS_BASE          : u32   = 0xE000E000;

/// ITM Base Address
pub const ITM_BASE          : u32   = 0xE0000000;

/// Core Debug Base Address
pub const CoreDebug_BASE    : u32   = 0xE000EDF0;

/// SysTick Base Address
pub const SysTick_BASE      : u32   = SCS_BASE + 0x0010;

/// NVIC Base Address
pub const NVIC_BASE         : u32   = SCS_BASE + 0x0100;

/// System Control Block Base Address
pub const SCB_BASE          : u32   = SCS_BASE + 0x0D00;

/// Memory Protection Unit
pub const MPU_BASE          : u32   = SCS_BASE + 0x0D90;

/// Floating Point Unit
pub const FPU_BASE          : u32   = SCS_BASE + 0x0F30;


// #define SCnSCB              ((SCnSCB_Type    *)     SCS_BASE      )   /*!< System control Register not in SCB
// #define SCB                 ((SCB_Type       *)     SCB_BASE      )   /*!< SCB configuration struct
// #define SysTick             ((SysTick_Type   *)     SysTick_BASE  )   /*!< SysTick configuration struct
// #define NVIC                ((NVIC_Type      *)     NVIC_BASE     )   /*!< NVIC configuration struct
// #define ITM                 ((ITM_Type       *)     ITM_BASE      )   /*!< ITM configuration struct
// #define CoreDebug           ((CoreDebug_Type *)     CoreDebug_BASE)   /*!< Core Debug configuration struct

