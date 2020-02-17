#[doc = "Reader of register BB_BBPRIOSCHARB"]
pub type R = crate::R<u32, super::BB_BBPRIOSCHARB>;
#[doc = "Writer for register BB_BBPRIOSCHARB"]
pub type W = crate::W<u32, super::BB_BBPRIOSCHARB>;
#[doc = "Register BB_BBPRIOSCHARB `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_BBPRIOSCHARB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Determine BLE priority scheduling arbitration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEPRIOMODE_A {
    #[doc = "0: BLE decision instant not used"]
    BLEPRIOMODE_0 = 0,
    #[doc = "1: BLE decision instant used"]
    BLEPRIOMODE_1 = 1,
}
impl From<BLEPRIOMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BLEPRIOMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLEPRIOMODE`"]
pub type BLEPRIOMODE_R = crate::R<bool, BLEPRIOMODE_A>;
impl BLEPRIOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLEPRIOMODE_A {
        match self.bits {
            false => BLEPRIOMODE_A::BLEPRIOMODE_0,
            true => BLEPRIOMODE_A::BLEPRIOMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLEPRIOMODE_0`"]
    #[inline(always)]
    pub fn is_blepriomode_0(&self) -> bool {
        *self == BLEPRIOMODE_A::BLEPRIOMODE_0
    }
    #[doc = "Checks if the value of the field is `BLEPRIOMODE_1`"]
    #[inline(always)]
    pub fn is_blepriomode_1(&self) -> bool {
        *self == BLEPRIOMODE_A::BLEPRIOMODE_1
    }
}
#[doc = "Write proxy for field `BLEPRIOMODE`"]
pub struct BLEPRIOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEPRIOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEPRIOMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BLE decision instant not used"]
    #[inline(always)]
    pub fn blepriomode_0(self) -> &'a mut W {
        self.variant(BLEPRIOMODE_A::BLEPRIOMODE_0)
    }
    #[doc = "BLE decision instant used"]
    #[inline(always)]
    pub fn blepriomode_1(self) -> &'a mut W {
        self.variant(BLEPRIOMODE_A::BLEPRIOMODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Determine the decision instant margin for priority scheduling arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEMARGIN_A {
    #[doc = "0: `0`"]
    BLEMARGIN_0 = 0,
}
impl From<BLEMARGIN_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEMARGIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEMARGIN`"]
pub type BLEMARGIN_R = crate::R<u8, BLEMARGIN_A>;
impl BLEMARGIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEMARGIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLEMARGIN_A::BLEMARGIN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEMARGIN_0`"]
    #[inline(always)]
    pub fn is_blemargin_0(&self) -> bool {
        *self == BLEMARGIN_A::BLEMARGIN_0
    }
}
#[doc = "Write proxy for field `BLEMARGIN`"]
pub struct BLEMARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEMARGIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEMARGIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn blemargin_0(self) -> &'a mut W {
        self.variant(BLEMARGIN_A::BLEMARGIN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Determine BLE priority scheduling arbitration mode"]
    #[inline(always)]
    pub fn blepriomode(&self) -> BLEPRIOMODE_R {
        BLEPRIOMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Determine the decision instant margin for priority scheduling arbitration"]
    #[inline(always)]
    pub fn blemargin(&self) -> BLEMARGIN_R {
        BLEMARGIN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Determine BLE priority scheduling arbitration mode"]
    #[inline(always)]
    pub fn blepriomode(&mut self) -> BLEPRIOMODE_W {
        BLEPRIOMODE_W { w: self }
    }
    #[doc = "Bits 0:7 - Determine the decision instant margin for priority scheduling arbitration"]
    #[inline(always)]
    pub fn blemargin(&mut self) -> BLEMARGIN_W {
        BLEMARGIN_W { w: self }
    }
}
