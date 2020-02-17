#[doc = "Reader of register ADC_INPUT_SEL[%s]"]
pub type R = crate::R<u32, super::ADC_INPUT_SEL>;
#[doc = "Writer for register ADC_INPUT_SEL[%s]"]
pub type W = crate::W<u32, super::ADC_INPUT_SEL>;
#[doc = "Register ADC_INPUT_SEL[%s]
`reset()`'s with value 0x67"]
impl crate::ResetValue for super::ADC_INPUT_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x67
    }
}
#[doc = "Positive input selection\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POS_INPUT_SEL_A {
    #[doc = "0: Select DIO0 as positive input"]
    ADC_POS_INPUT_DIO0 = 0,
    #[doc = "1: Select DIO1 as positive input"]
    ADC_POS_INPUT_DIO1 = 1,
    #[doc = "2: Select DIO2 as positive input"]
    ADC_POS_INPUT_DIO2 = 2,
    #[doc = "3: Select DIO3 as positive input"]
    ADC_POS_INPUT_DIO3 = 3,
    #[doc = "4: Select AOUT as positive input"]
    ADC_POS_INPUT_AOUT = 4,
    #[doc = "5: Select VDDC as positive input"]
    ADC_POS_INPUT_VDDC = 5,
    #[doc = "6: Select VBAT/2 as positive input"]
    ADC_POS_INPUT_VBAT_DIV2 = 6,
    #[doc = "7: Select ADC internal ground as positive input"]
    ADC_POS_INPUT_GND = 7,
}
impl From<POS_INPUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POS_INPUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `POS_INPUT_SEL`"]
pub type POS_INPUT_SEL_R = crate::R<u8, POS_INPUT_SEL_A>;
impl POS_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POS_INPUT_SEL_A {
        match self.bits {
            0 => POS_INPUT_SEL_A::ADC_POS_INPUT_DIO0,
            1 => POS_INPUT_SEL_A::ADC_POS_INPUT_DIO1,
            2 => POS_INPUT_SEL_A::ADC_POS_INPUT_DIO2,
            3 => POS_INPUT_SEL_A::ADC_POS_INPUT_DIO3,
            4 => POS_INPUT_SEL_A::ADC_POS_INPUT_AOUT,
            5 => POS_INPUT_SEL_A::ADC_POS_INPUT_VDDC,
            6 => POS_INPUT_SEL_A::ADC_POS_INPUT_VBAT_DIV2,
            7 => POS_INPUT_SEL_A::ADC_POS_INPUT_GND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_DIO0`"]
    #[inline(always)]
    pub fn is_adc_pos_input_dio0(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_DIO0
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_DIO1`"]
    #[inline(always)]
    pub fn is_adc_pos_input_dio1(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_DIO1
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_DIO2`"]
    #[inline(always)]
    pub fn is_adc_pos_input_dio2(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_DIO2
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_DIO3`"]
    #[inline(always)]
    pub fn is_adc_pos_input_dio3(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_DIO3
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_AOUT`"]
    #[inline(always)]
    pub fn is_adc_pos_input_aout(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_AOUT
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_VDDC`"]
    #[inline(always)]
    pub fn is_adc_pos_input_vddc(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_VDDC
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_VBAT_DIV2`"]
    #[inline(always)]
    pub fn is_adc_pos_input_vbat_div2(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_VBAT_DIV2
    }
    #[doc = "Checks if the value of the field is `ADC_POS_INPUT_GND`"]
    #[inline(always)]
    pub fn is_adc_pos_input_gnd(&self) -> bool {
        *self == POS_INPUT_SEL_A::ADC_POS_INPUT_GND
    }
}
#[doc = "Write proxy for field `POS_INPUT_SEL`"]
pub struct POS_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POS_INPUT_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select DIO0 as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_dio0(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_DIO0)
    }
    #[doc = "Select DIO1 as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_dio1(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_DIO1)
    }
    #[doc = "Select DIO2 as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_dio2(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_DIO2)
    }
    #[doc = "Select DIO3 as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_dio3(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_DIO3)
    }
    #[doc = "Select AOUT as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_aout(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_AOUT)
    }
    #[doc = "Select VDDC as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_vddc(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_VDDC)
    }
    #[doc = "Select VBAT/2 as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_vbat_div2(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_VBAT_DIV2)
    }
    #[doc = "Select ADC internal ground as positive input"]
    #[inline(always)]
    pub fn adc_pos_input_gnd(self) -> &'a mut W {
        self.variant(POS_INPUT_SEL_A::ADC_POS_INPUT_GND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Negative input selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEG_INPUT_SEL_A {
    #[doc = "0: Select DIO0 as negative input"]
    ADC_NEG_INPUT_DIO0 = 0,
    #[doc = "1: Select DIO1 as negative input"]
    ADC_NEG_INPUT_DIO1 = 1,
    #[doc = "2: Select DIO2 as negative input"]
    ADC_NEG_INPUT_DIO2 = 2,
    #[doc = "3: Select DIO3 as negative input"]
    ADC_NEG_INPUT_DIO3 = 3,
    #[doc = "4: Select AOUT as negative input"]
    ADC_NEG_INPUT_AOUT = 4,
    #[doc = "5: Select VDDC as negative input"]
    ADC_NEG_INPUT_VDDC = 5,
    #[doc = "6: Select VBAT/2 as negative input"]
    ADC_NEG_INPUT_VBAT_DIV2 = 6,
    #[doc = "7: Select ADC internal ground as negative input"]
    ADC_NEG_INPUT_GND = 7,
}
impl From<NEG_INPUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEG_INPUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NEG_INPUT_SEL`"]
pub type NEG_INPUT_SEL_R = crate::R<u8, NEG_INPUT_SEL_A>;
impl NEG_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEG_INPUT_SEL_A {
        match self.bits {
            0 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO0,
            1 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO1,
            2 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO2,
            3 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO3,
            4 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_AOUT,
            5 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_VDDC,
            6 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_VBAT_DIV2,
            7 => NEG_INPUT_SEL_A::ADC_NEG_INPUT_GND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_DIO0`"]
    #[inline(always)]
    pub fn is_adc_neg_input_dio0(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO0
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_DIO1`"]
    #[inline(always)]
    pub fn is_adc_neg_input_dio1(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO1
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_DIO2`"]
    #[inline(always)]
    pub fn is_adc_neg_input_dio2(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO2
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_DIO3`"]
    #[inline(always)]
    pub fn is_adc_neg_input_dio3(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO3
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_AOUT`"]
    #[inline(always)]
    pub fn is_adc_neg_input_aout(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_AOUT
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_VDDC`"]
    #[inline(always)]
    pub fn is_adc_neg_input_vddc(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_VDDC
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_VBAT_DIV2`"]
    #[inline(always)]
    pub fn is_adc_neg_input_vbat_div2(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_VBAT_DIV2
    }
    #[doc = "Checks if the value of the field is `ADC_NEG_INPUT_GND`"]
    #[inline(always)]
    pub fn is_adc_neg_input_gnd(&self) -> bool {
        *self == NEG_INPUT_SEL_A::ADC_NEG_INPUT_GND
    }
}
#[doc = "Write proxy for field `NEG_INPUT_SEL`"]
pub struct NEG_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEG_INPUT_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select DIO0 as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_dio0(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO0)
    }
    #[doc = "Select DIO1 as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_dio1(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO1)
    }
    #[doc = "Select DIO2 as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_dio2(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO2)
    }
    #[doc = "Select DIO3 as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_dio3(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_DIO3)
    }
    #[doc = "Select AOUT as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_aout(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_AOUT)
    }
    #[doc = "Select VDDC as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_vddc(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_VDDC)
    }
    #[doc = "Select VBAT/2 as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_vbat_div2(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_VBAT_DIV2)
    }
    #[doc = "Select ADC internal ground as negative input"]
    #[inline(always)]
    pub fn adc_neg_input_gnd(self) -> &'a mut W {
        self.variant(NEG_INPUT_SEL_A::ADC_NEG_INPUT_GND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Positive input selection"]
    #[inline(always)]
    pub fn pos_input_sel(&self) -> POS_INPUT_SEL_R {
        POS_INPUT_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Negative input selection"]
    #[inline(always)]
    pub fn neg_input_sel(&self) -> NEG_INPUT_SEL_R {
        NEG_INPUT_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Positive input selection"]
    #[inline(always)]
    pub fn pos_input_sel(&mut self) -> POS_INPUT_SEL_W {
        POS_INPUT_SEL_W { w: self }
    }
    #[doc = "Bits 0:2 - Negative input selection"]
    #[inline(always)]
    pub fn neg_input_sel(&mut self) -> NEG_INPUT_SEL_W {
        NEG_INPUT_SEL_W { w: self }
    }
}
