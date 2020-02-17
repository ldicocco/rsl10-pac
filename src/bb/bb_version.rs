#[doc = "Reader of register BB_VERSION"]
pub type R = crate::R<u32, super::BB_VERSION>;
#[doc = "RW-BLE core type (BLE v4.2)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYP_A {
    #[doc = "8: `1000`"]
    TYP_8 = 8,
}
impl From<TYP_A> for u8 {
    #[inline(always)]
    fn from(variant: TYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYP`"]
pub type TYP_R = crate::R<u8, TYP_A>;
impl TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYP_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(TYP_A::TYP_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TYP_8`"]
    #[inline(always)]
    pub fn is_typ_8(&self) -> bool {
        *self == TYP_A::TYP_8
    }
}
#[doc = "RW-BLE core version - major release number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REL_A {
    #[doc = "0: `0`"]
    REL_0 = 0,
}
impl From<REL_A> for u8 {
    #[inline(always)]
    fn from(variant: REL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REL`"]
pub type REL_R = crate::R<u8, REL_A>;
impl REL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REL_A::REL_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REL_0`"]
    #[inline(always)]
    pub fn is_rel_0(&self) -> bool {
        *self == REL_A::REL_0
    }
}
#[doc = "RW-BLE core upgrade - upgrade number\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UPG_A {
    #[doc = "9: `1001`"]
    UPG_09 = 9,
}
impl From<UPG_A> for u8 {
    #[inline(always)]
    fn from(variant: UPG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UPG`"]
pub type UPG_R = crate::R<u8, UPG_A>;
impl UPG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UPG_A> {
        use crate::Variant::*;
        match self.bits {
            9 => Val(UPG_A::UPG_09),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UPG_09`"]
    #[inline(always)]
    pub fn is_upg_09(&self) -> bool {
        *self == UPG_A::UPG_09
    }
}
#[doc = "RW-BLE core build - build number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUILD_A {
    #[doc = "1: `1`"]
    BUILD_1 = 1,
}
impl From<BUILD_A> for u8 {
    #[inline(always)]
    fn from(variant: BUILD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUILD`"]
pub type BUILD_R = crate::R<u8, BUILD_A>;
impl BUILD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BUILD_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BUILD_A::BUILD_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUILD_1`"]
    #[inline(always)]
    pub fn is_build_1(&self) -> bool {
        *self == BUILD_A::BUILD_1
    }
}
impl R {
    #[doc = "Bits 24:31 - RW-BLE core type (BLE v4.2)"]
    #[inline(always)]
    pub fn typ(&self) -> TYP_R {
        TYP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RW-BLE core version - major release number"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RW-BLE core upgrade - upgrade number"]
    #[inline(always)]
    pub fn upg(&self) -> UPG_R {
        UPG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - RW-BLE core build - build number"]
    #[inline(always)]
    pub fn build(&self) -> BUILD_R {
        BUILD_R::new((self.bits & 0xff) as u8)
    }
}
