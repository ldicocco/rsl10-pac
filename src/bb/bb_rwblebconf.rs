#[doc = "Reader of register BB_RWBLEBCONF"]
pub type R = crate::R<u32, super::BB_RWBLEBCONF>;
#[doc = "RW-BLE core dual mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMMODE_A {
    #[doc = "0: RW-BLE core is used as a standalone BLE device"]
    DMMODE_0 = 0,
    #[doc = "1: RW-BLE core is used in a dual mode device"]
    DMMODE_1 = 1,
}
impl From<DMMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMMODE`"]
pub type DMMODE_R = crate::R<bool, DMMODE_A>;
impl DMMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMMODE_A {
        match self.bits {
            false => DMMODE_A::DMMODE_0,
            true => DMMODE_A::DMMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMMODE_0`"]
    #[inline(always)]
    pub fn is_dmmode_0(&self) -> bool {
        *self == DMMODE_A::DMMODE_0
    }
    #[doc = "Checks if the value of the field is `DMMODE_1`"]
    #[inline(always)]
    pub fn is_dmmode_1(&self) -> bool {
        *self == DMMODE_A::DMMODE_1
    }
}
#[doc = "Number of supported isochronous channels\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISOPORTNB_A {
    #[doc = "0: No ISO/Audio Channel available"]
    NO_ISO_CH = 0,
    #[doc = "1: One ISO/Audio Channel available"]
    ONE_ISO_CH = 1,
    #[doc = "2: Two ISO/Audio Channels available"]
    TWO_ISO_CH = 2,
    #[doc = "3: Three ISO/Audio Channels available"]
    THREE_ISO_CH = 3,
}
impl From<ISOPORTNB_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOPORTNB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISOPORTNB`"]
pub type ISOPORTNB_R = crate::R<u8, ISOPORTNB_A>;
impl ISOPORTNB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOPORTNB_A {
        match self.bits {
            0 => ISOPORTNB_A::NO_ISO_CH,
            1 => ISOPORTNB_A::ONE_ISO_CH,
            2 => ISOPORTNB_A::TWO_ISO_CH,
            3 => ISOPORTNB_A::THREE_ISO_CH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ISO_CH`"]
    #[inline(always)]
    pub fn is_no_iso_ch(&self) -> bool {
        *self == ISOPORTNB_A::NO_ISO_CH
    }
    #[doc = "Checks if the value of the field is `ONE_ISO_CH`"]
    #[inline(always)]
    pub fn is_one_iso_ch(&self) -> bool {
        *self == ISOPORTNB_A::ONE_ISO_CH
    }
    #[doc = "Checks if the value of the field is `TWO_ISO_CH`"]
    #[inline(always)]
    pub fn is_two_iso_ch(&self) -> bool {
        *self == ISOPORTNB_A::TWO_ISO_CH
    }
    #[doc = "Checks if the value of the field is `THREE_ISO_CH`"]
    #[inline(always)]
    pub fn is_three_iso_ch(&self) -> bool {
        *self == ISOPORTNB_A::THREE_ISO_CH
    }
}
#[doc = "AES deciphering present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECIPHER_A {
    #[doc = "0: AES deciphering not present"]
    DECIPHER_0 = 0,
    #[doc = "1: AES deciphering present"]
    DECIPHER_1 = 1,
}
impl From<DECIPHER_A> for bool {
    #[inline(always)]
    fn from(variant: DECIPHER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECIPHER`"]
pub type DECIPHER_R = crate::R<bool, DECIPHER_A>;
impl DECIPHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECIPHER_A {
        match self.bits {
            false => DECIPHER_A::DECIPHER_0,
            true => DECIPHER_A::DECIPHER_1,
        }
    }
    #[doc = "Checks if the value of the field is `DECIPHER_0`"]
    #[inline(always)]
    pub fn is_decipher_0(&self) -> bool {
        *self == DECIPHER_A::DECIPHER_0
    }
    #[doc = "Checks if the value of the field is `DECIPHER_1`"]
    #[inline(always)]
    pub fn is_decipher_1(&self) -> bool {
        *self == DECIPHER_A::DECIPHER_1
    }
}
#[doc = "Coexistence mechanism\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COEX_A {
    #[doc = "0: Coexistence mechanism not present"]
    COEX_0 = 0,
    #[doc = "1: Coexistence mechanism present"]
    COEX_1 = 1,
}
impl From<COEX_A> for bool {
    #[inline(always)]
    fn from(variant: COEX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COEX`"]
pub type COEX_R = crate::R<bool, COEX_A>;
impl COEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COEX_A {
        match self.bits {
            false => COEX_A::COEX_0,
            true => COEX_A::COEX_1,
        }
    }
    #[doc = "Checks if the value of the field is `COEX_0`"]
    #[inline(always)]
    pub fn is_coex_0(&self) -> bool {
        *self == COEX_A::COEX_0
    }
    #[doc = "Checks if the value of the field is `COEX_1`"]
    #[inline(always)]
    pub fn is_coex_1(&self) -> bool {
        *self == COEX_A::COEX_1
    }
}
#[doc = "Support of the RF front-end\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RFIF_A {
    #[doc = "0: Ripple RF"]
    RFIF_RIPPLE = 0,
    #[doc = "1: External radio controller support"]
    RFIF_EXT = 1,
    #[doc = "2: Atlas radio"]
    RFIF_ATLAS = 2,
    #[doc = "3: IcyTRx radio"]
    RFIF_ICYTRX = 3,
}
impl From<RFIF_A> for u8 {
    #[inline(always)]
    fn from(variant: RFIF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RFIF`"]
pub type RFIF_R = crate::R<u8, RFIF_A>;
impl RFIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RFIF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RFIF_A::RFIF_RIPPLE),
            1 => Val(RFIF_A::RFIF_EXT),
            2 => Val(RFIF_A::RFIF_ATLAS),
            3 => Val(RFIF_A::RFIF_ICYTRX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFIF_RIPPLE`"]
    #[inline(always)]
    pub fn is_rfif_ripple(&self) -> bool {
        *self == RFIF_A::RFIF_RIPPLE
    }
    #[doc = "Checks if the value of the field is `RFIF_EXT`"]
    #[inline(always)]
    pub fn is_rfif_ext(&self) -> bool {
        *self == RFIF_A::RFIF_EXT
    }
    #[doc = "Checks if the value of the field is `RFIF_ATLAS`"]
    #[inline(always)]
    pub fn is_rfif_atlas(&self) -> bool {
        *self == RFIF_A::RFIF_ATLAS
    }
    #[doc = "Checks if the value of the field is `RFIF_ICYTRX`"]
    #[inline(always)]
    pub fn is_rfif_icytrx(&self) -> bool {
        *self == RFIF_A::RFIF_ICYTRX
    }
}
#[doc = "Diagnostic port\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USEDBG_A {
    #[doc = "0: Coexistence mechanism present"]
    USEDBG_0 = 0,
    #[doc = "1: Diagnostic port instantiated"]
    USEDBG_1 = 1,
}
impl From<USEDBG_A> for bool {
    #[inline(always)]
    fn from(variant: USEDBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USEDBG`"]
pub type USEDBG_R = crate::R<bool, USEDBG_A>;
impl USEDBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USEDBG_A {
        match self.bits {
            false => USEDBG_A::USEDBG_0,
            true => USEDBG_A::USEDBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `USEDBG_0`"]
    #[inline(always)]
    pub fn is_usedbg_0(&self) -> bool {
        *self == USEDBG_A::USEDBG_0
    }
    #[doc = "Checks if the value of the field is `USEDBG_1`"]
    #[inline(always)]
    pub fn is_usedbg_1(&self) -> bool {
        *self == USEDBG_A::USEDBG_1
    }
}
#[doc = "AES-CCM encryption\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USECRYPT_A {
    #[doc = "0: AES-CCM encryption block not present"]
    USECRYPT_0 = 0,
    #[doc = "1: AES-CCM encryption block present"]
    USECRYPT_1 = 1,
}
impl From<USECRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: USECRYPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USECRYPT`"]
pub type USECRYPT_R = crate::R<bool, USECRYPT_A>;
impl USECRYPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USECRYPT_A {
        match self.bits {
            false => USECRYPT_A::USECRYPT_0,
            true => USECRYPT_A::USECRYPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `USECRYPT_0`"]
    #[inline(always)]
    pub fn is_usecrypt_0(&self) -> bool {
        *self == USECRYPT_A::USECRYPT_0
    }
    #[doc = "Checks if the value of the field is `USECRYPT_1`"]
    #[inline(always)]
    pub fn is_usecrypt_1(&self) -> bool {
        *self == USECRYPT_A::USECRYPT_1
    }
}
#[doc = "Operating frequency (in MHz)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL_A {
    #[doc = "8: Default value is 8MHz"]
    CLK_SEL_8 = 8,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL`"]
pub type CLK_SEL_R = crate::R<u8, CLK_SEL_A>;
impl CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(CLK_SEL_A::CLK_SEL_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_8`"]
    #[inline(always)]
    pub fn is_clk_sel_8(&self) -> bool {
        *self == CLK_SEL_A::CLK_SEL_8
    }
}
#[doc = "Interruption mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMODE_A {
    #[doc = "0: Interrupts are edge level generated, i.e. pulse"]
    INTMODE_0 = 0,
    #[doc = "1: Interrupts are trigger level generated, i.e. stays active at 1 till acknowledgement"]
    INTMODE_1 = 1,
}
impl From<INTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: INTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTMODE`"]
pub type INTMODE_R = crate::R<bool, INTMODE_A>;
impl INTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMODE_A {
        match self.bits {
            false => INTMODE_A::INTMODE_0,
            true => INTMODE_A::INTMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTMODE_0`"]
    #[inline(always)]
    pub fn is_intmode_0(&self) -> bool {
        *self == INTMODE_A::INTMODE_0
    }
    #[doc = "Checks if the value of the field is `INTMODE_1`"]
    #[inline(always)]
    pub fn is_intmode_1(&self) -> bool {
        *self == INTMODE_A::INTMODE_1
    }
}
#[doc = "Processor bus type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSTYPE_A {
    #[doc = "0: Processor bus type: AHB bus"]
    BUSTYPE_0 = 0,
    #[doc = "1: Processor bus type: XBAR bus"]
    BUSTYPE_1 = 1,
}
impl From<BUSTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: BUSTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSTYPE`"]
pub type BUSTYPE_R = crate::R<bool, BUSTYPE_A>;
impl BUSTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSTYPE_A {
        match self.bits {
            false => BUSTYPE_A::BUSTYPE_0,
            true => BUSTYPE_A::BUSTYPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSTYPE_0`"]
    #[inline(always)]
    pub fn is_bustype_0(&self) -> bool {
        *self == BUSTYPE_A::BUSTYPE_0
    }
    #[doc = "Checks if the value of the field is `BUSTYPE_1`"]
    #[inline(always)]
    pub fn is_bustype_1(&self) -> bool {
        *self == BUSTYPE_A::BUSTYPE_1
    }
}
#[doc = "Processor bus width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_WIDTH_A {
    #[doc = "0: Processor bus width: 16 bits"]
    DATA_WIDTH_0 = 0,
    #[doc = "1: Processor bus width: 32 bits"]
    DATA_WIDTH_1 = 1,
}
impl From<DATA_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_WIDTH`"]
pub type DATA_WIDTH_R = crate::R<bool, DATA_WIDTH_A>;
impl DATA_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_WIDTH_A {
        match self.bits {
            false => DATA_WIDTH_A::DATA_WIDTH_0,
            true => DATA_WIDTH_A::DATA_WIDTH_1,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_WIDTH_0`"]
    #[inline(always)]
    pub fn is_data_width_0(&self) -> bool {
        *self == DATA_WIDTH_A::DATA_WIDTH_0
    }
    #[doc = "Checks if the value of the field is `DATA_WIDTH_1`"]
    #[inline(always)]
    pub fn is_data_width_1(&self) -> bool {
        *self == DATA_WIDTH_A::DATA_WIDTH_1
    }
}
#[doc = "Value of the RW_BLE_ADDRESS_WIDTH parameter concerted into binary\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADD_WIDTH_A {
    #[doc = "14: EM size is 16kB"]
    ADD_WIDTH_14 = 14,
}
impl From<ADD_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADD_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADD_WIDTH`"]
pub type ADD_WIDTH_R = crate::R<u8, ADD_WIDTH_A>;
impl ADD_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADD_WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            14 => Val(ADD_WIDTH_A::ADD_WIDTH_14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADD_WIDTH_14`"]
    #[inline(always)]
    pub fn is_add_width_14(&self) -> bool {
        *self == ADD_WIDTH_A::ADD_WIDTH_14
    }
}
impl R {
    #[doc = "Bit 31 - RW-BLE core dual mode"]
    #[inline(always)]
    pub fn dmmode(&self) -> DMMODE_R {
        DMMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Number of supported isochronous channels"]
    #[inline(always)]
    pub fn isoportnb(&self) -> ISOPORTNB_R {
        ISOPORTNB_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23 - AES deciphering present"]
    #[inline(always)]
    pub fn decipher(&self) -> DECIPHER_R {
        DECIPHER_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Coexistence mechanism"]
    #[inline(always)]
    pub fn coex(&self) -> COEX_R {
        COEX_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Support of the RF front-end"]
    #[inline(always)]
    pub fn rfif(&self) -> RFIF_R {
        RFIF_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Diagnostic port"]
    #[inline(always)]
    pub fn usedbg(&self) -> USEDBG_R {
        USEDBG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AES-CCM encryption"]
    #[inline(always)]
    pub fn usecrypt(&self) -> USECRYPT_R {
        USECRYPT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Operating frequency (in MHz)"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Interruption mode"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Processor bus type"]
    #[inline(always)]
    pub fn bustype(&self) -> BUSTYPE_R {
        BUSTYPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Processor bus width"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Value of the RW_BLE_ADDRESS_WIDTH parameter concerted into binary"]
    #[inline(always)]
    pub fn add_width(&self) -> ADD_WIDTH_R {
        ADD_WIDTH_R::new((self.bits & 0x1f) as u8)
    }
}
