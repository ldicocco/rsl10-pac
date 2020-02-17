#[doc = "Reader of register RF_REG39"]
pub type R = crate::R<u32, super::RF_REG39>;
#[doc = "If set to 1, the Xtal algorithm has finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_INFO_XTAL_FINISH_A {
    #[doc = "0: `0`"]
    ANALOG_INFO_XTAL_TRIM_RUNNING = 0,
    #[doc = "1: `1`"]
    ANALOG_INFO_XTAL_TRIM_FINISHED = 1,
}
impl From<ANALOG_INFO_XTAL_FINISH_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_INFO_XTAL_FINISH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANALOG_INFO_XTAL_FINISH`"]
pub type ANALOG_INFO_XTAL_FINISH_R = crate::R<bool, ANALOG_INFO_XTAL_FINISH_A>;
impl ANALOG_INFO_XTAL_FINISH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_INFO_XTAL_FINISH_A {
        match self.bits {
            false => ANALOG_INFO_XTAL_FINISH_A::ANALOG_INFO_XTAL_TRIM_RUNNING,
            true => ANALOG_INFO_XTAL_FINISH_A::ANALOG_INFO_XTAL_TRIM_FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INFO_XTAL_TRIM_RUNNING`"]
    #[inline(always)]
    pub fn is_analog_info_xtal_trim_running(&self) -> bool {
        *self == ANALOG_INFO_XTAL_FINISH_A::ANALOG_INFO_XTAL_TRIM_RUNNING
    }
    #[doc = "Checks if the value of the field is `ANALOG_INFO_XTAL_TRIM_FINISHED`"]
    #[inline(always)]
    pub fn is_analog_info_xtal_trim_finished(&self) -> bool {
        *self == ANALOG_INFO_XTAL_FINISH_A::ANALOG_INFO_XTAL_TRIM_FINISHED
    }
}
#[doc = "DLL locked signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_INFO_DLL_LOCKED_A {
    #[doc = "0: `0`"]
    ANALOG_INFO_DLL_UNLOCKED = 0,
    #[doc = "1: `1`"]
    ANALOG_INFO_DLL_LOCKED = 1,
}
impl From<ANALOG_INFO_DLL_LOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_INFO_DLL_LOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANALOG_INFO_DLL_LOCKED`"]
pub type ANALOG_INFO_DLL_LOCKED_R = crate::R<bool, ANALOG_INFO_DLL_LOCKED_A>;
impl ANALOG_INFO_DLL_LOCKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_INFO_DLL_LOCKED_A {
        match self.bits {
            false => ANALOG_INFO_DLL_LOCKED_A::ANALOG_INFO_DLL_UNLOCKED,
            true => ANALOG_INFO_DLL_LOCKED_A::ANALOG_INFO_DLL_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INFO_DLL_UNLOCKED`"]
    #[inline(always)]
    pub fn is_analog_info_dll_unlocked(&self) -> bool {
        *self == ANALOG_INFO_DLL_LOCKED_A::ANALOG_INFO_DLL_UNLOCKED
    }
    #[doc = "Checks if the value of the field is `ANALOG_INFO_DLL_LOCKED`"]
    #[inline(always)]
    pub fn is_analog_info_dll_locked(&self) -> bool {
        *self == ANALOG_INFO_DLL_LOCKED_A::ANALOG_INFO_DLL_LOCKED
    }
}
#[doc = "Ready signal of the digital clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_INFO_CLK_DIG_READY_A {
    #[doc = "0: `0`"]
    ANALOG_INFO_CLK_DIG_NOT_READY = 0,
    #[doc = "1: `1`"]
    ANALOG_INFO_CLK_DIG_READY = 1,
}
impl From<ANALOG_INFO_CLK_DIG_READY_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_INFO_CLK_DIG_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANALOG_INFO_CLK_DIG_READY`"]
pub type ANALOG_INFO_CLK_DIG_READY_R = crate::R<bool, ANALOG_INFO_CLK_DIG_READY_A>;
impl ANALOG_INFO_CLK_DIG_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_INFO_CLK_DIG_READY_A {
        match self.bits {
            false => ANALOG_INFO_CLK_DIG_READY_A::ANALOG_INFO_CLK_DIG_NOT_READY,
            true => ANALOG_INFO_CLK_DIG_READY_A::ANALOG_INFO_CLK_DIG_READY,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INFO_CLK_DIG_NOT_READY`"]
    #[inline(always)]
    pub fn is_analog_info_clk_dig_not_ready(&self) -> bool {
        *self == ANALOG_INFO_CLK_DIG_READY_A::ANALOG_INFO_CLK_DIG_NOT_READY
    }
    #[doc = "Checks if the value of the field is `ANALOG_INFO_CLK_DIG_READY`"]
    #[inline(always)]
    pub fn is_analog_info_clk_dig_ready(&self) -> bool {
        *self == ANALOG_INFO_CLK_DIG_READY_A::ANALOG_INFO_CLK_DIG_READY
    }
}
#[doc = "Reader of field `ANALOG_INFO_CLK_PLL_READY`"]
pub type ANALOG_INFO_CLK_PLL_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ANALOG_INFO_SUBBAND_HI`"]
pub type ANALOG_INFO_SUBBAND_HI_R = crate::R<bool, bool>;
#[doc = "Reader of field `ANALOG_INFO_SUBBAND_LO`"]
pub type ANALOG_INFO_SUBBAND_LO_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUBBAND_ERR_SB_FLL_ERR`"]
pub type SUBBAND_ERR_SB_FLL_ERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 13 - If set to 1, the Xtal algorithm has finished"]
    #[inline(always)]
    pub fn analog_info_xtal_finish(&self) -> ANALOG_INFO_XTAL_FINISH_R {
        ANALOG_INFO_XTAL_FINISH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DLL locked signal"]
    #[inline(always)]
    pub fn analog_info_dll_locked(&self) -> ANALOG_INFO_DLL_LOCKED_R {
        ANALOG_INFO_DLL_LOCKED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Ready signal of the digital clock"]
    #[inline(always)]
    pub fn analog_info_clk_dig_ready(&self) -> ANALOG_INFO_CLK_DIG_READY_R {
        ANALOG_INFO_CLK_DIG_READY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Ready signal of the PLL clock"]
    #[inline(always)]
    pub fn analog_info_clk_pll_ready(&self) -> ANALOG_INFO_CLK_PLL_READY_R {
        ANALOG_INFO_CLK_PLL_READY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of the subband comparator Hi"]
    #[inline(always)]
    pub fn analog_info_subband_hi(&self) -> ANALOG_INFO_SUBBAND_HI_R {
        ANALOG_INFO_SUBBAND_HI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of the subband comparator Lo"]
    #[inline(always)]
    pub fn analog_info_subband_lo(&self) -> ANALOG_INFO_SUBBAND_LO_R {
        ANALOG_INFO_SUBBAND_LO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - distance from the subband center (only available with the FLL method)"]
    #[inline(always)]
    pub fn subband_err_sb_fll_err(&self) -> SUBBAND_ERR_SB_FLL_ERR_R {
        SUBBAND_ERR_SB_FLL_ERR_R::new((self.bits & 0xff) as u8)
    }
}
