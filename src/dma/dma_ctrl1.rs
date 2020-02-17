#[doc = "Reader of register DMA_CTRL1[%s]"]
pub type R = crate::R<u32, super::DMA_CTRL1>;
#[doc = "Writer for register DMA_CTRL1[%s]"]
pub type W = crate::W<u32, super::DMA_CTRL1>;
#[doc = "Register DMA_CTRL1[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER_INT_VALUE`"]
pub type COUNTER_INT_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNTER_INT_VALUE`"]
pub struct COUNTER_INT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_INT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRANSFER_LENGTH`"]
pub type TRANSFER_LENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRANSFER_LENGTH`"]
pub struct TRANSFER_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Trigger a counter interrupt when the DMA transfer word count reaches this value"]
    #[inline(always)]
    pub fn counter_int_value(&self) -> COUNTER_INT_VALUE_R {
        COUNTER_INT_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - The length, in words, of each data transfer using DMA channel"]
    #[inline(always)]
    pub fn transfer_length(&self) -> TRANSFER_LENGTH_R {
        TRANSFER_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Trigger a counter interrupt when the DMA transfer word count reaches this value"]
    #[inline(always)]
    pub fn counter_int_value(&mut self) -> COUNTER_INT_VALUE_W {
        COUNTER_INT_VALUE_W { w: self }
    }
    #[doc = "Bits 0:15 - The length, in words, of each data transfer using DMA channel"]
    #[inline(always)]
    pub fn transfer_length(&mut self) -> TRANSFER_LENGTH_W {
        TRANSFER_LENGTH_W { w: self }
    }
}
