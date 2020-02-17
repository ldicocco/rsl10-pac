#[doc = "Reader of register CLK_DET_STATUS"]
pub type R = crate::R<u32, super::CLK_DET_STATUS>;
#[doc = "Clock detector interrupt status (cleared when read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DET_INT_STATUS_A {
    #[doc = "0: Clock detector interrupt not triggered"]
    CLK_DET_INT_FALSE = 0,
    #[doc = "1: Clock detector interrupt triggered"]
    CLK_DET_INT_TRUE = 1,
}
impl From<CLK_DET_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DET_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_DET_INT_STATUS`"]
pub type CLK_DET_INT_STATUS_R = crate::R<bool, CLK_DET_INT_STATUS_A>;
impl CLK_DET_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_INT_STATUS_A {
        match self.bits {
            false => CLK_DET_INT_STATUS_A::CLK_DET_INT_FALSE,
            true => CLK_DET_INT_STATUS_A::CLK_DET_INT_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DET_INT_FALSE`"]
    #[inline(always)]
    pub fn is_clk_det_int_false(&self) -> bool {
        *self == CLK_DET_INT_STATUS_A::CLK_DET_INT_FALSE
    }
    #[doc = "Checks if the value of the field is `CLK_DET_INT_TRUE`"]
    #[inline(always)]
    pub fn is_clk_det_int_true(&self) -> bool {
        *self == CLK_DET_INT_STATUS_A::CLK_DET_INT_TRUE
    }
}
#[doc = "Clock detector status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DET_STATUS_A {
    #[doc = "0: No clock detected"]
    CLK_NOT_ACTIVE = 0,
    #[doc = "1: Clock detected"]
    CLK_ACTIVE = 1,
}
impl From<CLK_DET_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DET_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_DET_STATUS`"]
pub type CLK_DET_STATUS_R = crate::R<bool, CLK_DET_STATUS_A>;
impl CLK_DET_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_STATUS_A {
        match self.bits {
            false => CLK_DET_STATUS_A::CLK_NOT_ACTIVE,
            true => CLK_DET_STATUS_A::CLK_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_clk_not_active(&self) -> bool {
        *self == CLK_DET_STATUS_A::CLK_NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `CLK_ACTIVE`"]
    #[inline(always)]
    pub fn is_clk_active(&self) -> bool {
        *self == CLK_DET_STATUS_A::CLK_ACTIVE
    }
}
impl R {
    #[doc = "Bit 1 - Clock detector interrupt status (cleared when read)"]
    #[inline(always)]
    pub fn clk_det_int_status(&self) -> CLK_DET_INT_STATUS_R {
        CLK_DET_INT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clock detector status"]
    #[inline(always)]
    pub fn clk_det_status(&self) -> CLK_DET_STATUS_R {
        CLK_DET_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
