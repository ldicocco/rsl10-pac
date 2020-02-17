#[doc = "Reader of register BB_DIAGSTAT"]
pub type R = crate::R<u32, super::BB_DIAGSTAT>;
#[doc = "Directly connected to ble_dbg3\\[7:0\\]
output (debug use only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG3STAT_A {
    #[doc = "0: `0`"]
    DIAG3STAT_0 = 0,
}
impl From<DIAG3STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG3STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG3STAT`"]
pub type DIAG3STAT_R = crate::R<u8, DIAG3STAT_A>;
impl DIAG3STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG3STAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG3STAT_A::DIAG3STAT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG3STAT_0`"]
    #[inline(always)]
    pub fn is_diag3stat_0(&self) -> bool {
        *self == DIAG3STAT_A::DIAG3STAT_0
    }
}
#[doc = "Directly connected to ble_dbg2\\[7:0\\]
output (debug use only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG2STAT_A {
    #[doc = "0: `0`"]
    DIAG2STAT_0 = 0,
}
impl From<DIAG2STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG2STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG2STAT`"]
pub type DIAG2STAT_R = crate::R<u8, DIAG2STAT_A>;
impl DIAG2STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG2STAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG2STAT_A::DIAG2STAT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG2STAT_0`"]
    #[inline(always)]
    pub fn is_diag2stat_0(&self) -> bool {
        *self == DIAG2STAT_A::DIAG2STAT_0
    }
}
#[doc = "Directly connected to ble_dbg1\\[7:0\\]
output (debug use only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG1STAT_A {
    #[doc = "0: `0`"]
    DIAG1STAT_0 = 0,
}
impl From<DIAG1STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG1STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG1STAT`"]
pub type DIAG1STAT_R = crate::R<u8, DIAG1STAT_A>;
impl DIAG1STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG1STAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG1STAT_A::DIAG1STAT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG1STAT_0`"]
    #[inline(always)]
    pub fn is_diag1stat_0(&self) -> bool {
        *self == DIAG1STAT_A::DIAG1STAT_0
    }
}
#[doc = "Directly connected to ble_dbg0\\[7:0\\]
output (debug use only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG0STAT_A {
    #[doc = "0: `0`"]
    DIAG0STAT_0 = 0,
}
impl From<DIAG0STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG0STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG0STAT`"]
pub type DIAG0STAT_R = crate::R<u8, DIAG0STAT_A>;
impl DIAG0STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG0STAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG0STAT_A::DIAG0STAT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG0STAT_0`"]
    #[inline(always)]
    pub fn is_diag0stat_0(&self) -> bool {
        *self == DIAG0STAT_A::DIAG0STAT_0
    }
}
impl R {
    #[doc = "Bits 24:31 - Directly connected to ble_dbg3\\[7:0\\]
output (debug use only)"]
    #[inline(always)]
    pub fn diag3stat(&self) -> DIAG3STAT_R {
        DIAG3STAT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Directly connected to ble_dbg2\\[7:0\\]
output (debug use only)"]
    #[inline(always)]
    pub fn diag2stat(&self) -> DIAG2STAT_R {
        DIAG2STAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Directly connected to ble_dbg1\\[7:0\\]
output (debug use only)"]
    #[inline(always)]
    pub fn diag1stat(&self) -> DIAG1STAT_R {
        DIAG1STAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Directly connected to ble_dbg0\\[7:0\\]
output (debug use only)"]
    #[inline(always)]
    pub fn diag0stat(&self) -> DIAG0STAT_R {
        DIAG0STAT_R::new((self.bits & 0xff) as u8)
    }
}
