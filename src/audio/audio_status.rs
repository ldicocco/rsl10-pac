#[doc = "Reader of register AUDIO_STATUS"]
pub type R = crate::R<u32, super::AUDIO_STATUS>;
#[doc = "Writer for register AUDIO_STATUS"]
pub type W = crate::W<u32, super::AUDIO_STATUS>;
#[doc = "Register AUDIO_STATUS `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::AUDIO_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Output driver feature status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_STATUS_A {
    #[doc = "0: Device does not have the output driver feature"]
    OD_DISABLED_DEVICE = 0,
    #[doc = "1: Device has the output driver feature"]
    OD_ENABLED_DEVICE = 1,
}
impl From<OD_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: OD_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_STATUS`"]
pub type OD_STATUS_R = crate::R<bool, OD_STATUS_A>;
impl OD_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_STATUS_A {
        match self.bits {
            false => OD_STATUS_A::OD_DISABLED_DEVICE,
            true => OD_STATUS_A::OD_ENABLED_DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `OD_DISABLED_DEVICE`"]
    #[inline(always)]
    pub fn is_od_disabled_device(&self) -> bool {
        *self == OD_STATUS_A::OD_DISABLED_DEVICE
    }
    #[doc = "Checks if the value of the field is `OD_ENABLED_DEVICE`"]
    #[inline(always)]
    pub fn is_od_enabled_device(&self) -> bool {
        *self == OD_STATUS_A::OD_ENABLED_DEVICE
    }
}
#[doc = "Reset the output driver underrun detection sticky bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_UNDERRUN_FLAG_CLEAR_AW {
    #[doc = "1: Reset the OD underrun detection sticky bit"]
    OD_UNDERRUN_FLAG_CLEAR = 1,
}
impl From<OD_UNDERRUN_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: OD_UNDERRUN_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OD_UNDERRUN_FLAG_CLEAR`"]
pub struct OD_UNDERRUN_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_UNDERRUN_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_UNDERRUN_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the OD underrun detection sticky bit"]
    #[inline(always)]
    pub fn od_underrun_flag_clear(self) -> &'a mut W {
        self.variant(OD_UNDERRUN_FLAG_CLEAR_AW::OD_UNDERRUN_FLAG_CLEAR)
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
#[doc = "Sticky bit indicating the detection of an output driver underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_UNDERRUN_FLAG_A {
    #[doc = "0: Indicates that no OD underrun has been detected"]
    OD_UNDERRUN_NOT_DETECTED = 0,
    #[doc = "1: Indicates that an OD underrun has been detected"]
    OD_UNDERRUN_DETECTED = 1,
}
impl From<OD_UNDERRUN_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: OD_UNDERRUN_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_UNDERRUN_FLAG`"]
pub type OD_UNDERRUN_FLAG_R = crate::R<bool, OD_UNDERRUN_FLAG_A>;
impl OD_UNDERRUN_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_UNDERRUN_FLAG_A {
        match self.bits {
            false => OD_UNDERRUN_FLAG_A::OD_UNDERRUN_NOT_DETECTED,
            true => OD_UNDERRUN_FLAG_A::OD_UNDERRUN_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `OD_UNDERRUN_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_od_underrun_not_detected(&self) -> bool {
        *self == OD_UNDERRUN_FLAG_A::OD_UNDERRUN_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `OD_UNDERRUN_DETECTED`"]
    #[inline(always)]
    pub fn is_od_underrun_detected(&self) -> bool {
        *self == OD_UNDERRUN_FLAG_A::OD_UNDERRUN_DETECTED
    }
}
#[doc = "Flag indicating that a new output driver sample is required\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_DATA_REQ_FLAG_A {
    #[doc = "0: Indicates that no new OD sample is required"]
    OD_DATA_NOT_REQUIRED = 0,
    #[doc = "1: Indicates that a new OD sample is required"]
    OD_DATA_REQUIRED = 1,
}
impl From<OD_DATA_REQ_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: OD_DATA_REQ_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_DATA_REQ_FLAG`"]
pub type OD_DATA_REQ_FLAG_R = crate::R<bool, OD_DATA_REQ_FLAG_A>;
impl OD_DATA_REQ_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_DATA_REQ_FLAG_A {
        match self.bits {
            false => OD_DATA_REQ_FLAG_A::OD_DATA_NOT_REQUIRED,
            true => OD_DATA_REQ_FLAG_A::OD_DATA_REQUIRED,
        }
    }
    #[doc = "Checks if the value of the field is `OD_DATA_NOT_REQUIRED`"]
    #[inline(always)]
    pub fn is_od_data_not_required(&self) -> bool {
        *self == OD_DATA_REQ_FLAG_A::OD_DATA_NOT_REQUIRED
    }
    #[doc = "Checks if the value of the field is `OD_DATA_REQUIRED`"]
    #[inline(always)]
    pub fn is_od_data_required(&self) -> bool {
        *self == OD_DATA_REQ_FLAG_A::OD_DATA_REQUIRED
    }
}
#[doc = "Reset the DMIC1 overrun detection sticky bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_OVERRUN_FLAG_CLEAR_AW {
    #[doc = "1: Reset the DMIC1 overrun detection sticky bit"]
    DMIC1_OVERRUN_FLAG_CLEAR = 1,
}
impl From<DMIC1_OVERRUN_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_OVERRUN_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DMIC1_OVERRUN_FLAG_CLEAR`"]
pub struct DMIC1_OVERRUN_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_OVERRUN_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_OVERRUN_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the DMIC1 overrun detection sticky bit"]
    #[inline(always)]
    pub fn dmic1_overrun_flag_clear(self) -> &'a mut W {
        self.variant(DMIC1_OVERRUN_FLAG_CLEAR_AW::DMIC1_OVERRUN_FLAG_CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Sticky bit indicating the detection of a DMIC1 overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_OVERRUN_FLAG_A {
    #[doc = "0: Indicates that no DMIC1 overrun has been detected"]
    DMIC1_OVERRUN_NOT_DETECTED = 0,
    #[doc = "1: Indicates that a DMIC1 overrun has been detected"]
    DMIC1_OVERRUN_DETECTED = 1,
}
impl From<DMIC1_OVERRUN_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_OVERRUN_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_OVERRUN_FLAG`"]
pub type DMIC1_OVERRUN_FLAG_R = crate::R<bool, DMIC1_OVERRUN_FLAG_A>;
impl DMIC1_OVERRUN_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_OVERRUN_FLAG_A {
        match self.bits {
            false => DMIC1_OVERRUN_FLAG_A::DMIC1_OVERRUN_NOT_DETECTED,
            true => DMIC1_OVERRUN_FLAG_A::DMIC1_OVERRUN_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_OVERRUN_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_dmic1_overrun_not_detected(&self) -> bool {
        *self == DMIC1_OVERRUN_FLAG_A::DMIC1_OVERRUN_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DMIC1_OVERRUN_DETECTED`"]
    #[inline(always)]
    pub fn is_dmic1_overrun_detected(&self) -> bool {
        *self == DMIC1_OVERRUN_FLAG_A::DMIC1_OVERRUN_DETECTED
    }
}
#[doc = "Flag indicating the availability of a new DMIC1 sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_DATA_RDY_FLAG_A {
    #[doc = "0: Indicates that no new DMIC1 sample is available"]
    DMIC1_DATA_NOT_READY = 0,
    #[doc = "1: Indicates that a new DMIC1 sample is available"]
    DMIC1_DATA_READY = 1,
}
impl From<DMIC1_DATA_RDY_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_DATA_RDY_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_DATA_RDY_FLAG`"]
pub type DMIC1_DATA_RDY_FLAG_R = crate::R<bool, DMIC1_DATA_RDY_FLAG_A>;
impl DMIC1_DATA_RDY_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_DATA_RDY_FLAG_A {
        match self.bits {
            false => DMIC1_DATA_RDY_FLAG_A::DMIC1_DATA_NOT_READY,
            true => DMIC1_DATA_RDY_FLAG_A::DMIC1_DATA_READY,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_DATA_NOT_READY`"]
    #[inline(always)]
    pub fn is_dmic1_data_not_ready(&self) -> bool {
        *self == DMIC1_DATA_RDY_FLAG_A::DMIC1_DATA_NOT_READY
    }
    #[doc = "Checks if the value of the field is `DMIC1_DATA_READY`"]
    #[inline(always)]
    pub fn is_dmic1_data_ready(&self) -> bool {
        *self == DMIC1_DATA_RDY_FLAG_A::DMIC1_DATA_READY
    }
}
#[doc = "Reset the DMIC0 overrun detection sticky bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_OVERRUN_FLAG_CLEAR_AW {
    #[doc = "1: Reset the DMIC0 overrun detection sticky bit"]
    DMIC0_OVERRUN_FLAG_CLEAR = 1,
}
impl From<DMIC0_OVERRUN_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_OVERRUN_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DMIC0_OVERRUN_FLAG_CLEAR`"]
pub struct DMIC0_OVERRUN_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_OVERRUN_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_OVERRUN_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the DMIC0 overrun detection sticky bit"]
    #[inline(always)]
    pub fn dmic0_overrun_flag_clear(self) -> &'a mut W {
        self.variant(DMIC0_OVERRUN_FLAG_CLEAR_AW::DMIC0_OVERRUN_FLAG_CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Sticky bit indicating the detection of a DMIC0 overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_OVERRUN_FLAG_A {
    #[doc = "0: Indicates that no DMIC0 overrun has been detected"]
    DMIC0_OVERRUN_NOT_DETECTED = 0,
    #[doc = "1: Indicates that a DMIC0 overrun has been detected"]
    DMIC0_OVERRUN_DETECTED = 1,
}
impl From<DMIC0_OVERRUN_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_OVERRUN_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_OVERRUN_FLAG`"]
pub type DMIC0_OVERRUN_FLAG_R = crate::R<bool, DMIC0_OVERRUN_FLAG_A>;
impl DMIC0_OVERRUN_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_OVERRUN_FLAG_A {
        match self.bits {
            false => DMIC0_OVERRUN_FLAG_A::DMIC0_OVERRUN_NOT_DETECTED,
            true => DMIC0_OVERRUN_FLAG_A::DMIC0_OVERRUN_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_OVERRUN_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_dmic0_overrun_not_detected(&self) -> bool {
        *self == DMIC0_OVERRUN_FLAG_A::DMIC0_OVERRUN_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DMIC0_OVERRUN_DETECTED`"]
    #[inline(always)]
    pub fn is_dmic0_overrun_detected(&self) -> bool {
        *self == DMIC0_OVERRUN_FLAG_A::DMIC0_OVERRUN_DETECTED
    }
}
#[doc = "Flag indicating the availability of a new DMIC0 sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_DATA_RDY_FLAG_A {
    #[doc = "0: Indicates that no new DMIC0 sample is available"]
    DMIC0_DATA_NOT_READY = 0,
    #[doc = "1: Indicates that a new DMIC0 sample is available"]
    DMIC0_DATA_READY = 1,
}
impl From<DMIC0_DATA_RDY_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_DATA_RDY_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_DATA_RDY_FLAG`"]
pub type DMIC0_DATA_RDY_FLAG_R = crate::R<bool, DMIC0_DATA_RDY_FLAG_A>;
impl DMIC0_DATA_RDY_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_DATA_RDY_FLAG_A {
        match self.bits {
            false => DMIC0_DATA_RDY_FLAG_A::DMIC0_DATA_NOT_READY,
            true => DMIC0_DATA_RDY_FLAG_A::DMIC0_DATA_READY,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_DATA_NOT_READY`"]
    #[inline(always)]
    pub fn is_dmic0_data_not_ready(&self) -> bool {
        *self == DMIC0_DATA_RDY_FLAG_A::DMIC0_DATA_NOT_READY
    }
    #[doc = "Checks if the value of the field is `DMIC0_DATA_READY`"]
    #[inline(always)]
    pub fn is_dmic0_data_ready(&self) -> bool {
        *self == DMIC0_DATA_RDY_FLAG_A::DMIC0_DATA_READY
    }
}
impl R {
    #[doc = "Bit 11 - Output driver feature status"]
    #[inline(always)]
    pub fn od_status(&self) -> OD_STATUS_R {
        OD_STATUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sticky bit indicating the detection of an output driver underrun"]
    #[inline(always)]
    pub fn od_underrun_flag(&self) -> OD_UNDERRUN_FLAG_R {
        OD_UNDERRUN_FLAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flag indicating that a new output driver sample is required"]
    #[inline(always)]
    pub fn od_data_req_flag(&self) -> OD_DATA_REQ_FLAG_R {
        OD_DATA_REQ_FLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky bit indicating the detection of a DMIC1 overrun"]
    #[inline(always)]
    pub fn dmic1_overrun_flag(&self) -> DMIC1_OVERRUN_FLAG_R {
        DMIC1_OVERRUN_FLAG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flag indicating the availability of a new DMIC1 sample"]
    #[inline(always)]
    pub fn dmic1_data_rdy_flag(&self) -> DMIC1_DATA_RDY_FLAG_R {
        DMIC1_DATA_RDY_FLAG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sticky bit indicating the detection of a DMIC0 overrun"]
    #[inline(always)]
    pub fn dmic0_overrun_flag(&self) -> DMIC0_OVERRUN_FLAG_R {
        DMIC0_OVERRUN_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Flag indicating the availability of a new DMIC0 sample"]
    #[inline(always)]
    pub fn dmic0_data_rdy_flag(&self) -> DMIC0_DATA_RDY_FLAG_R {
        DMIC0_DATA_RDY_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Reset the output driver underrun detection sticky bit"]
    #[inline(always)]
    pub fn od_underrun_flag_clear(&mut self) -> OD_UNDERRUN_FLAG_CLEAR_W {
        OD_UNDERRUN_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 6 - Reset the DMIC1 overrun detection sticky bit"]
    #[inline(always)]
    pub fn dmic1_overrun_flag_clear(&mut self) -> DMIC1_OVERRUN_FLAG_CLEAR_W {
        DMIC1_OVERRUN_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Reset the DMIC0 overrun detection sticky bit"]
    #[inline(always)]
    pub fn dmic0_overrun_flag_clear(&mut self) -> DMIC0_OVERRUN_FLAG_CLEAR_W {
        DMIC0_OVERRUN_FLAG_CLEAR_W { w: self }
    }
}
