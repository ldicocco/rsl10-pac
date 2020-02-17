#[doc = "Reader of register AUDIOSINK_CNT"]
pub type R = crate::R<u32, super::AUDIOSINK_CNT>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Sink clock counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
