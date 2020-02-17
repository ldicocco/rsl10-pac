#[doc = "Reader of register PCM_TX_DATA"]
pub type R = crate::R<u32, super::PCM_TX_DATA>;
#[doc = "Writer for register PCM_TX_DATA"]
pub type W = crate::W<u32, super::PCM_TX_DATA>;
#[doc = "Register PCM_TX_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::PCM_TX_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCM_TX_DATA`"]
pub type PCM_TX_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCM_TX_DATA`"]
pub struct PCM_TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM_TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data to transmit over the PCM interface"]
    #[inline(always)]
    pub fn pcm_tx_data(&self) -> PCM_TX_DATA_R {
        PCM_TX_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to transmit over the PCM interface"]
    #[inline(always)]
    pub fn pcm_tx_data(&mut self) -> PCM_TX_DATA_W {
        PCM_TX_DATA_W { w: self }
    }
}
