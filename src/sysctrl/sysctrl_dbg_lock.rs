#[doc = "Reader of register SYSCTRL_DBG_LOCK"]
pub type R = crate::R<u32, super::SYSCTRL_DBG_LOCK>;
#[doc = "Writer for register SYSCTRL_DBG_LOCK"]
pub type W = crate::W<u32, super::SYSCTRL_DBG_LOCK>;
#[doc = "Register SYSCTRL_DBG_LOCK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SYSCTRL_DBG_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Debug port access lock/unlock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DBG_LOCK_WR_AW {
    #[doc = "3012598965: Unlock debug port access"]
    DBG_ACCESS_UNLOCK = 3012598965,
    #[doc = "1282368331: Lock debug port access"]
    DBG_ACCESS_LOCK = 1282368331,
}
impl From<DBG_LOCK_WR_AW> for u32 {
    #[inline(always)]
    fn from(variant: DBG_LOCK_WR_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `DBG_LOCK_WR`"]
pub struct DBG_LOCK_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LOCK_WR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_LOCK_WR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unlock debug port access"]
    #[inline(always)]
    pub fn dbg_access_unlock(self) -> &'a mut W {
        self.variant(DBG_LOCK_WR_AW::DBG_ACCESS_UNLOCK)
    }
    #[doc = "Lock debug port access"]
    #[inline(always)]
    pub fn dbg_access_lock(self) -> &'a mut W {
        self.variant(DBG_LOCK_WR_AW::DBG_ACCESS_LOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Debug port access state\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_LOCK_RD_A {
    #[doc = "0: Debug port access is unlocked"]
    DBG_ACCESS_UNLOCKED = 0,
    #[doc = "1: Debug port access is locked"]
    DBG_ACCESS_LOCKED = 1,
}
impl From<DBG_LOCK_RD_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LOCK_RD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_LOCK_RD`"]
pub type DBG_LOCK_RD_R = crate::R<bool, DBG_LOCK_RD_A>;
impl DBG_LOCK_RD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_LOCK_RD_A {
        match self.bits {
            false => DBG_LOCK_RD_A::DBG_ACCESS_UNLOCKED,
            true => DBG_LOCK_RD_A::DBG_ACCESS_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_ACCESS_UNLOCKED`"]
    #[inline(always)]
    pub fn is_dbg_access_unlocked(&self) -> bool {
        *self == DBG_LOCK_RD_A::DBG_ACCESS_UNLOCKED
    }
    #[doc = "Checks if the value of the field is `DBG_ACCESS_LOCKED`"]
    #[inline(always)]
    pub fn is_dbg_access_locked(&self) -> bool {
        *self == DBG_LOCK_RD_A::DBG_ACCESS_LOCKED
    }
}
impl R {
    #[doc = "Bit 0 - Debug port access state"]
    #[inline(always)]
    pub fn dbg_lock_rd(&self) -> DBG_LOCK_RD_R {
        DBG_LOCK_RD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug port access lock/unlock"]
    #[inline(always)]
    pub fn dbg_lock_wr(&mut self) -> DBG_LOCK_WR_W {
        DBG_LOCK_WR_W { w: self }
    }
}
