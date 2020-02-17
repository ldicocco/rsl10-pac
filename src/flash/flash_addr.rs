#[doc = "Reader of register FLASH_ADDR"]
pub type R = crate::R<u32, super::FLASH_ADDR>;
#[doc = "Writer for register FLASH_ADDR"]
pub type W = crate::W<u32, super::FLASH_ADDR>;
#[doc = "Register FLASH_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_ADDR`"]
pub type FLASH_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLASH_ADDR`"]
pub struct FLASH_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 2)) | (((value as u32) & 0x0007_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:20 - Flash Byte Address"]
    #[inline(always)]
    pub fn flash_addr(&self) -> FLASH_ADDR_R {
        FLASH_ADDR_R::new(((self.bits >> 2) & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:20 - Flash Byte Address"]
    #[inline(always)]
    pub fn flash_addr(&mut self) -> FLASH_ADDR_W {
        FLASH_ADDR_W { w: self }
    }
}
