#[doc = "Reader of register I2C_CTRL0"]
pub type R = crate::R<u32, super::I2C_CTRL0>;
#[doc = "Writer for register I2C_CTRL0"]
pub type W = crate::W<u32, super::I2C_CTRL0>;
#[doc = "Register I2C_CTRL0 `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::I2C_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Prescaler used to divide SYSCLK to the correct SCL frequency when operating in I2C interface master mode. SCL is prescaled by (SPEED + 1) * 3. In slave mode controls the number of SYSCLK wait cycles in case of clock streching between the moment the data is put on the SDA line and the SCL line is released.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Master mode: prescale SCL from SYSCLK by 3."]
    I2C_MASTER_SPEED_3 = 0,
    #[doc = "1: Master mode: prescale SCL from SYSCLK by 6."]
    I2C_MASTER_SPEED_6 = 1,
    #[doc = "2: Master mode: prescale SCL from SYSCLK by 9."]
    I2C_MASTER_SPEED_9 = 2,
    #[doc = "3: Master mode: prescale SCL from SYSCLK by 12."]
    I2C_MASTER_SPEED_12 = 3,
    #[doc = "4: Master mode: prescale SCL from SYSCLK by 15."]
    I2C_MASTER_SPEED_15 = 4,
    #[doc = "5: Master mode: prescale SCL from SYSCLK by 18."]
    I2C_MASTER_SPEED_18 = 5,
    #[doc = "6: Master mode: prescale SCL from SYSCLK by 21."]
    I2C_MASTER_SPEED_21 = 6,
    #[doc = "7: Master mode: prescale SCL from SYSCLK by 24."]
    I2C_MASTER_SPEED_24 = 7,
    #[doc = "8: Master mode: prescale SCL from SYSCLK by 27."]
    I2C_MASTER_SPEED_27 = 8,
    #[doc = "9: Master mode: prescale SCL from SYSCLK by 30."]
    I2C_MASTER_SPEED_30 = 9,
    #[doc = "10: Master mode: prescale SCL from SYSCLK by 33."]
    I2C_MASTER_SPEED_33 = 10,
    #[doc = "11: Master mode: prescale SCL from SYSCLK by 36."]
    I2C_MASTER_SPEED_36 = 11,
    #[doc = "12: Master mode: prescale SCL from SYSCLK by 39."]
    I2C_MASTER_SPEED_39 = 12,
    #[doc = "13: Master mode: prescale SCL from SYSCLK by 42."]
    I2C_MASTER_SPEED_42 = 13,
    #[doc = "14: Master mode: prescale SCL from SYSCLK by 45."]
    I2C_MASTER_SPEED_45 = 14,
    #[doc = "15: Master mode: prescale SCL from SYSCLK by 48."]
    I2C_MASTER_SPEED_48 = 15,
    #[doc = "16: Master mode: prescale SCL from SYSCLK by 51."]
    I2C_MASTER_SPEED_51 = 16,
    #[doc = "17: Master mode: prescale SCL from SYSCLK by 54."]
    I2C_MASTER_SPEED_54 = 17,
    #[doc = "18: Master mode: prescale SCL from SYSCLK by 57."]
    I2C_MASTER_SPEED_57 = 18,
    #[doc = "19: Master mode: prescale SCL from SYSCLK by 60."]
    I2C_MASTER_SPEED_60 = 19,
    #[doc = "39: Master mode: prescale SCL from SYSCLK by 120."]
    I2C_MASTER_SPEED_120 = 39,
    #[doc = "255: Master mode: prescale SCL from SYSCLK by 768."]
    I2C_MASTER_SPEED_768 = 255,
    #[doc = "0: Slave Standard-mode: at least 250 ns +10 percent data set-up time with SYSCLK = 3 MHz; Slave Fast-mode: at least 100 ns +10 percent data set-up time with SYSCLK = 3, 4, 5, 8 MHz; Slave Fast-mode Plus: at least 50 ns +10 percent data set-up time with SYSCLK = 3, 4, 5, 8, 10, 12, 16 MHz"]
    I2C_SLAVE_SPEED_1 = 0,
    #[doc = "1: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 4, 5 MHz; Slave Fast-Mode: at least 100 ns +10 percent data set-up time with SYSCLK = 10, 12, 16 MHz; Slave Fast-mode Plus: at least 50 ns +10 percent data set-up time with SYSCLK = 20, 24 MHz"]
    I2C_SLAVE_SPEED_2 = 1,
    #[doc = "2: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 8, 10 MHz; Slave Fast-Mode: at least 100 ns +10 percent data set-up time with SYSCLK = 20, 24 MHz; Slave Fast-mode Plus: at least 50 ns +10 percent data set-up time with SYSCLK = 48 MHz"]
    I2C_SLAVE_SPEED_3 = 2,
    #[doc = "3: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 12 MHz"]
    I2C_SLAVE_SPEED_4 = 3,
    #[doc = "4: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 16 MHz"]
    I2C_SLAVE_SPEED_5 = 4,
    #[doc = "5: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 20 MHz; Slave Fast-Mode: at least 100 ns +10 percent data set-up time with SYSCLK = 48 MHz"]
    I2C_SLAVE_SPEED_6 = 5,
    #[doc = "6: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 24 MHz"]
    I2C_SLAVE_SPEED_7 = 6,
    #[doc = "13: Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 48 MHz"]
    I2C_SLAVE_SPEED_14 = 13,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPEED_A::I2C_MASTER_SPEED_3),
            1 => Val(SPEED_A::I2C_MASTER_SPEED_6),
            2 => Val(SPEED_A::I2C_MASTER_SPEED_9),
            3 => Val(SPEED_A::I2C_MASTER_SPEED_12),
            4 => Val(SPEED_A::I2C_MASTER_SPEED_15),
            5 => Val(SPEED_A::I2C_MASTER_SPEED_18),
            6 => Val(SPEED_A::I2C_MASTER_SPEED_21),
            7 => Val(SPEED_A::I2C_MASTER_SPEED_24),
            8 => Val(SPEED_A::I2C_MASTER_SPEED_27),
            9 => Val(SPEED_A::I2C_MASTER_SPEED_30),
            10 => Val(SPEED_A::I2C_MASTER_SPEED_33),
            11 => Val(SPEED_A::I2C_MASTER_SPEED_36),
            12 => Val(SPEED_A::I2C_MASTER_SPEED_39),
            13 => Val(SPEED_A::I2C_MASTER_SPEED_42),
            14 => Val(SPEED_A::I2C_MASTER_SPEED_45),
            15 => Val(SPEED_A::I2C_MASTER_SPEED_48),
            16 => Val(SPEED_A::I2C_MASTER_SPEED_51),
            17 => Val(SPEED_A::I2C_MASTER_SPEED_54),
            18 => Val(SPEED_A::I2C_MASTER_SPEED_57),
            19 => Val(SPEED_A::I2C_MASTER_SPEED_60),
            39 => Val(SPEED_A::I2C_MASTER_SPEED_120),
            255 => Val(SPEED_A::I2C_MASTER_SPEED_768),
            0 => Val(SPEED_A::I2C_SLAVE_SPEED_1),
            1 => Val(SPEED_A::I2C_SLAVE_SPEED_2),
            2 => Val(SPEED_A::I2C_SLAVE_SPEED_3),
            3 => Val(SPEED_A::I2C_SLAVE_SPEED_4),
            4 => Val(SPEED_A::I2C_SLAVE_SPEED_5),
            5 => Val(SPEED_A::I2C_SLAVE_SPEED_6),
            6 => Val(SPEED_A::I2C_SLAVE_SPEED_7),
            13 => Val(SPEED_A::I2C_SLAVE_SPEED_14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_3`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_3(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_3
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_6`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_6(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_6
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_9`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_9(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_9
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_12`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_12(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_12
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_15`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_15(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_15
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_18`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_18(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_18
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_21`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_21(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_21
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_24`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_24(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_24
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_27`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_27(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_27
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_30`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_30(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_30
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_33`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_33(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_33
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_36`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_36(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_36
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_39`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_39(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_39
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_42`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_42(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_42
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_45`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_45(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_45
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_48`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_48(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_48
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_51`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_51(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_51
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_54`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_54(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_54
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_57`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_57(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_57
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_60`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_60(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_60
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_120`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_120(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_120
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_SPEED_768`"]
    #[inline(always)]
    pub fn is_i2c_master_speed_768(&self) -> bool {
        *self == SPEED_A::I2C_MASTER_SPEED_768
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_1`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_1(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_1
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_2`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_2(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_2
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_3`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_3(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_3
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_4`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_4(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_4
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_5`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_5(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_5
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_6`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_6(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_6
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_7`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_7(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_7
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_SPEED_14`"]
    #[inline(always)]
    pub fn is_i2c_slave_speed_14(&self) -> bool {
        *self == SPEED_A::I2C_SLAVE_SPEED_14
    }
}
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 3."]
    #[inline(always)]
    pub fn i2c_master_speed_3(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_3)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 6."]
    #[inline(always)]
    pub fn i2c_master_speed_6(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_6)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 9."]
    #[inline(always)]
    pub fn i2c_master_speed_9(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_9)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 12."]
    #[inline(always)]
    pub fn i2c_master_speed_12(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_12)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 15."]
    #[inline(always)]
    pub fn i2c_master_speed_15(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_15)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 18."]
    #[inline(always)]
    pub fn i2c_master_speed_18(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_18)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 21."]
    #[inline(always)]
    pub fn i2c_master_speed_21(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_21)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 24."]
    #[inline(always)]
    pub fn i2c_master_speed_24(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_24)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 27."]
    #[inline(always)]
    pub fn i2c_master_speed_27(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_27)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 30."]
    #[inline(always)]
    pub fn i2c_master_speed_30(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_30)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 33."]
    #[inline(always)]
    pub fn i2c_master_speed_33(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_33)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 36."]
    #[inline(always)]
    pub fn i2c_master_speed_36(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_36)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 39."]
    #[inline(always)]
    pub fn i2c_master_speed_39(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_39)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 42."]
    #[inline(always)]
    pub fn i2c_master_speed_42(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_42)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 45."]
    #[inline(always)]
    pub fn i2c_master_speed_45(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_45)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 48."]
    #[inline(always)]
    pub fn i2c_master_speed_48(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_48)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 51."]
    #[inline(always)]
    pub fn i2c_master_speed_51(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_51)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 54."]
    #[inline(always)]
    pub fn i2c_master_speed_54(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_54)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 57."]
    #[inline(always)]
    pub fn i2c_master_speed_57(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_57)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 60."]
    #[inline(always)]
    pub fn i2c_master_speed_60(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_60)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 120."]
    #[inline(always)]
    pub fn i2c_master_speed_120(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_120)
    }
    #[doc = "Master mode: prescale SCL from SYSCLK by 768."]
    #[inline(always)]
    pub fn i2c_master_speed_768(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_MASTER_SPEED_768)
    }
    #[doc = "Slave Standard-mode: at least 250 ns +10 percent data set-up time with SYSCLK = 3 MHz; Slave Fast-mode: at least 100 ns +10 percent data set-up time with SYSCLK = 3, 4, 5, 8 MHz; Slave Fast-mode Plus: at least 50 ns +10 percent data set-up time with SYSCLK = 3, 4, 5, 8, 10, 12, 16 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_1(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_1)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 4, 5 MHz; Slave Fast-Mode: at least 100 ns +10 percent data set-up time with SYSCLK = 10, 12, 16 MHz; Slave Fast-mode Plus: at least 50 ns +10 percent data set-up time with SYSCLK = 20, 24 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_2(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_2)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 8, 10 MHz; Slave Fast-Mode: at least 100 ns +10 percent data set-up time with SYSCLK = 20, 24 MHz; Slave Fast-mode Plus: at least 50 ns +10 percent data set-up time with SYSCLK = 48 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_3(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_3)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 12 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_4(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_4)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 16 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_5(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_5)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 20 MHz; Slave Fast-Mode: at least 100 ns +10 percent data set-up time with SYSCLK = 48 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_6(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_6)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 24 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_7(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_7)
    }
    #[doc = "Slave Standard-Mode: at least 250 ns +10 percent data set-up time with SYSCLK = 48 MHz"]
    #[inline(always)]
    pub fn i2c_slave_speed_14(self) -> &'a mut W {
        self.variant(SPEED_A::I2C_SLAVE_SPEED_14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_ADDRESS`"]
pub type SLAVE_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLAVE_ADDRESS`"]
pub struct SLAVE_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Select whether data transfer will be controlled by the CM3 or the DMA for I2C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTROLLER_A {
    #[doc = "0: The CM3 controls data transfers using I2C"]
    I2C_CONTROLLER_CM3 = 0,
    #[doc = "1: The DMA controls data transfers using I2C"]
    I2C_CONTROLLER_DMA = 1,
}
impl From<CONTROLLER_A> for bool {
    #[inline(always)]
    fn from(variant: CONTROLLER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONTROLLER`"]
pub type CONTROLLER_R = crate::R<bool, CONTROLLER_A>;
impl CONTROLLER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTROLLER_A {
        match self.bits {
            false => CONTROLLER_A::I2C_CONTROLLER_CM3,
            true => CONTROLLER_A::I2C_CONTROLLER_DMA,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_CONTROLLER_CM3`"]
    #[inline(always)]
    pub fn is_i2c_controller_cm3(&self) -> bool {
        *self == CONTROLLER_A::I2C_CONTROLLER_CM3
    }
    #[doc = "Checks if the value of the field is `I2C_CONTROLLER_DMA`"]
    #[inline(always)]
    pub fn is_i2c_controller_dma(&self) -> bool {
        *self == CONTROLLER_A::I2C_CONTROLLER_DMA
    }
}
#[doc = "Write proxy for field `CONTROLLER`"]
pub struct CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROLLER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTROLLER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CM3 controls data transfers using I2C"]
    #[inline(always)]
    pub fn i2c_controller_cm3(self) -> &'a mut W {
        self.variant(CONTROLLER_A::I2C_CONTROLLER_CM3)
    }
    #[doc = "The DMA controls data transfers using I2C"]
    #[inline(always)]
    pub fn i2c_controller_dma(self) -> &'a mut W {
        self.variant(CONTROLLER_A::I2C_CONTROLLER_DMA)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Configure whether stop interrupts will be generated by the I2C interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_INT_ENABLE_A {
    #[doc = "0: Stop interrupts will not be generated"]
    I2C_STOP_INT_DISABLE = 0,
    #[doc = "1: A stop interrupt will be generated when a stop condition occurs for an active transaction"]
    I2C_STOP_INT_ENABLE = 1,
}
impl From<STOP_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOP_INT_ENABLE`"]
pub type STOP_INT_ENABLE_R = crate::R<bool, STOP_INT_ENABLE_A>;
impl STOP_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_INT_ENABLE_A {
        match self.bits {
            false => STOP_INT_ENABLE_A::I2C_STOP_INT_DISABLE,
            true => STOP_INT_ENABLE_A::I2C_STOP_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_STOP_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_i2c_stop_int_disable(&self) -> bool {
        *self == STOP_INT_ENABLE_A::I2C_STOP_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `I2C_STOP_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_i2c_stop_int_enable(&self) -> bool {
        *self == STOP_INT_ENABLE_A::I2C_STOP_INT_ENABLE
    }
}
#[doc = "Write proxy for field `STOP_INT_ENABLE`"]
pub struct STOP_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop interrupts will not be generated"]
    #[inline(always)]
    pub fn i2c_stop_int_disable(self) -> &'a mut W {
        self.variant(STOP_INT_ENABLE_A::I2C_STOP_INT_DISABLE)
    }
    #[doc = "A stop interrupt will be generated when a stop condition occurs for an active transaction"]
    #[inline(always)]
    pub fn i2c_stop_int_enable(self) -> &'a mut W {
        self.variant(STOP_INT_ENABLE_A::I2C_STOP_INT_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Select whether acknowledgement is automatically generated or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_ACK_ENABLE_A {
    #[doc = "0: Require manual acknowledgement of all I2C interface transfers"]
    I2C_AUTO_ACK_DISABLE = 0,
    #[doc = "1: Use automatic acknowledgement for I2C interface transfers"]
    I2C_AUTO_ACK_ENABLE = 1,
}
impl From<AUTO_ACK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_ACK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTO_ACK_ENABLE`"]
pub type AUTO_ACK_ENABLE_R = crate::R<bool, AUTO_ACK_ENABLE_A>;
impl AUTO_ACK_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_ACK_ENABLE_A {
        match self.bits {
            false => AUTO_ACK_ENABLE_A::I2C_AUTO_ACK_DISABLE,
            true => AUTO_ACK_ENABLE_A::I2C_AUTO_ACK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_AUTO_ACK_DISABLE`"]
    #[inline(always)]
    pub fn is_i2c_auto_ack_disable(&self) -> bool {
        *self == AUTO_ACK_ENABLE_A::I2C_AUTO_ACK_DISABLE
    }
    #[doc = "Checks if the value of the field is `I2C_AUTO_ACK_ENABLE`"]
    #[inline(always)]
    pub fn is_i2c_auto_ack_enable(&self) -> bool {
        *self == AUTO_ACK_ENABLE_A::I2C_AUTO_ACK_ENABLE
    }
}
#[doc = "Write proxy for field `AUTO_ACK_ENABLE`"]
pub struct AUTO_ACK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_ACK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_ACK_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Require manual acknowledgement of all I2C interface transfers"]
    #[inline(always)]
    pub fn i2c_auto_ack_disable(self) -> &'a mut W {
        self.variant(AUTO_ACK_ENABLE_A::I2C_AUTO_ACK_DISABLE)
    }
    #[doc = "Use automatic acknowledgement for I2C interface transfers"]
    #[inline(always)]
    pub fn i2c_auto_ack_enable(self) -> &'a mut W {
        self.variant(AUTO_ACK_ENABLE_A::I2C_AUTO_ACK_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable/disable the I2C sample clock (mandatory to enable the I2C)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_SAMPLE_CLK_ENABLE_A {
    #[doc = "0: Disable the I2C sample clock (I2C is disabled)"]
    I2C_SAMPLE_CLK_DISABLE = 0,
    #[doc = "1: Enable the I2C sample clock (I2C is enabled)"]
    I2C_SAMPLE_CLK_ENABLE = 1,
}
impl From<I2C_SAMPLE_CLK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_SAMPLE_CLK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_SAMPLE_CLK_ENABLE`"]
pub type I2C_SAMPLE_CLK_ENABLE_R = crate::R<bool, I2C_SAMPLE_CLK_ENABLE_A>;
impl I2C_SAMPLE_CLK_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_SAMPLE_CLK_ENABLE_A {
        match self.bits {
            false => I2C_SAMPLE_CLK_ENABLE_A::I2C_SAMPLE_CLK_DISABLE,
            true => I2C_SAMPLE_CLK_ENABLE_A::I2C_SAMPLE_CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_SAMPLE_CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_i2c_sample_clk_disable(&self) -> bool {
        *self == I2C_SAMPLE_CLK_ENABLE_A::I2C_SAMPLE_CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `I2C_SAMPLE_CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_i2c_sample_clk_enable(&self) -> bool {
        *self == I2C_SAMPLE_CLK_ENABLE_A::I2C_SAMPLE_CLK_ENABLE
    }
}
#[doc = "Write proxy for field `I2C_SAMPLE_CLK_ENABLE`"]
pub struct I2C_SAMPLE_CLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SAMPLE_CLK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_SAMPLE_CLK_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the I2C sample clock (I2C is disabled)"]
    #[inline(always)]
    pub fn i2c_sample_clk_disable(self) -> &'a mut W {
        self.variant(I2C_SAMPLE_CLK_ENABLE_A::I2C_SAMPLE_CLK_DISABLE)
    }
    #[doc = "Enable the I2C sample clock (I2C is enabled)"]
    #[inline(always)]
    pub fn i2c_sample_clk_enable(self) -> &'a mut W {
        self.variant(I2C_SAMPLE_CLK_ENABLE_A::I2C_SAMPLE_CLK_ENABLE)
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
#[doc = "Select whether the I2C interface will be enabled for slave mode or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_ENABLE_A {
    #[doc = "0: Disable I2C interface slave mode operation"]
    I2C_SLAVE_DISABLE = 0,
    #[doc = "1: Enable I2C interface slave mode operation"]
    I2C_SLAVE_ENABLE = 1,
}
impl From<SLAVE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLAVE_ENABLE`"]
pub type SLAVE_ENABLE_R = crate::R<bool, SLAVE_ENABLE_A>;
impl SLAVE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_ENABLE_A {
        match self.bits {
            false => SLAVE_ENABLE_A::I2C_SLAVE_DISABLE,
            true => SLAVE_ENABLE_A::I2C_SLAVE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_DISABLE`"]
    #[inline(always)]
    pub fn is_i2c_slave_disable(&self) -> bool {
        *self == SLAVE_ENABLE_A::I2C_SLAVE_DISABLE
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE_ENABLE`"]
    #[inline(always)]
    pub fn is_i2c_slave_enable(&self) -> bool {
        *self == SLAVE_ENABLE_A::I2C_SLAVE_ENABLE
    }
}
#[doc = "Write proxy for field `SLAVE_ENABLE`"]
pub struct SLAVE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable I2C interface slave mode operation"]
    #[inline(always)]
    pub fn i2c_slave_disable(self) -> &'a mut W {
        self.variant(SLAVE_ENABLE_A::I2C_SLAVE_DISABLE)
    }
    #[doc = "Enable I2C interface slave mode operation"]
    #[inline(always)]
    pub fn i2c_slave_enable(self) -> &'a mut W {
        self.variant(SLAVE_ENABLE_A::I2C_SLAVE_ENABLE)
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
    #[doc = "Bits 16:23 - Prescaler used to divide SYSCLK to the correct SCL frequency when operating in I2C interface master mode. SCL is prescaled by (SPEED + 1) * 3. In slave mode controls the number of SYSCLK wait cycles in case of clock streching between the moment the data is put on the SDA line and the SCL line is released."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Set the I2C slave address for this device"]
    #[inline(always)]
    pub fn slave_address(&self) -> SLAVE_ADDRESS_R {
        SLAVE_ADDRESS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 4 - Select whether data transfer will be controlled by the CM3 or the DMA for I2C"]
    #[inline(always)]
    pub fn controller(&self) -> CONTROLLER_R {
        CONTROLLER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configure whether stop interrupts will be generated by the I2C interface"]
    #[inline(always)]
    pub fn stop_int_enable(&self) -> STOP_INT_ENABLE_R {
        STOP_INT_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select whether acknowledgement is automatically generated or not"]
    #[inline(always)]
    pub fn auto_ack_enable(&self) -> AUTO_ACK_ENABLE_R {
        AUTO_ACK_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable/disable the I2C sample clock (mandatory to enable the I2C)"]
    #[inline(always)]
    pub fn i2c_sample_clk_enable(&self) -> I2C_SAMPLE_CLK_ENABLE_R {
        I2C_SAMPLE_CLK_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Select whether the I2C interface will be enabled for slave mode or not"]
    #[inline(always)]
    pub fn slave_enable(&self) -> SLAVE_ENABLE_R {
        SLAVE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Prescaler used to divide SYSCLK to the correct SCL frequency when operating in I2C interface master mode. SCL is prescaled by (SPEED + 1) * 3. In slave mode controls the number of SYSCLK wait cycles in case of clock streching between the moment the data is put on the SDA line and the SCL line is released."]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bits 8:14 - Set the I2C slave address for this device"]
    #[inline(always)]
    pub fn slave_address(&mut self) -> SLAVE_ADDRESS_W {
        SLAVE_ADDRESS_W { w: self }
    }
    #[doc = "Bit 4 - Select whether data transfer will be controlled by the CM3 or the DMA for I2C"]
    #[inline(always)]
    pub fn controller(&mut self) -> CONTROLLER_W {
        CONTROLLER_W { w: self }
    }
    #[doc = "Bit 3 - Configure whether stop interrupts will be generated by the I2C interface"]
    #[inline(always)]
    pub fn stop_int_enable(&mut self) -> STOP_INT_ENABLE_W {
        STOP_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Select whether acknowledgement is automatically generated or not"]
    #[inline(always)]
    pub fn auto_ack_enable(&mut self) -> AUTO_ACK_ENABLE_W {
        AUTO_ACK_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable/disable the I2C sample clock (mandatory to enable the I2C)"]
    #[inline(always)]
    pub fn i2c_sample_clk_enable(&mut self) -> I2C_SAMPLE_CLK_ENABLE_W {
        I2C_SAMPLE_CLK_ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - Select whether the I2C interface will be enabled for slave mode or not"]
    #[inline(always)]
    pub fn slave_enable(&mut self) -> SLAVE_ENABLE_W {
        SLAVE_ENABLE_W { w: self }
    }
}
