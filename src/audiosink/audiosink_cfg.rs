#[doc = "Reader of register AUDIOSINK_CFG"]
pub type R = crate::R<u32, super::AUDIOSINK_CFG>;
#[doc = "Writer for register AUDIOSINK_CFG"]
pub type W = crate::W<u32, super::AUDIOSINK_CFG>;
#[doc = "Register AUDIOSINK_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIOSINK_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Defines how over how many audio sink clock periods the period counter measures\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIODS_CFG_A {
    #[doc = "0: Measure 1 audio sink clock period"]
    AUDIO_SINK_PERIODS_1 = 0,
    #[doc = "1: Measure 2 audio sink clock periods"]
    AUDIO_SINK_PERIODS_2 = 1,
    #[doc = "2: Measure 3 audio sink clock periods"]
    AUDIO_SINK_PERIODS_3 = 2,
    #[doc = "3: Measure 4 audio sink clock periods"]
    AUDIO_SINK_PERIODS_4 = 3,
    #[doc = "4: Measure 5 audio sink clock periods"]
    AUDIO_SINK_PERIODS_5 = 4,
    #[doc = "5: Measure 6 audio sink clock periods"]
    AUDIO_SINK_PERIODS_6 = 5,
    #[doc = "6: Measure 7 audio sink clock periods"]
    AUDIO_SINK_PERIODS_7 = 6,
    #[doc = "7: Measure 8 audio sink clock periods"]
    AUDIO_SINK_PERIODS_8 = 7,
    #[doc = "8: Measure 9 audio sink clock periods"]
    AUDIO_SINK_PERIODS_9 = 8,
    #[doc = "9: Measure 10 audio sink clock periods"]
    AUDIO_SINK_PERIODS_10 = 9,
    #[doc = "10: Measure 11 audio sink clock periods"]
    AUDIO_SINK_PERIODS_11 = 10,
    #[doc = "11: Measure 12 audio sink clock periods"]
    AUDIO_SINK_PERIODS_12 = 11,
    #[doc = "12: Measure 13 audio sink clock periods"]
    AUDIO_SINK_PERIODS_13 = 12,
    #[doc = "13: Measure 14 audio sink clock periods"]
    AUDIO_SINK_PERIODS_14 = 13,
    #[doc = "14: Measure 15 audio sink clock periods"]
    AUDIO_SINK_PERIODS_15 = 14,
    #[doc = "15: Measure 16 audio sink clock periods"]
    AUDIO_SINK_PERIODS_16 = 15,
}
impl From<PERIODS_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIODS_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERIODS_CFG`"]
pub type PERIODS_CFG_R = crate::R<u8, PERIODS_CFG_A>;
impl PERIODS_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIODS_CFG_A {
        match self.bits {
            0 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_1,
            1 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_2,
            2 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_3,
            3 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_4,
            4 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_5,
            5 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_6,
            6 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_7,
            7 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_8,
            8 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_9,
            9 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_10,
            10 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_11,
            11 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_12,
            12 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_13,
            13 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_14,
            14 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_15,
            15 => PERIODS_CFG_A::AUDIO_SINK_PERIODS_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_1`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_1(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_1
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_2`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_2(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_2
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_3`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_3(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_3
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_4`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_4(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_4
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_5`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_5(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_5
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_6`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_6(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_6
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_7`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_7(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_7
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_8`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_8(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_8
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_9`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_9(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_9
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_10`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_10(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_10
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_11`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_11(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_11
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_12`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_12(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_12
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_13`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_13(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_13
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_14`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_14(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_14
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_15`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_15(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_15
    }
    #[doc = "Checks if the value of the field is `AUDIO_SINK_PERIODS_16`"]
    #[inline(always)]
    pub fn is_audio_sink_periods_16(&self) -> bool {
        *self == PERIODS_CFG_A::AUDIO_SINK_PERIODS_16
    }
}
#[doc = "Write proxy for field `PERIODS_CFG`"]
pub struct PERIODS_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIODS_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIODS_CFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Measure 1 audio sink clock period"]
    #[inline(always)]
    pub fn audio_sink_periods_1(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_1)
    }
    #[doc = "Measure 2 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_2(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_2)
    }
    #[doc = "Measure 3 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_3(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_3)
    }
    #[doc = "Measure 4 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_4(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_4)
    }
    #[doc = "Measure 5 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_5(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_5)
    }
    #[doc = "Measure 6 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_6(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_6)
    }
    #[doc = "Measure 7 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_7(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_7)
    }
    #[doc = "Measure 8 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_8(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_8)
    }
    #[doc = "Measure 9 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_9(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_9)
    }
    #[doc = "Measure 10 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_10(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_10)
    }
    #[doc = "Measure 11 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_11(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_11)
    }
    #[doc = "Measure 12 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_12(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_12)
    }
    #[doc = "Measure 13 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_13(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_13)
    }
    #[doc = "Measure 14 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_14(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_14)
    }
    #[doc = "Measure 15 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_15(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_15)
    }
    #[doc = "Measure 16 audio sink clock periods"]
    #[inline(always)]
    pub fn audio_sink_periods_16(self) -> &'a mut W {
        self.variant(PERIODS_CFG_A::AUDIO_SINK_PERIODS_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Defines how over how many audio sink clock periods the period counter measures"]
    #[inline(always)]
    pub fn periods_cfg(&self) -> PERIODS_CFG_R {
        PERIODS_CFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines how over how many audio sink clock periods the period counter measures"]
    #[inline(always)]
    pub fn periods_cfg(&mut self) -> PERIODS_CFG_W {
        PERIODS_CFG_W { w: self }
    }
}
