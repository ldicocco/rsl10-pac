#[doc = "Reader of register BB_AESCNTL"]
pub type R = crate::R<u32, super::BB_AESCNTL>;
#[doc = "Writer for register BB_AESCNTL"]
pub type W = crate::W<u32, super::BB_AESCNTL>;
#[doc = "Register BB_AESCNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_AESCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cipher mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_MODE_A {
    #[doc = "0: Cipher mode"]
    AES_MODE_0 = 0,
    #[doc = "1: Decipher mode"]
    AES_MODE_1 = 1,
}
impl From<AES_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AES_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AES_MODE`"]
pub type AES_MODE_R = crate::R<bool, AES_MODE_A>;
impl AES_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_MODE_A {
        match self.bits {
            false => AES_MODE_A::AES_MODE_0,
            true => AES_MODE_A::AES_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AES_MODE_0`"]
    #[inline(always)]
    pub fn is_aes_mode_0(&self) -> bool {
        *self == AES_MODE_A::AES_MODE_0
    }
    #[doc = "Checks if the value of the field is `AES_MODE_1`"]
    #[inline(always)]
    pub fn is_aes_mode_1(&self) -> bool {
        *self == AES_MODE_A::AES_MODE_1
    }
}
#[doc = "Write proxy for field `AES_MODE`"]
pub struct AES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cipher mode"]
    #[inline(always)]
    pub fn aes_mode_0(self) -> &'a mut W {
        self.variant(AES_MODE_A::AES_MODE_0)
    }
    #[doc = "Decipher mode"]
    #[inline(always)]
    pub fn aes_mode_1(self) -> &'a mut W {
        self.variant(AES_MODE_A::AES_MODE_1)
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
#[doc = "Starts AES-128 ciphering/deciphering process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_START_A {
    #[doc = "0: `0`"]
    AES_START_0 = 0,
    #[doc = "1: Starts AES-128 ciphering/deciphering process (the bit is reset once the process is finished)"]
    AES_START_1 = 1,
}
impl From<AES_START_A> for bool {
    #[inline(always)]
    fn from(variant: AES_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AES_START`"]
pub type AES_START_R = crate::R<bool, AES_START_A>;
impl AES_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_START_A {
        match self.bits {
            false => AES_START_A::AES_START_0,
            true => AES_START_A::AES_START_1,
        }
    }
    #[doc = "Checks if the value of the field is `AES_START_0`"]
    #[inline(always)]
    pub fn is_aes_start_0(&self) -> bool {
        *self == AES_START_A::AES_START_0
    }
    #[doc = "Checks if the value of the field is `AES_START_1`"]
    #[inline(always)]
    pub fn is_aes_start_1(&self) -> bool {
        *self == AES_START_A::AES_START_1
    }
}
#[doc = "Write proxy for field `AES_START`"]
pub struct AES_START_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aes_start_0(self) -> &'a mut W {
        self.variant(AES_START_A::AES_START_0)
    }
    #[doc = "Starts AES-128 ciphering/deciphering process (the bit is reset once the process is finished)"]
    #[inline(always)]
    pub fn aes_start_1(self) -> &'a mut W {
        self.variant(AES_START_A::AES_START_1)
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
    #[doc = "Bit 1 - Cipher mode control"]
    #[inline(always)]
    pub fn aes_mode(&self) -> AES_MODE_R {
        AES_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Starts AES-128 ciphering/deciphering process"]
    #[inline(always)]
    pub fn aes_start(&self) -> AES_START_R {
        AES_START_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Cipher mode control"]
    #[inline(always)]
    pub fn aes_mode(&mut self) -> AES_MODE_W {
        AES_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Starts AES-128 ciphering/deciphering process"]
    #[inline(always)]
    pub fn aes_start(&mut self) -> AES_START_W {
        AES_START_W { w: self }
    }
}
