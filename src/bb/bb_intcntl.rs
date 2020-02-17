#[doc = "Reader of register BB_INTCNTL"]
pub type R = crate::R<u32, super::BB_INTCNTL>;
#[doc = "Writer for register BB_INTCNTL"]
pub type W = crate::W<u32, super::BB_INTCNTL>;
#[doc = "Register BB_INTCNTL `reset()`'s with value 0x811f"]
impl crate::ResetValue for super::BB_INTCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x811f
    }
}
#[doc = "CSCNT interrupt mask during event allowing to enable CSCNT interrupt generation during events\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCNTDEVMSK_A {
    #[doc = "0: CSCNT interrupt not generated during events"]
    CSCNTDEVMSK_0 = 0,
    #[doc = "1: CSCNT interrupt generated during events"]
    CSCNTDEVMSK_1 = 1,
}
impl From<CSCNTDEVMSK_A> for bool {
    #[inline(always)]
    fn from(variant: CSCNTDEVMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCNTDEVMSK`"]
pub type CSCNTDEVMSK_R = crate::R<bool, CSCNTDEVMSK_A>;
impl CSCNTDEVMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCNTDEVMSK_A {
        match self.bits {
            false => CSCNTDEVMSK_A::CSCNTDEVMSK_0,
            true => CSCNTDEVMSK_A::CSCNTDEVMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSCNTDEVMSK_0`"]
    #[inline(always)]
    pub fn is_cscntdevmsk_0(&self) -> bool {
        *self == CSCNTDEVMSK_A::CSCNTDEVMSK_0
    }
    #[doc = "Checks if the value of the field is `CSCNTDEVMSK_1`"]
    #[inline(always)]
    pub fn is_cscntdevmsk_1(&self) -> bool {
        *self == CSCNTDEVMSK_A::CSCNTDEVMSK_1
    }
}
#[doc = "Write proxy for field `CSCNTDEVMSK`"]
pub struct CSCNTDEVMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNTDEVMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCNTDEVMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSCNT interrupt not generated during events"]
    #[inline(always)]
    pub fn cscntdevmsk_0(self) -> &'a mut W {
        self.variant(CSCNTDEVMSK_A::CSCNTDEVMSK_0)
    }
    #[doc = "CSCNT interrupt generated during events"]
    #[inline(always)]
    pub fn cscntdevmsk_1(self) -> &'a mut W {
        self.variant(CSCNTDEVMSK_A::CSCNTDEVMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Audio channel 2 interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT2MSK_A {
    #[doc = "0: Interrupt not generated"]
    AUDIOINT2MSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    AUDIOINT2MSK_1 = 1,
}
impl From<AUDIOINT2MSK_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT2MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT2MSK`"]
pub type AUDIOINT2MSK_R = crate::R<bool, AUDIOINT2MSK_A>;
impl AUDIOINT2MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT2MSK_A {
        match self.bits {
            false => AUDIOINT2MSK_A::AUDIOINT2MSK_0,
            true => AUDIOINT2MSK_A::AUDIOINT2MSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2MSK_0`"]
    #[inline(always)]
    pub fn is_audioint2msk_0(&self) -> bool {
        *self == AUDIOINT2MSK_A::AUDIOINT2MSK_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2MSK_1`"]
    #[inline(always)]
    pub fn is_audioint2msk_1(&self) -> bool {
        *self == AUDIOINT2MSK_A::AUDIOINT2MSK_1
    }
}
#[doc = "Write proxy for field `AUDIOINT2MSK`"]
pub struct AUDIOINT2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIOINT2MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIOINT2MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn audioint2msk_0(self) -> &'a mut W {
        self.variant(AUDIOINT2MSK_A::AUDIOINT2MSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn audioint2msk_1(self) -> &'a mut W {
        self.variant(AUDIOINT2MSK_A::AUDIOINT2MSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Audio channel 1 interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT1MSK_A {
    #[doc = "0: Interrupt not generated"]
    AUDIOINT1MSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    AUDIOINT1MSK_1 = 1,
}
impl From<AUDIOINT1MSK_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT1MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT1MSK`"]
pub type AUDIOINT1MSK_R = crate::R<bool, AUDIOINT1MSK_A>;
impl AUDIOINT1MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT1MSK_A {
        match self.bits {
            false => AUDIOINT1MSK_A::AUDIOINT1MSK_0,
            true => AUDIOINT1MSK_A::AUDIOINT1MSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1MSK_0`"]
    #[inline(always)]
    pub fn is_audioint1msk_0(&self) -> bool {
        *self == AUDIOINT1MSK_A::AUDIOINT1MSK_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1MSK_1`"]
    #[inline(always)]
    pub fn is_audioint1msk_1(&self) -> bool {
        *self == AUDIOINT1MSK_A::AUDIOINT1MSK_1
    }
}
#[doc = "Write proxy for field `AUDIOINT1MSK`"]
pub struct AUDIOINT1MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIOINT1MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIOINT1MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn audioint1msk_0(self) -> &'a mut W {
        self.variant(AUDIOINT1MSK_A::AUDIOINT1MSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn audioint1msk_1(self) -> &'a mut W {
        self.variant(AUDIOINT1MSK_A::AUDIOINT1MSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Audio channel 0 interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT0MSK_A {
    #[doc = "0: Interrupt not generated"]
    AUDIOINT0MSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    AUDIOINT0MSK_1 = 1,
}
impl From<AUDIOINT0MSK_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT0MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT0MSK`"]
pub type AUDIOINT0MSK_R = crate::R<bool, AUDIOINT0MSK_A>;
impl AUDIOINT0MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT0MSK_A {
        match self.bits {
            false => AUDIOINT0MSK_A::AUDIOINT0MSK_0,
            true => AUDIOINT0MSK_A::AUDIOINT0MSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0MSK_0`"]
    #[inline(always)]
    pub fn is_audioint0msk_0(&self) -> bool {
        *self == AUDIOINT0MSK_A::AUDIOINT0MSK_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0MSK_1`"]
    #[inline(always)]
    pub fn is_audioint0msk_1(&self) -> bool {
        *self == AUDIOINT0MSK_A::AUDIOINT0MSK_1
    }
}
#[doc = "Write proxy for field `AUDIOINT0MSK`"]
pub struct AUDIOINT0MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIOINT0MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIOINT0MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn audioint0msk_0(self) -> &'a mut W {
        self.variant(AUDIOINT0MSK_A::AUDIOINT0MSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn audioint0msk_1(self) -> &'a mut W {
        self.variant(AUDIOINT0MSK_A::AUDIOINT0MSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "SW triggered interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    SWINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    SWINTMSK_1 = 1,
}
impl From<SWINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: SWINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWINTMSK`"]
pub type SWINTMSK_R = crate::R<bool, SWINTMSK_A>;
impl SWINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWINTMSK_A {
        match self.bits {
            false => SWINTMSK_A::SWINTMSK_0,
            true => SWINTMSK_A::SWINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWINTMSK_0`"]
    #[inline(always)]
    pub fn is_swintmsk_0(&self) -> bool {
        *self == SWINTMSK_A::SWINTMSK_0
    }
    #[doc = "Checks if the value of the field is `SWINTMSK_1`"]
    #[inline(always)]
    pub fn is_swintmsk_1(&self) -> bool {
        *self == SWINTMSK_A::SWINTMSK_1
    }
}
#[doc = "Write proxy for field `SWINTMSK`"]
pub struct SWINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn swintmsk_0(self) -> &'a mut W {
        self.variant(SWINTMSK_A::SWINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn swintmsk_1(self) -> &'a mut W {
        self.variant(SWINTMSK_A::SWINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "End of event / anticipated pre-fetch abort interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTAPFAINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    EVENTAPFAINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    EVENTAPFAINTMSK_1 = 1,
}
impl From<EVENTAPFAINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTAPFAINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTAPFAINTMSK`"]
pub type EVENTAPFAINTMSK_R = crate::R<bool, EVENTAPFAINTMSK_A>;
impl EVENTAPFAINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTAPFAINTMSK_A {
        match self.bits {
            false => EVENTAPFAINTMSK_A::EVENTAPFAINTMSK_0,
            true => EVENTAPFAINTMSK_A::EVENTAPFAINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTAPFAINTMSK_0`"]
    #[inline(always)]
    pub fn is_eventapfaintmsk_0(&self) -> bool {
        *self == EVENTAPFAINTMSK_A::EVENTAPFAINTMSK_0
    }
    #[doc = "Checks if the value of the field is `EVENTAPFAINTMSK_1`"]
    #[inline(always)]
    pub fn is_eventapfaintmsk_1(&self) -> bool {
        *self == EVENTAPFAINTMSK_A::EVENTAPFAINTMSK_1
    }
}
#[doc = "Write proxy for field `EVENTAPFAINTMSK`"]
pub struct EVENTAPFAINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTAPFAINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTAPFAINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn eventapfaintmsk_0(self) -> &'a mut W {
        self.variant(EVENTAPFAINTMSK_A::EVENTAPFAINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn eventapfaintmsk_1(self) -> &'a mut W {
        self.variant(EVENTAPFAINTMSK_A::EVENTAPFAINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Fine target timer mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINETGTIMINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    FINETGTIMINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    FINETGTIMINTMSK_1 = 1,
}
impl From<FINETGTIMINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: FINETGTIMINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINETGTIMINTMSK`"]
pub type FINETGTIMINTMSK_R = crate::R<bool, FINETGTIMINTMSK_A>;
impl FINETGTIMINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINETGTIMINTMSK_A {
        match self.bits {
            false => FINETGTIMINTMSK_A::FINETGTIMINTMSK_0,
            true => FINETGTIMINTMSK_A::FINETGTIMINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FINETGTIMINTMSK_0`"]
    #[inline(always)]
    pub fn is_finetgtimintmsk_0(&self) -> bool {
        *self == FINETGTIMINTMSK_A::FINETGTIMINTMSK_0
    }
    #[doc = "Checks if the value of the field is `FINETGTIMINTMSK_1`"]
    #[inline(always)]
    pub fn is_finetgtimintmsk_1(&self) -> bool {
        *self == FINETGTIMINTMSK_A::FINETGTIMINTMSK_1
    }
}
#[doc = "Write proxy for field `FINETGTIMINTMSK`"]
pub struct FINETGTIMINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETGTIMINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINETGTIMINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn finetgtimintmsk_0(self) -> &'a mut W {
        self.variant(FINETGTIMINTMSK_A::FINETGTIMINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn finetgtimintmsk_1(self) -> &'a mut W {
        self.variant(FINETGTIMINTMSK_A::FINETGTIMINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Gross target timer mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROSSTGTIMINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    GROSSTGTIMINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    GROSSTGTIMINTMSK_1 = 1,
}
impl From<GROSSTGTIMINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: GROSSTGTIMINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GROSSTGTIMINTMSK`"]
pub type GROSSTGTIMINTMSK_R = crate::R<bool, GROSSTGTIMINTMSK_A>;
impl GROSSTGTIMINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROSSTGTIMINTMSK_A {
        match self.bits {
            false => GROSSTGTIMINTMSK_A::GROSSTGTIMINTMSK_0,
            true => GROSSTGTIMINTMSK_A::GROSSTGTIMINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `GROSSTGTIMINTMSK_0`"]
    #[inline(always)]
    pub fn is_grosstgtimintmsk_0(&self) -> bool {
        *self == GROSSTGTIMINTMSK_A::GROSSTGTIMINTMSK_0
    }
    #[doc = "Checks if the value of the field is `GROSSTGTIMINTMSK_1`"]
    #[inline(always)]
    pub fn is_grosstgtimintmsk_1(&self) -> bool {
        *self == GROSSTGTIMINTMSK_A::GROSSTGTIMINTMSK_1
    }
}
#[doc = "Write proxy for field `GROSSTGTIMINTMSK`"]
pub struct GROSSTGTIMINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GROSSTGTIMINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROSSTGTIMINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn grosstgtimintmsk_0(self) -> &'a mut W {
        self.variant(GROSSTGTIMINTMSK_A::GROSSTGTIMINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn grosstgtimintmsk_1(self) -> &'a mut W {
        self.variant(GROSSTGTIMINTMSK_A::GROSSTGTIMINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    ERRORINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    ERRORINTMSK_1 = 1,
}
impl From<ERRORINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRORINTMSK`"]
pub type ERRORINTMSK_R = crate::R<bool, ERRORINTMSK_A>;
impl ERRORINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORINTMSK_A {
        match self.bits {
            false => ERRORINTMSK_A::ERRORINTMSK_0,
            true => ERRORINTMSK_A::ERRORINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRORINTMSK_0`"]
    #[inline(always)]
    pub fn is_errorintmsk_0(&self) -> bool {
        *self == ERRORINTMSK_A::ERRORINTMSK_0
    }
    #[doc = "Checks if the value of the field is `ERRORINTMSK_1`"]
    #[inline(always)]
    pub fn is_errorintmsk_1(&self) -> bool {
        *self == ERRORINTMSK_A::ERRORINTMSK_1
    }
}
#[doc = "Write proxy for field `ERRORINTMSK`"]
pub struct ERRORINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRORINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn errorintmsk_0(self) -> &'a mut W {
        self.variant(ERRORINTMSK_A::ERRORINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn errorintmsk_1(self) -> &'a mut W {
        self.variant(ERRORINTMSK_A::ERRORINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Encryption engine interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    CRYPTINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    CRYPTINTMSK_1 = 1,
}
impl From<CRYPTINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTINTMSK`"]
pub type CRYPTINTMSK_R = crate::R<bool, CRYPTINTMSK_A>;
impl CRYPTINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTINTMSK_A {
        match self.bits {
            false => CRYPTINTMSK_A::CRYPTINTMSK_0,
            true => CRYPTINTMSK_A::CRYPTINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRYPTINTMSK_0`"]
    #[inline(always)]
    pub fn is_cryptintmsk_0(&self) -> bool {
        *self == CRYPTINTMSK_A::CRYPTINTMSK_0
    }
    #[doc = "Checks if the value of the field is `CRYPTINTMSK_1`"]
    #[inline(always)]
    pub fn is_cryptintmsk_1(&self) -> bool {
        *self == CRYPTINTMSK_A::CRYPTINTMSK_1
    }
}
#[doc = "Write proxy for field `CRYPTINTMSK`"]
pub struct CRYPTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn cryptintmsk_0(self) -> &'a mut W {
        self.variant(CRYPTINTMSK_A::CRYPTINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn cryptintmsk_1(self) -> &'a mut W {
        self.variant(CRYPTINTMSK_A::CRYPTINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "End of event interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    EVENTINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    EVENTINTMSK_1 = 1,
}
impl From<EVENTINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTINTMSK`"]
pub type EVENTINTMSK_R = crate::R<bool, EVENTINTMSK_A>;
impl EVENTINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTINTMSK_A {
        match self.bits {
            false => EVENTINTMSK_A::EVENTINTMSK_0,
            true => EVENTINTMSK_A::EVENTINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTINTMSK_0`"]
    #[inline(always)]
    pub fn is_eventintmsk_0(&self) -> bool {
        *self == EVENTINTMSK_A::EVENTINTMSK_0
    }
    #[doc = "Checks if the value of the field is `EVENTINTMSK_1`"]
    #[inline(always)]
    pub fn is_eventintmsk_1(&self) -> bool {
        *self == EVENTINTMSK_A::EVENTINTMSK_1
    }
}
#[doc = "Write proxy for field `EVENTINTMSK`"]
pub struct EVENTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn eventintmsk_0(self) -> &'a mut W {
        self.variant(EVENTINTMSK_A::EVENTINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn eventintmsk_1(self) -> &'a mut W {
        self.variant(EVENTINTMSK_A::EVENTINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Sleep mode interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    SLPINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    SLPINTMSK_1 = 1,
}
impl From<SLPINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: SLPINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLPINTMSK`"]
pub type SLPINTMSK_R = crate::R<bool, SLPINTMSK_A>;
impl SLPINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPINTMSK_A {
        match self.bits {
            false => SLPINTMSK_A::SLPINTMSK_0,
            true => SLPINTMSK_A::SLPINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLPINTMSK_0`"]
    #[inline(always)]
    pub fn is_slpintmsk_0(&self) -> bool {
        *self == SLPINTMSK_A::SLPINTMSK_0
    }
    #[doc = "Checks if the value of the field is `SLPINTMSK_1`"]
    #[inline(always)]
    pub fn is_slpintmsk_1(&self) -> bool {
        *self == SLPINTMSK_A::SLPINTMSK_1
    }
}
#[doc = "Write proxy for field `SLPINTMSK`"]
pub struct SLPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLPINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn slpintmsk_0(self) -> &'a mut W {
        self.variant(SLPINTMSK_A::SLPINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn slpintmsk_1(self) -> &'a mut W {
        self.variant(SLPINTMSK_A::SLPINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Rx interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    RXINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    RXINTMSK_1 = 1,
}
impl From<RXINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: RXINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXINTMSK`"]
pub type RXINTMSK_R = crate::R<bool, RXINTMSK_A>;
impl RXINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINTMSK_A {
        match self.bits {
            false => RXINTMSK_A::RXINTMSK_0,
            true => RXINTMSK_A::RXINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXINTMSK_0`"]
    #[inline(always)]
    pub fn is_rxintmsk_0(&self) -> bool {
        *self == RXINTMSK_A::RXINTMSK_0
    }
    #[doc = "Checks if the value of the field is `RXINTMSK_1`"]
    #[inline(always)]
    pub fn is_rxintmsk_1(&self) -> bool {
        *self == RXINTMSK_A::RXINTMSK_1
    }
}
#[doc = "Write proxy for field `RXINTMSK`"]
pub struct RXINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn rxintmsk_0(self) -> &'a mut W {
        self.variant(RXINTMSK_A::RXINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn rxintmsk_1(self) -> &'a mut W {
        self.variant(RXINTMSK_A::RXINTMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "625us base time interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCNTINTMSK_A {
    #[doc = "0: Interrupt not generated"]
    CSCNTINTMSK_0 = 0,
    #[doc = "1: Interrupt generated"]
    CSCNTINTMSK_1 = 1,
}
impl From<CSCNTINTMSK_A> for bool {
    #[inline(always)]
    fn from(variant: CSCNTINTMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCNTINTMSK`"]
pub type CSCNTINTMSK_R = crate::R<bool, CSCNTINTMSK_A>;
impl CSCNTINTMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCNTINTMSK_A {
        match self.bits {
            false => CSCNTINTMSK_A::CSCNTINTMSK_0,
            true => CSCNTINTMSK_A::CSCNTINTMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSCNTINTMSK_0`"]
    #[inline(always)]
    pub fn is_cscntintmsk_0(&self) -> bool {
        *self == CSCNTINTMSK_A::CSCNTINTMSK_0
    }
    #[doc = "Checks if the value of the field is `CSCNTINTMSK_1`"]
    #[inline(always)]
    pub fn is_cscntintmsk_1(&self) -> bool {
        *self == CSCNTINTMSK_A::CSCNTINTMSK_1
    }
}
#[doc = "Write proxy for field `CSCNTINTMSK`"]
pub struct CSCNTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNTINTMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCNTINTMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not generated"]
    #[inline(always)]
    pub fn cscntintmsk_0(self) -> &'a mut W {
        self.variant(CSCNTINTMSK_A::CSCNTINTMSK_0)
    }
    #[doc = "Interrupt generated"]
    #[inline(always)]
    pub fn cscntintmsk_1(self) -> &'a mut W {
        self.variant(CSCNTINTMSK_A::CSCNTINTMSK_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - CSCNT interrupt mask during event allowing to enable CSCNT interrupt generation during events"]
    #[inline(always)]
    pub fn cscntdevmsk(&self) -> CSCNTDEVMSK_R {
        CSCNTDEVMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Audio channel 2 interrupt mask"]
    #[inline(always)]
    pub fn audioint2msk(&self) -> AUDIOINT2MSK_R {
        AUDIOINT2MSK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Audio channel 1 interrupt mask"]
    #[inline(always)]
    pub fn audioint1msk(&self) -> AUDIOINT1MSK_R {
        AUDIOINT1MSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Audio channel 0 interrupt mask"]
    #[inline(always)]
    pub fn audioint0msk(&self) -> AUDIOINT0MSK_R {
        AUDIOINT0MSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SW triggered interrupt mask"]
    #[inline(always)]
    pub fn swintmsk(&self) -> SWINTMSK_R {
        SWINTMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt mask"]
    #[inline(always)]
    pub fn eventapfaintmsk(&self) -> EVENTAPFAINTMSK_R {
        EVENTAPFAINTMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fine target timer mask"]
    #[inline(always)]
    pub fn finetgtimintmsk(&self) -> FINETGTIMINTMSK_R {
        FINETGTIMINTMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Gross target timer mask"]
    #[inline(always)]
    pub fn grosstgtimintmsk(&self) -> GROSSTGTIMINTMSK_R {
        GROSSTGTIMINTMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error interrupt mask"]
    #[inline(always)]
    pub fn errorintmsk(&self) -> ERRORINTMSK_R {
        ERRORINTMSK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Encryption engine interrupt mask"]
    #[inline(always)]
    pub fn cryptintmsk(&self) -> CRYPTINTMSK_R {
        CRYPTINTMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of event interrupt mask"]
    #[inline(always)]
    pub fn eventintmsk(&self) -> EVENTINTMSK_R {
        EVENTINTMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sleep mode interrupt mask"]
    #[inline(always)]
    pub fn slpintmsk(&self) -> SLPINTMSK_R {
        SLPINTMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx interrupt mask"]
    #[inline(always)]
    pub fn rxintmsk(&self) -> RXINTMSK_R {
        RXINTMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 625us base time interrupt mask"]
    #[inline(always)]
    pub fn cscntintmsk(&self) -> CSCNTINTMSK_R {
        CSCNTINTMSK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - CSCNT interrupt mask during event allowing to enable CSCNT interrupt generation during events"]
    #[inline(always)]
    pub fn cscntdevmsk(&mut self) -> CSCNTDEVMSK_W {
        CSCNTDEVMSK_W { w: self }
    }
    #[doc = "Bit 12 - Audio channel 2 interrupt mask"]
    #[inline(always)]
    pub fn audioint2msk(&mut self) -> AUDIOINT2MSK_W {
        AUDIOINT2MSK_W { w: self }
    }
    #[doc = "Bit 11 - Audio channel 1 interrupt mask"]
    #[inline(always)]
    pub fn audioint1msk(&mut self) -> AUDIOINT1MSK_W {
        AUDIOINT1MSK_W { w: self }
    }
    #[doc = "Bit 10 - Audio channel 0 interrupt mask"]
    #[inline(always)]
    pub fn audioint0msk(&mut self) -> AUDIOINT0MSK_W {
        AUDIOINT0MSK_W { w: self }
    }
    #[doc = "Bit 9 - SW triggered interrupt mask"]
    #[inline(always)]
    pub fn swintmsk(&mut self) -> SWINTMSK_W {
        SWINTMSK_W { w: self }
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt mask"]
    #[inline(always)]
    pub fn eventapfaintmsk(&mut self) -> EVENTAPFAINTMSK_W {
        EVENTAPFAINTMSK_W { w: self }
    }
    #[doc = "Bit 7 - Fine target timer mask"]
    #[inline(always)]
    pub fn finetgtimintmsk(&mut self) -> FINETGTIMINTMSK_W {
        FINETGTIMINTMSK_W { w: self }
    }
    #[doc = "Bit 6 - Gross target timer mask"]
    #[inline(always)]
    pub fn grosstgtimintmsk(&mut self) -> GROSSTGTIMINTMSK_W {
        GROSSTGTIMINTMSK_W { w: self }
    }
    #[doc = "Bit 5 - Error interrupt mask"]
    #[inline(always)]
    pub fn errorintmsk(&mut self) -> ERRORINTMSK_W {
        ERRORINTMSK_W { w: self }
    }
    #[doc = "Bit 4 - Encryption engine interrupt mask"]
    #[inline(always)]
    pub fn cryptintmsk(&mut self) -> CRYPTINTMSK_W {
        CRYPTINTMSK_W { w: self }
    }
    #[doc = "Bit 3 - End of event interrupt mask"]
    #[inline(always)]
    pub fn eventintmsk(&mut self) -> EVENTINTMSK_W {
        EVENTINTMSK_W { w: self }
    }
    #[doc = "Bit 2 - Sleep mode interrupt mask"]
    #[inline(always)]
    pub fn slpintmsk(&mut self) -> SLPINTMSK_W {
        SLPINTMSK_W { w: self }
    }
    #[doc = "Bit 1 - Rx interrupt mask"]
    #[inline(always)]
    pub fn rxintmsk(&mut self) -> RXINTMSK_W {
        RXINTMSK_W { w: self }
    }
    #[doc = "Bit 0 - 625us base time interrupt mask"]
    #[inline(always)]
    pub fn cscntintmsk(&mut self) -> CSCNTINTMSK_W {
        CSCNTINTMSK_W { w: self }
    }
}
