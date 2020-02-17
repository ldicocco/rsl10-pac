#[doc = "Reader of register DIO_PAD_CFG"]
pub type R = crate::R<u32, super::DIO_PAD_CFG>;
#[doc = "Writer for register DIO_PAD_CFG"]
pub type W = crate::W<u32, super::DIO_PAD_CFG>;
#[doc = "Register DIO_PAD_CFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DIO_PAD_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Drive strength configuration (scales the individual drive strengths)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIVE_A {
    #[doc = "0: All pad regular drive strengths (allowed for all VDDO voltages)"]
    PAD_LOW_DRIVE = 0,
    #[doc = "1: All pad drive strengths increased by ~50 percent (only allowed for VDDO up to 2.7 V)"]
    PAD_HIGH_DRIVE = 1,
}
impl From<DRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRIVE`"]
pub type DRIVE_R = crate::R<bool, DRIVE_A>;
impl DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            false => DRIVE_A::PAD_LOW_DRIVE,
            true => DRIVE_A::PAD_HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_pad_low_drive(&self) -> bool {
        *self == DRIVE_A::PAD_LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `PAD_HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_pad_high_drive(&self) -> bool {
        *self == DRIVE_A::PAD_HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `DRIVE`"]
pub struct DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All pad regular drive strengths (allowed for all VDDO voltages)"]
    #[inline(always)]
    pub fn pad_low_drive(self) -> &'a mut W {
        self.variant(DRIVE_A::PAD_LOW_DRIVE)
    }
    #[doc = "All pad drive strengths increased by ~50 percent (only allowed for VDDO up to 2.7 V)"]
    #[inline(always)]
    pub fn pad_high_drive(self) -> &'a mut W {
        self.variant(DRIVE_A::PAD_HIGH_DRIVE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Drive strength configuration (scales the individual drive strengths)"]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive strength configuration (scales the individual drive strengths)"]
    #[inline(always)]
    pub fn drive(&mut self) -> DRIVE_W {
        DRIVE_W { w: self }
    }
}
