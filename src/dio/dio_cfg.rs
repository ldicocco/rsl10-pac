#[doc = "Reader of register DIO_CFG[%s]"]
pub type R = crate::R<u32, super::DIO_CFG>;
#[doc = "Writer for register DIO_CFG[%s]"]
pub type W = crate::W<u32, super::DIO_CFG>;
#[doc = "Register DIO_CFG[%s]
`reset()`'s with value 0x313f"]
impl crate::ResetValue for super::DIO_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x313f
    }
}
#[doc = "Drive strength configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_A {
    #[doc = "0: 2x drive strength"]
    DIO_2X_DRIVE = 0,
    #[doc = "1: 3x drive strength"]
    DIO_3X_DRIVE = 1,
    #[doc = "2: 5x drive strength"]
    DIO_5X_DRIVE = 2,
    #[doc = "3: 6x drive strength"]
    DIO_6X_DRIVE = 3,
}
impl From<DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRIVE`"]
pub type DRIVE_R = crate::R<u8, DRIVE_A>;
impl DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::DIO_2X_DRIVE,
            1 => DRIVE_A::DIO_3X_DRIVE,
            2 => DRIVE_A::DIO_5X_DRIVE,
            3 => DRIVE_A::DIO_6X_DRIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIO_2X_DRIVE`"]
    #[inline(always)]
    pub fn is_dio_2x_drive(&self) -> bool {
        *self == DRIVE_A::DIO_2X_DRIVE
    }
    #[doc = "Checks if the value of the field is `DIO_3X_DRIVE`"]
    #[inline(always)]
    pub fn is_dio_3x_drive(&self) -> bool {
        *self == DRIVE_A::DIO_3X_DRIVE
    }
    #[doc = "Checks if the value of the field is `DIO_5X_DRIVE`"]
    #[inline(always)]
    pub fn is_dio_5x_drive(&self) -> bool {
        *self == DRIVE_A::DIO_5X_DRIVE
    }
    #[doc = "Checks if the value of the field is `DIO_6X_DRIVE`"]
    #[inline(always)]
    pub fn is_dio_6x_drive(&self) -> bool {
        *self == DRIVE_A::DIO_6X_DRIVE
    }
}
#[doc = "Write proxy for field `DRIVE`"]
pub struct DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2x drive strength"]
    #[inline(always)]
    pub fn dio_2x_drive(self) -> &'a mut W {
        self.variant(DRIVE_A::DIO_2X_DRIVE)
    }
    #[doc = "3x drive strength"]
    #[inline(always)]
    pub fn dio_3x_drive(self) -> &'a mut W {
        self.variant(DRIVE_A::DIO_3X_DRIVE)
    }
    #[doc = "5x drive strength"]
    #[inline(always)]
    pub fn dio_5x_drive(self) -> &'a mut W {
        self.variant(DRIVE_A::DIO_5X_DRIVE)
    }
    #[doc = "6x drive strength"]
    #[inline(always)]
    pub fn dio_6x_drive(self) -> &'a mut W {
        self.variant(DRIVE_A::DIO_6X_DRIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Low Pass Filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPF_A {
    #[doc = "0: Disable low pass filter"]
    DIO_LPF_DISABLE = 0,
    #[doc = "1: Enable low pass filter"]
    DIO_LPF_ENABLE = 1,
}
impl From<LPF_A> for bool {
    #[inline(always)]
    fn from(variant: LPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPF`"]
pub type LPF_R = crate::R<bool, LPF_A>;
impl LPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPF_A {
        match self.bits {
            false => LPF_A::DIO_LPF_DISABLE,
            true => LPF_A::DIO_LPF_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DIO_LPF_DISABLE`"]
    #[inline(always)]
    pub fn is_dio_lpf_disable(&self) -> bool {
        *self == LPF_A::DIO_LPF_DISABLE
    }
    #[doc = "Checks if the value of the field is `DIO_LPF_ENABLE`"]
    #[inline(always)]
    pub fn is_dio_lpf_enable(&self) -> bool {
        *self == LPF_A::DIO_LPF_ENABLE
    }
}
#[doc = "Write proxy for field `LPF`"]
pub struct LPF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable low pass filter"]
    #[inline(always)]
    pub fn dio_lpf_disable(self) -> &'a mut W {
        self.variant(LPF_A::DIO_LPF_DISABLE)
    }
    #[doc = "Enable low pass filter"]
    #[inline(always)]
    pub fn dio_lpf_enable(self) -> &'a mut W {
        self.variant(LPF_A::DIO_LPF_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Pull selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PULL_CTRL_A {
    #[doc = "0: No pull selected"]
    DIO_NO_PULL = 0,
    #[doc = "1: Weak pull-up selected"]
    DIO_WEAK_PULL_UP = 1,
    #[doc = "2: Weak pull-down selected"]
    DIO_WEAK_PULL_DOWN = 2,
    #[doc = "3: Strong pull-up selected"]
    DIO_STRONG_PULL_UP = 3,
}
impl From<PULL_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: PULL_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PULL_CTRL`"]
pub type PULL_CTRL_R = crate::R<u8, PULL_CTRL_A>;
impl PULL_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULL_CTRL_A {
        match self.bits {
            0 => PULL_CTRL_A::DIO_NO_PULL,
            1 => PULL_CTRL_A::DIO_WEAK_PULL_UP,
            2 => PULL_CTRL_A::DIO_WEAK_PULL_DOWN,
            3 => PULL_CTRL_A::DIO_STRONG_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIO_NO_PULL`"]
    #[inline(always)]
    pub fn is_dio_no_pull(&self) -> bool {
        *self == PULL_CTRL_A::DIO_NO_PULL
    }
    #[doc = "Checks if the value of the field is `DIO_WEAK_PULL_UP`"]
    #[inline(always)]
    pub fn is_dio_weak_pull_up(&self) -> bool {
        *self == PULL_CTRL_A::DIO_WEAK_PULL_UP
    }
    #[doc = "Checks if the value of the field is `DIO_WEAK_PULL_DOWN`"]
    #[inline(always)]
    pub fn is_dio_weak_pull_down(&self) -> bool {
        *self == PULL_CTRL_A::DIO_WEAK_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `DIO_STRONG_PULL_UP`"]
    #[inline(always)]
    pub fn is_dio_strong_pull_up(&self) -> bool {
        *self == PULL_CTRL_A::DIO_STRONG_PULL_UP
    }
}
#[doc = "Write proxy for field `PULL_CTRL`"]
pub struct PULL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULL_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No pull selected"]
    #[inline(always)]
    pub fn dio_no_pull(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::DIO_NO_PULL)
    }
    #[doc = "Weak pull-up selected"]
    #[inline(always)]
    pub fn dio_weak_pull_up(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::DIO_WEAK_PULL_UP)
    }
    #[doc = "Weak pull-down selected"]
    #[inline(always)]
    pub fn dio_weak_pull_down(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::DIO_WEAK_PULL_DOWN)
    }
    #[doc = "Strong pull-up selected"]
    #[inline(always)]
    pub fn dio_strong_pull_up(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::DIO_STRONG_PULL_UP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "IO mode selection\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO_MODE_A {
    #[doc = "0: Input (GPIO input mode)"]
    DIO_MODE_GPIO_IN_0 = 0,
    #[doc = "1: Input (GPIO input mode)"]
    DIO_MODE_GPIO_IN_1 = 1,
    #[doc = "2: Output low (GPIO output mode)"]
    DIO_MODE_GPIO_OUT_0 = 2,
    #[doc = "3: Output high (GPIO output mode)"]
    DIO_MODE_GPIO_OUT_1 = 3,
    #[doc = "4: Output USRCLK (user clock) signal"]
    DIO_MODE_USRCLK = 4,
    #[doc = "5: Output SLOWCLK (slow clock) signal"]
    DIO_MODE_SLOWCLK = 5,
    #[doc = "6: Output SYSCLK (system clock) signal"]
    DIO_MODE_SYSCLK = 6,
    #[doc = "7: Output PCM_SERO interface signal"]
    DIO_MODE_PCM_SERO = 7,
    #[doc = "8: Output PCM_FRAME interface signal"]
    DIO_MODE_PCM_FRAME = 8,
    #[doc = "9: Output SPI0_SERO interface signal"]
    DIO_MODE_SPI0_SERO = 9,
    #[doc = "10: Output SPI0_CS interface signal"]
    DIO_MODE_SPI0_CS = 10,
    #[doc = "11: Output SPI0_CLK interface signal"]
    DIO_MODE_SPI0_CLK = 11,
    #[doc = "12: Output SPI1_SERO interface signal"]
    DIO_MODE_SPI1_SERO = 12,
    #[doc = "13: Output SPI1_CS interface signal"]
    DIO_MODE_SPI1_CS = 13,
    #[doc = "14: Output SPI1_CLK interface signal"]
    DIO_MODE_SPI1_CLK = 14,
    #[doc = "15: Output UART_TX interface signal"]
    DIO_MODE_UART_TX = 15,
    #[doc = "16: Output SCL interface signal (open collector)"]
    DIO_MODE_SCL = 16,
    #[doc = "17: Output SDA interface signal (open collector)"]
    DIO_MODE_SDA = 17,
    #[doc = "18: Output PWM0 interface signal"]
    DIO_MODE_PWM0 = 18,
    #[doc = "19: Output PWM0 interface signal inverted"]
    DIO_MODE_PWM0_INV = 19,
    #[doc = "20: Output PWM1 interface signal"]
    DIO_MODE_PWM1 = 20,
    #[doc = "21: Output PWM1 interface signal inverted"]
    DIO_MODE_PWM1_INV = 21,
    #[doc = "22: Output LPDSP32-TDO interface signal"]
    DIO_MODE_LPDSP32_TDO = 22,
    #[doc = "23: Output RFCLK signal"]
    DIO_MODE_RFCLK = 23,
    #[doc = "24: Output RCCLK signal"]
    DIO_MODE_RCCLK = 24,
    #[doc = "25: Output of JTCK divider signal"]
    DIO_MODE_JTCK_DIV = 25,
    #[doc = "26: Output of EXTCLK divider signal"]
    DIO_MODE_EXTCLK_DIV = 26,
    #[doc = "27: Output STANDBYCLK signal"]
    DIO_MODE_STANDBYCLK = 27,
    #[doc = "28: Output baseband controller TX data signal"]
    DIO_MODE_BB_TX_DATA = 28,
    #[doc = "29: Output baseband controller TX data valid signal"]
    DIO_MODE_BB_TX_DATA_VALID = 29,
    #[doc = "30: Output baseband controller BLE synchronization signal"]
    DIO_MODE_BB_SYNC_P = 30,
    #[doc = "31: Output baseband controller BLE AUDIO0 synchronization signal"]
    DIO_MODE_BB_AUDIO0_SYNC_P = 31,
    #[doc = "32: Output baseband controller BLE AUDIO1 synchronization signal"]
    DIO_MODE_BB_AUDIO1_SYNC_P = 32,
    #[doc = "33: Output baseband controller BLE AUDIO2 synchronization signal"]
    DIO_MODE_BB_AUDIO2_SYNC_P = 33,
    #[doc = "34: Output baseband controller SPI_CSN signal"]
    DIO_MODE_BB_SPI_CSN = 34,
    #[doc = "35: Output baseband controller SPI_CLK signal"]
    DIO_MODE_BB_SPI_CLK = 35,
    #[doc = "36: Output baseband controller SPI_MOSI signal"]
    DIO_MODE_BB_SPI_MOSI = 36,
    #[doc = "37: Output baseband controller diagnostic port 0 (bit 0) signal"]
    DIO_MODE_BB_DBG0_0 = 37,
    #[doc = "38: Output baseband controller diagnostic port 0 (bit 1) signal"]
    DIO_MODE_BB_DBG0_1 = 38,
    #[doc = "39: Output baseband controller diagnostic port 0 (bit 2) signal"]
    DIO_MODE_BB_DBG0_2 = 39,
    #[doc = "40: Output baseband controller diagnostic port 0 (bit 3) signal"]
    DIO_MODE_BB_DBG0_3 = 40,
    #[doc = "41: Output baseband controller diagnostic port 0 (bit 4) signal"]
    DIO_MODE_BB_DBG0_4 = 41,
    #[doc = "42: Output baseband controller diagnostic port 0 (bit 5) signal"]
    DIO_MODE_BB_DBG0_5 = 42,
    #[doc = "43: Output baseband controller diagnostic port 0 (bit 6) signal"]
    DIO_MODE_BB_DBG0_6 = 43,
    #[doc = "44: Output baseband controller diagnostic port 0 (bit 7) signal"]
    DIO_MODE_BB_DBG0_7 = 44,
    #[doc = "45: Output RF front-end SPI_MISO interface signal"]
    DIO_MODE_RF_SPI_MISO = 45,
    #[doc = "46: Output RF front-end GPIO0 output (RX_DATA) signal"]
    DIO_MODE_RF_GPIO0 = 46,
    #[doc = "47: Output RF front-end GPIO1 output (RX_CLK) signal"]
    DIO_MODE_RF_GPIO1 = 47,
    #[doc = "48: Output RF front-end GPIO2 output signal"]
    DIO_MODE_RF_GPIO2 = 48,
    #[doc = "49: Output RF front-end GPIO3 output signal"]
    DIO_MODE_RF_GPIO3 = 49,
    #[doc = "50: Output RF front-end GPIO4 output signal"]
    DIO_MODE_RF_GPIO4 = 50,
    #[doc = "51: Output RF front-end GPIO5 output signal"]
    DIO_MODE_RF_GPIO5 = 51,
    #[doc = "52: Output RF front-end GPIO6 output signal"]
    DIO_MODE_RF_GPIO6 = 52,
    #[doc = "53: Output RF front-end GPIO7 output signal"]
    DIO_MODE_RF_GPIO7 = 53,
    #[doc = "54: Output RF front-end GPIO8 output signal"]
    DIO_MODE_RF_GPIO8 = 54,
    #[doc = "55: Output RF front-end GPIO9 output signal"]
    DIO_MODE_RF_GPIO9 = 55,
    #[doc = "56: Output the AUDIOCLK (audio clock) signal"]
    DIO_MODE_AUDIOCLK = 56,
    #[doc = "57: Output the AUDIOSLOWCLK (slow audio clock) signal"]
    DIO_MODE_AUDIOSLOWCLK = 57,
    #[doc = "58: Output OD + signal"]
    DIO_MODE_OD_P = 58,
    #[doc = "59: Output OD - signal"]
    DIO_MODE_OD_N = 59,
    #[doc = "60: Output audio synchronization pulse"]
    DIO_MODE_AUDIO_SYNC_PULSE = 60,
    #[doc = "61: Output audio synchronization missed pulse"]
    DIO_MODE_AUDIO_SYNC_MISSED = 61,
    #[doc = "62: Input mode"]
    DIO_MODE_INPUT = 62,
    #[doc = "63: Disabled"]
    DIO_MODE_DISABLE = 63,
}
impl From<IO_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IO_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO_MODE`"]
pub type IO_MODE_R = crate::R<u8, IO_MODE_A>;
impl IO_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MODE_A {
        match self.bits {
            0 => IO_MODE_A::DIO_MODE_GPIO_IN_0,
            1 => IO_MODE_A::DIO_MODE_GPIO_IN_1,
            2 => IO_MODE_A::DIO_MODE_GPIO_OUT_0,
            3 => IO_MODE_A::DIO_MODE_GPIO_OUT_1,
            4 => IO_MODE_A::DIO_MODE_USRCLK,
            5 => IO_MODE_A::DIO_MODE_SLOWCLK,
            6 => IO_MODE_A::DIO_MODE_SYSCLK,
            7 => IO_MODE_A::DIO_MODE_PCM_SERO,
            8 => IO_MODE_A::DIO_MODE_PCM_FRAME,
            9 => IO_MODE_A::DIO_MODE_SPI0_SERO,
            10 => IO_MODE_A::DIO_MODE_SPI0_CS,
            11 => IO_MODE_A::DIO_MODE_SPI0_CLK,
            12 => IO_MODE_A::DIO_MODE_SPI1_SERO,
            13 => IO_MODE_A::DIO_MODE_SPI1_CS,
            14 => IO_MODE_A::DIO_MODE_SPI1_CLK,
            15 => IO_MODE_A::DIO_MODE_UART_TX,
            16 => IO_MODE_A::DIO_MODE_SCL,
            17 => IO_MODE_A::DIO_MODE_SDA,
            18 => IO_MODE_A::DIO_MODE_PWM0,
            19 => IO_MODE_A::DIO_MODE_PWM0_INV,
            20 => IO_MODE_A::DIO_MODE_PWM1,
            21 => IO_MODE_A::DIO_MODE_PWM1_INV,
            22 => IO_MODE_A::DIO_MODE_LPDSP32_TDO,
            23 => IO_MODE_A::DIO_MODE_RFCLK,
            24 => IO_MODE_A::DIO_MODE_RCCLK,
            25 => IO_MODE_A::DIO_MODE_JTCK_DIV,
            26 => IO_MODE_A::DIO_MODE_EXTCLK_DIV,
            27 => IO_MODE_A::DIO_MODE_STANDBYCLK,
            28 => IO_MODE_A::DIO_MODE_BB_TX_DATA,
            29 => IO_MODE_A::DIO_MODE_BB_TX_DATA_VALID,
            30 => IO_MODE_A::DIO_MODE_BB_SYNC_P,
            31 => IO_MODE_A::DIO_MODE_BB_AUDIO0_SYNC_P,
            32 => IO_MODE_A::DIO_MODE_BB_AUDIO1_SYNC_P,
            33 => IO_MODE_A::DIO_MODE_BB_AUDIO2_SYNC_P,
            34 => IO_MODE_A::DIO_MODE_BB_SPI_CSN,
            35 => IO_MODE_A::DIO_MODE_BB_SPI_CLK,
            36 => IO_MODE_A::DIO_MODE_BB_SPI_MOSI,
            37 => IO_MODE_A::DIO_MODE_BB_DBG0_0,
            38 => IO_MODE_A::DIO_MODE_BB_DBG0_1,
            39 => IO_MODE_A::DIO_MODE_BB_DBG0_2,
            40 => IO_MODE_A::DIO_MODE_BB_DBG0_3,
            41 => IO_MODE_A::DIO_MODE_BB_DBG0_4,
            42 => IO_MODE_A::DIO_MODE_BB_DBG0_5,
            43 => IO_MODE_A::DIO_MODE_BB_DBG0_6,
            44 => IO_MODE_A::DIO_MODE_BB_DBG0_7,
            45 => IO_MODE_A::DIO_MODE_RF_SPI_MISO,
            46 => IO_MODE_A::DIO_MODE_RF_GPIO0,
            47 => IO_MODE_A::DIO_MODE_RF_GPIO1,
            48 => IO_MODE_A::DIO_MODE_RF_GPIO2,
            49 => IO_MODE_A::DIO_MODE_RF_GPIO3,
            50 => IO_MODE_A::DIO_MODE_RF_GPIO4,
            51 => IO_MODE_A::DIO_MODE_RF_GPIO5,
            52 => IO_MODE_A::DIO_MODE_RF_GPIO6,
            53 => IO_MODE_A::DIO_MODE_RF_GPIO7,
            54 => IO_MODE_A::DIO_MODE_RF_GPIO8,
            55 => IO_MODE_A::DIO_MODE_RF_GPIO9,
            56 => IO_MODE_A::DIO_MODE_AUDIOCLK,
            57 => IO_MODE_A::DIO_MODE_AUDIOSLOWCLK,
            58 => IO_MODE_A::DIO_MODE_OD_P,
            59 => IO_MODE_A::DIO_MODE_OD_N,
            60 => IO_MODE_A::DIO_MODE_AUDIO_SYNC_PULSE,
            61 => IO_MODE_A::DIO_MODE_AUDIO_SYNC_MISSED,
            62 => IO_MODE_A::DIO_MODE_INPUT,
            63 => IO_MODE_A::DIO_MODE_DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_GPIO_IN_0`"]
    #[inline(always)]
    pub fn is_dio_mode_gpio_in_0(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_GPIO_IN_0
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_GPIO_IN_1`"]
    #[inline(always)]
    pub fn is_dio_mode_gpio_in_1(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_GPIO_IN_1
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_GPIO_OUT_0`"]
    #[inline(always)]
    pub fn is_dio_mode_gpio_out_0(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_GPIO_OUT_0
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_GPIO_OUT_1`"]
    #[inline(always)]
    pub fn is_dio_mode_gpio_out_1(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_GPIO_OUT_1
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_USRCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_usrclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_USRCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SLOWCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_slowclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SLOWCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SYSCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_sysclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SYSCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_PCM_SERO`"]
    #[inline(always)]
    pub fn is_dio_mode_pcm_sero(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_PCM_SERO
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_PCM_FRAME`"]
    #[inline(always)]
    pub fn is_dio_mode_pcm_frame(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_PCM_FRAME
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SPI0_SERO`"]
    #[inline(always)]
    pub fn is_dio_mode_spi0_sero(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SPI0_SERO
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SPI0_CS`"]
    #[inline(always)]
    pub fn is_dio_mode_spi0_cs(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SPI0_CS
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SPI0_CLK`"]
    #[inline(always)]
    pub fn is_dio_mode_spi0_clk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SPI0_CLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SPI1_SERO`"]
    #[inline(always)]
    pub fn is_dio_mode_spi1_sero(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SPI1_SERO
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SPI1_CS`"]
    #[inline(always)]
    pub fn is_dio_mode_spi1_cs(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SPI1_CS
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SPI1_CLK`"]
    #[inline(always)]
    pub fn is_dio_mode_spi1_clk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SPI1_CLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_UART_TX`"]
    #[inline(always)]
    pub fn is_dio_mode_uart_tx(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_UART_TX
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SCL`"]
    #[inline(always)]
    pub fn is_dio_mode_scl(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SCL
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_SDA`"]
    #[inline(always)]
    pub fn is_dio_mode_sda(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_SDA
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_PWM0`"]
    #[inline(always)]
    pub fn is_dio_mode_pwm0(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_PWM0
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_PWM0_INV`"]
    #[inline(always)]
    pub fn is_dio_mode_pwm0_inv(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_PWM0_INV
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_PWM1`"]
    #[inline(always)]
    pub fn is_dio_mode_pwm1(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_PWM1
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_PWM1_INV`"]
    #[inline(always)]
    pub fn is_dio_mode_pwm1_inv(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_PWM1_INV
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_LPDSP32_TDO`"]
    #[inline(always)]
    pub fn is_dio_mode_lpdsp32_tdo(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_LPDSP32_TDO
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RFCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_rfclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RFCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RCCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_rcclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RCCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_JTCK_DIV`"]
    #[inline(always)]
    pub fn is_dio_mode_jtck_div(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_JTCK_DIV
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_EXTCLK_DIV`"]
    #[inline(always)]
    pub fn is_dio_mode_extclk_div(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_EXTCLK_DIV
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_STANDBYCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_standbyclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_STANDBYCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_TX_DATA`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_tx_data(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_TX_DATA
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_TX_DATA_VALID`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_tx_data_valid(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_TX_DATA_VALID
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_SYNC_P`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_sync_p(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_SYNC_P
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_AUDIO0_SYNC_P`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_audio0_sync_p(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_AUDIO0_SYNC_P
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_AUDIO1_SYNC_P`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_audio1_sync_p(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_AUDIO1_SYNC_P
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_AUDIO2_SYNC_P`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_audio2_sync_p(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_AUDIO2_SYNC_P
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_SPI_CSN`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_spi_csn(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_SPI_CSN
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_SPI_CLK`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_spi_clk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_SPI_CLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_spi_mosi(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_SPI_MOSI
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_0`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_0(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_0
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_1`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_1(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_1
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_2`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_2(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_2
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_3`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_3(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_3
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_4`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_4(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_4
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_5`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_5(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_5
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_6`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_6(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_6
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_BB_DBG0_7`"]
    #[inline(always)]
    pub fn is_dio_mode_bb_dbg0_7(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_BB_DBG0_7
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_SPI_MISO`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_spi_miso(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO0`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio0(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO0
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO1`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio1(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO1
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO2`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio2(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO2
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO3`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio3(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO3
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO4`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio4(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO4
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO5`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio5(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO5
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO6`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio6(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO6
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO7`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio7(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO7
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO8`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio8(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO8
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_RF_GPIO9`"]
    #[inline(always)]
    pub fn is_dio_mode_rf_gpio9(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_RF_GPIO9
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_AUDIOCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_audioclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_AUDIOCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_AUDIOSLOWCLK`"]
    #[inline(always)]
    pub fn is_dio_mode_audioslowclk(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_AUDIOSLOWCLK
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_OD_P`"]
    #[inline(always)]
    pub fn is_dio_mode_od_p(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_OD_P
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_OD_N`"]
    #[inline(always)]
    pub fn is_dio_mode_od_n(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_OD_N
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_AUDIO_SYNC_PULSE`"]
    #[inline(always)]
    pub fn is_dio_mode_audio_sync_pulse(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_AUDIO_SYNC_PULSE
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_AUDIO_SYNC_MISSED`"]
    #[inline(always)]
    pub fn is_dio_mode_audio_sync_missed(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_AUDIO_SYNC_MISSED
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_INPUT`"]
    #[inline(always)]
    pub fn is_dio_mode_input(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_INPUT
    }
    #[doc = "Checks if the value of the field is `DIO_MODE_DISABLE`"]
    #[inline(always)]
    pub fn is_dio_mode_disable(&self) -> bool {
        *self == IO_MODE_A::DIO_MODE_DISABLE
    }
}
#[doc = "Write proxy for field `IO_MODE`"]
pub struct IO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input (GPIO input mode)"]
    #[inline(always)]
    pub fn dio_mode_gpio_in_0(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_GPIO_IN_0)
    }
    #[doc = "Input (GPIO input mode)"]
    #[inline(always)]
    pub fn dio_mode_gpio_in_1(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_GPIO_IN_1)
    }
    #[doc = "Output low (GPIO output mode)"]
    #[inline(always)]
    pub fn dio_mode_gpio_out_0(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_GPIO_OUT_0)
    }
    #[doc = "Output high (GPIO output mode)"]
    #[inline(always)]
    pub fn dio_mode_gpio_out_1(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_GPIO_OUT_1)
    }
    #[doc = "Output USRCLK (user clock) signal"]
    #[inline(always)]
    pub fn dio_mode_usrclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_USRCLK)
    }
    #[doc = "Output SLOWCLK (slow clock) signal"]
    #[inline(always)]
    pub fn dio_mode_slowclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SLOWCLK)
    }
    #[doc = "Output SYSCLK (system clock) signal"]
    #[inline(always)]
    pub fn dio_mode_sysclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SYSCLK)
    }
    #[doc = "Output PCM_SERO interface signal"]
    #[inline(always)]
    pub fn dio_mode_pcm_sero(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_PCM_SERO)
    }
    #[doc = "Output PCM_FRAME interface signal"]
    #[inline(always)]
    pub fn dio_mode_pcm_frame(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_PCM_FRAME)
    }
    #[doc = "Output SPI0_SERO interface signal"]
    #[inline(always)]
    pub fn dio_mode_spi0_sero(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SPI0_SERO)
    }
    #[doc = "Output SPI0_CS interface signal"]
    #[inline(always)]
    pub fn dio_mode_spi0_cs(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SPI0_CS)
    }
    #[doc = "Output SPI0_CLK interface signal"]
    #[inline(always)]
    pub fn dio_mode_spi0_clk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SPI0_CLK)
    }
    #[doc = "Output SPI1_SERO interface signal"]
    #[inline(always)]
    pub fn dio_mode_spi1_sero(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SPI1_SERO)
    }
    #[doc = "Output SPI1_CS interface signal"]
    #[inline(always)]
    pub fn dio_mode_spi1_cs(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SPI1_CS)
    }
    #[doc = "Output SPI1_CLK interface signal"]
    #[inline(always)]
    pub fn dio_mode_spi1_clk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SPI1_CLK)
    }
    #[doc = "Output UART_TX interface signal"]
    #[inline(always)]
    pub fn dio_mode_uart_tx(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_UART_TX)
    }
    #[doc = "Output SCL interface signal (open collector)"]
    #[inline(always)]
    pub fn dio_mode_scl(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SCL)
    }
    #[doc = "Output SDA interface signal (open collector)"]
    #[inline(always)]
    pub fn dio_mode_sda(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_SDA)
    }
    #[doc = "Output PWM0 interface signal"]
    #[inline(always)]
    pub fn dio_mode_pwm0(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_PWM0)
    }
    #[doc = "Output PWM0 interface signal inverted"]
    #[inline(always)]
    pub fn dio_mode_pwm0_inv(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_PWM0_INV)
    }
    #[doc = "Output PWM1 interface signal"]
    #[inline(always)]
    pub fn dio_mode_pwm1(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_PWM1)
    }
    #[doc = "Output PWM1 interface signal inverted"]
    #[inline(always)]
    pub fn dio_mode_pwm1_inv(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_PWM1_INV)
    }
    #[doc = "Output LPDSP32-TDO interface signal"]
    #[inline(always)]
    pub fn dio_mode_lpdsp32_tdo(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_LPDSP32_TDO)
    }
    #[doc = "Output RFCLK signal"]
    #[inline(always)]
    pub fn dio_mode_rfclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RFCLK)
    }
    #[doc = "Output RCCLK signal"]
    #[inline(always)]
    pub fn dio_mode_rcclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RCCLK)
    }
    #[doc = "Output of JTCK divider signal"]
    #[inline(always)]
    pub fn dio_mode_jtck_div(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_JTCK_DIV)
    }
    #[doc = "Output of EXTCLK divider signal"]
    #[inline(always)]
    pub fn dio_mode_extclk_div(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_EXTCLK_DIV)
    }
    #[doc = "Output STANDBYCLK signal"]
    #[inline(always)]
    pub fn dio_mode_standbyclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_STANDBYCLK)
    }
    #[doc = "Output baseband controller TX data signal"]
    #[inline(always)]
    pub fn dio_mode_bb_tx_data(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_TX_DATA)
    }
    #[doc = "Output baseband controller TX data valid signal"]
    #[inline(always)]
    pub fn dio_mode_bb_tx_data_valid(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_TX_DATA_VALID)
    }
    #[doc = "Output baseband controller BLE synchronization signal"]
    #[inline(always)]
    pub fn dio_mode_bb_sync_p(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_SYNC_P)
    }
    #[doc = "Output baseband controller BLE AUDIO0 synchronization signal"]
    #[inline(always)]
    pub fn dio_mode_bb_audio0_sync_p(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_AUDIO0_SYNC_P)
    }
    #[doc = "Output baseband controller BLE AUDIO1 synchronization signal"]
    #[inline(always)]
    pub fn dio_mode_bb_audio1_sync_p(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_AUDIO1_SYNC_P)
    }
    #[doc = "Output baseband controller BLE AUDIO2 synchronization signal"]
    #[inline(always)]
    pub fn dio_mode_bb_audio2_sync_p(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_AUDIO2_SYNC_P)
    }
    #[doc = "Output baseband controller SPI_CSN signal"]
    #[inline(always)]
    pub fn dio_mode_bb_spi_csn(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_SPI_CSN)
    }
    #[doc = "Output baseband controller SPI_CLK signal"]
    #[inline(always)]
    pub fn dio_mode_bb_spi_clk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_SPI_CLK)
    }
    #[doc = "Output baseband controller SPI_MOSI signal"]
    #[inline(always)]
    pub fn dio_mode_bb_spi_mosi(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_SPI_MOSI)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 0) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_0(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_0)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 1) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_1(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_1)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 2) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_2(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_2)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 3) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_3(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_3)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 4) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_4(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_4)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 5) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_5(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_5)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 6) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_6(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_6)
    }
    #[doc = "Output baseband controller diagnostic port 0 (bit 7) signal"]
    #[inline(always)]
    pub fn dio_mode_bb_dbg0_7(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_BB_DBG0_7)
    }
    #[doc = "Output RF front-end SPI_MISO interface signal"]
    #[inline(always)]
    pub fn dio_mode_rf_spi_miso(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_SPI_MISO)
    }
    #[doc = "Output RF front-end GPIO0 output (RX_DATA) signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio0(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO0)
    }
    #[doc = "Output RF front-end GPIO1 output (RX_CLK) signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio1(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO1)
    }
    #[doc = "Output RF front-end GPIO2 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio2(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO2)
    }
    #[doc = "Output RF front-end GPIO3 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio3(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO3)
    }
    #[doc = "Output RF front-end GPIO4 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio4(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO4)
    }
    #[doc = "Output RF front-end GPIO5 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio5(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO5)
    }
    #[doc = "Output RF front-end GPIO6 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio6(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO6)
    }
    #[doc = "Output RF front-end GPIO7 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio7(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO7)
    }
    #[doc = "Output RF front-end GPIO8 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio8(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO8)
    }
    #[doc = "Output RF front-end GPIO9 output signal"]
    #[inline(always)]
    pub fn dio_mode_rf_gpio9(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_RF_GPIO9)
    }
    #[doc = "Output the AUDIOCLK (audio clock) signal"]
    #[inline(always)]
    pub fn dio_mode_audioclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_AUDIOCLK)
    }
    #[doc = "Output the AUDIOSLOWCLK (slow audio clock) signal"]
    #[inline(always)]
    pub fn dio_mode_audioslowclk(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_AUDIOSLOWCLK)
    }
    #[doc = "Output OD + signal"]
    #[inline(always)]
    pub fn dio_mode_od_p(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_OD_P)
    }
    #[doc = "Output OD - signal"]
    #[inline(always)]
    pub fn dio_mode_od_n(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_OD_N)
    }
    #[doc = "Output audio synchronization pulse"]
    #[inline(always)]
    pub fn dio_mode_audio_sync_pulse(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_AUDIO_SYNC_PULSE)
    }
    #[doc = "Output audio synchronization missed pulse"]
    #[inline(always)]
    pub fn dio_mode_audio_sync_missed(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_AUDIO_SYNC_MISSED)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn dio_mode_input(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_INPUT)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dio_mode_disable(self) -> &'a mut W {
        self.variant(IO_MODE_A::DIO_MODE_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Drive strength configuration"]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Low Pass Filter enable"]
    #[inline(always)]
    pub fn lpf(&self) -> LPF_R {
        LPF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Pull selection"]
    #[inline(always)]
    pub fn pull_ctrl(&self) -> PULL_CTRL_R {
        PULL_CTRL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - IO mode selection"]
    #[inline(always)]
    pub fn io_mode(&self) -> IO_MODE_R {
        IO_MODE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Drive strength configuration"]
    #[inline(always)]
    pub fn drive(&mut self) -> DRIVE_W {
        DRIVE_W { w: self }
    }
    #[doc = "Bit 10 - Low Pass Filter enable"]
    #[inline(always)]
    pub fn lpf(&mut self) -> LPF_W {
        LPF_W { w: self }
    }
    #[doc = "Bits 8:9 - Pull selection"]
    #[inline(always)]
    pub fn pull_ctrl(&mut self) -> PULL_CTRL_W {
        PULL_CTRL_W { w: self }
    }
    #[doc = "Bits 0:5 - IO mode selection"]
    #[inline(always)]
    pub fn io_mode(&mut self) -> IO_MODE_W {
        IO_MODE_W { w: self }
    }
}
