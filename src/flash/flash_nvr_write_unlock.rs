#[doc = "Reader of register FLASH_NVR_WRITE_UNLOCK"]
pub type R = crate::R<u32, super::FLASH_NVR_WRITE_UNLOCK>;
#[doc = "Writer for register FLASH_NVR_WRITE_UNLOCK"]
pub type W = crate::W<u32, super::FLASH_NVR_WRITE_UNLOCK>;
#[doc = "Register FLASH_NVR_WRITE_UNLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_NVR_WRITE_UNLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "32-bit key to allow for write accesses NVR sectors of the Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum UNLOCK_KEY_AW {
    #[doc = "1907585525: 32-bit key to allow for Write access to the Flash NVR sector"]
    FLASH_NVR_KEY = 1907585525,
}
impl From<UNLOCK_KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: UNLOCK_KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `UNLOCK_KEY`"]
pub struct UNLOCK_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNLOCK_KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32-bit key to allow for Write access to the Flash NVR sector"]
    #[inline(always)]
    pub fn flash_nvr_key(self) -> &'a mut W {
        self.variant(UNLOCK_KEY_AW::FLASH_NVR_KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:31 - 32-bit key to allow for write accesses NVR sectors of the Flash"]
    #[inline(always)]
    pub fn unlock_key(&mut self) -> UNLOCK_KEY_W {
        UNLOCK_KEY_W { w: self }
    }
}
