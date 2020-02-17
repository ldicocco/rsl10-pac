#[doc = "Reader of register DIO_MODE"]
pub type R = crate::R<u32, super::DIO_MODE>;
#[doc = "DIO\\[15:0\\]
mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GPIO_A {
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO0_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO1_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO2_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO3_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO4_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO5_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO6_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO7_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO8_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO9_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO10_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO11_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO12_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO13_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO14_IS_NOT_GPIO = 0,
    #[doc = "0: This DIO is not configured as a CM3 controlled GPIO"]
    DIO15_IS_NOT_GPIO = 0,
    #[doc = "1: This DIO is configured as a CM3 controlled GPIO"]
    DIO0_IS_GPIO = 1,
    #[doc = "2: This DIO is configured as a CM3 controlled GPIO"]
    DIO1_IS_GPIO = 2,
    #[doc = "4: This DIO is configured as a CM3 controlled GPIO"]
    DIO2_IS_GPIO = 4,
    #[doc = "8: This DIO is configured as a CM3 controlled GPIO"]
    DIO3_IS_GPIO = 8,
    #[doc = "16: This DIO is configured as a CM3 controlled GPIO"]
    DIO4_IS_GPIO = 16,
    #[doc = "32: This DIO is configured as a CM3 controlled GPIO"]
    DIO5_IS_GPIO = 32,
    #[doc = "64: This DIO is configured as a CM3 controlled GPIO"]
    DIO6_IS_GPIO = 64,
    #[doc = "128: This DIO is configured as a CM3 controlled GPIO"]
    DIO7_IS_GPIO = 128,
    #[doc = "256: This DIO is configured as a CM3 controlled GPIO"]
    DIO8_IS_GPIO = 256,
    #[doc = "512: This DIO is configured as a CM3 controlled GPIO"]
    DIO9_IS_GPIO = 512,
    #[doc = "1024: This DIO is configured as a CM3 controlled GPIO"]
    DIO10_IS_GPIO = 1024,
    #[doc = "2048: This DIO is configured as a CM3 controlled GPIO"]
    DIO11_IS_GPIO = 2048,
    #[doc = "4096: This DIO is configured as a CM3 controlled GPIO"]
    DIO12_IS_GPIO = 4096,
    #[doc = "8192: This DIO is configured as a CM3 controlled GPIO"]
    DIO13_IS_GPIO = 8192,
    #[doc = "16384: This DIO is configured as a CM3 controlled GPIO"]
    DIO14_IS_GPIO = 16384,
    #[doc = "32768: This DIO is configured as a CM3 controlled GPIO"]
    DIO15_IS_GPIO = 32768,
}
impl From<GPIO_A> for u16 {
    #[inline(always)]
    fn from(variant: GPIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO`"]
pub type GPIO_R = crate::R<u16, GPIO_A>;
impl GPIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, GPIO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_A::DIO0_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO1_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO2_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO3_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO4_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO5_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO6_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO7_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO8_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO9_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO10_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO11_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO12_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO13_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO14_IS_NOT_GPIO),
            0 => Val(GPIO_A::DIO15_IS_NOT_GPIO),
            1 => Val(GPIO_A::DIO0_IS_GPIO),
            2 => Val(GPIO_A::DIO1_IS_GPIO),
            4 => Val(GPIO_A::DIO2_IS_GPIO),
            8 => Val(GPIO_A::DIO3_IS_GPIO),
            16 => Val(GPIO_A::DIO4_IS_GPIO),
            32 => Val(GPIO_A::DIO5_IS_GPIO),
            64 => Val(GPIO_A::DIO6_IS_GPIO),
            128 => Val(GPIO_A::DIO7_IS_GPIO),
            256 => Val(GPIO_A::DIO8_IS_GPIO),
            512 => Val(GPIO_A::DIO9_IS_GPIO),
            1024 => Val(GPIO_A::DIO10_IS_GPIO),
            2048 => Val(GPIO_A::DIO11_IS_GPIO),
            4096 => Val(GPIO_A::DIO12_IS_GPIO),
            8192 => Val(GPIO_A::DIO13_IS_GPIO),
            16384 => Val(GPIO_A::DIO14_IS_GPIO),
            32768 => Val(GPIO_A::DIO15_IS_GPIO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIO0_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio0_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO0_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO1_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio1_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO1_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO2_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio2_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO2_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO3_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio3_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO3_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO4_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio4_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO4_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO5_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio5_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO5_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO6_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio6_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO6_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO7_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio7_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO7_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO8_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio8_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO8_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO9_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio9_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO9_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO10_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio10_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO10_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO11_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio11_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO11_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO12_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio12_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO12_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO13_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio13_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO13_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO14_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio14_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO14_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO15_IS_NOT_GPIO`"]
    #[inline(always)]
    pub fn is_dio15_is_not_gpio(&self) -> bool {
        *self == GPIO_A::DIO15_IS_NOT_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO0_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio0_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO0_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO1_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio1_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO1_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO2_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio2_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO2_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO3_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio3_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO3_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO4_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio4_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO4_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO5_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio5_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO5_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO6_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio6_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO6_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO7_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio7_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO7_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO8_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio8_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO8_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO9_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio9_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO9_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO10_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio10_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO10_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO11_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio11_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO11_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO12_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio12_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO12_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO13_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio13_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO13_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO14_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio14_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO14_IS_GPIO
    }
    #[doc = "Checks if the value of the field is `DIO15_IS_GPIO`"]
    #[inline(always)]
    pub fn is_dio15_is_gpio(&self) -> bool {
        *self == GPIO_A::DIO15_IS_GPIO
    }
}
impl R {
    #[doc = "Bits 0:15 - DIO\\[15:0\\]
mode"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xffff) as u16)
    }
}
