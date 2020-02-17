#[doc = "Reader of register DIO_INT_DEBOUNCE"]
pub type R = crate::R<u32, super::DIO_INT_DEBOUNCE>;
#[doc = "Writer for register DIO_INT_DEBOUNCE"]
pub type W = crate::W<u32, super::DIO_INT_DEBOUNCE>;
#[doc = "Register DIO_INT_DEBOUNCE `reset()`'s with value 0"]
impl crate::ResetValue for super::DIO_INT_DEBOUNCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt button debounce filter clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBOUNCE_CLK_A {
    #[doc = "0: Button debounce filter runs on SLOWCLK divided by 32"]
    DIO_DEBOUNCE_SLOWCLK_DIV32 = 0,
    #[doc = "1: Button debounce filter runs on SLOWCLK divided by 1024"]
    DIO_DEBOUNCE_SLOWCLK_DIV1024 = 1,
}
impl From<DEBOUNCE_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: DEBOUNCE_CLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEBOUNCE_CLK`"]
pub type DEBOUNCE_CLK_R = crate::R<bool, DEBOUNCE_CLK_A>;
impl DEBOUNCE_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBOUNCE_CLK_A {
        match self.bits {
            false => DEBOUNCE_CLK_A::DIO_DEBOUNCE_SLOWCLK_DIV32,
            true => DEBOUNCE_CLK_A::DIO_DEBOUNCE_SLOWCLK_DIV1024,
        }
    }
    #[doc = "Checks if the value of the field is `DIO_DEBOUNCE_SLOWCLK_DIV32`"]
    #[inline(always)]
    pub fn is_dio_debounce_slowclk_div32(&self) -> bool {
        *self == DEBOUNCE_CLK_A::DIO_DEBOUNCE_SLOWCLK_DIV32
    }
    #[doc = "Checks if the value of the field is `DIO_DEBOUNCE_SLOWCLK_DIV1024`"]
    #[inline(always)]
    pub fn is_dio_debounce_slowclk_div1024(&self) -> bool {
        *self == DEBOUNCE_CLK_A::DIO_DEBOUNCE_SLOWCLK_DIV1024
    }
}
#[doc = "Write proxy for field `DEBOUNCE_CLK`"]
pub struct DEBOUNCE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCE_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBOUNCE_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Button debounce filter runs on SLOWCLK divided by 32"]
    #[inline(always)]
    pub fn dio_debounce_slowclk_div32(self) -> &'a mut W {
        self.variant(DEBOUNCE_CLK_A::DIO_DEBOUNCE_SLOWCLK_DIV32)
    }
    #[doc = "Button debounce filter runs on SLOWCLK divided by 1024"]
    #[inline(always)]
    pub fn dio_debounce_slowclk_div1024(self) -> &'a mut W {
        self.variant(DEBOUNCE_CLK_A::DIO_DEBOUNCE_SLOWCLK_DIV1024)
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
#[doc = "Reader of field `DEBOUNCE_COUNT`"]
pub type DEBOUNCE_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBOUNCE_COUNT`"]
pub struct DEBOUNCE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Interrupt button debounce filter clock"]
    #[inline(always)]
    pub fn debounce_clk(&self) -> DEBOUNCE_CLK_R {
        DEBOUNCE_CLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Interrupt button debounce filter count"]
    #[inline(always)]
    pub fn debounce_count(&self) -> DEBOUNCE_COUNT_R {
        DEBOUNCE_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Interrupt button debounce filter clock"]
    #[inline(always)]
    pub fn debounce_clk(&mut self) -> DEBOUNCE_CLK_W {
        DEBOUNCE_CLK_W { w: self }
    }
    #[doc = "Bits 0:7 - Interrupt button debounce filter count"]
    #[inline(always)]
    pub fn debounce_count(&mut self) -> DEBOUNCE_COUNT_W {
        DEBOUNCE_COUNT_W { w: self }
    }
}
