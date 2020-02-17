#[doc = "Reader of register AHBREGS_CHIP_ID_NUM"]
pub type R = crate::R<u32, super::AHBREGS_CHIP_ID_NUM>;
#[doc = "Reader of field `CHIP_FAMILY`"]
pub type CHIP_FAMILY_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHIP_VERSION`"]
pub type CHIP_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHIP_MAJOR_REVISION`"]
pub type CHIP_MAJOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHIP_MINOR_REVISION`"]
pub type CHIP_MINOR_REVISION_R = crate::R<u8, u8>;
#[doc = "OD feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_FEATURE_A {
    #[doc = "0: Device does not have the output driver feature"]
    CHIP_ID_OD_NOT_PRESENT = 0,
    #[doc = "1: Device has the output driver feature"]
    CHIP_ID_OD_PRESENT = 1,
}
impl From<OD_FEATURE_A> for bool {
    #[inline(always)]
    fn from(variant: OD_FEATURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_FEATURE`"]
pub type OD_FEATURE_R = crate::R<bool, OD_FEATURE_A>;
impl OD_FEATURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_FEATURE_A {
        match self.bits {
            false => OD_FEATURE_A::CHIP_ID_OD_NOT_PRESENT,
            true => OD_FEATURE_A::CHIP_ID_OD_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `CHIP_ID_OD_NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_chip_id_od_not_present(&self) -> bool {
        *self == OD_FEATURE_A::CHIP_ID_OD_NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `CHIP_ID_OD_PRESENT`"]
    #[inline(always)]
    pub fn is_chip_id_od_present(&self) -> bool {
        *self == OD_FEATURE_A::CHIP_ID_OD_PRESENT
    }
}
#[doc = "LPDSP32 feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_FEATURE_A {
    #[doc = "0: Device does not have the LPDSP32 feature"]
    CHIP_ID_LPDSP32_NOT_PRESENT = 0,
    #[doc = "1: Device has the LPDSP32 feature"]
    CHIP_ID_LPDSP32_PRESENT = 1,
}
impl From<LPDSP32_FEATURE_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_FEATURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_FEATURE`"]
pub type LPDSP32_FEATURE_R = crate::R<bool, LPDSP32_FEATURE_A>;
impl LPDSP32_FEATURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_FEATURE_A {
        match self.bits {
            false => LPDSP32_FEATURE_A::CHIP_ID_LPDSP32_NOT_PRESENT,
            true => LPDSP32_FEATURE_A::CHIP_ID_LPDSP32_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `CHIP_ID_LPDSP32_NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_chip_id_lpdsp32_not_present(&self) -> bool {
        *self == LPDSP32_FEATURE_A::CHIP_ID_LPDSP32_NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `CHIP_ID_LPDSP32_PRESENT`"]
    #[inline(always)]
    pub fn is_chip_id_lpdsp32_present(&self) -> bool {
        *self == LPDSP32_FEATURE_A::CHIP_ID_LPDSP32_PRESENT
    }
}
#[doc = "AOBLE feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOBLE_FEATURE_A {
    #[doc = "0: Device does not have the Audio over BLE feature"]
    CHIP_ID_AOBLE_NOT_PRESENT = 0,
    #[doc = "1: Device has the Audio over BLE feature"]
    CHIP_ID_AOBLE_PRESENT = 1,
}
impl From<AOBLE_FEATURE_A> for bool {
    #[inline(always)]
    fn from(variant: AOBLE_FEATURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AOBLE_FEATURE`"]
pub type AOBLE_FEATURE_R = crate::R<bool, AOBLE_FEATURE_A>;
impl AOBLE_FEATURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOBLE_FEATURE_A {
        match self.bits {
            false => AOBLE_FEATURE_A::CHIP_ID_AOBLE_NOT_PRESENT,
            true => AOBLE_FEATURE_A::CHIP_ID_AOBLE_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `CHIP_ID_AOBLE_NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_chip_id_aoble_not_present(&self) -> bool {
        *self == AOBLE_FEATURE_A::CHIP_ID_AOBLE_NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `CHIP_ID_AOBLE_PRESENT`"]
    #[inline(always)]
    pub fn is_chip_id_aoble_present(&self) -> bool {
        *self == AOBLE_FEATURE_A::CHIP_ID_AOBLE_PRESENT
    }
}
impl R {
    #[doc = "Bits 24:31 - Chip Family number"]
    #[inline(always)]
    pub fn chip_family(&self) -> CHIP_FAMILY_R {
        CHIP_FAMILY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Chip Version number"]
    #[inline(always)]
    pub fn chip_version(&self) -> CHIP_VERSION_R {
        CHIP_VERSION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Chip Major Revision number"]
    #[inline(always)]
    pub fn chip_major_revision(&self) -> CHIP_MAJOR_REVISION_R {
        CHIP_MAJOR_REVISION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 3:7 - Chip Minor Revision number"]
    #[inline(always)]
    pub fn chip_minor_revision(&self) -> CHIP_MINOR_REVISION_R {
        CHIP_MINOR_REVISION_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - OD feature"]
    #[inline(always)]
    pub fn od_feature(&self) -> OD_FEATURE_R {
        OD_FEATURE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPDSP32 feature"]
    #[inline(always)]
    pub fn lpdsp32_feature(&self) -> LPDSP32_FEATURE_R {
        LPDSP32_FEATURE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AOBLE feature"]
    #[inline(always)]
    pub fn aoble_feature(&self) -> AOBLE_FEATURE_R {
        AOBLE_FEATURE_R::new((self.bits & 0x01) != 0)
    }
}
