#[doc = "Reader of register SYSCTRL_DBG_LOCK_KEY[%s]"]
pub type R = crate::R<u32, super::SYSCTRL_DBG_LOCK_KEY>;
#[doc = "Writer for register SYSCTRL_DBG_LOCK_KEY[%s]"]
pub type W = crate::W<u32, super::SYSCTRL_DBG_LOCK_KEY>;
#[doc = "Register SYSCTRL_DBG_LOCK_KEY[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_DBG_LOCK_KEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBG_LOCK_KEY`"]
pub type DBG_LOCK_KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DBG_LOCK_KEY`"]
pub struct DBG_LOCK_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LOCK_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Debug port key"]
    #[inline(always)]
    pub fn dbg_lock_key(&self) -> DBG_LOCK_KEY_R {
        DBG_LOCK_KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug port key"]
    #[inline(always)]
    pub fn dbg_lock_key(&mut self) -> DBG_LOCK_KEY_W {
        DBG_LOCK_KEY_W { w: self }
    }
}
