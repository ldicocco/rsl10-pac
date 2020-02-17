#[doc = "Reader of register ADC_BATMON_COUNT_VAL"]
pub type R = crate::R<u32, super::ADC_BATMON_COUNT_VAL>;
#[doc = "Reader of field `SUPPLY_COUNT_VALUE`"]
pub type SUPPLY_COUNT_VALUE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of times the battery voltage has fallen below the battery monitor voltage threshold. The counter is reset when read."]
    #[inline(always)]
    pub fn supply_count_value(&self) -> SUPPLY_COUNT_VALUE_R {
        SUPPLY_COUNT_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
