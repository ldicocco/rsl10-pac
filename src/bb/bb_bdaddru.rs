#[doc = "Reader of register BB_BDADDRU"]
pub type R = crate::R<u32, super::BB_BDADDRU>;
#[doc = "Writer for register BB_BDADDRU"]
pub type W = crate::W<u32, super::BB_BDADDRU>;
#[doc = "Register BB_BDADDRU `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_BDADDRU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BLE device address privacy indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIV_NPUB_A {
    #[doc = "0: Public Bluetooth device address"]
    PRIV_NPUB_0 = 0,
    #[doc = "1: Private Bluetooth device address"]
    PRIV_NPUB_1 = 1,
}
impl From<PRIV_NPUB_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV_NPUB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRIV_NPUB`"]
pub type PRIV_NPUB_R = crate::R<bool, PRIV_NPUB_A>;
impl PRIV_NPUB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIV_NPUB_A {
        match self.bits {
            false => PRIV_NPUB_A::PRIV_NPUB_0,
            true => PRIV_NPUB_A::PRIV_NPUB_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRIV_NPUB_0`"]
    #[inline(always)]
    pub fn is_priv_npub_0(&self) -> bool {
        *self == PRIV_NPUB_A::PRIV_NPUB_0
    }
    #[doc = "Checks if the value of the field is `PRIV_NPUB_1`"]
    #[inline(always)]
    pub fn is_priv_npub_1(&self) -> bool {
        *self == PRIV_NPUB_A::PRIV_NPUB_1
    }
}
#[doc = "Write proxy for field `PRIV_NPUB`"]
pub struct PRIV_NPUB_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_NPUB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIV_NPUB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Public Bluetooth device address"]
    #[inline(always)]
    pub fn priv_npub_0(self) -> &'a mut W {
        self.variant(PRIV_NPUB_A::PRIV_NPUB_0)
    }
    #[doc = "Private Bluetooth device address"]
    #[inline(always)]
    pub fn priv_npub_1(self) -> &'a mut W {
        self.variant(PRIV_NPUB_A::PRIV_NPUB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "BLE device address (MSB part)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BDADDRU_A {
    #[doc = "0: `0`"]
    BDADDRU_0 = 0,
}
impl From<BDADDRU_A> for u16 {
    #[inline(always)]
    fn from(variant: BDADDRU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BDADDRU`"]
pub type BDADDRU_R = crate::R<u16, BDADDRU_A>;
impl BDADDRU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BDADDRU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BDADDRU_A::BDADDRU_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BDADDRU_0`"]
    #[inline(always)]
    pub fn is_bdaddru_0(&self) -> bool {
        *self == BDADDRU_A::BDADDRU_0
    }
}
#[doc = "Write proxy for field `BDADDRU`"]
pub struct BDADDRU_W<'a> {
    w: &'a mut W,
}
impl<'a> BDADDRU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDADDRU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bdaddru_0(self) -> &'a mut W {
        self.variant(BDADDRU_A::BDADDRU_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - BLE device address privacy indicator"]
    #[inline(always)]
    pub fn priv_npub(&self) -> PRIV_NPUB_R {
        PRIV_NPUB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - BLE device address (MSB part)"]
    #[inline(always)]
    pub fn bdaddru(&self) -> BDADDRU_R {
        BDADDRU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - BLE device address privacy indicator"]
    #[inline(always)]
    pub fn priv_npub(&mut self) -> PRIV_NPUB_W {
        PRIV_NPUB_W { w: self }
    }
    #[doc = "Bits 0:15 - BLE device address (MSB part)"]
    #[inline(always)]
    pub fn bdaddru(&mut self) -> BDADDRU_W {
        BDADDRU_W { w: self }
    }
}
