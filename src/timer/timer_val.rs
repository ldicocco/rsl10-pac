#[doc = "Reader of register TIMER_VAL[%s]"]
pub type R = crate::R<u32, super::TIMER_VAL>;
#[doc = "Reader of field `TIMER_VALUE`"]
pub type TIMER_VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Current timer 0 value"]
    #[inline(always)]
    pub fn timer_value(&self) -> TIMER_VALUE_R {
        TIMER_VALUE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
