#[doc = "Reader of register SYSCTRL_MEM_ERROR"]
pub type R = crate::R<u32, super::SYSCTRL_MEM_ERROR>;
#[doc = "Writer for register SYSCTRL_MEM_ERROR"]
pub type W = crate::W<u32, super::SYSCTRL_MEM_ERROR>;
#[doc = "Register SYSCTRL_MEM_ERROR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_MEM_ERROR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write a 1 to clear the memory error flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEM_ERROR_CLEAR_AW {
    #[doc = "1: Clear the memory error flags"]
    MEM_ERROR_CLEAR = 1,
}
impl From<MEM_ERROR_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: MEM_ERROR_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MEM_ERROR_CLEAR`"]
pub struct MEM_ERROR_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_ERROR_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_ERROR_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the memory error flags"]
    #[inline(always)]
    pub fn mem_error_clear(self) -> &'a mut W {
        self.variant(MEM_ERROR_CLEAR_AW::MEM_ERROR_CLEAR)
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
#[doc = "Baseband memory error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_MEM_ERROR_A {
    #[doc = "0: No baseband memory error detected"]
    BB_MEM_NO_ERROR_DETECTED = 0,
    #[doc = "1: Baseband has accessed an isolated memory"]
    BB_MEM_ERROR_DETECTED = 1,
}
impl From<BB_MEM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: BB_MEM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_MEM_ERROR`"]
pub type BB_MEM_ERROR_R = crate::R<bool, BB_MEM_ERROR_A>;
impl BB_MEM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_MEM_ERROR_A {
        match self.bits {
            false => BB_MEM_ERROR_A::BB_MEM_NO_ERROR_DETECTED,
            true => BB_MEM_ERROR_A::BB_MEM_ERROR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `BB_MEM_NO_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_bb_mem_no_error_detected(&self) -> bool {
        *self == BB_MEM_ERROR_A::BB_MEM_NO_ERROR_DETECTED
    }
    #[doc = "Checks if the value of the field is `BB_MEM_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_bb_mem_error_detected(&self) -> bool {
        *self == BB_MEM_ERROR_A::BB_MEM_ERROR_DETECTED
    }
}
#[doc = "Flash copier memory error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_COPIER_MEM_ERROR_A {
    #[doc = "0: No Flash copier memory error detected"]
    FLASH_COPIER_MEM_NO_ERROR_DETECTED = 0,
    #[doc = "1: Flash copier has accessed an isolated memory"]
    FLASH_COPIER_MEM_ERROR_DETECTED = 1,
}
impl From<FLASH_COPIER_MEM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_COPIER_MEM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH_COPIER_MEM_ERROR`"]
pub type FLASH_COPIER_MEM_ERROR_R = crate::R<bool, FLASH_COPIER_MEM_ERROR_A>;
impl FLASH_COPIER_MEM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_COPIER_MEM_ERROR_A {
        match self.bits {
            false => FLASH_COPIER_MEM_ERROR_A::FLASH_COPIER_MEM_NO_ERROR_DETECTED,
            true => FLASH_COPIER_MEM_ERROR_A::FLASH_COPIER_MEM_ERROR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_COPIER_MEM_NO_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_flash_copier_mem_no_error_detected(&self) -> bool {
        *self == FLASH_COPIER_MEM_ERROR_A::FLASH_COPIER_MEM_NO_ERROR_DETECTED
    }
    #[doc = "Checks if the value of the field is `FLASH_COPIER_MEM_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_flash_copier_mem_error_detected(&self) -> bool {
        *self == FLASH_COPIER_MEM_ERROR_A::FLASH_COPIER_MEM_ERROR_DETECTED
    }
}
#[doc = "DMA memory error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_MEM_ERROR_A {
    #[doc = "0: No DMA memory error detected"]
    DMA_MEM_NO_ERROR_DETECTED = 0,
    #[doc = "1: DMA has accessed an isolated memory"]
    DMA_MEM_ERROR_DETECTED = 1,
}
impl From<DMA_MEM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_MEM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_MEM_ERROR`"]
pub type DMA_MEM_ERROR_R = crate::R<bool, DMA_MEM_ERROR_A>;
impl DMA_MEM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_MEM_ERROR_A {
        match self.bits {
            false => DMA_MEM_ERROR_A::DMA_MEM_NO_ERROR_DETECTED,
            true => DMA_MEM_ERROR_A::DMA_MEM_ERROR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_MEM_NO_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_dma_mem_no_error_detected(&self) -> bool {
        *self == DMA_MEM_ERROR_A::DMA_MEM_NO_ERROR_DETECTED
    }
    #[doc = "Checks if the value of the field is `DMA_MEM_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_dma_mem_error_detected(&self) -> bool {
        *self == DMA_MEM_ERROR_A::DMA_MEM_ERROR_DETECTED
    }
}
#[doc = "LPDSP32 data memory error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_DMEM_ERROR_A {
    #[doc = "0: No LPDSP32 data memory error detected"]
    LPDSP32_DMEM_NO_ERROR_DETECTED = 0,
    #[doc = "1: LPDSP32 has accessed an isolated data memory"]
    LPDSP32_DMEM_ERROR_DETECTED = 1,
}
impl From<LPDSP32_DMEM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_DMEM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_DMEM_ERROR`"]
pub type LPDSP32_DMEM_ERROR_R = crate::R<bool, LPDSP32_DMEM_ERROR_A>;
impl LPDSP32_DMEM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_DMEM_ERROR_A {
        match self.bits {
            false => LPDSP32_DMEM_ERROR_A::LPDSP32_DMEM_NO_ERROR_DETECTED,
            true => LPDSP32_DMEM_ERROR_A::LPDSP32_DMEM_ERROR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_DMEM_NO_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_lpdsp32_dmem_no_error_detected(&self) -> bool {
        *self == LPDSP32_DMEM_ERROR_A::LPDSP32_DMEM_NO_ERROR_DETECTED
    }
    #[doc = "Checks if the value of the field is `LPDSP32_DMEM_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_lpdsp32_dmem_error_detected(&self) -> bool {
        *self == LPDSP32_DMEM_ERROR_A::LPDSP32_DMEM_ERROR_DETECTED
    }
}
#[doc = "LPDSP32 program memory error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_PMEM_ERROR_A {
    #[doc = "0: No LPDSP32 program memory error detected"]
    LPDSP32_PMEM_NO_ERROR_DETECTED = 0,
    #[doc = "1: LPDSP32 has accessed an isolated program memory"]
    LPDSP32_PMEM_ERROR_DETECTED = 1,
}
impl From<LPDSP32_PMEM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_PMEM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_PMEM_ERROR`"]
pub type LPDSP32_PMEM_ERROR_R = crate::R<bool, LPDSP32_PMEM_ERROR_A>;
impl LPDSP32_PMEM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_PMEM_ERROR_A {
        match self.bits {
            false => LPDSP32_PMEM_ERROR_A::LPDSP32_PMEM_NO_ERROR_DETECTED,
            true => LPDSP32_PMEM_ERROR_A::LPDSP32_PMEM_ERROR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_PMEM_NO_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_lpdsp32_pmem_no_error_detected(&self) -> bool {
        *self == LPDSP32_PMEM_ERROR_A::LPDSP32_PMEM_NO_ERROR_DETECTED
    }
    #[doc = "Checks if the value of the field is `LPDSP32_PMEM_ERROR_DETECTED`"]
    #[inline(always)]
    pub fn is_lpdsp32_pmem_error_detected(&self) -> bool {
        *self == LPDSP32_PMEM_ERROR_A::LPDSP32_PMEM_ERROR_DETECTED
    }
}
impl R {
    #[doc = "Bit 4 - Baseband memory error flag"]
    #[inline(always)]
    pub fn bb_mem_error(&self) -> BB_MEM_ERROR_R {
        BB_MEM_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash copier memory error flag"]
    #[inline(always)]
    pub fn flash_copier_mem_error(&self) -> FLASH_COPIER_MEM_ERROR_R {
        FLASH_COPIER_MEM_ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA memory error flag"]
    #[inline(always)]
    pub fn dma_mem_error(&self) -> DMA_MEM_ERROR_R {
        DMA_MEM_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPDSP32 data memory error flag"]
    #[inline(always)]
    pub fn lpdsp32_dmem_error(&self) -> LPDSP32_DMEM_ERROR_R {
        LPDSP32_DMEM_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPDSP32 program memory error flag"]
    #[inline(always)]
    pub fn lpdsp32_pmem_error(&self) -> LPDSP32_PMEM_ERROR_R {
        LPDSP32_PMEM_ERROR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Write a 1 to clear the memory error flags"]
    #[inline(always)]
    pub fn mem_error_clear(&mut self) -> MEM_ERROR_CLEAR_W {
        MEM_ERROR_CLEAR_W { w: self }
    }
}
