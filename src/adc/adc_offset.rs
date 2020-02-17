#[doc = "Reader of register ADC_OFFSET"]
pub type R = crate::R<u32, super::ADC_OFFSET>;
#[doc = "Writer for register ADC_OFFSET"]
pub type W = crate::W<u32, super::ADC_OFFSET>;
#[doc = "Register ADC_OFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - 15-bit ADC signed offset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - 15-bit ADC signed offset"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
