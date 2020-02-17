#[doc = "Reader of register DEBUG_DHCSR"]
pub type R = crate::R<u32, super::DEBUG_DHCSR>;
#[doc = "Writer for register DEBUG_DHCSR"]
pub type W = crate::W<u32, super::DEBUG_DHCSR>;
#[doc = "Register DEBUG_DHCSR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DEBUG_DHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Debug key must be written to this field in order to write the rest of the register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DBGKEY_AW {
    #[doc = "41055: DEBUG_HALT_KEY"]
    DEBUG_HALT_KEY = 41055,
}
impl From<DBGKEY_AW> for u16 {
    #[inline(always)]
    fn from(variant: DBGKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `DBGKEY`"]
pub struct DBGKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGKEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DEBUG_HALT_KEY"]
    #[inline(always)]
    pub fn debug_halt_key(self) -> &'a mut W {
        self.variant(DBGKEY_AW::DEBUG_HALT_KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `S_RESET_ST`"]
pub type S_RESET_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `S_RETIRE_ST`"]
pub type S_RETIRE_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `S_LOCKUP`"]
pub type S_LOCKUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `S_SLEEP`"]
pub type S_SLEEP_R = crate::R<bool, bool>;
#[doc = "Reader of field `S_HALT`"]
pub type S_HALT_R = crate::R<bool, bool>;
#[doc = "Reader of field `S_REGRDY`"]
pub type S_REGRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `C_SNAPSTALL`"]
pub type C_SNAPSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_SNAPSTALL`"]
pub struct C_SNAPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> C_SNAPSTALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `C_MASKINTS`"]
pub type C_MASKINTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_MASKINTS`"]
pub struct C_MASKINTS_W<'a> {
    w: &'a mut W,
}
impl<'a> C_MASKINTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `C_STEP`"]
pub type C_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_STEP`"]
pub struct C_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> C_STEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `C_HALT`"]
pub type C_HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_HALT`"]
pub struct C_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> C_HALT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `C_DEBUGEN`"]
pub type C_DEBUGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_DEBUGEN`"]
pub struct C_DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DEBUGEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - Core has been reset or is being rest. Bit is cleared on read."]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Indicates if core is in lockup state"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Indicates if core is in sleep mode"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates is core is halted"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Indicates register read/write operation is completed"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set to break a stalled memory access"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask interrupts while stepping"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single step the processor"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Halt the processor"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable halt mode debugging"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Debug key must be written to this field in order to write the rest of the register"]
    #[inline(always)]
    pub fn dbgkey(&mut self) -> DBGKEY_W {
        DBGKEY_W { w: self }
    }
    #[doc = "Bit 5 - Set to break a stalled memory access"]
    #[inline(always)]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W {
        C_SNAPSTALL_W { w: self }
    }
    #[doc = "Bit 3 - Mask interrupts while stepping"]
    #[inline(always)]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W {
        C_MASKINTS_W { w: self }
    }
    #[doc = "Bit 2 - Single step the processor"]
    #[inline(always)]
    pub fn c_step(&mut self) -> C_STEP_W {
        C_STEP_W { w: self }
    }
    #[doc = "Bit 1 - Halt the processor"]
    #[inline(always)]
    pub fn c_halt(&mut self) -> C_HALT_W {
        C_HALT_W { w: self }
    }
    #[doc = "Bit 0 - Enable halt mode debugging"]
    #[inline(always)]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W {
        C_DEBUGEN_W { w: self }
    }
}
