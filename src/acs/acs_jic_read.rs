#[doc = "Reader of register ACS_JIC_READ"]
pub type R = crate::R<u32, super::ACS_JIC_READ>;
#[doc = "Reader of field `BYTE0_RO`"]
pub type BYTE0_RO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - JIC read only register bits (returning signals from analog part: tied to 1)"]
    #[inline(always)]
    pub fn byte0_ro(&self) -> BYTE0_RO_R {
        BYTE0_RO_R::new((self.bits & 0xff) as u8)
    }
}
