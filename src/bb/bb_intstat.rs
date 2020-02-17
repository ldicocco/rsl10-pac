#[doc = "Reader of register BB_INTSTAT"]
pub type R = crate::R<u32, super::BB_INTSTAT>;
#[doc = "Audio channel 2 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT2STAT_A {
    #[doc = "0: No Audio interrupt"]
    AUDIOINT2STAT_0 = 0,
    #[doc = "1: An Audio interrupt is pending."]
    AUDIOINT2STAT_1 = 1,
}
impl From<AUDIOINT2STAT_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT2STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT2STAT`"]
pub type AUDIOINT2STAT_R = crate::R<bool, AUDIOINT2STAT_A>;
impl AUDIOINT2STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT2STAT_A {
        match self.bits {
            false => AUDIOINT2STAT_A::AUDIOINT2STAT_0,
            true => AUDIOINT2STAT_A::AUDIOINT2STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2STAT_0`"]
    #[inline(always)]
    pub fn is_audioint2stat_0(&self) -> bool {
        *self == AUDIOINT2STAT_A::AUDIOINT2STAT_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2STAT_1`"]
    #[inline(always)]
    pub fn is_audioint2stat_1(&self) -> bool {
        *self == AUDIOINT2STAT_A::AUDIOINT2STAT_1
    }
}
#[doc = "Audio channel 1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT1STAT_A {
    #[doc = "0: No Audio interrupt"]
    AUDIOINT1STAT_0 = 0,
    #[doc = "1: An Audio interrupt is pending."]
    AUDIOINT1STAT_1 = 1,
}
impl From<AUDIOINT1STAT_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT1STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT1STAT`"]
pub type AUDIOINT1STAT_R = crate::R<bool, AUDIOINT1STAT_A>;
impl AUDIOINT1STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT1STAT_A {
        match self.bits {
            false => AUDIOINT1STAT_A::AUDIOINT1STAT_0,
            true => AUDIOINT1STAT_A::AUDIOINT1STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1STAT_0`"]
    #[inline(always)]
    pub fn is_audioint1stat_0(&self) -> bool {
        *self == AUDIOINT1STAT_A::AUDIOINT1STAT_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1STAT_1`"]
    #[inline(always)]
    pub fn is_audioint1stat_1(&self) -> bool {
        *self == AUDIOINT1STAT_A::AUDIOINT1STAT_1
    }
}
#[doc = "Audio channel 0 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT0STAT_A {
    #[doc = "0: No Audio interrupt"]
    AUDIOINT0STAT_0 = 0,
    #[doc = "1: An Audio interrupt is pending."]
    AUDIOINT0STAT_1 = 1,
}
impl From<AUDIOINT0STAT_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT0STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT0STAT`"]
pub type AUDIOINT0STAT_R = crate::R<bool, AUDIOINT0STAT_A>;
impl AUDIOINT0STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT0STAT_A {
        match self.bits {
            false => AUDIOINT0STAT_A::AUDIOINT0STAT_0,
            true => AUDIOINT0STAT_A::AUDIOINT0STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0STAT_0`"]
    #[inline(always)]
    pub fn is_audioint0stat_0(&self) -> bool {
        *self == AUDIOINT0STAT_A::AUDIOINT0STAT_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0STAT_1`"]
    #[inline(always)]
    pub fn is_audioint0stat_1(&self) -> bool {
        *self == AUDIOINT0STAT_A::AUDIOINT0STAT_1
    }
}
#[doc = "SW triggered interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINTSTAT_A {
    #[doc = "0: No SW triggered interrupt"]
    SWINTSTAT_0 = 0,
    #[doc = "1: A SW triggered interrupt is pending"]
    SWINTSTAT_1 = 1,
}
impl From<SWINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: SWINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWINTSTAT`"]
pub type SWINTSTAT_R = crate::R<bool, SWINTSTAT_A>;
impl SWINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWINTSTAT_A {
        match self.bits {
            false => SWINTSTAT_A::SWINTSTAT_0,
            true => SWINTSTAT_A::SWINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWINTSTAT_0`"]
    #[inline(always)]
    pub fn is_swintstat_0(&self) -> bool {
        *self == SWINTSTAT_A::SWINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `SWINTSTAT_1`"]
    #[inline(always)]
    pub fn is_swintstat_1(&self) -> bool {
        *self == SWINTSTAT_A::SWINTSTAT_1
    }
}
#[doc = "End of event / anticipated pre-fetch abort interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTAPFAINTSTAT_A {
    #[doc = "0: No end of event interrupt"]
    EVENTAPFAINTSTAT_0 = 0,
    #[doc = "1: An end of event interrupt is pending"]
    EVENTAPFAINTSTAT_1 = 1,
}
impl From<EVENTAPFAINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTAPFAINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTAPFAINTSTAT`"]
pub type EVENTAPFAINTSTAT_R = crate::R<bool, EVENTAPFAINTSTAT_A>;
impl EVENTAPFAINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTAPFAINTSTAT_A {
        match self.bits {
            false => EVENTAPFAINTSTAT_A::EVENTAPFAINTSTAT_0,
            true => EVENTAPFAINTSTAT_A::EVENTAPFAINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTAPFAINTSTAT_0`"]
    #[inline(always)]
    pub fn is_eventapfaintstat_0(&self) -> bool {
        *self == EVENTAPFAINTSTAT_A::EVENTAPFAINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `EVENTAPFAINTSTAT_1`"]
    #[inline(always)]
    pub fn is_eventapfaintstat_1(&self) -> bool {
        *self == EVENTAPFAINTSTAT_A::EVENTAPFAINTSTAT_1
    }
}
#[doc = "Masked fine target timer error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINETGTIMINTSTAT_A {
    #[doc = "0: No fine target timer interrupt"]
    FINETGTIMINTSTAT_0 = 0,
    #[doc = "1: A fine target timer interrupt is pending"]
    FINETGTIMINTSTAT_1 = 1,
}
impl From<FINETGTIMINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: FINETGTIMINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINETGTIMINTSTAT`"]
pub type FINETGTIMINTSTAT_R = crate::R<bool, FINETGTIMINTSTAT_A>;
impl FINETGTIMINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINETGTIMINTSTAT_A {
        match self.bits {
            false => FINETGTIMINTSTAT_A::FINETGTIMINTSTAT_0,
            true => FINETGTIMINTSTAT_A::FINETGTIMINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FINETGTIMINTSTAT_0`"]
    #[inline(always)]
    pub fn is_finetgtimintstat_0(&self) -> bool {
        *self == FINETGTIMINTSTAT_A::FINETGTIMINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `FINETGTIMINTSTAT_1`"]
    #[inline(always)]
    pub fn is_finetgtimintstat_1(&self) -> bool {
        *self == FINETGTIMINTSTAT_A::FINETGTIMINTSTAT_1
    }
}
#[doc = "Masked gross target timer interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROSSTGTIMINTSTAT_A {
    #[doc = "0: No gross target timer interrupt"]
    GROSSTGTIMINTSTAT_0 = 0,
    #[doc = "1: A gross target timer interrupt is pending"]
    GROSSTGTIMINTSTAT_1 = 1,
}
impl From<GROSSTGTIMINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: GROSSTGTIMINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GROSSTGTIMINTSTAT`"]
pub type GROSSTGTIMINTSTAT_R = crate::R<bool, GROSSTGTIMINTSTAT_A>;
impl GROSSTGTIMINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROSSTGTIMINTSTAT_A {
        match self.bits {
            false => GROSSTGTIMINTSTAT_A::GROSSTGTIMINTSTAT_0,
            true => GROSSTGTIMINTSTAT_A::GROSSTGTIMINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `GROSSTGTIMINTSTAT_0`"]
    #[inline(always)]
    pub fn is_grosstgtimintstat_0(&self) -> bool {
        *self == GROSSTGTIMINTSTAT_A::GROSSTGTIMINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `GROSSTGTIMINTSTAT_1`"]
    #[inline(always)]
    pub fn is_grosstgtimintstat_1(&self) -> bool {
        *self == GROSSTGTIMINTSTAT_A::GROSSTGTIMINTSTAT_1
    }
}
#[doc = "Masked error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORINTSTAT_A {
    #[doc = "0: No error interrupt"]
    ERRORINTSTAT_0 = 0,
    #[doc = "1: An error interrupt is pending"]
    ERRORINTSTAT_1 = 1,
}
impl From<ERRORINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRORINTSTAT`"]
pub type ERRORINTSTAT_R = crate::R<bool, ERRORINTSTAT_A>;
impl ERRORINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORINTSTAT_A {
        match self.bits {
            false => ERRORINTSTAT_A::ERRORINTSTAT_0,
            true => ERRORINTSTAT_A::ERRORINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRORINTSTAT_0`"]
    #[inline(always)]
    pub fn is_errorintstat_0(&self) -> bool {
        *self == ERRORINTSTAT_A::ERRORINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `ERRORINTSTAT_1`"]
    #[inline(always)]
    pub fn is_errorintstat_1(&self) -> bool {
        *self == ERRORINTSTAT_A::ERRORINTSTAT_1
    }
}
#[doc = "Masked encryption engine interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTINTSTAT_A {
    #[doc = "0: No encryption / decryption interrupt"]
    CRYPTINTSTAT_0 = 0,
    #[doc = "1: An encryption / decryption interrupt is pending"]
    CRYPTINTSTAT_1 = 1,
}
impl From<CRYPTINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTINTSTAT`"]
pub type CRYPTINTSTAT_R = crate::R<bool, CRYPTINTSTAT_A>;
impl CRYPTINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTINTSTAT_A {
        match self.bits {
            false => CRYPTINTSTAT_A::CRYPTINTSTAT_0,
            true => CRYPTINTSTAT_A::CRYPTINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRYPTINTSTAT_0`"]
    #[inline(always)]
    pub fn is_cryptintstat_0(&self) -> bool {
        *self == CRYPTINTSTAT_A::CRYPTINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `CRYPTINTSTAT_1`"]
    #[inline(always)]
    pub fn is_cryptintstat_1(&self) -> bool {
        *self == CRYPTINTSTAT_A::CRYPTINTSTAT_1
    }
}
#[doc = "Masked end of event interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTINTSTAT_A {
    #[doc = "0: No end of advertising / scanning / connection interrupt"]
    EVENTINTSTAT_0 = 0,
    #[doc = "1: An end of advertising / scanning / connection interrupt is pending"]
    EVENTINTSTAT_1 = 1,
}
impl From<EVENTINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTINTSTAT`"]
pub type EVENTINTSTAT_R = crate::R<bool, EVENTINTSTAT_A>;
impl EVENTINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTINTSTAT_A {
        match self.bits {
            false => EVENTINTSTAT_A::EVENTINTSTAT_0,
            true => EVENTINTSTAT_A::EVENTINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTINTSTAT_0`"]
    #[inline(always)]
    pub fn is_eventintstat_0(&self) -> bool {
        *self == EVENTINTSTAT_A::EVENTINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `EVENTINTSTAT_1`"]
    #[inline(always)]
    pub fn is_eventintstat_1(&self) -> bool {
        *self == EVENTINTSTAT_A::EVENTINTSTAT_1
    }
}
#[doc = "Masked sleep interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPINTSTAT_A {
    #[doc = "0: No end of sleep mode interrupt"]
    SLPINTSTAT_0 = 0,
    #[doc = "1: An end of sleep mode interrupt is pending"]
    SLPINTSTAT_1 = 1,
}
impl From<SLPINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: SLPINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLPINTSTAT`"]
pub type SLPINTSTAT_R = crate::R<bool, SLPINTSTAT_A>;
impl SLPINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPINTSTAT_A {
        match self.bits {
            false => SLPINTSTAT_A::SLPINTSTAT_0,
            true => SLPINTSTAT_A::SLPINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLPINTSTAT_0`"]
    #[inline(always)]
    pub fn is_slpintstat_0(&self) -> bool {
        *self == SLPINTSTAT_A::SLPINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `SLPINTSTAT_1`"]
    #[inline(always)]
    pub fn is_slpintstat_1(&self) -> bool {
        *self == SLPINTSTAT_A::SLPINTSTAT_1
    }
}
#[doc = "Masked packet reception interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINTSTAT_A {
    #[doc = "0: No Rx interrupt"]
    RXINTSTAT_0 = 0,
    #[doc = "1: An Rx interrupt is pending"]
    RXINTSTAT_1 = 1,
}
impl From<RXINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: RXINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXINTSTAT`"]
pub type RXINTSTAT_R = crate::R<bool, RXINTSTAT_A>;
impl RXINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINTSTAT_A {
        match self.bits {
            false => RXINTSTAT_A::RXINTSTAT_0,
            true => RXINTSTAT_A::RXINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXINTSTAT_0`"]
    #[inline(always)]
    pub fn is_rxintstat_0(&self) -> bool {
        *self == RXINTSTAT_A::RXINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `RXINTSTAT_1`"]
    #[inline(always)]
    pub fn is_rxintstat_1(&self) -> bool {
        *self == RXINTSTAT_A::RXINTSTAT_1
    }
}
#[doc = "Masked 625us base time reference interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCNTINTSTAT_A {
    #[doc = "0: No 625us base time interrupt"]
    CSCNTINTSTAT_0 = 0,
    #[doc = "1: A 625us base time interrupt is pending"]
    CSCNTINTSTAT_1 = 1,
}
impl From<CSCNTINTSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: CSCNTINTSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCNTINTSTAT`"]
pub type CSCNTINTSTAT_R = crate::R<bool, CSCNTINTSTAT_A>;
impl CSCNTINTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCNTINTSTAT_A {
        match self.bits {
            false => CSCNTINTSTAT_A::CSCNTINTSTAT_0,
            true => CSCNTINTSTAT_A::CSCNTINTSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSCNTINTSTAT_0`"]
    #[inline(always)]
    pub fn is_cscntintstat_0(&self) -> bool {
        *self == CSCNTINTSTAT_A::CSCNTINTSTAT_0
    }
    #[doc = "Checks if the value of the field is `CSCNTINTSTAT_1`"]
    #[inline(always)]
    pub fn is_cscntintstat_1(&self) -> bool {
        *self == CSCNTINTSTAT_A::CSCNTINTSTAT_1
    }
}
impl R {
    #[doc = "Bit 12 - Audio channel 2 interrupt status"]
    #[inline(always)]
    pub fn audioint2stat(&self) -> AUDIOINT2STAT_R {
        AUDIOINT2STAT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Audio channel 1 interrupt status"]
    #[inline(always)]
    pub fn audioint1stat(&self) -> AUDIOINT1STAT_R {
        AUDIOINT1STAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Audio channel 0 interrupt status"]
    #[inline(always)]
    pub fn audioint0stat(&self) -> AUDIOINT0STAT_R {
        AUDIOINT0STAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SW triggered interrupt status"]
    #[inline(always)]
    pub fn swintstat(&self) -> SWINTSTAT_R {
        SWINTSTAT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt status"]
    #[inline(always)]
    pub fn eventapfaintstat(&self) -> EVENTAPFAINTSTAT_R {
        EVENTAPFAINTSTAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Masked fine target timer error interrupt status"]
    #[inline(always)]
    pub fn finetgtimintstat(&self) -> FINETGTIMINTSTAT_R {
        FINETGTIMINTSTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Masked gross target timer interrupt status"]
    #[inline(always)]
    pub fn grosstgtimintstat(&self) -> GROSSTGTIMINTSTAT_R {
        GROSSTGTIMINTSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Masked error interrupt status"]
    #[inline(always)]
    pub fn errorintstat(&self) -> ERRORINTSTAT_R {
        ERRORINTSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Masked encryption engine interrupt status"]
    #[inline(always)]
    pub fn cryptintstat(&self) -> CRYPTINTSTAT_R {
        CRYPTINTSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Masked end of event interrupt status"]
    #[inline(always)]
    pub fn eventintstat(&self) -> EVENTINTSTAT_R {
        EVENTINTSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Masked sleep interrupt status"]
    #[inline(always)]
    pub fn slpintstat(&self) -> SLPINTSTAT_R {
        SLPINTSTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Masked packet reception interrupt status"]
    #[inline(always)]
    pub fn rxintstat(&self) -> RXINTSTAT_R {
        RXINTSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Masked 625us base time reference interrupt status"]
    #[inline(always)]
    pub fn cscntintstat(&self) -> CSCNTINTSTAT_R {
        CSCNTINTSTAT_R::new((self.bits & 0x01) != 0)
    }
}
