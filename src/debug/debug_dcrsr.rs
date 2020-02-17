#[doc = "Reader of register DEBUG_DCRSR"]
pub type R = crate::R<u32, super::DEBUG_DCRSR>;
#[doc = "Writer for register DEBUG_DCRSR"]
pub type W = crate::W<u32, super::DEBUG_DCRSR>;
#[doc = "Register DEBUG_DCRSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG_DCRSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates direction of register transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGWNR_AW {
    #[doc = "0: Indicates register read"]
    REGWNR_READ = 0,
    #[doc = "1: Indicates register write"]
    REGWNR_WRITE = 1,
}
impl From<REGWNR_AW> for bool {
    #[inline(always)]
    fn from(variant: REGWNR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `REGWnR`"]
pub struct REGWNR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGWNR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGWNR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicates register read"]
    #[inline(always)]
    pub fn regwnr_read(self) -> &'a mut W {
        self.variant(REGWNR_AW::REGWNR_READ)
    }
    #[doc = "Indicates register write"]
    #[inline(always)]
    pub fn regwnr_write(self) -> &'a mut W {
        self.variant(REGWNR_AW::REGWNR_WRITE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Indicates register to be accessed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REGSEL_AW {
    #[doc = "0: Select R0"]
    REGSEL_R0 = 0,
    #[doc = "1: Select R1"]
    REGSEL_R1 = 1,
    #[doc = "2: Select R2"]
    REGSEL_R2 = 2,
    #[doc = "3: Select R3"]
    REGSEL_R3 = 3,
    #[doc = "4: Select R4"]
    REGSEL_R4 = 4,
    #[doc = "5: Select R5"]
    REGSEL_R5 = 5,
    #[doc = "6: Select R6"]
    REGSEL_R6 = 6,
    #[doc = "7: Select R7"]
    REGSEL_R7 = 7,
    #[doc = "8: Select R8"]
    REGSEL_R8 = 8,
    #[doc = "9: Select R9"]
    REGSEL_R9 = 9,
    #[doc = "10: Select R10"]
    REGSEL_R10 = 10,
    #[doc = "11: Select R11"]
    REGSEL_R11 = 11,
    #[doc = "12: Select R12"]
    REGSEL_R12 = 12,
    #[doc = "13: Select R13"]
    REGSEL_R13 = 13,
    #[doc = "14: Select R14"]
    REGSEL_R14 = 14,
    #[doc = "15: Select R15"]
    REGSEL_R15 = 15,
    #[doc = "16: Select xPSR/flags"]
    REGSEL_PSRFLGS = 16,
    #[doc = "17: Select main stack pointer"]
    REGSEL_MSP = 17,
    #[doc = "18: Select process stack pointer"]
    REGSEL_PSP = 18,
    #[doc = "20: Access other registers including control, FAULTMASK, BASEPRI and PRIMASK"]
    REGSEL_SPECREG = 20,
}
impl From<REGSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: REGSEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `REGSEL`"]
pub struct REGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select R0"]
    #[inline(always)]
    pub fn regsel_r0(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R0)
    }
    #[doc = "Select R1"]
    #[inline(always)]
    pub fn regsel_r1(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R1)
    }
    #[doc = "Select R2"]
    #[inline(always)]
    pub fn regsel_r2(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R2)
    }
    #[doc = "Select R3"]
    #[inline(always)]
    pub fn regsel_r3(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R3)
    }
    #[doc = "Select R4"]
    #[inline(always)]
    pub fn regsel_r4(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R4)
    }
    #[doc = "Select R5"]
    #[inline(always)]
    pub fn regsel_r5(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R5)
    }
    #[doc = "Select R6"]
    #[inline(always)]
    pub fn regsel_r6(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R6)
    }
    #[doc = "Select R7"]
    #[inline(always)]
    pub fn regsel_r7(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R7)
    }
    #[doc = "Select R8"]
    #[inline(always)]
    pub fn regsel_r8(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R8)
    }
    #[doc = "Select R9"]
    #[inline(always)]
    pub fn regsel_r9(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R9)
    }
    #[doc = "Select R10"]
    #[inline(always)]
    pub fn regsel_r10(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R10)
    }
    #[doc = "Select R11"]
    #[inline(always)]
    pub fn regsel_r11(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R11)
    }
    #[doc = "Select R12"]
    #[inline(always)]
    pub fn regsel_r12(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R12)
    }
    #[doc = "Select R13"]
    #[inline(always)]
    pub fn regsel_r13(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R13)
    }
    #[doc = "Select R14"]
    #[inline(always)]
    pub fn regsel_r14(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R14)
    }
    #[doc = "Select R15"]
    #[inline(always)]
    pub fn regsel_r15(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_R15)
    }
    #[doc = "Select xPSR/flags"]
    #[inline(always)]
    pub fn regsel_psrflgs(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_PSRFLGS)
    }
    #[doc = "Select main stack pointer"]
    #[inline(always)]
    pub fn regsel_msp(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_MSP)
    }
    #[doc = "Select process stack pointer"]
    #[inline(always)]
    pub fn regsel_psp(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_PSP)
    }
    #[doc = "Access other registers including control, FAULTMASK, BASEPRI and PRIMASK"]
    #[inline(always)]
    pub fn regsel_specreg(self) -> &'a mut W {
        self.variant(REGSEL_AW::REGSEL_SPECREG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 16 - Indicates direction of register transfer"]
    #[inline(always)]
    pub fn regwn_r(&mut self) -> REGWNR_W {
        REGWNR_W { w: self }
    }
    #[doc = "Bits 0:4 - Indicates register to be accessed"]
    #[inline(always)]
    pub fn regsel(&mut self) -> REGSEL_W {
        REGSEL_W { w: self }
    }
}
