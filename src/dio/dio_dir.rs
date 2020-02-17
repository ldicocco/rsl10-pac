#[doc = "Reader of register DIO_DIR"]
pub type R = crate::R<u32, super::DIO_DIR>;
#[doc = "Writer for register DIO_DIR"]
pub type W = crate::W<u32, super::DIO_DIR>;
#[doc = "Register DIO_DIR `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::DIO_DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Get DIO\\[15:0\\]
direction\n\nValue on reset: 32768"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DIO_A {
    #[doc = "0: DIO\\[15:0\\]
is an input"]
    DIO0_INPUT = 0,
    #[doc = "1: DIO\\[14:0\\]
is an output"]
    DIO0_OUTPUT = 1,
    #[doc = "2: DIO\\[14:0\\]
is an output"]
    DIO1_OUTPUT = 2,
    #[doc = "4: DIO\\[14:0\\]
is an output"]
    DIO2_OUTPUT = 4,
    #[doc = "8: DIO\\[14:0\\]
is an output"]
    DIO3_OUTPUT = 8,
    #[doc = "16: DIO\\[14:0\\]
is an output"]
    DIO4_OUTPUT = 16,
    #[doc = "32: DIO\\[14:0\\]
is an output"]
    DIO5_OUTPUT = 32,
    #[doc = "64: DIO\\[14:0\\]
is an output"]
    DIO6_OUTPUT = 64,
    #[doc = "128: DIO\\[14:0\\]
is an output"]
    DIO7_OUTPUT = 128,
    #[doc = "256: DIO\\[14:0\\]
is an output"]
    DIO8_OUTPUT = 256,
    #[doc = "512: DIO\\[14:0\\]
is an output"]
    DIO9_OUTPUT = 512,
    #[doc = "1024: DIO\\[14:0\\]
is an output"]
    DIO10_OUTPUT = 1024,
    #[doc = "2048: DIO\\[14:0\\]
is an output"]
    DIO11_OUTPUT = 2048,
    #[doc = "4096: DIO\\[14:0\\]
is an output"]
    DIO12_OUTPUT = 4096,
    #[doc = "8192: DIO\\[14:0\\]
is an output"]
    DIO13_OUTPUT = 8192,
    #[doc = "16384: DIO\\[14:0\\]
is an output"]
    DIO14_OUTPUT = 16384,
    #[doc = "32768: DIO15 is an output"]
    DIO15_OUTPUT = 32768,
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
            0 => Val(DIO_A::DIO0_INPUT),
            1 => Val(DIO_A::DIO0_OUTPUT),
            2 => Val(DIO_A::DIO1_OUTPUT),
            4 => Val(DIO_A::DIO2_OUTPUT),
            8 => Val(DIO_A::DIO3_OUTPUT),
            16 => Val(DIO_A::DIO4_OUTPUT),
            32 => Val(DIO_A::DIO5_OUTPUT),
            64 => Val(DIO_A::DIO6_OUTPUT),
            128 => Val(DIO_A::DIO7_OUTPUT),
            256 => Val(DIO_A::DIO8_OUTPUT),
            512 => Val(DIO_A::DIO9_OUTPUT),
            1024 => Val(DIO_A::DIO10_OUTPUT),
            2048 => Val(DIO_A::DIO11_OUTPUT),
            4096 => Val(DIO_A::DIO12_OUTPUT),
            8192 => Val(DIO_A::DIO13_OUTPUT),
            16384 => Val(DIO_A::DIO14_OUTPUT),
            32768 => Val(DIO_A::DIO15_OUTPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIO0_INPUT`"]
    #[inline(always)]
    pub fn is_dio0_input(&self) -> bool {
        *self == DIO_A::DIO0_INPUT
    }
    #[doc = "Checks if the value of the field is `DIO0_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio0_output(&self) -> bool {
        *self == DIO_A::DIO0_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO1_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio1_output(&self) -> bool {
        *self == DIO_A::DIO1_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO2_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio2_output(&self) -> bool {
        *self == DIO_A::DIO2_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO3_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio3_output(&self) -> bool {
        *self == DIO_A::DIO3_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO4_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio4_output(&self) -> bool {
        *self == DIO_A::DIO4_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO5_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio5_output(&self) -> bool {
        *self == DIO_A::DIO5_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO6_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio6_output(&self) -> bool {
        *self == DIO_A::DIO6_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO7_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio7_output(&self) -> bool {
        *self == DIO_A::DIO7_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO8_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio8_output(&self) -> bool {
        *self == DIO_A::DIO8_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO9_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio9_output(&self) -> bool {
        *self == DIO_A::DIO9_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO10_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio10_output(&self) -> bool {
        *self == DIO_A::DIO10_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO11_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio11_output(&self) -> bool {
        *self == DIO_A::DIO11_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO12_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio12_output(&self) -> bool {
        *self == DIO_A::DIO12_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO13_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio13_output(&self) -> bool {
        *self == DIO_A::DIO13_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO14_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio14_output(&self) -> bool {
        *self == DIO_A::DIO14_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DIO15_OUTPUT`"]
    #[inline(always)]
    pub fn is_dio15_output(&self) -> bool {
        *self == DIO_A::DIO15_OUTPUT
    }
}
#[doc = "Set DIO\\[15:0\\]
GPIO direction (only in IO_MODE is 000XX)\n\nValue on reset: 32768"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GPIO_AW {
    #[doc = "0: Set DIO to input if IO_MODE is 0b000XX"]
    GPIO0_INPUT = 0,
    #[doc = "1: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO0_OUTPUT = 1,
    #[doc = "2: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO1_OUTPUT = 2,
    #[doc = "4: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO2_OUTPUT = 4,
    #[doc = "8: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO3_OUTPUT = 8,
    #[doc = "16: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO4_OUTPUT = 16,
    #[doc = "32: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO5_OUTPUT = 32,
    #[doc = "64: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO6_OUTPUT = 64,
    #[doc = "128: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO7_OUTPUT = 128,
    #[doc = "256: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO8_OUTPUT = 256,
    #[doc = "512: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO9_OUTPUT = 512,
    #[doc = "1024: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO10_OUTPUT = 1024,
    #[doc = "2048: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO11_OUTPUT = 2048,
    #[doc = "4096: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO12_OUTPUT = 4096,
    #[doc = "8192: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO13_OUTPUT = 8192,
    #[doc = "16384: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO14_OUTPUT = 16384,
    #[doc = "32768: Set DIO to output if IO_MODE is 0b000XX"]
    GPIO15_OUTPUT = 32768,
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
    #[doc = "Set DIO to input if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio0_input(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO0_INPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio0_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO0_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio1_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO1_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio2_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO2_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio3_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO3_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio4_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO4_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio5_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO5_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio6_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO6_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio7_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO7_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio8_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO8_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio9_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO9_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio10_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO10_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio11_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO11_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio12_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO12_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio13_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO13_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio14_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO14_OUTPUT)
    }
    #[doc = "Set DIO to output if IO_MODE is 0b000XX"]
    #[inline(always)]
    pub fn gpio15_output(self) -> &'a mut W {
        self.variant(GPIO_AW::GPIO15_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Get DIO\\[15:0\\]
direction"]
    #[inline(always)]
    pub fn dio(&self) -> DIO_R {
        DIO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set DIO\\[15:0\\]
GPIO direction (only in IO_MODE is 000XX)"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
}
