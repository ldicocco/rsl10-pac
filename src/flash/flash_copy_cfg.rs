#[doc = "Reader of register FLASH_COPY_CFG"]
pub type R = crate::R<u32, super::FLASH_COPY_CFG>;
#[doc = "Writer for register FLASH_COPY_CFG"]
pub type W = crate::W<u32, super::FLASH_COPY_CFG>;
#[doc = "Register FLASH_COPY_CFG `reset()`'s with value 0x0002_0000"]
impl crate::ResetValue for super::FLASH_COPY_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0000
    }
}
#[doc = "Comparator address increment/decrement by 1 or 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ADDR_STEP_A {
    #[doc = "0: Address increment/decrement by 1 between two reads"]
    COMP_ADDR_STEP_1 = 0,
    #[doc = "1: Address increment/decrement by 2 between two reads"]
    COMP_ADDR_STEP_2 = 1,
}
impl From<COMP_ADDR_STEP_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_ADDR_STEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP_ADDR_STEP`"]
pub type COMP_ADDR_STEP_R = crate::R<bool, COMP_ADDR_STEP_A>;
impl COMP_ADDR_STEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_ADDR_STEP_A {
        match self.bits {
            false => COMP_ADDR_STEP_A::COMP_ADDR_STEP_1,
            true => COMP_ADDR_STEP_A::COMP_ADDR_STEP_2,
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ADDR_STEP_1`"]
    #[inline(always)]
    pub fn is_comp_addr_step_1(&self) -> bool {
        *self == COMP_ADDR_STEP_A::COMP_ADDR_STEP_1
    }
    #[doc = "Checks if the value of the field is `COMP_ADDR_STEP_2`"]
    #[inline(always)]
    pub fn is_comp_addr_step_2(&self) -> bool {
        *self == COMP_ADDR_STEP_A::COMP_ADDR_STEP_2
    }
}
#[doc = "Write proxy for field `COMP_ADDR_STEP`"]
pub struct COMP_ADDR_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ADDR_STEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ADDR_STEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Address increment/decrement by 1 between two reads"]
    #[inline(always)]
    pub fn comp_addr_step_1(self) -> &'a mut W {
        self.variant(COMP_ADDR_STEP_A::COMP_ADDR_STEP_1)
    }
    #[doc = "Address increment/decrement by 2 between two reads"]
    #[inline(always)]
    pub fn comp_addr_step_2(self) -> &'a mut W {
        self.variant(COMP_ADDR_STEP_A::COMP_ADDR_STEP_2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Comparator address-up or address-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ADDR_DIR_A {
    #[doc = "0: FLASH_COPIER address count-down"]
    COMP_ADDR_DOWN = 0,
    #[doc = "1: FLASH_COPIER address count-up"]
    COMP_ADDR_UP = 1,
}
impl From<COMP_ADDR_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_ADDR_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP_ADDR_DIR`"]
pub type COMP_ADDR_DIR_R = crate::R<bool, COMP_ADDR_DIR_A>;
impl COMP_ADDR_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_ADDR_DIR_A {
        match self.bits {
            false => COMP_ADDR_DIR_A::COMP_ADDR_DOWN,
            true => COMP_ADDR_DIR_A::COMP_ADDR_UP,
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ADDR_DOWN`"]
    #[inline(always)]
    pub fn is_comp_addr_down(&self) -> bool {
        *self == COMP_ADDR_DIR_A::COMP_ADDR_DOWN
    }
    #[doc = "Checks if the value of the field is `COMP_ADDR_UP`"]
    #[inline(always)]
    pub fn is_comp_addr_up(&self) -> bool {
        *self == COMP_ADDR_DIR_A::COMP_ADDR_UP
    }
}
#[doc = "Write proxy for field `COMP_ADDR_DIR`"]
pub struct COMP_ADDR_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_ADDR_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ADDR_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLASH_COPIER address count-down"]
    #[inline(always)]
    pub fn comp_addr_down(self) -> &'a mut W {
        self.variant(COMP_ADDR_DIR_A::COMP_ADDR_DOWN)
    }
    #[doc = "FLASH_COPIER address count-up"]
    #[inline(always)]
    pub fn comp_addr_up(self) -> &'a mut W {
        self.variant(COMP_ADDR_DIR_A::COMP_ADDR_UP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Comparator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_MODE_A {
    #[doc = "0: FLASH_DATA\\[1:0\\]
compare with eFlash DOUT"]
    COMP_MODE_CONSTANT = 0,
    #[doc = "1: Odd address compare with FLASH_DATA\\[1:0\\], even address compare with inverse FLASH_DATA\\[1:0\\]"]
    COMP_MODE_CHBK = 1,
}
impl From<COMP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP_MODE`"]
pub type COMP_MODE_R = crate::R<bool, COMP_MODE_A>;
impl COMP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_MODE_A {
        match self.bits {
            false => COMP_MODE_A::COMP_MODE_CONSTANT,
            true => COMP_MODE_A::COMP_MODE_CHBK,
        }
    }
    #[doc = "Checks if the value of the field is `COMP_MODE_CONSTANT`"]
    #[inline(always)]
    pub fn is_comp_mode_constant(&self) -> bool {
        *self == COMP_MODE_A::COMP_MODE_CONSTANT
    }
    #[doc = "Checks if the value of the field is `COMP_MODE_CHBK`"]
    #[inline(always)]
    pub fn is_comp_mode_chbk(&self) -> bool {
        *self == COMP_MODE_A::COMP_MODE_CHBK
    }
}
#[doc = "Write proxy for field `COMP_MODE`"]
pub struct COMP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLASH_DATA\\[1:0\\]
compare with eFlash DOUT"]
    #[inline(always)]
    pub fn comp_mode_constant(self) -> &'a mut W {
        self.variant(COMP_MODE_A::COMP_MODE_CONSTANT)
    }
    #[doc = "Odd address compare with FLASH_DATA\\[1:0\\], even address compare with inverse FLASH_DATA\\[1:0\\]"]
    #[inline(always)]
    pub fn comp_mode_chbk(self) -> &'a mut W {
        self.variant(COMP_MODE_A::COMP_MODE_CHBK)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Destination copier is the CRC or memories\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPY_DEST_A {
    #[doc = "0: Copy Flash to memory"]
    COPY_TO_MEM = 0,
    #[doc = "1: Copy Flash to CRC"]
    COPY_TO_CRC = 1,
}
impl From<COPY_DEST_A> for bool {
    #[inline(always)]
    fn from(variant: COPY_DEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COPY_DEST`"]
pub type COPY_DEST_R = crate::R<bool, COPY_DEST_A>;
impl COPY_DEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPY_DEST_A {
        match self.bits {
            false => COPY_DEST_A::COPY_TO_MEM,
            true => COPY_DEST_A::COPY_TO_CRC,
        }
    }
    #[doc = "Checks if the value of the field is `COPY_TO_MEM`"]
    #[inline(always)]
    pub fn is_copy_to_mem(&self) -> bool {
        *self == COPY_DEST_A::COPY_TO_MEM
    }
    #[doc = "Checks if the value of the field is `COPY_TO_CRC`"]
    #[inline(always)]
    pub fn is_copy_to_crc(&self) -> bool {
        *self == COPY_DEST_A::COPY_TO_CRC
    }
}
#[doc = "Write proxy for field `COPY_DEST`"]
pub struct COPY_DEST_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_DEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPY_DEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Copy Flash to memory"]
    #[inline(always)]
    pub fn copy_to_mem(self) -> &'a mut W {
        self.variant(COPY_DEST_A::COPY_TO_MEM)
    }
    #[doc = "Copy Flash to CRC"]
    #[inline(always)]
    pub fn copy_to_crc(self) -> &'a mut W {
        self.variant(COPY_DEST_A::COPY_TO_CRC)
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
#[doc = "Select copier mode (32-bit or 40-bit)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPY_MODE_A {
    #[doc = "0: Copy Flash to 32-bit memory"]
    COPY_TO_32BIT = 0,
    #[doc = "1: Copy Flash to 40-bit memory"]
    COPY_TO_40BIT = 1,
}
impl From<COPY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: COPY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COPY_MODE`"]
pub type COPY_MODE_R = crate::R<bool, COPY_MODE_A>;
impl COPY_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPY_MODE_A {
        match self.bits {
            false => COPY_MODE_A::COPY_TO_32BIT,
            true => COPY_MODE_A::COPY_TO_40BIT,
        }
    }
    #[doc = "Checks if the value of the field is `COPY_TO_32BIT`"]
    #[inline(always)]
    pub fn is_copy_to_32bit(&self) -> bool {
        *self == COPY_MODE_A::COPY_TO_32BIT
    }
    #[doc = "Checks if the value of the field is `COPY_TO_40BIT`"]
    #[inline(always)]
    pub fn is_copy_to_40bit(&self) -> bool {
        *self == COPY_MODE_A::COPY_TO_40BIT
    }
}
#[doc = "Write proxy for field `COPY_MODE`"]
pub struct COPY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPY_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Copy Flash to 32-bit memory"]
    #[inline(always)]
    pub fn copy_to_32bit(self) -> &'a mut W {
        self.variant(COPY_MODE_A::COPY_TO_32BIT)
    }
    #[doc = "Copy Flash to 40-bit memory"]
    #[inline(always)]
    pub fn copy_to_40bit(self) -> &'a mut W {
        self.variant(COPY_MODE_A::COPY_TO_40BIT)
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
#[doc = "Copier or Comparator Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Flash copier mode"]
    COPY_MODE = 0,
    #[doc = "1: Flash comparator mode"]
    COMPARATOR_MODE = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::COPY_MODE,
            true => MODE_A::COMPARATOR_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `COPY_MODE`"]
    #[inline(always)]
    pub fn is_copy_mode(&self) -> bool {
        *self == MODE_A::COPY_MODE
    }
    #[doc = "Checks if the value of the field is `COMPARATOR_MODE`"]
    #[inline(always)]
    pub fn is_comparator_mode(&self) -> bool {
        *self == MODE_A::COMPARATOR_MODE
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash copier mode"]
    #[inline(always)]
    pub fn copy_mode(self) -> &'a mut W {
        self.variant(MODE_A::COPY_MODE)
    }
    #[doc = "Flash comparator mode"]
    #[inline(always)]
    pub fn comparator_mode(self) -> &'a mut W {
        self.variant(MODE_A::COMPARATOR_MODE)
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
    #[doc = "Bit 18 - Comparator address increment/decrement by 1 or 2"]
    #[inline(always)]
    pub fn comp_addr_step(&self) -> COMP_ADDR_STEP_R {
        COMP_ADDR_STEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Comparator address-up or address-down"]
    #[inline(always)]
    pub fn comp_addr_dir(&self) -> COMP_ADDR_DIR_R {
        COMP_ADDR_DIR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Comparator Mode"]
    #[inline(always)]
    pub fn comp_mode(&self) -> COMP_MODE_R {
        COMP_MODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination copier is the CRC or memories"]
    #[inline(always)]
    pub fn copy_dest(&self) -> COPY_DEST_R {
        COPY_DEST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select copier mode (32-bit or 40-bit)"]
    #[inline(always)]
    pub fn copy_mode(&self) -> COPY_MODE_R {
        COPY_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Copier or Comparator Mode Configuration"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Comparator address increment/decrement by 1 or 2"]
    #[inline(always)]
    pub fn comp_addr_step(&mut self) -> COMP_ADDR_STEP_W {
        COMP_ADDR_STEP_W { w: self }
    }
    #[doc = "Bit 17 - Comparator address-up or address-down"]
    #[inline(always)]
    pub fn comp_addr_dir(&mut self) -> COMP_ADDR_DIR_W {
        COMP_ADDR_DIR_W { w: self }
    }
    #[doc = "Bit 16 - Comparator Mode"]
    #[inline(always)]
    pub fn comp_mode(&mut self) -> COMP_MODE_W {
        COMP_MODE_W { w: self }
    }
    #[doc = "Bit 9 - Destination copier is the CRC or memories"]
    #[inline(always)]
    pub fn copy_dest(&mut self) -> COPY_DEST_W {
        COPY_DEST_W { w: self }
    }
    #[doc = "Bit 8 - Select copier mode (32-bit or 40-bit)"]
    #[inline(always)]
    pub fn copy_mode(&mut self) -> COPY_MODE_W {
        COPY_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Copier or Comparator Mode Configuration"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
