#![allow(dead_code)]

//! Timer registers

use ::volatile_reg32::*;
use super::constants::{ TIM1_BASE,
                        TIM2_BASE,
                        TIM3_BASE,
                        TIM4_BASE,
                        TIM5_BASE,
                        TIM6_BASE,
                        TIM7_BASE,
                        TIM8_BASE,
                        TIM9_BASE,
                        TIM10_BASE,
                        TIM11_BASE,
                        TIM12_BASE,
                        TIM13_BASE,
                        TIM14_BASE,
                        };

pub struct TimRegs
{
    /// TIM control register 1
    pub cr1     : VolatileReg32,

    /// TIM control register 2
    pub cr2     : VolatileReg32,

    /// TIM slave mode control register
    pub smcr    : VolatileReg32,

    /// TIM DMA/interrupt enable register
    pub dier    : VolatileReg32,

    /// TIM status register
    pub sr      : VolatileReg32,

    /// TIM event generation register
    pub egr     : VolatileReg32,

    /// TIM capture/compare mode register 1
    pub ccmr1   : VolatileReg32,

    /// TIM capture/compare mode register 2
    pub ccmr2   : VolatileReg32,

    /// TIM capture/compare enable register
    pub ccer    : VolatileReg32,

    /// TIM counter register
    pub cnt     : VolatileReg32,

    /// TIM prescaler
    pub psc     : VolatileReg32,

    /// TIM auto-reload register
    pub arr     : VolatileReg32,

    /// TIM repetition counter register
    pub rcr     : VolatileReg32,

    /// TIM capture/compare register 1
    pub ccr1    : VolatileReg32,

    /// TIM capture/compare register 2
    pub ccr2    : VolatileReg32,

    /// TIM capture/compare register 3
    pub ccr3    : VolatileReg32,

    /// TIM capture/compare register 4
    pub ccr4    : VolatileReg32,

    /// TIM break and dead-time register
    pub bdtr    : VolatileReg32,

    /// TIM DMA control register
    pub dcr     : VolatileReg32,

    /// TIM DMA address for full transfer
    pub dmar    : VolatileReg32,

    /// TIM option register
    pub or      : VolatileReg32,
}

pub enum TimInst {
    TIM1,   // TIM1_BASE
    TIM2,   // TIM2_BASE
    TIM3,   // TIM3_BASE
    TIM4,   // TIM4_BASE
    TIM5,   // TIM5_BASE
    TIM6,   // TIM6_BASE
    TIM7,   // TIM7_BASE
    TIM8,   // TIM8_BASE
    TIM9,   // TIM9_BASE
    TIM10,  // TIM10_BASE
    TIM11,  // TIM11_BASE
    TIM12,  // TIM12_BASE
    TIM13,  // TIM13_BASE
    TIM14,  // TIM14_BASE
}

impl TimInst {
    pub fn init(tim_inst: TimInst) -> TimRegs {
        let tim_base = match tim_inst {
            TimInst::TIM1   => TIM1_BASE,
            TimInst::TIM2   => TIM2_BASE,
            TimInst::TIM3   => TIM3_BASE,
            TimInst::TIM4   => TIM4_BASE,
            TimInst::TIM5   => TIM5_BASE,
            TimInst::TIM6   => TIM6_BASE,
            TimInst::TIM7   => TIM7_BASE,
            TimInst::TIM8   => TIM8_BASE,
            TimInst::TIM9   => TIM9_BASE,
            TimInst::TIM10  => TIM10_BASE,
            TimInst::TIM11  => TIM11_BASE,
            TimInst::TIM12  => TIM12_BASE,
            TimInst::TIM13  => TIM13_BASE,
            TimInst::TIM14  => TIM14_BASE,
        } as *mut u32;

        let tim = TimRegs {
            cr1     : VolatileReg32::new(tim_base),
            cr2     : VolatileReg32::new_offset(tim_base, 1),
            smcr    : VolatileReg32::new_offset(tim_base, 2),
            dier    : VolatileReg32::new_offset(tim_base, 3),
            sr      : VolatileReg32::new_offset(tim_base, 4),
            egr     : VolatileReg32::new_offset(tim_base, 5),
            ccmr1   : VolatileReg32::new_offset(tim_base, 6),
            ccmr2   : VolatileReg32::new_offset(tim_base, 7),
            ccer    : VolatileReg32::new_offset(tim_base, 8),
            cnt     : VolatileReg32::new_offset(tim_base, 9),
            psc     : VolatileReg32::new_offset(tim_base, 10),
            arr     : VolatileReg32::new_offset(tim_base, 11),
            rcr     : VolatileReg32::new_offset(tim_base, 12),
            ccr1    : VolatileReg32::new_offset(tim_base, 13),
            ccr2    : VolatileReg32::new_offset(tim_base, 14),
            ccr3    : VolatileReg32::new_offset(tim_base, 15),
            ccr4    : VolatileReg32::new_offset(tim_base, 16),
            bdtr    : VolatileReg32::new_offset(tim_base, 17),
            dcr     : VolatileReg32::new_offset(tim_base, 18),
            dmar    : VolatileReg32::new_offset(tim_base, 19),
            or      : VolatileReg32::new_offset(tim_base, 20),
        };

        tim
    }
}

// Bit definition for TIM_CR1 register
pub const TIM_CR1_CEN           : u32   = 0x00000001;       // Counter enable
pub const TIM_CR1_UDIS          : u32   = 0x00000002;       // Update disable
pub const TIM_CR1_URS           : u32   = 0x00000004;       // Update request source
pub const TIM_CR1_OPM           : u32   = 0x00000008;       // One pulse mode
pub const TIM_CR1_DIR           : u32   = 0x00000010;       // Direction

pub const TIM_CR1_CMS           : u32   = 0x00000060;       // CMS[1:0] bits (Center-aligned mode selection)
pub const TIM_CR1_CMS_0         : u32   = 0x00000020;       // Bit 0
pub const TIM_CR1_CMS_1         : u32   = 0x00000040;       // Bit 1

pub const TIM_CR1_ARPE          : u32   = 0x00000080;       // Auto-reload preload enable

pub const TIM_CR1_CKD           : u32   = 0x00000300;       // CKD[1:0] bits (clock division)
pub const TIM_CR1_CKD_0         : u32   = 0x00000100;       // Bit 0
pub const TIM_CR1_CKD_1         : u32   = 0x00000200;       // Bit 1

// Bit definition for TIM_CR2 register
pub const TIM_CR2_CCPC          : u32   = 0x00000001;       // Capture/Compare Preloaded Control
pub const TIM_CR2_CCUS          : u32   = 0x00000004;       // Capture/Compare Control Update Selection
pub const TIM_CR2_CCDS          : u32   = 0x00000008;       // Capture/Compare DMA Selection

pub const TIM_CR2_MMS           : u32   = 0x00000070;       // MMS[2:0] bits (Master Mode Selection)
pub const TIM_CR2_MMS_0         : u32   = 0x00000010;       // Bit 0
pub const TIM_CR2_MMS_1         : u32   = 0x00000020;       // Bit 1
pub const TIM_CR2_MMS_2         : u32   = 0x00000040;       // Bit 2

pub const TIM_CR2_TI1S          : u32   = 0x00000080;       // TI1 Selection
pub const TIM_CR2_OIS1          : u32   = 0x00000100;       // Output Idle state 1 (OC1 output)
pub const TIM_CR2_OIS1N         : u32   = 0x00000200;       // Output Idle state 1 (OC1N output)
pub const TIM_CR2_OIS2          : u32   = 0x00000400;       // Output Idle state 2 (OC2 output)
pub const TIM_CR2_OIS2N         : u32   = 0x00000800;       // Output Idle state 2 (OC2N output)
pub const TIM_CR2_OIS3          : u32   = 0x00001000;       // Output Idle state 3 (OC3 output)
pub const TIM_CR2_OIS3N         : u32   = 0x00002000;       // Output Idle state 3 (OC3N output)
pub const TIM_CR2_OIS4          : u32   = 0x00004000;       // Output Idle state 4 (OC4 output)

// Bit definition for TIM_SMCR register
pub const TIM_SMCR_SMS          : u32   = 0x00000007;       // SMS[2:0] bits (Slave mode selection)
pub const TIM_SMCR_SMS_0        : u32   = 0x00000001;       // Bit 0
pub const TIM_SMCR_SMS_1        : u32   = 0x00000002;       // Bit 1
pub const TIM_SMCR_SMS_2        : u32   = 0x00000004;       // Bit 2

pub const TIM_SMCR_TS           : u32   = 0x00000070;       // TS[2:0] bits (Trigger selection)
pub const TIM_SMCR_TS_0         : u32   = 0x00000010;       // Bit 0
pub const TIM_SMCR_TS_1         : u32   = 0x00000020;       // Bit 1
pub const TIM_SMCR_TS_2         : u32   = 0x00000040;       // Bit 2

pub const TIM_SMCR_MSM          : u32   = 0x00000080;       // Master/slave mode

pub const TIM_SMCR_ETF          : u32   = 0x00000F00;       // ETF[3:0] bits (External trigger filter)
pub const TIM_SMCR_ETF_0        : u32   = 0x00000100;       // Bit 0
pub const TIM_SMCR_ETF_1        : u32   = 0x00000200;       // Bit 1
pub const TIM_SMCR_ETF_2        : u32   = 0x00000400;       // Bit 2
pub const TIM_SMCR_ETF_3        : u32   = 0x00000800;       // Bit 3

pub const TIM_SMCR_ETPS         : u32   = 0x00003000;       // ETPS[1:0] bits (External trigger prescaler)
pub const TIM_SMCR_ETPS_0       : u32   = 0x00001000;       // Bit 0
pub const TIM_SMCR_ETPS_1       : u32   = 0x00002000;       // Bit 1

pub const TIM_SMCR_ECE          : u32   = 0x00004000;       // External clock enable
pub const TIM_SMCR_ETP          : u32   = 0x00008000;       // External trigger polarity

// Bit definition for TIM_DIER register
pub const TIM_DIER_UIE          : u32   = 0x00000001;       // Update interrupt enable
pub const TIM_DIER_CC1IE        : u32   = 0x00000002;       // Capture/Compare 1 interrupt enable
pub const TIM_DIER_CC2IE        : u32   = 0x00000004;       // Capture/Compare 2 interrupt enable
pub const TIM_DIER_CC3IE        : u32   = 0x00000008;       // Capture/Compare 3 interrupt enable
pub const TIM_DIER_CC4IE        : u32   = 0x00000010;       // Capture/Compare 4 interrupt enable
pub const TIM_DIER_COMIE        : u32   = 0x00000020;       // COM interrupt enable
pub const TIM_DIER_TIE          : u32   = 0x00000040;       // Trigger interrupt enable
pub const TIM_DIER_BIE          : u32   = 0x00000080;       // Break interrupt enable
pub const TIM_DIER_UDE          : u32   = 0x00000100;       // Update DMA request enable
pub const TIM_DIER_CC1DE        : u32   = 0x00000200;       // Capture/Compare 1 DMA request enable
pub const TIM_DIER_CC2DE        : u32   = 0x00000400;       // Capture/Compare 2 DMA request enable
pub const TIM_DIER_CC3DE        : u32   = 0x00000800;       // Capture/Compare 3 DMA request enable
pub const TIM_DIER_CC4DE        : u32   = 0x00001000;       // Capture/Compare 4 DMA request enable
pub const TIM_DIER_COMDE        : u32   = 0x00002000;       // COM DMA request enable
pub const TIM_DIER_TDE          : u32   = 0x00004000;       // Trigger DMA request enable

// Bit definition for TIM_SR register
pub const TIM_SR_UIF            : u32   = 0x00000001;       // Update interrupt Flag
pub const TIM_SR_CC1IF          : u32   = 0x00000002;       // Capture/Compare 1 interrupt Flag
pub const TIM_SR_CC2IF          : u32   = 0x00000004;       // Capture/Compare 2 interrupt Flag
pub const TIM_SR_CC3IF          : u32   = 0x00000008;       // Capture/Compare 3 interrupt Flag
pub const TIM_SR_CC4IF          : u32   = 0x00000010;       // Capture/Compare 4 interrupt Flag
pub const TIM_SR_COMIF          : u32   = 0x00000020;       // COM interrupt Flag
pub const TIM_SR_TIF            : u32   = 0x00000040;       // Trigger interrupt Flag
pub const TIM_SR_BIF            : u32   = 0x00000080;       // Break interrupt Flag
pub const TIM_SR_CC1OF          : u32   = 0x00000200;       // Capture/Compare 1 Overcapture Flag
pub const TIM_SR_CC2OF          : u32   = 0x00000400;       // Capture/Compare 2 Overcapture Flag
pub const TIM_SR_CC3OF          : u32   = 0x00000800;       // Capture/Compare 3 Overcapture Flag
pub const TIM_SR_CC4OF          : u32   = 0x00001000;       // Capture/Compare 4 Overcapture Flag

// Bit definition for TIM_EGR register
pub const TIM_EGR_UG            : u32   = 0x00000001;       // Update Generation
pub const TIM_EGR_CC1G          : u32   = 0x00000002;       // Capture/Compare 1 Generation
pub const TIM_EGR_CC2G          : u32   = 0x00000004;       // Capture/Compare 2 Generation
pub const TIM_EGR_CC3G          : u32   = 0x00000008;       // Capture/Compare 3 Generation
pub const TIM_EGR_CC4G          : u32   = 0x00000010;       // Capture/Compare 4 Generation
pub const TIM_EGR_COMG          : u32   = 0x00000020;       // Capture/Compare Control Update Generation
pub const TIM_EGR_TG            : u32   = 0x00000040;       // Trigger Generation
pub const TIM_EGR_BG            : u32   = 0x00000080;       // Break Generation

// Bit definition for TIM_CCMR1 register
pub const TIM_CCMR1_CC1S        : u32   = 0x00000003;       // CC1S[1:0] bits (Capture/Compare 1 Selection)
pub const TIM_CCMR1_CC1S_0      : u32   = 0x00000001;       // Bit 0
pub const TIM_CCMR1_CC1S_1      : u32   = 0x00000002;       // Bit 1

pub const TIM_CCMR1_OC1FE       : u32   = 0x00000004;       // Output Compare 1 Fast enable
pub const TIM_CCMR1_OC1PE       : u32   = 0x00000008;       // Output Compare 1 Preload enable

pub const TIM_CCMR1_OC1M        : u32   = 0x00000070;       // OC1M[2:0] bits (Output Compare 1 Mode)
pub const TIM_CCMR1_OC1M_0      : u32   = 0x00000010;       // Bit 0
pub const TIM_CCMR1_OC1M_1      : u32   = 0x00000020;       // Bit 1
pub const TIM_CCMR1_OC1M_2      : u32   = 0x00000040;       // Bit 2

pub const TIM_CCMR1_OC1CE       : u32   = 0x00000080;       // Output Compare 1Clear Enable

pub const TIM_CCMR1_CC2S        : u32   = 0x00000300;       // CC2S[1:0] bits (Capture/Compare 2 Selection)
pub const TIM_CCMR1_CC2S_0      : u32   = 0x00000100;       // Bit 0
pub const TIM_CCMR1_CC2S_1      : u32   = 0x00000200;       // Bit 1

pub const TIM_CCMR1_OC2FE       : u32   = 0x00000400;       // Output Compare 2 Fast enable
pub const TIM_CCMR1_OC2PE       : u32   = 0x00000800;       // Output Compare 2 Preload enable

pub const TIM_CCMR1_OC2M        : u32   = 0x00007000;       // OC2M[2:0] bits (Output Compare 2 Mode)
pub const TIM_CCMR1_OC2M_0      : u32   = 0x00001000;       // Bit 0
pub const TIM_CCMR1_OC2M_1      : u32   = 0x00002000;       // Bit 1
pub const TIM_CCMR1_OC2M_2      : u32   = 0x00004000;       // Bit 2

pub const TIM_CCMR1_OC2CE       : u32   = 0x00008000;       // Output Compare 2 Clear Enable

//----------------------------------------------------------------------------

pub const TIM_CCMR1_IC1PSC      : u32   = 0x0000000C;       // IC1PSC[1:0] bits (Input Capture 1 Prescaler)
pub const TIM_CCMR1_IC1PSC_0    : u32   = 0x00000004;       // Bit 0
pub const TIM_CCMR1_IC1PSC_1    : u32   = 0x00000008;       // Bit 1

pub const TIM_CCMR1_IC1F        : u32   = 0x000000F0;       // IC1F[3:0] bits (Input Capture 1 Filter)
pub const TIM_CCMR1_IC1F_0      : u32   = 0x00000010;       // Bit 0
pub const TIM_CCMR1_IC1F_1      : u32   = 0x00000020;       // Bit 1
pub const TIM_CCMR1_IC1F_2      : u32   = 0x00000040;       // Bit 2
pub const TIM_CCMR1_IC1F_3      : u32   = 0x00000080;       // Bit 3

pub const TIM_CCMR1_IC2PSC      : u32   = 0x00000C00;       // IC2PSC[1:0] bits (Input Capture 2 Prescaler)
pub const TIM_CCMR1_IC2PSC_0    : u32   = 0x00000400;       // Bit 0
pub const TIM_CCMR1_IC2PSC_1    : u32   = 0x00000800;       // Bit 1

pub const TIM_CCMR1_IC2F        : u32   = 0x0000F000;       // IC2F[3:0] bits (Input Capture 2 Filter)
pub const TIM_CCMR1_IC2F_0      : u32   = 0x00001000;       // Bit 0
pub const TIM_CCMR1_IC2F_1      : u32   = 0x00002000;       // Bit 1
pub const TIM_CCMR1_IC2F_2      : u32   = 0x00004000;       // Bit 2
pub const TIM_CCMR1_IC2F_3      : u32   = 0x00008000;       // Bit 3

// Bit definition for TIM_CCMR2 register
pub const TIM_CCMR2_CC3S        : u32   = 0x00000003;       // CC3S[1:0] bits (Capture/Compare 3 Selection)
pub const TIM_CCMR2_CC3S_0      : u32   = 0x00000001;       // Bit 0
pub const TIM_CCMR2_CC3S_1      : u32   = 0x00000002;       // Bit 1

pub const TIM_CCMR2_OC3FE       : u32   = 0x00000004;       // Output Compare 3 Fast enable
pub const TIM_CCMR2_OC3PE       : u32   = 0x00000008;       // Output Compare 3 Preload enable

pub const TIM_CCMR2_OC3M        : u32   = 0x00000070;       // OC3M[2:0] bits (Output Compare 3 Mode)
pub const TIM_CCMR2_OC3M_0      : u32   = 0x00000010;       // Bit 0
pub const TIM_CCMR2_OC3M_1      : u32   = 0x00000020;       // Bit 1
pub const TIM_CCMR2_OC3M_2      : u32   = 0x00000040;       // Bit 2

pub const TIM_CCMR2_OC3CE       : u32   = 0x00000080;       // Output Compare 3 Clear Enable

pub const TIM_CCMR2_CC4S        : u32   = 0x00000300;       // CC4S[1:0] bits (Capture/Compare 4 Selection)
pub const TIM_CCMR2_CC4S_0      : u32   = 0x00000100;       // Bit 0
pub const TIM_CCMR2_CC4S_1      : u32   = 0x00000200;       // Bit 1

pub const TIM_CCMR2_OC4FE       : u32   = 0x00000400;       // Output Compare 4 Fast enable
pub const TIM_CCMR2_OC4PE       : u32   = 0x00000800;       // Output Compare 4 Preload enable

pub const TIM_CCMR2_OC4M        : u32   = 0x00007000;       // OC4M[2:0] bits (Output Compare 4 Mode)
pub const TIM_CCMR2_OC4M_0      : u32   = 0x00001000;       // Bit 0
pub const TIM_CCMR2_OC4M_1      : u32   = 0x00002000;       // Bit 1
pub const TIM_CCMR2_OC4M_2      : u32   = 0x00004000;       // Bit 2

pub const TIM_CCMR2_OC4CE       : u32   = 0x00008000;       // Output Compare 4 Clear Enable

//----------------------------------------------------------------------------

pub const TIM_CCMR2_IC3PSC      : u32   = 0x0000000C;       // IC3PSC[1:0] bits (Input Capture 3 Prescaler)
pub const TIM_CCMR2_IC3PSC_0    : u32   = 0x00000004;       // Bit 0
pub const TIM_CCMR2_IC3PSC_1    : u32   = 0x00000008;       // Bit 1

pub const TIM_CCMR2_IC3F        : u32   = 0x000000F0;       // IC3F[3:0] bits (Input Capture 3 Filter)
pub const TIM_CCMR2_IC3F_0      : u32   = 0x00000010;       // Bit 0
pub const TIM_CCMR2_IC3F_1      : u32   = 0x00000020;       // Bit 1
pub const TIM_CCMR2_IC3F_2      : u32   = 0x00000040;       // Bit 2
pub const TIM_CCMR2_IC3F_3      : u32   = 0x00000080;       // Bit 3

pub const TIM_CCMR2_IC4PSC      : u32   = 0x00000C00;       // IC4PSC[1:0] bits (Input Capture 4 Prescaler)
pub const TIM_CCMR2_IC4PSC_0    : u32   = 0x00000400;       // Bit 0
pub const TIM_CCMR2_IC4PSC_1    : u32   = 0x00000800;       // Bit 1

pub const TIM_CCMR2_IC4F        : u32   = 0x0000F000;       // IC4F[3:0] bits (Input Capture 4 Filter)
pub const TIM_CCMR2_IC4F_0      : u32   = 0x00001000;       // Bit 0
pub const TIM_CCMR2_IC4F_1      : u32   = 0x00002000;       // Bit 1
pub const TIM_CCMR2_IC4F_2      : u32   = 0x00004000;       // Bit 2
pub const TIM_CCMR2_IC4F_3      : u32   = 0x00008000;       // Bit 3

// Bit definition for TIM_CCER register
pub const TIM_CCER_CC1E         : u32   = 0x00000001;       // Capture/Compare 1 output enable
pub const TIM_CCER_CC1P         : u32   = 0x00000002;       // Capture/Compare 1 output Polarity
pub const TIM_CCER_CC1NE        : u32   = 0x00000004;       // Capture/Compare 1 Complementary output enable
pub const TIM_CCER_CC1NP        : u32   = 0x00000008;       // Capture/Compare 1 Complementary output Polarity
pub const TIM_CCER_CC2E         : u32   = 0x00000010;       // Capture/Compare 2 output enable
pub const TIM_CCER_CC2P         : u32   = 0x00000020;       // Capture/Compare 2 output Polarity
pub const TIM_CCER_CC2NE        : u32   = 0x00000040;       // Capture/Compare 2 Complementary output enable
pub const TIM_CCER_CC2NP        : u32   = 0x00000080;       // Capture/Compare 2 Complementary output Polarity
pub const TIM_CCER_CC3E         : u32   = 0x00000100;       // Capture/Compare 3 output enable
pub const TIM_CCER_CC3P         : u32   = 0x00000200;       // Capture/Compare 3 output Polarity
pub const TIM_CCER_CC3NE        : u32   = 0x00000400;       // Capture/Compare 3 Complementary output enable
pub const TIM_CCER_CC3NP        : u32   = 0x00000800;       // Capture/Compare 3 Complementary output Polarity
pub const TIM_CCER_CC4E         : u32   = 0x00001000;       // Capture/Compare 4 output enable
pub const TIM_CCER_CC4P         : u32   = 0x00002000;       // Capture/Compare 4 output Polarity
pub const TIM_CCER_CC4NP        : u32   = 0x00008000;       // Capture/Compare 4 Complementary output Polarity

// Bit definition for TIM_CNT register
pub const TIM_CNT_CNT           :u32    = 0x0000FFFF;       // Counter Value

// Bit definition for TIM_PSC register
pub const TIM_PSC_PSC           : u32   = 0x0000FFFF;       // Prescaler Value

// Bit definition for TIM_ARR register
pub const TIM_ARR_ARR           : u32   = 0x0000FFFF;       // actual auto-reload Value

// Bit definition for TIM_RCR register
pub const TIM_RCR_REP           : u32   = 0x000000FF;       // Repetition Counter Value

// Bit definition for TIM_CCR1 register
pub const TIM_CCR1_CCR1         : u32   = 0x0000FFFF;       // Capture/Compare 1 Value

// Bit definition for TIM_CCR2 register
pub const TIM_CCR2_CCR2         : u32   = 0x0000FFFF;       // Capture/Compare 2 Value

// Bit definition for TIM_CCR3 register
pub const TIM_CCR3_CCR3         : u32   = 0x0000FFFF;       // Capture/Compare 3 Value

// Bit definition for TIM_CCR4 register
pub const TIM_CCR4_CCR4         : u32   = 0x0000FFFF;       // Capture/Compare 4 Value

// Bit definition for TIM_BDTR register
pub const TIM_BDTR_DTG          : u32   = 0x000000FF;       // DTG[0:7] bits (Dead-Time Generator set-up)
pub const TIM_BDTR_DTG_0        : u32   = 0x00000001;       // Bit 0
pub const TIM_BDTR_DTG_1        : u32   = 0x00000002;       // Bit 1
pub const TIM_BDTR_DTG_2        : u32   = 0x00000004;       // Bit 2
pub const TIM_BDTR_DTG_3        : u32   = 0x00000008;       // Bit 3
pub const TIM_BDTR_DTG_4        : u32   = 0x00000010;       // Bit 4
pub const TIM_BDTR_DTG_5        : u32   = 0x00000020;       // Bit 5
pub const TIM_BDTR_DTG_6        : u32   = 0x00000040;       // Bit 6
pub const TIM_BDTR_DTG_7        : u32   = 0x00000080;       // Bit 7

pub const TIM_BDTR_LOCK         : u32   = 0x00000300;       // LOCK[1:0] bits (Lock Configuration)
pub const TIM_BDTR_LOCK_0       : u32   = 0x00000100;       // Bit 0
pub const TIM_BDTR_LOCK_1       : u32   = 0x00000200;       // Bit 1

pub const TIM_BDTR_OSSI         : u32   = 0x00000400;       // Off-State Selection for Idle mode
pub const TIM_BDTR_OSSR         : u32   = 0x00000800;       // Off-State Selection for Run mode
pub const TIM_BDTR_BKE          : u32   = 0x00001000;       // Break enable
pub const TIM_BDTR_BKP          : u32   = 0x00002000;       // Break Polarity
pub const TIM_BDTR_AOE          : u32   = 0x00004000;       // Automatic Output enable
pub const TIM_BDTR_MOE          : u32   = 0x00008000;       // Main Output enable

// Bit definition for TIM_DCR register
pub const TIM_DCR_DBA           : u32   = 0x0000001F;       // DBA[4:0] bits (DMA Base Address)
pub const TIM_DCR_DBA_0         : u32   = 0x00000001;       // Bit 0
pub const TIM_DCR_DBA_1         : u32   = 0x00000002;       // Bit 1
pub const TIM_DCR_DBA_2         : u32   = 0x00000004;       // Bit 2
pub const TIM_DCR_DBA_3         : u32   = 0x00000008;       // Bit 3
pub const TIM_DCR_DBA_4         : u32   = 0x00000010;       // Bit 4

pub const TIM_DCR_DBL           : u32   = 0x00001F00;       // DBL[4:0] bits (DMA Burst Length)
pub const TIM_DCR_DBL_0         : u32   = 0x00000100;       // Bit 0
pub const TIM_DCR_DBL_1         : u32   = 0x00000200;       // Bit 1
pub const TIM_DCR_DBL_2         : u32   = 0x00000400;       // Bit 2
pub const TIM_DCR_DBL_3         : u32   = 0x00000800;       // Bit 3
pub const TIM_DCR_DBL_4         : u32   = 0x00001000;       // Bit 4

// Bit definition for TIM_DMAR register
pub const TIM_DMAR_DMAB         : u32   = 0x0000FFFF;       // DMA register for burst accesses

// Bit definition for TIM_OR register
pub const TIM_OR_TI4_RMP        : u32   = 0x000000C0;       // TI4_RMP[1:0] bits (TIM5 Input 4 remap)
pub const TIM_OR_TI4_RMP_0      : u32   = 0x00000040;       // Bit 0
pub const TIM_OR_TI4_RMP_1      : u32   = 0x00000080;       // Bit 1
pub const TIM_OR_ITR1_RMP       : u32   = 0x00000C00;       // ITR1_RMP[1:0] bits (TIM2 Internal trigger 1 remap)
pub const TIM_OR_ITR1_RMP_0     : u32   = 0x00000400;       // Bit 0
pub const TIM_OR_ITR1_RMP_1     : u32   = 0x00000800;       // Bit 1

