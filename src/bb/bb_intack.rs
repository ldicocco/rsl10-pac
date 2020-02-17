#[doc = "Reader of register BB_INTACK"]
pub type R = crate::R<u32, super::BB_INTACK>;
#[doc = "Writer for register BB_INTACK"]
pub type W = crate::W<u32, super::BB_INTACK>;
#[doc = "Register BB_INTACK `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_INTACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Audio channel 2 interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT2ACK_A {
    #[doc = "0: `0`"]
    AUDIOINT2ACK_0 = 0,
    #[doc = "1: Acknowledges the Audio channel 2 interrupt. This bit resets AUDIOINT2STAT and AUDIOINT2RAWSTAT flags."]
    AUDIOINT2ACK_1 = 1,
}
impl From<AUDIOINT2ACK_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT2ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT2ACK`"]
pub type AUDIOINT2ACK_R = crate::R<bool, AUDIOINT2ACK_A>;
impl AUDIOINT2ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT2ACK_A {
        match self.bits {
            false => AUDIOINT2ACK_A::AUDIOINT2ACK_0,
            true => AUDIOINT2ACK_A::AUDIOINT2ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2ACK_0`"]
    #[inline(always)]
    pub fn is_audioint2ack_0(&self) -> bool {
        *self == AUDIOINT2ACK_A::AUDIOINT2ACK_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT2ACK_1`"]
    #[inline(always)]
    pub fn is_audioint2ack_1(&self) -> bool {
        *self == AUDIOINT2ACK_A::AUDIOINT2ACK_1
    }
}
#[doc = "Audio channel 1 interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT1ACK_A {
    #[doc = "0: `0`"]
    AUDIOINT1ACK_0 = 0,
    #[doc = "1: Acknowledges the Audio channel 1 interrupt. This bit resets AUDIOINT2STAT and AUDIOINT2RAWSTAT flags."]
    AUDIOINT1ACK_1 = 1,
}
impl From<AUDIOINT1ACK_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT1ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT1ACK`"]
pub type AUDIOINT1ACK_R = crate::R<bool, AUDIOINT1ACK_A>;
impl AUDIOINT1ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT1ACK_A {
        match self.bits {
            false => AUDIOINT1ACK_A::AUDIOINT1ACK_0,
            true => AUDIOINT1ACK_A::AUDIOINT1ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1ACK_0`"]
    #[inline(always)]
    pub fn is_audioint1ack_0(&self) -> bool {
        *self == AUDIOINT1ACK_A::AUDIOINT1ACK_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT1ACK_1`"]
    #[inline(always)]
    pub fn is_audioint1ack_1(&self) -> bool {
        *self == AUDIOINT1ACK_A::AUDIOINT1ACK_1
    }
}
#[doc = "Audio channel 0 interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIOINT0ACK_A {
    #[doc = "0: `0`"]
    AUDIOINT0ACK_0 = 0,
    #[doc = "1: Acknowledges the Audio channel 0 interrupt. This bit resets AUDIOINT2STAT and AUDIOINT2RAWSTAT flags."]
    AUDIOINT0ACK_1 = 1,
}
impl From<AUDIOINT0ACK_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIOINT0ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUDIOINT0ACK`"]
pub type AUDIOINT0ACK_R = crate::R<bool, AUDIOINT0ACK_A>;
impl AUDIOINT0ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOINT0ACK_A {
        match self.bits {
            false => AUDIOINT0ACK_A::AUDIOINT0ACK_0,
            true => AUDIOINT0ACK_A::AUDIOINT0ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0ACK_0`"]
    #[inline(always)]
    pub fn is_audioint0ack_0(&self) -> bool {
        *self == AUDIOINT0ACK_A::AUDIOINT0ACK_0
    }
    #[doc = "Checks if the value of the field is `AUDIOINT0ACK_1`"]
    #[inline(always)]
    pub fn is_audioint0ack_1(&self) -> bool {
        *self == AUDIOINT0ACK_A::AUDIOINT0ACK_1
    }
}
#[doc = "SW triggered interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINTACK_AW {
    #[doc = "0: `0`"]
    SWINTACK_0 = 0,
    #[doc = "1: Acknowledges the SW triggered interrupt. This bit resets SWINTSTAT and SWINTRAWSTAT flags."]
    SWINTACK_1 = 1,
}
impl From<SWINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: SWINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWINTACK`"]
pub struct SWINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn swintack_0(self) -> &'a mut W {
        self.variant(SWINTACK_AW::SWINTACK_0)
    }
    #[doc = "Acknowledges the SW triggered interrupt. This bit resets SWINTSTAT and SWINTRAWSTAT flags."]
    #[inline(always)]
    pub fn swintack_1(self) -> &'a mut W {
        self.variant(SWINTACK_AW::SWINTACK_1)
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
#[doc = "End of event / anticipated pre-fetch abort interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTAPFAINTACK_AW {
    #[doc = "0: `0`"]
    EVENTAPFAINTACK_0 = 0,
    #[doc = "1: Acknowledges the end of event / anticipated pre-fetch abort interrupt. This bit resets EVENTAPFAINTSTAT and EVENTAPFAINTRAWSTAT flags."]
    EVENTAPFAINTACK_1 = 1,
}
impl From<EVENTAPFAINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: EVENTAPFAINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EVENTAPFAINTACK`"]
pub struct EVENTAPFAINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTAPFAINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTAPFAINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn eventapfaintack_0(self) -> &'a mut W {
        self.variant(EVENTAPFAINTACK_AW::EVENTAPFAINTACK_0)
    }
    #[doc = "Acknowledges the end of event / anticipated pre-fetch abort interrupt. This bit resets EVENTAPFAINTSTAT and EVENTAPFAINTRAWSTAT flags."]
    #[inline(always)]
    pub fn eventapfaintack_1(self) -> &'a mut W {
        self.variant(EVENTAPFAINTACK_AW::EVENTAPFAINTACK_1)
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
#[doc = "Fine target timer interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINETGTIMINTACK_AW {
    #[doc = "0: `0`"]
    FINETGTIMINTACK_0 = 0,
    #[doc = "1: Acknowledges the fine timer interrupt. This bit resets FINETGTIMINTSTAT and FINETGTIMINTRAWSTAT flags"]
    FINETGTIMINTACK_1 = 1,
}
impl From<FINETGTIMINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: FINETGTIMINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FINETGTIMINTACK`"]
pub struct FINETGTIMINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETGTIMINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINETGTIMINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn finetgtimintack_0(self) -> &'a mut W {
        self.variant(FINETGTIMINTACK_AW::FINETGTIMINTACK_0)
    }
    #[doc = "Acknowledges the fine timer interrupt. This bit resets FINETGTIMINTSTAT and FINETGTIMINTRAWSTAT flags"]
    #[inline(always)]
    pub fn finetgtimintack_1(self) -> &'a mut W {
        self.variant(FINETGTIMINTACK_AW::FINETGTIMINTACK_1)
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
#[doc = "Gross target timer interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROSSTGTIMINTACK_AW {
    #[doc = "0: `0`"]
    GROSSTGTIMINTACK_0 = 0,
    #[doc = "1: Acknowledges the gross timer interrupt. This bit resets GROSSTGTIMINTSTAT and GROSSTGTIMINTRAWSTAT flags"]
    GROSSTGTIMINTACK_1 = 1,
}
impl From<GROSSTGTIMINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: GROSSTGTIMINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `GROSSTGTIMINTACK`"]
pub struct GROSSTGTIMINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> GROSSTGTIMINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROSSTGTIMINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn grosstgtimintack_0(self) -> &'a mut W {
        self.variant(GROSSTGTIMINTACK_AW::GROSSTGTIMINTACK_0)
    }
    #[doc = "Acknowledges the gross timer interrupt. This bit resets GROSSTGTIMINTSTAT and GROSSTGTIMINTRAWSTAT flags"]
    #[inline(always)]
    pub fn grosstgtimintack_1(self) -> &'a mut W {
        self.variant(GROSSTGTIMINTACK_AW::GROSSTGTIMINTACK_1)
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
#[doc = "Error interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORINTACK_AW {
    #[doc = "0: `0`"]
    ERRORINTACK_0 = 0,
    #[doc = "1: Acknowledges the error interrupt. This bit resets ERRORINTSTAT and ERRORINTRAWSTAT flags"]
    ERRORINTACK_1 = 1,
}
impl From<ERRORINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRORINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERRORINTACK`"]
pub struct ERRORINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRORINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn errorintack_0(self) -> &'a mut W {
        self.variant(ERRORINTACK_AW::ERRORINTACK_0)
    }
    #[doc = "Acknowledges the error interrupt. This bit resets ERRORINTSTAT and ERRORINTRAWSTAT flags"]
    #[inline(always)]
    pub fn errorintack_1(self) -> &'a mut W {
        self.variant(ERRORINTACK_AW::ERRORINTACK_1)
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
#[doc = "Encryption engine interrupt acknowledgement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTINTACK_AW {
    #[doc = "0: `0`"]
    CRYPTINTACK_0 = 0,
    #[doc = "1: Acknowledges the encryption engine interrupt. This bit resets CRYPTINTSTAT and CRYPTINTRAWSTAT flags"]
    CRYPTINTACK_1 = 1,
}
impl From<CRYPTINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: CRYPTINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CRYPTINTACK`"]
pub struct CRYPTINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cryptintack_0(self) -> &'a mut W {
        self.variant(CRYPTINTACK_AW::CRYPTINTACK_0)
    }
    #[doc = "Acknowledges the encryption engine interrupt. This bit resets CRYPTINTSTAT and CRYPTINTRAWSTAT flags"]
    #[inline(always)]
    pub fn cryptintack_1(self) -> &'a mut W {
        self.variant(CRYPTINTACK_AW::CRYPTINTACK_1)
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
#[doc = "End of event interrupt acknowledgment bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTINTACK_AW {
    #[doc = "0: `0`"]
    EVENTINTACK_0 = 0,
    #[doc = "1: Acknowledges the end of advertising / scanning / connection interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags"]
    EVENTINTACK_1 = 1,
}
impl From<EVENTINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: EVENTINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EVENTINTACK`"]
pub struct EVENTINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn eventintack_0(self) -> &'a mut W {
        self.variant(EVENTINTACK_AW::EVENTINTACK_0)
    }
    #[doc = "Acknowledges the end of advertising / scanning / connection interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags"]
    #[inline(always)]
    pub fn eventintack_1(self) -> &'a mut W {
        self.variant(EVENTINTACK_AW::EVENTINTACK_1)
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
#[doc = "End of deep sleep interrupt acknowledgment bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPINTACK_AW {
    #[doc = "0: `0`"]
    SLPINTACK_0 = 0,
    #[doc = "1: Acknowledges the end of sleep mode interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags"]
    SLPINTACK_1 = 1,
}
impl From<SLPINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: SLPINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SLPINTACK`"]
pub struct SLPINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLPINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slpintack_0(self) -> &'a mut W {
        self.variant(SLPINTACK_AW::SLPINTACK_0)
    }
    #[doc = "Acknowledges the end of sleep mode interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags"]
    #[inline(always)]
    pub fn slpintack_1(self) -> &'a mut W {
        self.variant(SLPINTACK_AW::SLPINTACK_1)
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
#[doc = "Packet reception interrupt acknowledgment bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINTACK_AW {
    #[doc = "0: `0`"]
    RXINTACK_0 = 0,
    #[doc = "1: Acknowledges the Rx interrupt. This bit resets RXINTSTAT and RXINTRAWSTAT flags"]
    RXINTACK_1 = 1,
}
impl From<RXINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: RXINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXINTACK`"]
pub struct RXINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxintack_0(self) -> &'a mut W {
        self.variant(RXINTACK_AW::RXINTACK_0)
    }
    #[doc = "Acknowledges the Rx interrupt. This bit resets RXINTSTAT and RXINTRAWSTAT flags"]
    #[inline(always)]
    pub fn rxintack_1(self) -> &'a mut W {
        self.variant(RXINTACK_AW::RXINTACK_1)
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
#[doc = "625us base time reference interrupt acknowledgment bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCNTINTACK_AW {
    #[doc = "0: `0`"]
    CSCNTINTACK_0 = 0,
    #[doc = "1: Acknowledges the CLKN interrupt. This bit resets CLKINTSTAT and CLKINTRAWSTAT flags"]
    CSCNTINTACK_1 = 1,
}
impl From<CSCNTINTACK_AW> for bool {
    #[inline(always)]
    fn from(variant: CSCNTINTACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CSCNTINTACK`"]
pub struct CSCNTINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNTINTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCNTINTACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cscntintack_0(self) -> &'a mut W {
        self.variant(CSCNTINTACK_AW::CSCNTINTACK_0)
    }
    #[doc = "Acknowledges the CLKN interrupt. This bit resets CLKINTSTAT and CLKINTRAWSTAT flags"]
    #[inline(always)]
    pub fn cscntintack_1(self) -> &'a mut W {
        self.variant(CSCNTINTACK_AW::CSCNTINTACK_1)
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
    #[doc = "Bit 12 - Audio channel 2 interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn audioint2ack(&self) -> AUDIOINT2ACK_R {
        AUDIOINT2ACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Audio channel 1 interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn audioint1ack(&self) -> AUDIOINT1ACK_R {
        AUDIOINT1ACK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Audio channel 0 interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn audioint0ack(&self) -> AUDIOINT0ACK_R {
        AUDIOINT0ACK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - SW triggered interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn swintack(&mut self) -> SWINTACK_W {
        SWINTACK_W { w: self }
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn eventapfaintack(&mut self) -> EVENTAPFAINTACK_W {
        EVENTAPFAINTACK_W { w: self }
    }
    #[doc = "Bit 7 - Fine target timer interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn finetgtimintack(&mut self) -> FINETGTIMINTACK_W {
        FINETGTIMINTACK_W { w: self }
    }
    #[doc = "Bit 6 - Gross target timer interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn grosstgtimintack(&mut self) -> GROSSTGTIMINTACK_W {
        GROSSTGTIMINTACK_W { w: self }
    }
    #[doc = "Bit 5 - Error interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn errorintack(&mut self) -> ERRORINTACK_W {
        ERRORINTACK_W { w: self }
    }
    #[doc = "Bit 4 - Encryption engine interrupt acknowledgement bit"]
    #[inline(always)]
    pub fn cryptintack(&mut self) -> CRYPTINTACK_W {
        CRYPTINTACK_W { w: self }
    }
    #[doc = "Bit 3 - End of event interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn eventintack(&mut self) -> EVENTINTACK_W {
        EVENTINTACK_W { w: self }
    }
    #[doc = "Bit 2 - End of deep sleep interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn slpintack(&mut self) -> SLPINTACK_W {
        SLPINTACK_W { w: self }
    }
    #[doc = "Bit 1 - Packet reception interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn rxintack(&mut self) -> RXINTACK_W {
        RXINTACK_W { w: self }
    }
    #[doc = "Bit 0 - 625us base time reference interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn cscntintack(&mut self) -> CSCNTINTACK_W {
        CSCNTINTACK_W { w: self }
    }
}
