#[doc = "Reader of register FLASH_PATCH_ADDR[%s]"]
pub type R = crate::R<u32, super::FLASH_PATCH_ADDR>;
#[doc = "Writer for register FLASH_PATCH_ADDR[%s]"]
pub type W = crate::W<u32, super::FLASH_PATCH_ADDR>;
#[doc = "Register FLASH_PATCH_ADDR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_PATCH_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PATCH_ADDR`"]
pub type PATCH_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PATCH_ADDR`"]
pub struct PATCH_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 11)) | (((value as u32) & 0x03ff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn patch_addr(&self) -> PATCH_ADDR_R {
        PATCH_ADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn patch_addr(&mut self) -> PATCH_ADDR_W {
        PATCH_ADDR_W { w: self }
    }
}
