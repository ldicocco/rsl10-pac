#[doc = "Reader of register ACS_VCC_CTRL"]
pub type R = crate::R<u32, super::ACS_VCC_CTRL>;
#[doc = "Writer for register ACS_VCC_CTRL"]
pub type W = crate::W<u32, super::ACS_VCC_CTRL>;
#[doc = "Register ACS_VCC_CTRL `reset()`'s with value 0x0004_020a"]
impl crate::ResetValue for super::ACS_VCC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_020a
    }
}
#[doc = "Inductor charge current trimming\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICH_TRIM_A {
    #[doc = "0: Charge pump max current to 16 mA"]
    VCC_ICHTRIM_16MA = 0,
    #[doc = "1: Charge pump max current to 32 mA"]
    VCC_ICHTRIM_32MA = 1,
    #[doc = "3: Charge pump max current to 64 mA"]
    VCC_ICHTRIM_64MA = 3,
    #[doc = "4: Charge pump max current to 80 mA"]
    VCC_ICHTRIM_80MA = 4,
    #[doc = "15: Charge pump max current to 256 mA"]
    VCC_ICHTRIM_256MA = 15,
}
impl From<ICH_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: ICH_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ICH_TRIM`"]
pub type ICH_TRIM_R = crate::R<u8, ICH_TRIM_A>;
impl ICH_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ICH_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ICH_TRIM_A::VCC_ICHTRIM_16MA),
            1 => Val(ICH_TRIM_A::VCC_ICHTRIM_32MA),
            3 => Val(ICH_TRIM_A::VCC_ICHTRIM_64MA),
            4 => Val(ICH_TRIM_A::VCC_ICHTRIM_80MA),
            15 => Val(ICH_TRIM_A::VCC_ICHTRIM_256MA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VCC_ICHTRIM_16MA`"]
    #[inline(always)]
    pub fn is_vcc_ichtrim_16ma(&self) -> bool {
        *self == ICH_TRIM_A::VCC_ICHTRIM_16MA
    }
    #[doc = "Checks if the value of the field is `VCC_ICHTRIM_32MA`"]
    #[inline(always)]
    pub fn is_vcc_ichtrim_32ma(&self) -> bool {
        *self == ICH_TRIM_A::VCC_ICHTRIM_32MA
    }
    #[doc = "Checks if the value of the field is `VCC_ICHTRIM_64MA`"]
    #[inline(always)]
    pub fn is_vcc_ichtrim_64ma(&self) -> bool {
        *self == ICH_TRIM_A::VCC_ICHTRIM_64MA
    }
    #[doc = "Checks if the value of the field is `VCC_ICHTRIM_80MA`"]
    #[inline(always)]
    pub fn is_vcc_ichtrim_80ma(&self) -> bool {
        *self == ICH_TRIM_A::VCC_ICHTRIM_80MA
    }
    #[doc = "Checks if the value of the field is `VCC_ICHTRIM_256MA`"]
    #[inline(always)]
    pub fn is_vcc_ichtrim_256ma(&self) -> bool {
        *self == ICH_TRIM_A::VCC_ICHTRIM_256MA
    }
}
#[doc = "Write proxy for field `ICH_TRIM`"]
pub struct ICH_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICH_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICH_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Charge pump max current to 16 mA"]
    #[inline(always)]
    pub fn vcc_ichtrim_16ma(self) -> &'a mut W {
        self.variant(ICH_TRIM_A::VCC_ICHTRIM_16MA)
    }
    #[doc = "Charge pump max current to 32 mA"]
    #[inline(always)]
    pub fn vcc_ichtrim_32ma(self) -> &'a mut W {
        self.variant(ICH_TRIM_A::VCC_ICHTRIM_32MA)
    }
    #[doc = "Charge pump max current to 64 mA"]
    #[inline(always)]
    pub fn vcc_ichtrim_64ma(self) -> &'a mut W {
        self.variant(ICH_TRIM_A::VCC_ICHTRIM_64MA)
    }
    #[doc = "Charge pump max current to 80 mA"]
    #[inline(always)]
    pub fn vcc_ichtrim_80ma(self) -> &'a mut W {
        self.variant(ICH_TRIM_A::VCC_ICHTRIM_80MA)
    }
    #[doc = "Charge pump max current to 256 mA"]
    #[inline(always)]
    pub fn vcc_ichtrim_256ma(self) -> &'a mut W {
        self.variant(ICH_TRIM_A::VCC_ICHTRIM_256MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Enable CCM mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCM_ENABLE_A {
    #[doc = "0: Discontinous current mode"]
    VCC_DCM_MODE = 0,
    #[doc = "1: Continuous current mode enabled"]
    VCC_CCM_MODE = 1,
}
impl From<CCM_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CCM_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCM_ENABLE`"]
pub type CCM_ENABLE_R = crate::R<bool, CCM_ENABLE_A>;
impl CCM_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCM_ENABLE_A {
        match self.bits {
            false => CCM_ENABLE_A::VCC_DCM_MODE,
            true => CCM_ENABLE_A::VCC_CCM_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `VCC_DCM_MODE`"]
    #[inline(always)]
    pub fn is_vcc_dcm_mode(&self) -> bool {
        *self == CCM_ENABLE_A::VCC_DCM_MODE
    }
    #[doc = "Checks if the value of the field is `VCC_CCM_MODE`"]
    #[inline(always)]
    pub fn is_vcc_ccm_mode(&self) -> bool {
        *self == CCM_ENABLE_A::VCC_CCM_MODE
    }
}
#[doc = "Write proxy for field `CCM_ENABLE`"]
pub struct CCM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCM_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Discontinous current mode"]
    #[inline(always)]
    pub fn vcc_dcm_mode(self) -> &'a mut W {
        self.variant(CCM_ENABLE_A::VCC_DCM_MODE)
    }
    #[doc = "Continuous current mode enabled"]
    #[inline(always)]
    pub fn vcc_ccm_mode(self) -> &'a mut W {
        self.variant(CCM_ENABLE_A::VCC_CCM_MODE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Pulse mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSE_CTRL_A {
    #[doc = "0: Single pulse per clock cycle"]
    VCC_SINGLE_PULSE = 0,
    #[doc = "1: Multi pulses enabled (until VCC > VCC_TRIM)"]
    VCC_MULTI_PULSE = 1,
}
impl From<PULSE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: PULSE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PULSE_CTRL`"]
pub type PULSE_CTRL_R = crate::R<bool, PULSE_CTRL_A>;
impl PULSE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULSE_CTRL_A {
        match self.bits {
            false => PULSE_CTRL_A::VCC_SINGLE_PULSE,
            true => PULSE_CTRL_A::VCC_MULTI_PULSE,
        }
    }
    #[doc = "Checks if the value of the field is `VCC_SINGLE_PULSE`"]
    #[inline(always)]
    pub fn is_vcc_single_pulse(&self) -> bool {
        *self == PULSE_CTRL_A::VCC_SINGLE_PULSE
    }
    #[doc = "Checks if the value of the field is `VCC_MULTI_PULSE`"]
    #[inline(always)]
    pub fn is_vcc_multi_pulse(&self) -> bool {
        *self == PULSE_CTRL_A::VCC_MULTI_PULSE
    }
}
#[doc = "Write proxy for field `PULSE_CTRL`"]
pub struct PULSE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULSE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single pulse per clock cycle"]
    #[inline(always)]
    pub fn vcc_single_pulse(self) -> &'a mut W {
        self.variant(PULSE_CTRL_A::VCC_SINGLE_PULSE)
    }
    #[doc = "Multi pulses enabled (until VCC > VCC_TRIM)"]
    #[inline(always)]
    pub fn vcc_multi_pulse(self) -> &'a mut W {
        self.variant(PULSE_CTRL_A::VCC_MULTI_PULSE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Charge mode control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHARGE_CTRL_A {
    #[doc = "0: Constant charge transfer (valid for VBAT > VCC+0.2)"]
    VCC_CONSTANT_CHARGE = 0,
    #[doc = "1: Constant inductor maximum charge current"]
    VCC_CONSTANT_IMAX = 1,
}
impl From<CHARGE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CHARGE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHARGE_CTRL`"]
pub type CHARGE_CTRL_R = crate::R<bool, CHARGE_CTRL_A>;
impl CHARGE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHARGE_CTRL_A {
        match self.bits {
            false => CHARGE_CTRL_A::VCC_CONSTANT_CHARGE,
            true => CHARGE_CTRL_A::VCC_CONSTANT_IMAX,
        }
    }
    #[doc = "Checks if the value of the field is `VCC_CONSTANT_CHARGE`"]
    #[inline(always)]
    pub fn is_vcc_constant_charge(&self) -> bool {
        *self == CHARGE_CTRL_A::VCC_CONSTANT_CHARGE
    }
    #[doc = "Checks if the value of the field is `VCC_CONSTANT_IMAX`"]
    #[inline(always)]
    pub fn is_vcc_constant_imax(&self) -> bool {
        *self == CHARGE_CTRL_A::VCC_CONSTANT_IMAX
    }
}
#[doc = "Write proxy for field `CHARGE_CTRL`"]
pub struct CHARGE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHARGE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHARGE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Constant charge transfer (valid for VBAT > VCC+0.2)"]
    #[inline(always)]
    pub fn vcc_constant_charge(self) -> &'a mut W {
        self.variant(CHARGE_CTRL_A::VCC_CONSTANT_CHARGE)
    }
    #[doc = "Constant inductor maximum charge current"]
    #[inline(always)]
    pub fn vcc_constant_imax(self) -> &'a mut W {
        self.variant(CHARGE_CTRL_A::VCC_CONSTANT_IMAX)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable buck converter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUCK_ENABLE_A {
    #[doc = "0: Linear mode"]
    VCC_LDO = 0,
    #[doc = "1: Buck converter mode enabled"]
    VCC_BUCK = 1,
}
impl From<BUCK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BUCK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUCK_ENABLE`"]
pub type BUCK_ENABLE_R = crate::R<bool, BUCK_ENABLE_A>;
impl BUCK_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUCK_ENABLE_A {
        match self.bits {
            false => BUCK_ENABLE_A::VCC_LDO,
            true => BUCK_ENABLE_A::VCC_BUCK,
        }
    }
    #[doc = "Checks if the value of the field is `VCC_LDO`"]
    #[inline(always)]
    pub fn is_vcc_ldo(&self) -> bool {
        *self == BUCK_ENABLE_A::VCC_LDO
    }
    #[doc = "Checks if the value of the field is `VCC_BUCK`"]
    #[inline(always)]
    pub fn is_vcc_buck(&self) -> bool {
        *self == BUCK_ENABLE_A::VCC_BUCK
    }
}
#[doc = "Write proxy for field `BUCK_ENABLE`"]
pub struct BUCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUCK_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Linear mode"]
    #[inline(always)]
    pub fn vcc_ldo(self) -> &'a mut W {
        self.variant(BUCK_ENABLE_A::VCC_LDO)
    }
    #[doc = "Buck converter mode enabled"]
    #[inline(always)]
    pub fn vcc_buck(self) -> &'a mut W {
        self.variant(BUCK_ENABLE_A::VCC_BUCK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Output voltage trimming configuration in 10 mV steps\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VTRIM_A {
    #[doc = "0: Output voltage 1.00V"]
    VCC_TRIM_1P00V = 0,
    #[doc = "5: Output voltage 1.05V"]
    VCC_TRIM_1P05V = 5,
    #[doc = "10: Output voltage 1.10V"]
    VCC_TRIM_1P10V = 10,
    #[doc = "15: Output voltage 1.15V"]
    VCC_TRIM_1P15V = 15,
    #[doc = "20: Output voltage 1.20V"]
    VCC_TRIM_1P20V = 20,
    #[doc = "25: Output voltage 1.25V"]
    VCC_TRIM_1P25V = 25,
    #[doc = "31: Output voltage 1.31V"]
    VCC_TRIM_1P31V = 31,
}
impl From<VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VTRIM`"]
pub type VTRIM_R = crate::R<u8, VTRIM_A>;
impl VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VTRIM_A::VCC_TRIM_1P00V),
            5 => Val(VTRIM_A::VCC_TRIM_1P05V),
            10 => Val(VTRIM_A::VCC_TRIM_1P10V),
            15 => Val(VTRIM_A::VCC_TRIM_1P15V),
            20 => Val(VTRIM_A::VCC_TRIM_1P20V),
            25 => Val(VTRIM_A::VCC_TRIM_1P25V),
            31 => Val(VTRIM_A::VCC_TRIM_1P31V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P00V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p00v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P00V
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P05V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p05v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P05V
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P10V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p10v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P10V
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P15V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p15v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P15V
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P20V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p20v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P20V
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P25V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p25v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P25V
    }
    #[doc = "Checks if the value of the field is `VCC_TRIM_1P31V`"]
    #[inline(always)]
    pub fn is_vcc_trim_1p31v(&self) -> bool {
        *self == VTRIM_A::VCC_TRIM_1P31V
    }
}
#[doc = "Write proxy for field `VTRIM`"]
pub struct VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output voltage 1.00V"]
    #[inline(always)]
    pub fn vcc_trim_1p00v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P00V)
    }
    #[doc = "Output voltage 1.05V"]
    #[inline(always)]
    pub fn vcc_trim_1p05v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P05V)
    }
    #[doc = "Output voltage 1.10V"]
    #[inline(always)]
    pub fn vcc_trim_1p10v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P10V)
    }
    #[doc = "Output voltage 1.15V"]
    #[inline(always)]
    pub fn vcc_trim_1p15v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P15V)
    }
    #[doc = "Output voltage 1.20V"]
    #[inline(always)]
    pub fn vcc_trim_1p20v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P20V)
    }
    #[doc = "Output voltage 1.25V"]
    #[inline(always)]
    pub fn vcc_trim_1p25v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P25V)
    }
    #[doc = "Output voltage 1.31V"]
    #[inline(always)]
    pub fn vcc_trim_1p31v(self) -> &'a mut W {
        self.variant(VTRIM_A::VCC_TRIM_1P31V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Inductor charge current trimming"]
    #[inline(always)]
    pub fn ich_trim(&self) -> ICH_TRIM_R {
        ICH_TRIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Enable CCM mode"]
    #[inline(always)]
    pub fn ccm_enable(&self) -> CCM_ENABLE_R {
        CCM_ENABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pulse mode control"]
    #[inline(always)]
    pub fn pulse_ctrl(&self) -> PULSE_CTRL_R {
        PULSE_CTRL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Charge mode control"]
    #[inline(always)]
    pub fn charge_ctrl(&self) -> CHARGE_CTRL_R {
        CHARGE_CTRL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable buck converter mode"]
    #[inline(always)]
    pub fn buck_enable(&self) -> BUCK_ENABLE_R {
        BUCK_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Inductor charge current trimming"]
    #[inline(always)]
    pub fn ich_trim(&mut self) -> ICH_TRIM_W {
        ICH_TRIM_W { w: self }
    }
    #[doc = "Bit 11 - Enable CCM mode"]
    #[inline(always)]
    pub fn ccm_enable(&mut self) -> CCM_ENABLE_W {
        CCM_ENABLE_W { w: self }
    }
    #[doc = "Bit 10 - Pulse mode control"]
    #[inline(always)]
    pub fn pulse_ctrl(&mut self) -> PULSE_CTRL_W {
        PULSE_CTRL_W { w: self }
    }
    #[doc = "Bit 9 - Charge mode control"]
    #[inline(always)]
    pub fn charge_ctrl(&mut self) -> CHARGE_CTRL_W {
        CHARGE_CTRL_W { w: self }
    }
    #[doc = "Bit 8 - Enable buck converter mode"]
    #[inline(always)]
    pub fn buck_enable(&mut self) -> BUCK_ENABLE_W {
        BUCK_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:4 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
