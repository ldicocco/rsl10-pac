#[doc = "Reader of register DIO_DATA"]
pub type R = crate::R<u32, super::DIO_DATA>;
#[doc = "Writer for register DIO_DATA"]
pub type W = crate::W<u32, super::DIO_DATA>;
#[doc = "Register DIO_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DIO_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DIO\\[15:0\\]
read data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DIO_A {
    #[doc = "0: DIO pad is low"]
    DIO0_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO1_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO2_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO3_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO4_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO5_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO6_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO7_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO8_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO9_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO10_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO11_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO12_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO13_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO14_LOW = 0,
    #[doc = "0: DIO pad is low"]
    DIO15_LOW = 0,
    #[doc = "1: DIO pad is high"]
    DIO0_HIGH = 1,
    #[doc = "2: DIO pad is high"]
    DIO1_HIGH = 2,
    #[doc = "4: DIO pad is high"]
    DIO2_HIGH = 4,
    #[doc = "8: DIO pad is high"]
    DIO3_HIGH = 8,
    #[doc = "16: DIO pad is high"]
    DIO4_HIGH = 16,
    #[doc = "32: DIO pad is high"]
    DIO5_HIGH = 32,
    #[doc = "64: DIO pad is high"]
    DIO6_HIGH = 64,
    #[doc = "128: DIO pad is high"]
    DIO7_HIGH = 128,
    #[doc = "256: DIO pad is high"]
    DIO8_HIGH = 256,
    #[doc = "512: DIO pad is high"]
    DIO9_HIGH = 512,
    #[doc = "1024: DIO pad is high"]
    DIO10_HIGH = 1024,
    #[doc = "2048: DIO pad is high"]
    DIO11_HIGH = 2048,
    #[doc = "4096: DIO pad is high"]
    DIO12_HIGH = 4096,
    #[doc = "8192: DIO pad is high"]
    DIO13_HIGH = 8192,
    #[doc = "16384: DIO pad is high"]
    DIO14_HIGH = 16384,
    #[doc = "32768: DIO pad is high"]
    DIO15_HIGH = 32768,
}
impl From<DIO_A> for u16 {
    #[inline(always)]
    fn from(variant: DIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIO`"]
pub type DIO_R = crate::R<u16, DIO_A>;
impl DIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, DIO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIO_A::DIO0_LOW),
            0 => Val(DIO_A::DIO1_LOW),
            0 => Val(DIO_A::DIO2_LOW),
            0 => Val(DIO_A::DIO3_LOW),
            0 => Val(DIO_A::DIO4_LOW),
            0 => Val(DIO_A::DIO5_LOW),
            0 => Val(DIO_A::DIO6_LOW),
            0 => Val(DIO_A::DIO7_LOW),
            0 => Val(DIO_A::DIO8_LOW),
            0 => Val(DIO_A::DIO9_LOW),
            0 => Val(DIO_A::DIO10_LOW),
            0 => Val(DIO_A::DIO11_LOW),
            0 => Val(DIO_A::DIO12_LOW),
            0 => Val(DIO_A::DIO13_LOW),
            0 => Val(DIO_A::DIO14_LOW),
            0 => Val(DIO_A::DIO15_LOW),
            1 => Val(DIO_A::DIO0_HIGH),
            2 => Val(DIO_A::DIO1_HIGH),
            4 => Val(DIO_A::DIO2_HIGH),
            8 => Val(DIO_A::DIO3_HIGH),
            16 => Val(DIO_A::DIO4_HIGH),
            32 => Val(DIO_A::DIO5_HIGH),
            64 => Val(DIO_A::DIO6_HIGH),
            128 => Val(DIO_A::DIO7_HIGH),
            256 => Val(DIO_A::DIO8_HIGH),
            512 => Val(DIO_A::DIO9_HIGH),
            1024 => Val(DIO_A::DIO10_HIGH),
            2048 => Val(DIO_A::DIO11_HIGH),
            4096 => Val(DIO_A::DIO12_HIGH),
            8192 => Val(DIO_A::DIO13_HIGH),
            16384 => Val(DIO_A::DIO14_HIGH),
            32768 => Val(DIO_A::DIO15_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIO0_LOW`"]
    #[inline(always)]
    pub fn is_dio0_low(&self) -> bool {
        *self == DIO_A::DIO0_LOW
    }
    #[doc = "Checks if the value of the field is `DIO1_LOW`"]
    #[inline(always)]
    pub fn is_dio1_low(&self) -> bool {
        *self == DIO_A::DIO1_LOW
    }
    #[doc = "Checks if the value of the field is `DIO2_LOW`"]
    #[inline(always)]
    pub fn is_dio2_low(&self) -> bool {
        *self == DIO_A::DIO2_LOW
    }
    #[doc = "Checks if the value of the field is `DIO3_LOW`"]
    #[inline(always)]
    pub fn is_dio3_low(&self) -> bool {
        *self == DIO_A::DIO3_LOW
    }
    #[doc = "Checks if the value of the field is `DIO4_LOW`"]
    #[inline(always)]
    pub fn is_dio4_low(&self) -> bool {
        *self == DIO_A::DIO4_LOW
    }
    #[doc = "Checks if the value of the field is `DIO5_LOW`"]
    #[inline(always)]
    pub fn is_dio5_low(&self) -> bool {
        *self == DIO_A::DIO5_LOW
    }
    #[doc = "Checks if the value of the field is `DIO6_LOW`"]
    #[inline(always)]
    pub fn is_dio6_low(&self) -> bool {
        *self == DIO_A::DIO6_LOW
    }
    #[doc = "Checks if the value of the field is `DIO7_LOW`"]
    #[inline(always)]
    pub fn is_dio7_low(&self) -> bool {
        *self == DIO_A::DIO7_LOW
    }
    #[doc = "Checks if the value of the field is `DIO8_LOW`"]
    #[inline(always)]
    pub fn is_dio8_low(&self) -> bool {
        *self == DIO_A::DIO8_LOW
    }
    #[doc = "Checks if the value of the field is `DIO9_LOW`"]
    #[inline(always)]
    pub fn is_dio9_low(&self) -> bool {
        *self == DIO_A::DIO9_LOW
    }
    #[doc = "Checks if the value of the field is `DIO10_LOW`"]
    #[inline(always)]
    pub fn is_dio10_low(&self) -> bool {
        *self == DIO_A::DIO10_LOW
    }
    #[doc = "Checks if the value of the field is `DIO11_LOW`"]
    #[inline(always)]
    pub fn is_dio11_low(&self) -> bool {
        *self == DIO_A::DIO11_LOW
    }
    #[doc = "Checks if the value of the field is `DIO12_LOW`"]
    #[inline(always)]
    pub fn is_dio12_low(&self) -> bool {
        *self == DIO_A::DIO12_LOW
    }
    #[doc = "Checks if the value of the field is `DIO13_LOW`"]
    #[inline(always)]
    pub fn is_dio13_low(&self) -> bool {
        *self == DIO_A::DIO13_LOW
    }
    #[doc = "Checks if the value of the field is `DIO14_LOW`"]
    #[inline(always)]
    pub fn is_dio14_low(&self) -> bool {
        *self == DIO_A::DIO14_LOW
    }
    #[doc = "Checks if the value of the field is `DIO15_LOW`"]
    #[inline(always)]
    pub fn is_dio15_low(&self) -> bool {
        *self == DIO_A::DIO15_LOW
    }
    #[doc = "Checks if the value of the field is `DIO0_HIGH`"]
    #[inline(always)]
    pub fn is_dio0_high(&self) -> bool {
        *self == DIO_A::DIO0_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO1_HIGH`"]
    #[inline(always)]
    pub fn is_dio1_high(&self) -> bool {
        *self == DIO_A::DIO1_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO2_HIGH`"]
    #[inline(always)]
    pub fn is_dio2_high(&self) -> bool {
        *self == DIO_A::DIO2_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO3_HIGH`"]
    #[inline(always)]
    pub fn is_dio3_high(&self) -> bool {
        *self == DIO_A::DIO3_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO4_HIGH`"]
    #[inline(always)]
    pub fn is_dio4_high(&self) -> bool {
        *self == DIO_A::DIO4_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO5_HIGH`"]
    #[inline(always)]
    pub fn is_dio5_high(&self) -> bool {
        *self == DIO_A::DIO5_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO6_HIGH`"]
    #[inline(always)]
    pub fn is_dio6_high(&self) -> bool {
        *self == DIO_A::DIO6_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO7_HIGH`"]
    #[inline(always)]
    pub fn is_dio7_high(&self) -> bool {
        *self == DIO_A::DIO7_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO8_HIGH`"]
    #[inline(always)]
    pub fn is_dio8_high(&self) -> bool {
        *self == DIO_A::DIO8_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO9_HIGH`"]
    #[inline(always)]
    pub fn is_dio9_high(&self) -> bool {
        *self == DIO_A::DIO9_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO10_HIGH`"]
    #[inline(always)]
    pub fn is_dio10_high(&self) -> bool {
        *self == DIO_A::DIO10_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO11_HIGH`"]
    #[inline(always)]
    pub fn is_dio11_high(&self) -> bool {
        *self == DIO_A::DIO11_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO12_HIGH`"]
    #[inline(always)]
    pub fn is_dio12_high(&self) -> bool {
        *self == DIO_A::DIO12_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO13_HIGH`"]
    #[inline(always)]
    pub fn is_dio13_high(&self) -> bool {
        *self == DIO_A::DIO13_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO14_HIGH`"]
    #[inline(always)]
    pub fn is_dio14_high(&self) -> bool {
        *self == DIO_A::DIO14_HIGH
    }
    #[doc = "Checks if the value of the field is `DIO15_HIGH`"]
    #[inline(always)]
    pub fn is_dio15_high(&self) -> bool {
        *self == DIO_A::DIO15_HIGH
    }
}
#[doc = "GPIO\\[15:0\\]
write data (updates output data of DIOs only for pads with IO_MODE 0b000XX)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GPIO_AW {
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO0_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO1_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO2_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO3_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO4_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO5_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO6_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO7_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO8_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO9_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO10_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO11_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO12_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO13_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO14_LOW = 0,
    #[doc = "0: Set the DIO pad to low if IO_MODE is 0b0000XX"]
    GPIO15_LOW = 0,
    #[doc = "1: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO0_HIGH = 1,
    #[doc = "2: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO1_HIGH = 2,
    #[doc = "4: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO2_HIGH = 4,
    #[doc = "8: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO3_HIGH = 8,
    #[doc = "16: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO4_HIGH = 16,
    #[doc = "32: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO5_HIGH = 32,
    #[doc = "64: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO6_HIGH = 64,
    #[doc = "128: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO7_HIGH = 128,
    #[doc = "256: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO8_HIGH = 256,
    #[doc = "512: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO9_HIGH = 512,
    #[doc = "1024: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO10_HIGH = 1024,
    #[doc = "2048: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO11_HIGH = 2048,
    #[doc = "4096: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO12_HIGH = 4096,
    #[doc = "8192: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO13_HIGH = 8192,
    #[doc = "16384: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO14_HIGH = 16384,
    #[doc = "32768: Set the DIO pad to high if IO_MODE is 0b0000XX"]
    GPIO15_HIGH = 32768,
}
impl From<GPIO_AW> for u16 {
    #[inline(always)]
    fn from(variant: GPIO_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `GPIO`"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio0_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO0_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio1_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO1_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio2_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO2_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio3_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO3_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio4_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO4_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio5_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO5_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio6_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO6_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio7_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO7_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio8_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO8_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio9_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO9_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio10_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO10_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio11_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO11_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio12_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO12_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio13_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO13_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio14_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO14_LOW)
    }
    #[doc = "Set the DIO pad to low if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio15_low(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO15_LOW)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio0_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO0_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio1_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO1_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio2_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO2_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio3_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO3_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio4_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO4_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio5_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO5_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio6_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO6_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio7_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO7_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio8_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO8_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio9_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO9_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio10_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO10_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio11_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO11_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio12_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO12_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio13_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO13_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio14_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO14_HIGH)
    }
    #[doc = "Set the DIO pad to high if IO_MODE is 0b0000XX"]
    #[inline(always)]
    pub fn gpio15_high(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO15_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DIO\\[15:0\\]
read data"]
    #[inline(always)]
    pub fn dio(&self) -> DIO_R {
        DIO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO\\[15:0\\]
write data (updates output data of DIOs only for pads with IO_MODE 0b000XX)"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
}
