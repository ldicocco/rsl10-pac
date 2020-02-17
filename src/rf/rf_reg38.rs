#[doc = "Reader of register RF_REG38"]
pub type R = crate::R<u32, super::RF_REG38>;
#[doc = "Reader of field `LINK_QUAL_PKT_LINK_QUALITY_PKT`"]
pub type LINK_QUAL_PKT_LINK_QUALITY_PKT_R = crate::R<u8, u8>;
#[doc = "Reader of field `LINK_QUAL_LINK_QUALITY`"]
pub type LINK_QUAL_LINK_QUALITY_R = crate::R<u8, u8>;
#[doc = "Reader of field `FEI_AFC_FEI_AFC`"]
pub type FEI_AFC_FEI_AFC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 24:31 - Link quality indicator sampled during the packet reception. Note that the Viterbi algorithm as to be enabled."]
    #[inline(always)]
    pub fn link_qual_pkt_link_quality_pkt(&self) -> LINK_QUAL_PKT_LINK_QUALITY_PKT_R {
        LINK_QUAL_PKT_LINK_QUALITY_PKT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Instantaneous link quality indicator. Note that the Viterbi algorithm as to be enabled."]
    #[inline(always)]
    pub fn link_qual_link_quality(&self) -> LINK_QUAL_LINK_QUALITY_R {
        LINK_QUAL_LINK_QUALITY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - Frequency error indicator sampled during the AFC."]
    #[inline(always)]
    pub fn fei_afc_fei_afc(&self) -> FEI_AFC_FEI_AFC_R {
        FEI_AFC_FEI_AFC_R::new((self.bits & 0xffff) as u16)
    }
}
