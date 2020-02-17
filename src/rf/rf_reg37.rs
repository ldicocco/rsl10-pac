#[doc = "Reader of register RF_REG37"]
pub type R = crate::R<u32, super::RF_REG37>;
#[doc = "Reader of field `FEI_PKT_FEI_PKT`"]
pub type FEI_PKT_FEI_PKT_R = crate::R<u16, u16>;
#[doc = "Reader of field `FEI_FEI_OUT`"]
pub type FEI_FEI_OUT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Frequency error indicator sampled during the packet reception."]
    #[inline(always)]
    pub fn fei_pkt_fei_pkt(&self) -> FEI_PKT_FEI_PKT_R {
        FEI_PKT_FEI_PKT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Frequency error indicator"]
    #[inline(always)]
    pub fn fei_fei_out(&self) -> FEI_FEI_OUT_R {
        FEI_FEI_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
