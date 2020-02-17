#[doc = "Reader of register FLASH_COPY_WORD_CNT"]
pub type R = crate::R<u32, super::FLASH_COPY_WORD_CNT>;
#[doc = "Writer for register FLASH_COPY_WORD_CNT"]
pub type W = crate::W<u32, super::FLASH_COPY_WORD_CNT>;
#[doc = "Register FLASH_COPY_WORD_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_COPY_WORD_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COPY_WORD_CNT`"]
pub type COPY_WORD_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COPY_WORD_CNT`"]
pub struct COPY_WORD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_WORD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Number of words to copy / compare"]
    #[inline(always)]
    pub fn copy_word_cnt(&self) -> COPY_WORD_CNT_R {
        COPY_WORD_CNT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Number of words to copy / compare"]
    #[inline(always)]
    pub fn copy_word_cnt(&mut self) -> COPY_WORD_CNT_W {
        COPY_WORD_CNT_W { w: self }
    }
}
