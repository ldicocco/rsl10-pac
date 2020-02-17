#[doc = "Reader of register SYSCTRL_WAKEUP_ADDR"]
pub type R = crate::R<u32, super::SYSCTRL_WAKEUP_ADDR>;
#[doc = "Writer for register SYSCTRL_WAKEUP_ADDR"]
pub type W = crate::W<u32, super::SYSCTRL_WAKEUP_ADDR>;
#[doc = "Register SYSCTRL_WAKEUP_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_WAKEUP_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKEUP_ADDR`"]
pub type WAKEUP_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WAKEUP_ADDR`"]
pub struct WAKEUP_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Wakeup restore address in unpacked 32-bit format. When written, the WAKEUP_ADDR_PACKED field of SYSCTRL_MEM_ACCESS_CFG is updated. Bits 0-12 must be 0x0000 or 0x1FE8 (top or bottom of memory instance). Bits 17-20, 22-28 and 30-31 must be zero. When the WAKEUP_ADDR_PACKED field does not point to memory that is currently accessible, then SYSCTRL_WAKEUP_ADDR reads back as all zeros."]
    #[inline(always)]
    pub fn wakeup_addr(&self) -> WAKEUP_ADDR_R {
        WAKEUP_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Wakeup restore address in unpacked 32-bit format. When written, the WAKEUP_ADDR_PACKED field of SYSCTRL_MEM_ACCESS_CFG is updated. Bits 0-12 must be 0x0000 or 0x1FE8 (top or bottom of memory instance). Bits 17-20, 22-28 and 30-31 must be zero. When the WAKEUP_ADDR_PACKED field does not point to memory that is currently accessible, then SYSCTRL_WAKEUP_ADDR reads back as all zeros."]
    #[inline(always)]
    pub fn wakeup_addr(&mut self) -> WAKEUP_ADDR_W {
        WAKEUP_ADDR_W { w: self }
    }
}
