#[doc = "Reader of register BB_INTRAWSTAT"]
pub type R = crate::R<u32, super::BB_INTRAWSTAT>;
#[doc = "Audio channel 2 interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT2RAWSTAT_A {
    #[doc = "0: No Audio interrupt"]
    AUDIOINT2RAWSTAT_0 = 0,
    #[doc = "1: An Audio interrupt is pending."]
    AUDIOINT2RAWSTAT_1 = 1,
}
impl From<AUDIOINT2RAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT2RAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT2RAWSTAT`"]
pub type AUDIOINT2RAWSTAT_R = crate::R<bool, AUDIOINT2RAWSTAT_A>;
impl AUDIOINT2RAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT2RAWSTAT_A {
        match self.bits {
            false => AUDIOINT2RAWSTAT_A::AUDIOINT2RAWSTAT_0,
            true => AUDIOINT2RAWSTAT_A::AUDIOINT2RAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2RAWSTAT_0`"]
    #[inline(always)]
    pub fn is_audioint2rawstat_0(&self) -> bool {
        *self == AUDIOINT2RAWSTAT_A::AUDIOINT2RAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2RAWSTAT_1`"]
    #[inline(always)]
    pub fn is_audioint2rawstat_1(&self) -> bool {
        *self == AUDIOINT2RAWSTAT_A::AUDIOINT2RAWSTAT_1
    }
}
#[doc = "Audio channel 1 interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT1RAWSTAT_A {
    #[doc = "0: No Audio interrupt"]
    AUDIOINT1RAWSTAT_0 = 0,
    #[doc = "1: An Audio interrupt is pending."]
    AUDIOINT1RAWSTAT_1 = 1,
}
impl From<AUDIOINT1RAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT1RAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT1RAWSTAT`"]
pub type AUDIOINT1RAWSTAT_R = crate::R<bool, AUDIOINT1RAWSTAT_A>;
impl AUDIOINT1RAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT1RAWSTAT_A {
        match self.bits {
            false => AUDIOINT1RAWSTAT_A::AUDIOINT1RAWSTAT_0,
            true => AUDIOINT1RAWSTAT_A::AUDIOINT1RAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1RAWSTAT_0`"]
    #[inline(always)]
    pub fn is_audioint1rawstat_0(&self) -> bool {
        *self == AUDIOINT1RAWSTAT_A::AUDIOINT1RAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1RAWSTAT_1`"]
    #[inline(always)]
    pub fn is_audioint1rawstat_1(&self) -> bool {
        *self == AUDIOINT1RAWSTAT_A::AUDIOINT1RAWSTAT_1
    }
}
#[doc = "Audio channel 0 interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT0RAWSTAT_A {
    #[doc = "0: No Audio interrupt"]
    AUDIOINT0RAWSTAT_0 = 0,
    #[doc = "1: An Audio interrupt is pending."]
    AUDIOINT0RAWSTAT_1 = 1,
}
impl From<AUDIOINT0RAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT0RAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT0RAWSTAT`"]
pub type AUDIOINT0RAWSTAT_R = crate::R<bool, AUDIOINT0RAWSTAT_A>;
impl AUDIOINT0RAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT0RAWSTAT_A {
        match self.bits {
            false => AUDIOINT0RAWSTAT_A::AUDIOINT0RAWSTAT_0,
            true => AUDIOINT0RAWSTAT_A::AUDIOINT0RAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0RAWSTAT_0`"]
    #[inline(always)]
    pub fn is_audioint0rawstat_0(&self) -> bool {
        *self == AUDIOINT0RAWSTAT_A::AUDIOINT0RAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0RAWSTAT_1`"]
    #[inline(always)]
    pub fn is_audioint0rawstat_1(&self) -> bool {
        *self == AUDIOINT0RAWSTAT_A::AUDIOINT0RAWSTAT_1
    }
}
#[doc = "SW triggered interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINTRAWSTAT_A {
    #[doc = "0: No SW triggered interrupt"]
    SWINTRAWSTAT_0 = 0,
    #[doc = "1: A SW triggered interrupt is pending"]
    SWINTRAWSTAT_1 = 1,
}
impl From<SWINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: SWINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWINTRAWSTAT`"]
pub type SWINTRAWSTAT_R = crate::R<bool, SWINTRAWSTAT_A>;
impl SWINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWINTRAWSTAT_A {
        match self.bits {
            false => SWINTRAWSTAT_A::SWINTRAWSTAT_0,
            true => SWINTRAWSTAT_A::SWINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_swintrawstat_0(&self) -> bool {
        *self == SWINTRAWSTAT_A::SWINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `SWINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_swintrawstat_1(&self) -> bool {
        *self == SWINTRAWSTAT_A::SWINTRAWSTAT_1
    }
}
#[doc = "End of event / anticipated pre-fetch abort interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTAPFAINTRAWSTAT_A {
    #[doc = "0: No end of event interrupt"]
    EVENTAPFAINTRAWSTAT_0 = 0,
    #[doc = "1: An end of event interrupt is pending"]
    EVENTAPFAINTRAWSTAT_1 = 1,
}
impl From<EVENTAPFAINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTAPFAINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTAPFAINTRAWSTAT`"]
pub type EVENTAPFAINTRAWSTAT_R = crate::R<bool, EVENTAPFAINTRAWSTAT_A>;
impl EVENTAPFAINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTAPFAINTRAWSTAT_A {
        match self.bits {
            false => EVENTAPFAINTRAWSTAT_A::EVENTAPFAINTRAWSTAT_0,
            true => EVENTAPFAINTRAWSTAT_A::EVENTAPFAINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTAPFAINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_eventapfaintrawstat_0(&self) -> bool {
        *self == EVENTAPFAINTRAWSTAT_A::EVENTAPFAINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `EVENTAPFAINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_eventapfaintrawstat_1(&self) -> bool {
        *self == EVENTAPFAINTRAWSTAT_A::EVENTAPFAINTRAWSTAT_1
    }
}
#[doc = "Masked fine target timer error interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINETGTIMINTRAWSTAT_A {
    #[doc = "0: No fine target timer interrupt"]
    FINETGTIMINTRAWSTAT_0 = 0,
    #[doc = "1: A fine target timer interrupt is pending"]
    FINETGTIMINTRAWSTAT_1 = 1,
}
impl From<FINETGTIMINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: FINETGTIMINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINETGTIMINTRAWSTAT`"]
pub type FINETGTIMINTRAWSTAT_R = crate::R<bool, FINETGTIMINTRAWSTAT_A>;
impl FINETGTIMINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINETGTIMINTRAWSTAT_A {
        match self.bits {
            false => FINETGTIMINTRAWSTAT_A::FINETGTIMINTRAWSTAT_0,
            true => FINETGTIMINTRAWSTAT_A::FINETGTIMINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FINETGTIMINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_finetgtimintrawstat_0(&self) -> bool {
        *self == FINETGTIMINTRAWSTAT_A::FINETGTIMINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `FINETGTIMINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_finetgtimintrawstat_1(&self) -> bool {
        *self == FINETGTIMINTRAWSTAT_A::FINETGTIMINTRAWSTAT_1
    }
}
#[doc = "Masked gross target timer interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROSSTGTIMINTRAWSTAT_A {
    #[doc = "0: No gross target timer interrupt"]
    GROSSTGTIMINTRAWSTAT_0 = 0,
    #[doc = "1: A gross target timer interrupt is pending"]
    GROSSTGTIMINTRAWSTAT_1 = 1,
}
impl From<GROSSTGTIMINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: GROSSTGTIMINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GROSSTGTIMINTRAWSTAT`"]
pub type GROSSTGTIMINTRAWSTAT_R = crate::R<bool, GROSSTGTIMINTRAWSTAT_A>;
impl GROSSTGTIMINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROSSTGTIMINTRAWSTAT_A {
        match self.bits {
            false => GROSSTGTIMINTRAWSTAT_A::GROSSTGTIMINTRAWSTAT_0,
            true => GROSSTGTIMINTRAWSTAT_A::GROSSTGTIMINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `GROSSTGTIMINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_grosstgtimintrawstat_0(&self) -> bool {
        *self == GROSSTGTIMINTRAWSTAT_A::GROSSTGTIMINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `GROSSTGTIMINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_grosstgtimintrawstat_1(&self) -> bool {
        *self == GROSSTGTIMINTRAWSTAT_A::GROSSTGTIMINTRAWSTAT_1
    }
}
#[doc = "Masked error interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORINTRAWSTAT_A {
    #[doc = "0: No error interrupt"]
    ERRORINTRAWSTAT_0 = 0,
    #[doc = "1: An error interrupt is pending"]
    ERRORINTRAWSTAT_1 = 1,
}
impl From<ERRORINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRORINTRAWSTAT`"]
pub type ERRORINTRAWSTAT_R = crate::R<bool, ERRORINTRAWSTAT_A>;
impl ERRORINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORINTRAWSTAT_A {
        match self.bits {
            false => ERRORINTRAWSTAT_A::ERRORINTRAWSTAT_0,
            true => ERRORINTRAWSTAT_A::ERRORINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRORINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_errorintrawstat_0(&self) -> bool {
        *self == ERRORINTRAWSTAT_A::ERRORINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `ERRORINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_errorintrawstat_1(&self) -> bool {
        *self == ERRORINTRAWSTAT_A::ERRORINTRAWSTAT_1
    }
}
#[doc = "Masked encryption engine interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTINTRAWSTAT_A {
    #[doc = "0: No encryption / decryption interrupt"]
    CRYPTINTRAWSTAT_0 = 0,
    #[doc = "1: An encryption / decryption interrupt is pending"]
    CRYPTINTRAWSTAT_1 = 1,
}
impl From<CRYPTINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTINTRAWSTAT`"]
pub type CRYPTINTRAWSTAT_R = crate::R<bool, CRYPTINTRAWSTAT_A>;
impl CRYPTINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTINTRAWSTAT_A {
        match self.bits {
            false => CRYPTINTRAWSTAT_A::CRYPTINTRAWSTAT_0,
            true => CRYPTINTRAWSTAT_A::CRYPTINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRYPTINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_cryptintrawstat_0(&self) -> bool {
        *self == CRYPTINTRAWSTAT_A::CRYPTINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `CRYPTINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_cryptintrawstat_1(&self) -> bool {
        *self == CRYPTINTRAWSTAT_A::CRYPTINTRAWSTAT_1
    }
}
#[doc = "Masked end of event interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTINTRAWSTAT_A {
    #[doc = "0: No end of advertising / scanning / connection interrupt"]
    EVENTINTRAWSTAT_0 = 0,
    #[doc = "1: An end of advertising / scanning / connection interrupt is pending"]
    EVENTINTRAWSTAT_1 = 1,
}
impl From<EVENTINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTINTRAWSTAT`"]
pub type EVENTINTRAWSTAT_R = crate::R<bool, EVENTINTRAWSTAT_A>;
impl EVENTINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTINTRAWSTAT_A {
        match self.bits {
            false => EVENTINTRAWSTAT_A::EVENTINTRAWSTAT_0,
            true => EVENTINTRAWSTAT_A::EVENTINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_eventintrawstat_0(&self) -> bool {
        *self == EVENTINTRAWSTAT_A::EVENTINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `EVENTINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_eventintrawstat_1(&self) -> bool {
        *self == EVENTINTRAWSTAT_A::EVENTINTRAWSTAT_1
    }
}
#[doc = "Masked sleep interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPINTRAWSTAT_A {
    #[doc = "0: No end of sleep mode interrupt"]
    SLPINTRAWSTAT_0 = 0,
    #[doc = "1: An end of sleep mode interrupt is pending"]
    SLPINTRAWSTAT_1 = 1,
}
impl From<SLPINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: SLPINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLPINTRAWSTAT`"]
pub type SLPINTRAWSTAT_R = crate::R<bool, SLPINTRAWSTAT_A>;
impl SLPINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPINTRAWSTAT_A {
        match self.bits {
            false => SLPINTRAWSTAT_A::SLPINTRAWSTAT_0,
            true => SLPINTRAWSTAT_A::SLPINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLPINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_slpintrawstat_0(&self) -> bool {
        *self == SLPINTRAWSTAT_A::SLPINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `SLPINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_slpintrawstat_1(&self) -> bool {
        *self == SLPINTRAWSTAT_A::SLPINTRAWSTAT_1
    }
}
#[doc = "Masked packet reception interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINTRAWSTAT_A {
    #[doc = "0: No Rx interrupt"]
    RXINTRAWSTAT_0 = 0,
    #[doc = "1: An Rx interrupt is pending"]
    RXINTRAWSTAT_1 = 1,
}
impl From<RXINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: RXINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXINTRAWSTAT`"]
pub type RXINTRAWSTAT_R = crate::R<bool, RXINTRAWSTAT_A>;
impl RXINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINTRAWSTAT_A {
        match self.bits {
            false => RXINTRAWSTAT_A::RXINTRAWSTAT_0,
            true => RXINTRAWSTAT_A::RXINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_rxintrawstat_0(&self) -> bool {
        *self == RXINTRAWSTAT_A::RXINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `RXINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_rxintrawstat_1(&self) -> bool {
        *self == RXINTRAWSTAT_A::RXINTRAWSTAT_1
    }
}
#[doc = "Masked 625us base time reference interrupt raw status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCNTINTRAWSTAT_A {
    #[doc = "0: No 625us base time interrupt"]
    CSCNTINTRAWSTAT_0 = 0,
    #[doc = "1: A 625us base time interrupt is pending"]
    CSCNTINTRAWSTAT_1 = 1,
}
impl From<CSCNTINTRAWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: CSCNTINTRAWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCNTINTRAWSTAT`"]
pub type CSCNTINTRAWSTAT_R = crate::R<bool, CSCNTINTRAWSTAT_A>;
impl CSCNTINTRAWSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCNTINTRAWSTAT_A {
        match self.bits {
            false => CSCNTINTRAWSTAT_A::CSCNTINTRAWSTAT_0,
            true => CSCNTINTRAWSTAT_A::CSCNTINTRAWSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSCNTINTRAWSTAT_0`"]
    #[inline(always)]
    pub fn is_cscntintrawstat_0(&self) -> bool {
        *self == CSCNTINTRAWSTAT_A::CSCNTINTRAWSTAT_0
    }
    #[doc = "Checks if the value of the field is `CSCNTINTRAWSTAT_1`"]
    #[inline(always)]
    pub fn is_cscntintrawstat_1(&self) -> bool {
        *self == CSCNTINTRAWSTAT_A::CSCNTINTRAWSTAT_1
    }
}
impl R {
    #[doc = "Bit 12 - Audio channel 2 interrupt raw status"]
    #[inline(always)]
    pub fn audioint2rawstat(&self) -> AUDIOINT2RAWSTAT_R {
        AUDIOINT2RAWSTAT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Audio channel 1 interrupt raw status"]
    #[inline(always)]
    pub fn audioint1rawstat(&self) -> AUDIOINT1RAWSTAT_R {
        AUDIOINT1RAWSTAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Audio channel 0 interrupt raw status"]
    #[inline(always)]
    pub fn audioint0rawstat(&self) -> AUDIOINT0RAWSTAT_R {
        AUDIOINT0RAWSTAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SW triggered interrupt raw status"]
    #[inline(always)]
    pub fn swintrawstat(&self) -> SWINTRAWSTAT_R {
        SWINTRAWSTAT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt raw status"]
    #[inline(always)]
    pub fn eventapfaintrawstat(&self) -> EVENTAPFAINTRAWSTAT_R {
        EVENTAPFAINTRAWSTAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Masked fine target timer error interrupt raw status"]
    #[inline(always)]
    pub fn finetgtimintrawstat(&self) -> FINETGTIMINTRAWSTAT_R {
        FINETGTIMINTRAWSTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Masked gross target timer interrupt raw status"]
    #[inline(always)]
    pub fn grosstgtimintrawstat(&self) -> GROSSTGTIMINTRAWSTAT_R {
        GROSSTGTIMINTRAWSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Masked error interrupt raw status"]
    #[inline(always)]
    pub fn errorintrawstat(&self) -> ERRORINTRAWSTAT_R {
        ERRORINTRAWSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Masked encryption engine interrupt raw status"]
    #[inline(always)]
    pub fn cryptintrawstat(&self) -> CRYPTINTRAWSTAT_R {
        CRYPTINTRAWSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Masked end of event interrupt raw status"]
    #[inline(always)]
    pub fn eventintrawstat(&self) -> EVENTINTRAWSTAT_R {
        EVENTINTRAWSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Masked sleep interrupt raw status"]
    #[inline(always)]
    pub fn slpintrawstat(&self) -> SLPINTRAWSTAT_R {
        SLPINTRAWSTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Masked packet reception interrupt raw status"]
    #[inline(always)]
    pub fn rxintrawstat(&self) -> RXINTRAWSTAT_R {
        RXINTRAWSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Masked 625us base time reference interrupt raw status"]
    #[inline(always)]
    pub fn cscntintrawstat(&self) -> CSCNTINTRAWSTAT_R {
        CSCNTINTRAWSTAT_R::new((self.bits & 0x01) != 0)
    }
}
