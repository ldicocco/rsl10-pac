#[doc = "Reader of register BB_COEXIFCNTL1"]
pub type R = crate::R<u32, super::BB_COEXIFCNTL1>;
#[doc = "Writer for register BB_COEXIFCNTL1"]
pub type W = crate::W<u32, super::BB_COEXIFCNTL1>;
#[doc = "Register BB_COEXIFCNTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_COEXIFCNTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Determines the threshold for Rx priority setting (applies on ble_rx if WLCRXPRIOMODE equals \"10\")\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLCPRXTHR_A {
    #[doc = "0: If ble_pti\\[3:0\\]
output value is greater than WLCPRXTHR, then Rx BLE priority is considered as high, and must be provided to the Coexistence interface"]
    WLCPRXTHR_0 = 0,
}
impl From<WLCPRXTHR_A> for u8 {
    #[inline(always)]
    fn from(variant: WLCPRXTHR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLCPRXTHR`"]
pub type WLCPRXTHR_R = crate::R<u8, WLCPRXTHR_A>;
impl WLCPRXTHR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLCPRXTHR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLCPRXTHR_A::WLCPRXTHR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WLCPRXTHR_0`"]
    #[inline(always)]
    pub fn is_wlcprxthr_0(&self) -> bool {
        *self == WLCPRXTHR_A::WLCPRXTHR_0
    }
}
#[doc = "Write proxy for field `WLCPRXTHR`"]
pub struct WLCPRXTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPRXTHR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLCPRXTHR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "If ble_pti\\[3:0\\]
output value is greater than WLCPRXTHR, then Rx BLE priority is considered as high, and must be provided to the Coexistence interface"]
    #[inline(always)]
    pub fn wlcprxthr_0(self) -> &'a mut W {
        self.variant(WLCPRXTHR_A::WLCPRXTHR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Determines the threshold for priority setting (applies on ble_tx if WLCTXPRIOMODE equals \"10\")\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLCPTXTHR_A {
    #[doc = "0: If ble_pti\\[3:0\\]
output value is greater than WLCPTXTHR, then Tx BLE priority is considered as high, and must be provided to the Coexistence interface"]
    WLCPTXTHR_0 = 0,
}
impl From<WLCPTXTHR_A> for u8 {
    #[inline(always)]
    fn from(variant: WLCPTXTHR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLCPTXTHR`"]
pub type WLCPTXTHR_R = crate::R<u8, WLCPTXTHR_A>;
impl WLCPTXTHR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLCPTXTHR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLCPTXTHR_A::WLCPTXTHR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WLCPTXTHR_0`"]
    #[inline(always)]
    pub fn is_wlcptxthr_0(&self) -> bool {
        *self == WLCPTXTHR_A::WLCPTXTHR_0
    }
}
#[doc = "Write proxy for field `WLCPTXTHR`"]
pub struct WLCPTXTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPTXTHR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLCPTXTHR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "If ble_pti\\[3:0\\]
output value is greater than WLCPTXTHR, then Tx BLE priority is considered as high, and must be provided to the Coexistence interface"]
    #[inline(always)]
    pub fn wlcptxthr_0(self) -> &'a mut W {
        self.variant(WLCPTXTHR_A::WLCPTXTHR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Determines how many us the priority information must be maintained (applies on ble_tx and ble_rx if WLCTXPRIOMODE equals \"10\")\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLCPDURATION_A {
    #[doc = "0: Note that if WLCPDURATION = 0x00, then Tx/Rx priority levels are maintained till Tx/Rx EN are de-asserted."]
    WLCPDURATION_0 = 0,
}
impl From<WLCPDURATION_A> for u8 {
    #[inline(always)]
    fn from(variant: WLCPDURATION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLCPDURATION`"]
pub type WLCPDURATION_R = crate::R<u8, WLCPDURATION_A>;
impl WLCPDURATION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLCPDURATION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLCPDURATION_A::WLCPDURATION_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WLCPDURATION_0`"]
    #[inline(always)]
    pub fn is_wlcpduration_0(&self) -> bool {
        *self == WLCPDURATION_A::WLCPDURATION_0
    }
}
#[doc = "Write proxy for field `WLCPDURATION`"]
pub struct WLCPDURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPDURATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLCPDURATION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Note that if WLCPDURATION = 0x00, then Tx/Rx priority levels are maintained till Tx/Rx EN are de-asserted."]
    #[inline(always)]
    pub fn wlcpduration_0(self) -> &'a mut W {
        self.variant(WLCPDURATION_A::WLCPDURATION_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Determines the delay (in us) in Tx/Rx enables rises the time BLE Tx/Rx priority has to be provided (applies on ble_tx and ble_rx if WLCTXPRIOMODE equals \"10\")\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLCPDELAY_A {
    #[doc = "0: `0`"]
    WLCPDELAY_0 = 0,
}
impl From<WLCPDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: WLCPDELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLCPDELAY`"]
pub type WLCPDELAY_R = crate::R<u8, WLCPDELAY_A>;
impl WLCPDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLCPDELAY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLCPDELAY_A::WLCPDELAY_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WLCPDELAY_0`"]
    #[inline(always)]
    pub fn is_wlcpdelay_0(&self) -> bool {
        *self == WLCPDELAY_A::WLCPDELAY_0
    }
}
#[doc = "Write proxy for field `WLCPDELAY`"]
pub struct WLCPDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLCPDELAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn wlcpdelay_0(self) -> &'a mut W {
        self.variant(WLCPDELAY_A::WLCPDELAY_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - Determines the threshold for Rx priority setting (applies on ble_rx if WLCRXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcprxthr(&self) -> WLCPRXTHR_R {
        WLCPRXTHR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Determines the threshold for priority setting (applies on ble_tx if WLCTXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcptxthr(&self) -> WLCPTXTHR_R {
        WLCPTXTHR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - Determines how many us the priority information must be maintained (applies on ble_tx and ble_rx if WLCTXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcpduration(&self) -> WLCPDURATION_R {
        WLCPDURATION_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - Determines the delay (in us) in Tx/Rx enables rises the time BLE Tx/Rx priority has to be provided (applies on ble_tx and ble_rx if WLCTXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcpdelay(&self) -> WLCPDELAY_R {
        WLCPDELAY_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - Determines the threshold for Rx priority setting (applies on ble_rx if WLCRXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcprxthr(&mut self) -> WLCPRXTHR_W {
        WLCPRXTHR_W { w: self }
    }
    #[doc = "Bits 16:20 - Determines the threshold for priority setting (applies on ble_tx if WLCTXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcptxthr(&mut self) -> WLCPTXTHR_W {
        WLCPTXTHR_W { w: self }
    }
    #[doc = "Bits 8:14 - Determines how many us the priority information must be maintained (applies on ble_tx and ble_rx if WLCTXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcpduration(&mut self) -> WLCPDURATION_W {
        WLCPDURATION_W { w: self }
    }
    #[doc = "Bits 0:6 - Determines the delay (in us) in Tx/Rx enables rises the time BLE Tx/Rx priority has to be provided (applies on ble_tx and ble_rx if WLCTXPRIOMODE equals \"10\")"]
    #[inline(always)]
    pub fn wlcpdelay(&mut self) -> WLCPDELAY_W {
        WLCPDELAY_W { w: self }
    }
}
