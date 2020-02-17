#[doc = "Reader of register DIO_INT_CFG[%s]"]
pub type R = crate::R<u32, super::DIO_INT_CFG>;
#[doc = "Writer for register DIO_INT_CFG[%s]"]
pub type W = crate::W<u32, super::DIO_INT_CFG>;
#[doc = "Register DIO_INT_CFG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DIO_INT_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt button debounce filter enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBOUNCE_ENABLE_A {
    #[doc = "0: Button debounce filter disabled"]
    DIO_DEBOUNCE_DISABLE = 0,
    #[doc = "1: Button debounce filter enabled"]
    DIO_DEBOUNCE_ENABLE = 1,
}
impl From<DEBOUNCE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DEBOUNCE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEBOUNCE_ENABLE`"]
pub type DEBOUNCE_ENABLE_R = crate::R<bool, DEBOUNCE_ENABLE_A>;
impl DEBOUNCE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBOUNCE_ENABLE_A {
        match self.bits {
            false => DEBOUNCE_ENABLE_A::DIO_DEBOUNCE_DISABLE,
            true => DEBOUNCE_ENABLE_A::DIO_DEBOUNCE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DIO_DEBOUNCE_DISABLE`"]
    #[inline(always)]
    pub fn is_dio_debounce_disable(&self) -> bool {
        *self == DEBOUNCE_ENABLE_A::DIO_DEBOUNCE_DISABLE
    }
    #[doc = "Checks if the value of the field is `DIO_DEBOUNCE_ENABLE`"]
    #[inline(always)]
    pub fn is_dio_debounce_enable(&self) -> bool {
        *self == DEBOUNCE_ENABLE_A::DIO_DEBOUNCE_ENABLE
    }
}
#[doc = "Write proxy for field `DEBOUNCE_ENABLE`"]
pub struct DEBOUNCE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBOUNCE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Button debounce filter disabled"]
    #[inline(always)]
    pub fn dio_debounce_disable(self) -> &'a mut W {
        self.variant(DEBOUNCE_ENABLE_A::DIO_DEBOUNCE_DISABLE)
    }
    #[doc = "Button debounce filter enabled"]
    #[inline(always)]
    pub fn dio_debounce_enable(self) -> &'a mut W {
        self.variant(DEBOUNCE_ENABLE_A::DIO_DEBOUNCE_ENABLE)
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
#[doc = "Interrupt event configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVENT_A {
    #[doc = "0: Interrupt not triggered"]
    DIO_EVENT_NONE = 0,
    #[doc = "1: Interrupt triggered on high state"]
    DIO_EVENT_HIGH_LEVEL = 1,
    #[doc = "2: Interrupt triggered on low state"]
    DIO_EVENT_LOW_LEVEL = 2,
    #[doc = "3: Interrupt triggered on rising edge"]
    DIO_EVENT_RISING_EDGE = 3,
    #[doc = "4: Interrupt triggered on falling edge"]
    DIO_EVENT_FALLING_EDGE = 4,
    #[doc = "5: Interrupt triggered on any edge"]
    DIO_EVENT_TRANSITION = 5,
}
impl From<EVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVENT`"]
pub type EVENT_R = crate::R<u8, EVENT_A>;
impl EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EVENT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EVENT_A::DIO_EVENT_NONE),
            1 => Val(EVENT_A::DIO_EVENT_HIGH_LEVEL),
            2 => Val(EVENT_A::DIO_EVENT_LOW_LEVEL),
            3 => Val(EVENT_A::DIO_EVENT_RISING_EDGE),
            4 => Val(EVENT_A::DIO_EVENT_FALLING_EDGE),
            5 => Val(EVENT_A::DIO_EVENT_TRANSITION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIO_EVENT_NONE`"]
    #[inline(always)]
    pub fn is_dio_event_none(&self) -> bool {
        *self == EVENT_A::DIO_EVENT_NONE
    }
    #[doc = "Checks if the value of the field is `DIO_EVENT_HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_dio_event_high_level(&self) -> bool {
        *self == EVENT_A::DIO_EVENT_HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `DIO_EVENT_LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_dio_event_low_level(&self) -> bool {
        *self == EVENT_A::DIO_EVENT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DIO_EVENT_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_dio_event_rising_edge(&self) -> bool {
        *self == EVENT_A::DIO_EVENT_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `DIO_EVENT_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_dio_event_falling_edge(&self) -> bool {
        *self == EVENT_A::DIO_EVENT_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `DIO_EVENT_TRANSITION`"]
    #[inline(always)]
    pub fn is_dio_event_transition(&self) -> bool {
        *self == EVENT_A::DIO_EVENT_TRANSITION
    }
}
#[doc = "Write proxy for field `EVENT`"]
pub struct EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt not triggered"]
    #[inline(always)]
    pub fn dio_event_none(self) -> &'a mut W {
        self.variant(EVENT_A::DIO_EVENT_NONE)
    }
    #[doc = "Interrupt triggered on high state"]
    #[inline(always)]
    pub fn dio_event_high_level(self) -> &'a mut W {
        self.variant(EVENT_A::DIO_EVENT_HIGH_LEVEL)
    }
    #[doc = "Interrupt triggered on low state"]
    #[inline(always)]
    pub fn dio_event_low_level(self) -> &'a mut W {
        self.variant(EVENT_A::DIO_EVENT_LOW_LEVEL)
    }
    #[doc = "Interrupt triggered on rising edge"]
    #[inline(always)]
    pub fn dio_event_rising_edge(self) -> &'a mut W {
        self.variant(EVENT_A::DIO_EVENT_RISING_EDGE)
    }
    #[doc = "Interrupt triggered on falling edge"]
    #[inline(always)]
    pub fn dio_event_falling_edge(self) -> &'a mut W {
        self.variant(EVENT_A::DIO_EVENT_FALLING_EDGE)
    }
    #[doc = "Interrupt triggered on any edge"]
    #[inline(always)]
    pub fn dio_event_transition(self) -> &'a mut W {
        self.variant(EVENT_A::DIO_EVENT_TRANSITION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Interrupt input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    DIO_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    DIO_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    DIO_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    DIO_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    DIO_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    DIO_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    DIO_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    DIO_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    DIO_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    DIO_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    DIO_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    DIO_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    DIO_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    DIO_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    DIO_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    DIO_SRC_DIO_15 = 15,
    #[doc = "16: Select EXTCLK as source"]
    DIO_SRC_EXTCLK = 16,
    #[doc = "17: Select WAKEUP as source"]
    DIO_SRC_WAKEUP = 17,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::DIO_SRC_DIO_0),
            1 => Val(SRC_A::DIO_SRC_DIO_1),
            2 => Val(SRC_A::DIO_SRC_DIO_2),
            3 => Val(SRC_A::DIO_SRC_DIO_3),
            4 => Val(SRC_A::DIO_SRC_DIO_4),
            5 => Val(SRC_A::DIO_SRC_DIO_5),
            6 => Val(SRC_A::DIO_SRC_DIO_6),
            7 => Val(SRC_A::DIO_SRC_DIO_7),
            8 => Val(SRC_A::DIO_SRC_DIO_8),
            9 => Val(SRC_A::DIO_SRC_DIO_9),
            10 => Val(SRC_A::DIO_SRC_DIO_10),
            11 => Val(SRC_A::DIO_SRC_DIO_11),
            12 => Val(SRC_A::DIO_SRC_DIO_12),
            13 => Val(SRC_A::DIO_SRC_DIO_13),
            14 => Val(SRC_A::DIO_SRC_DIO_14),
            15 => Val(SRC_A::DIO_SRC_DIO_15),
            16 => Val(SRC_A::DIO_SRC_EXTCLK),
            17 => Val(SRC_A::DIO_SRC_WAKEUP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_dio_src_dio_0(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_dio_src_dio_1(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_dio_src_dio_2(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_dio_src_dio_3(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_dio_src_dio_4(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_dio_src_dio_5(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_dio_src_dio_6(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_dio_src_dio_7(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_dio_src_dio_8(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_dio_src_dio_9(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_dio_src_dio_10(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_dio_src_dio_11(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_dio_src_dio_12(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_dio_src_dio_13(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_dio_src_dio_14(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_dio_src_dio_15(&self) -> bool {
        *self == SRC_A::DIO_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_EXTCLK`"]
    #[inline(always)]
    pub fn is_dio_src_extclk(&self) -> bool {
        *self == SRC_A::DIO_SRC_EXTCLK
    }
    #[doc = "Checks if the value of the field is `DIO_SRC_WAKEUP`"]
    #[inline(always)]
    pub fn is_dio_src_wakeup(&self) -> bool {
        *self == SRC_A::DIO_SRC_WAKEUP
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_0(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_1(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_2(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_3(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_4(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_5(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_6(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_7(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_8(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_9(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_10(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_11(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_12(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_13(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_14(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn dio_src_dio_15(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_DIO_15)
    }
    #[doc = "Select EXTCLK as source"]
    #[inline(always)]
    pub fn dio_src_extclk(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_EXTCLK)
    }
    #[doc = "Select WAKEUP as source"]
    #[inline(always)]
    pub fn dio_src_wakeup(self) -> &'a mut W {
        self.variant(SRC_A::DIO_SRC_WAKEUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Interrupt button debounce filter enable/disable"]
    #[inline(always)]
    pub fn debounce_enable(&self) -> DEBOUNCE_ENABLE_R {
        DEBOUNCE_ENABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Interrupt event configuration"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - Interrupt input selection"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - Interrupt button debounce filter enable/disable"]
    #[inline(always)]
    pub fn debounce_enable(&mut self) -> DEBOUNCE_ENABLE_W {
        DEBOUNCE_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt event configuration"]
    #[inline(always)]
    pub fn event(&mut self) -> EVENT_W {
        EVENT_W { w: self }
    }
    #[doc = "Bits 0:4 - Interrupt input selection"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
