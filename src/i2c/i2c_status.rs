#[doc = "Reader of register I2C_STATUS"]
pub type R = crate::R<u32, super::I2C_STATUS>;
#[doc = "Same as BUS_ERROR_S: Bus error status bit (sticky) - This status bit is automatically reset when the I2C_STATUS register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_S_A {
    #[doc = "0: I2C interface is not and has not been in the bus error state"]
    I2C_NO_ERROR_S = 0,
    #[doc = "1: I2C interface is or has been in the bus error state"]
    I2C_ERROR_S = 1,
}
impl From<ERROR_S_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR_S`"]
pub type ERROR_S_R = crate::R<bool, ERROR_S_A>;
impl ERROR_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_S_A {
        match self.bits {
            false => ERROR_S_A::I2C_NO_ERROR_S,
            true => ERROR_S_A::I2C_ERROR_S,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NO_ERROR_S`"]
    #[inline(always)]
    pub fn is_i2c_no_error_s(&self) -> bool {
        *self == ERROR_S_A::I2C_NO_ERROR_S
    }
    #[doc = "Checks if the value of the field is `I2C_ERROR_S`"]
    #[inline(always)]
    pub fn is_i2c_error_s(&self) -> bool {
        *self == ERROR_S_A::I2C_ERROR_S
    }
}
#[doc = "Bus error status bit (sticky) - This status bit is automatically reset when the I2C_STATUS register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_ERROR_S_A {
    #[doc = "0: I2C interface is not and has not been in the bus error state"]
    I2C_NO_BUS_ERROR_S = 0,
    #[doc = "1: I2C interface is or has been in the bus error state"]
    I2C_BUS_ERROR_S = 1,
}
impl From<BUS_ERROR_S_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERROR_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUS_ERROR_S`"]
pub type BUS_ERROR_S_R = crate::R<bool, BUS_ERROR_S_A>;
impl BUS_ERROR_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_ERROR_S_A {
        match self.bits {
            false => BUS_ERROR_S_A::I2C_NO_BUS_ERROR_S,
            true => BUS_ERROR_S_A::I2C_BUS_ERROR_S,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NO_BUS_ERROR_S`"]
    #[inline(always)]
    pub fn is_i2c_no_bus_error_s(&self) -> bool {
        *self == BUS_ERROR_S_A::I2C_NO_BUS_ERROR_S
    }
    #[doc = "Checks if the value of the field is `I2C_BUS_ERROR_S`"]
    #[inline(always)]
    pub fn is_i2c_bus_error_s(&self) -> bool {
        *self == BUS_ERROR_S_A::I2C_BUS_ERROR_S
    }
}
#[doc = "Master frame start pending status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_PENDING_A {
    #[doc = "0: No pending master start frame"]
    I2C_START_NOT_PENDING = 0,
    #[doc = "1: A master frame is pending to start (bit is set when I2C_ADDR_START is written)"]
    I2C_START_PENDING = 1,
}
impl From<START_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: START_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START_PENDING`"]
pub type START_PENDING_R = crate::R<bool, START_PENDING_A>;
impl START_PENDING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_PENDING_A {
        match self.bits {
            false => START_PENDING_A::I2C_START_NOT_PENDING,
            true => START_PENDING_A::I2C_START_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_START_NOT_PENDING`"]
    #[inline(always)]
    pub fn is_i2c_start_not_pending(&self) -> bool {
        *self == START_PENDING_A::I2C_START_NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `I2C_START_PENDING`"]
    #[inline(always)]
    pub fn is_i2c_start_pending(&self) -> bool {
        *self == START_PENDING_A::I2C_START_PENDING
    }
}
#[doc = "Master mode status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_MODE_A {
    #[doc = "0: I2C interface is not operating in master mode"]
    I2C_MASTER_INACTIVE = 0,
    #[doc = "1: I2C interface is operating in master mode"]
    I2C_MASTER_ACTIVE = 1,
}
impl From<MASTER_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTER_MODE`"]
pub type MASTER_MODE_R = crate::R<bool, MASTER_MODE_A>;
impl MASTER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_MODE_A {
        match self.bits {
            false => MASTER_MODE_A::I2C_MASTER_INACTIVE,
            true => MASTER_MODE_A::I2C_MASTER_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_INACTIVE`"]
    #[inline(always)]
    pub fn is_i2c_master_inactive(&self) -> bool {
        *self == MASTER_MODE_A::I2C_MASTER_INACTIVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER_ACTIVE`"]
    #[inline(always)]
    pub fn is_i2c_master_active(&self) -> bool {
        *self == MASTER_MODE_A::I2C_MASTER_ACTIVE
    }
}
#[doc = "Indicate if the I2C interface is currently requesting DMA data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REQ_A {
    #[doc = "0: The I2C interface is not requesting a DMA action"]
    I2C_NO_DMA_REQUEST = 0,
    #[doc = "1: The I2C interface is requesting a DMA action"]
    I2C_DMA_REQUEST = 1,
}
impl From<DMA_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_REQ`"]
pub type DMA_REQ_R = crate::R<bool, DMA_REQ_A>;
impl DMA_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_REQ_A {
        match self.bits {
            false => DMA_REQ_A::I2C_NO_DMA_REQUEST,
            true => DMA_REQ_A::I2C_DMA_REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NO_DMA_REQUEST`"]
    #[inline(always)]
    pub fn is_i2c_no_dma_request(&self) -> bool {
        *self == DMA_REQ_A::I2C_NO_DMA_REQUEST
    }
    #[doc = "Checks if the value of the field is `I2C_DMA_REQUEST`"]
    #[inline(always)]
    pub fn is_i2c_dma_request(&self) -> bool {
        *self == DMA_REQ_A::I2C_DMA_REQUEST
    }
}
#[doc = "Indicate if an I2C stop bit has been detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_DETECT_A {
    #[doc = "0: No stop condition has been detected on the I2C bus"]
    I2C_NO_STOP_DETECTED = 0,
    #[doc = "1: A stop condition has been detected on the I2C bus"]
    I2C_STOP_DETECTED = 1,
}
impl From<STOP_DETECT_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_DETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOP_DETECT`"]
pub type STOP_DETECT_R = crate::R<bool, STOP_DETECT_A>;
impl STOP_DETECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_DETECT_A {
        match self.bits {
            false => STOP_DETECT_A::I2C_NO_STOP_DETECTED,
            true => STOP_DETECT_A::I2C_STOP_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NO_STOP_DETECTED`"]
    #[inline(always)]
    pub fn is_i2c_no_stop_detected(&self) -> bool {
        *self == STOP_DETECT_A::I2C_NO_STOP_DETECTED
    }
    #[doc = "Checks if the value of the field is `I2C_STOP_DETECTED`"]
    #[inline(always)]
    pub fn is_i2c_stop_detected(&self) -> bool {
        *self == STOP_DETECT_A::I2C_STOP_DETECTED
    }
}
#[doc = "Indicate that I2C interface either needs data to transmit or has received data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_EVENT_A {
    #[doc = "0: No I2C data is needed or available"]
    I2C_NON_DATA_EVENT = 0,
    #[doc = "1: I2C data is needed or is available"]
    I2C_DATA_EVENT = 1,
}
impl From<DATA_EVENT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_EVENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_EVENT`"]
pub type DATA_EVENT_R = crate::R<bool, DATA_EVENT_A>;
impl DATA_EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_EVENT_A {
        match self.bits {
            false => DATA_EVENT_A::I2C_NON_DATA_EVENT,
            true => DATA_EVENT_A::I2C_DATA_EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NON_DATA_EVENT`"]
    #[inline(always)]
    pub fn is_i2c_non_data_event(&self) -> bool {
        *self == DATA_EVENT_A::I2C_NON_DATA_EVENT
    }
    #[doc = "Checks if the value of the field is `I2C_DATA_EVENT`"]
    #[inline(always)]
    pub fn is_i2c_data_event(&self) -> bool {
        *self == DATA_EVENT_A::I2C_DATA_EVENT
    }
}
#[doc = "Same as BUS_ERROR: Indicate if the I2C interface has detected a bus error (automatically cleared when a stop condition is detected)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: I2C interface is not in the bus error state"]
    I2C_NO_ERROR = 0,
    #[doc = "1: I2C interface is in the bus error state"]
    I2C_ERROR = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::I2C_NO_ERROR,
            true => ERROR_A::I2C_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NO_ERROR`"]
    #[inline(always)]
    pub fn is_i2c_no_error(&self) -> bool {
        *self == ERROR_A::I2C_NO_ERROR
    }
    #[doc = "Checks if the value of the field is `I2C_ERROR`"]
    #[inline(always)]
    pub fn is_i2c_error(&self) -> bool {
        *self == ERROR_A::I2C_ERROR
    }
}
#[doc = "Indicate if the I2C interface has detected a bus error (automatically cleared when a stop condition is detected)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_ERROR_A {
    #[doc = "0: I2C interface is not in the bus error state"]
    I2C_NO_BUS_ERROR = 0,
    #[doc = "1: I2C interface is in the bus error state"]
    I2C_BUS_ERROR = 1,
}
impl From<BUS_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUS_ERROR`"]
pub type BUS_ERROR_R = crate::R<bool, BUS_ERROR_A>;
impl BUS_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_ERROR_A {
        match self.bits {
            false => BUS_ERROR_A::I2C_NO_BUS_ERROR,
            true => BUS_ERROR_A::I2C_BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_NO_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_i2c_no_bus_error(&self) -> bool {
        *self == BUS_ERROR_A::I2C_NO_BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `I2C_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_i2c_bus_error(&self) -> bool {
        *self == BUS_ERROR_A::I2C_BUS_ERROR
    }
}
#[doc = "Indicate if the I2C data buffer is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_FULL_A {
    #[doc = "0: The I2C interface buffer is empty"]
    I2C_BUFFER_EMPTY = 0,
    #[doc = "1: The I2C interface buffer is full"]
    I2C_BUFFER_FULL = 1,
}
impl From<BUFFER_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUFFER_FULL`"]
pub type BUFFER_FULL_R = crate::R<bool, BUFFER_FULL_A>;
impl BUFFER_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_FULL_A {
        match self.bits {
            false => BUFFER_FULL_A::I2C_BUFFER_EMPTY,
            true => BUFFER_FULL_A::I2C_BUFFER_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_BUFFER_EMPTY`"]
    #[inline(always)]
    pub fn is_i2c_buffer_empty(&self) -> bool {
        *self == BUFFER_FULL_A::I2C_BUFFER_EMPTY
    }
    #[doc = "Checks if the value of the field is `I2C_BUFFER_FULL`"]
    #[inline(always)]
    pub fn is_i2c_buffer_full(&self) -> bool {
        *self == BUFFER_FULL_A::I2C_BUFFER_FULL
    }
}
#[doc = "Indicate if the I2C interface is holding the clock signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_STRETCH_A {
    #[doc = "0: The I2C clock line is not being stretched"]
    I2C_CLK_NOT_STRETCHED = 0,
    #[doc = "1: The I2C SCL line is being held low"]
    I2C_CLK_STRETCHED = 1,
}
impl From<CLK_STRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_STRETCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_STRETCH`"]
pub type CLK_STRETCH_R = crate::R<bool, CLK_STRETCH_A>;
impl CLK_STRETCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_STRETCH_A {
        match self.bits {
            false => CLK_STRETCH_A::I2C_CLK_NOT_STRETCHED,
            true => CLK_STRETCH_A::I2C_CLK_STRETCHED,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_CLK_NOT_STRETCHED`"]
    #[inline(always)]
    pub fn is_i2c_clk_not_stretched(&self) -> bool {
        *self == CLK_STRETCH_A::I2C_CLK_NOT_STRETCHED
    }
    #[doc = "Checks if the value of the field is `I2C_CLK_STRETCHED`"]
    #[inline(always)]
    pub fn is_i2c_clk_stretched(&self) -> bool {
        *self == CLK_STRETCH_A::I2C_CLK_STRETCHED
    }
}
#[doc = "Indicate if the I2C interface bus is free\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_FREE_A {
    #[doc = "0: One or both of the I2C bus lines is currently 0"]
    I2C_BUS_BUSY = 0,
    #[doc = "1: Both I2C bus lines are currently free"]
    I2C_BUS_FREE = 1,
}
impl From<BUS_FREE_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_FREE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUS_FREE`"]
pub type BUS_FREE_R = crate::R<bool, BUS_FREE_A>;
impl BUS_FREE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_FREE_A {
        match self.bits {
            false => BUS_FREE_A::I2C_BUS_BUSY,
            true => BUS_FREE_A::I2C_BUS_FREE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_BUS_BUSY`"]
    #[inline(always)]
    pub fn is_i2c_bus_busy(&self) -> bool {
        *self == BUS_FREE_A::I2C_BUS_BUSY
    }
    #[doc = "Checks if the value of the field is `I2C_BUS_FREE`"]
    #[inline(always)]
    pub fn is_i2c_bus_free(&self) -> bool {
        *self == BUS_FREE_A::I2C_BUS_FREE
    }
}
#[doc = "Indicate if the I2C data register holds an address or data byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_DATA_A {
    #[doc = "0: The I2C data register holds data"]
    I2C_DATA_IS_DATA = 0,
    #[doc = "1: The I2C data register holds an address"]
    I2C_DATA_IS_ADDR = 1,
}
impl From<ADDR_DATA_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR_DATA`"]
pub type ADDR_DATA_R = crate::R<bool, ADDR_DATA_A>;
impl ADDR_DATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_DATA_A {
        match self.bits {
            false => ADDR_DATA_A::I2C_DATA_IS_DATA,
            true => ADDR_DATA_A::I2C_DATA_IS_ADDR,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_DATA_IS_DATA`"]
    #[inline(always)]
    pub fn is_i2c_data_is_data(&self) -> bool {
        *self == ADDR_DATA_A::I2C_DATA_IS_DATA
    }
    #[doc = "Checks if the value of the field is `I2C_DATA_IS_ADDR`"]
    #[inline(always)]
    pub fn is_i2c_data_is_addr(&self) -> bool {
        *self == ADDR_DATA_A::I2C_DATA_IS_ADDR
    }
}
#[doc = "Indicate whether the I2C bus transfer is a read or a write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WRITE_A {
    #[doc = "0: The current I2C transfer is a write"]
    I2C_IS_WRITE = 0,
    #[doc = "1: The current I2C transfer is a read"]
    I2C_IS_READ = 1,
}
impl From<READ_WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_WRITE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READ_WRITE`"]
pub type READ_WRITE_R = crate::R<bool, READ_WRITE_A>;
impl READ_WRITE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_WRITE_A {
        match self.bits {
            false => READ_WRITE_A::I2C_IS_WRITE,
            true => READ_WRITE_A::I2C_IS_READ,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_IS_WRITE`"]
    #[inline(always)]
    pub fn is_i2c_is_write(&self) -> bool {
        *self == READ_WRITE_A::I2C_IS_WRITE
    }
    #[doc = "Checks if the value of the field is `I2C_IS_READ`"]
    #[inline(always)]
    pub fn is_i2c_is_read(&self) -> bool {
        *self == READ_WRITE_A::I2C_IS_READ
    }
}
#[doc = "Indicate whether the I2C bus transfer is using the general call address or another address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEN_CALL_A {
    #[doc = "0: The address used for the current I2C transfer is not the general call address"]
    I2C_ADDR_OTHER = 0,
    #[doc = "1: The address used for the current I2C transfer is the general call address"]
    I2C_ADDR_GEN_CALL = 1,
}
impl From<GEN_CALL_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_CALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GEN_CALL`"]
pub type GEN_CALL_R = crate::R<bool, GEN_CALL_A>;
impl GEN_CALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEN_CALL_A {
        match self.bits {
            false => GEN_CALL_A::I2C_ADDR_OTHER,
            true => GEN_CALL_A::I2C_ADDR_GEN_CALL,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_ADDR_OTHER`"]
    #[inline(always)]
    pub fn is_i2c_addr_other(&self) -> bool {
        *self == GEN_CALL_A::I2C_ADDR_OTHER
    }
    #[doc = "Checks if the value of the field is `I2C_ADDR_GEN_CALL`"]
    #[inline(always)]
    pub fn is_i2c_addr_gen_call(&self) -> bool {
        *self == GEN_CALL_A::I2C_ADDR_GEN_CALL
    }
}
#[doc = "Indicate whether an acknowledge or a not acknowledge has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_STATUS_A {
    #[doc = "0: Indicate that the last I2C byte was acknowledged"]
    I2C_HAS_ACK = 0,
    #[doc = "1: Indicate that the last I2C byte was not acknowledged"]
    I2C_HAS_NACK = 1,
}
impl From<ACK_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACK_STATUS`"]
pub type ACK_STATUS_R = crate::R<bool, ACK_STATUS_A>;
impl ACK_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_STATUS_A {
        match self.bits {
            false => ACK_STATUS_A::I2C_HAS_ACK,
            true => ACK_STATUS_A::I2C_HAS_NACK,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_HAS_ACK`"]
    #[inline(always)]
    pub fn is_i2c_has_ack(&self) -> bool {
        *self == ACK_STATUS_A::I2C_HAS_ACK
    }
    #[doc = "Checks if the value of the field is `I2C_HAS_NACK`"]
    #[inline(always)]
    pub fn is_i2c_has_nack(&self) -> bool {
        *self == ACK_STATUS_A::I2C_HAS_NACK
    }
}
impl R {
    #[doc = "Bit 15 - Same as BUS_ERROR_S: Bus error status bit (sticky) - This status bit is automatically reset when the I2C_STATUS register is read."]
    #[inline(always)]
    pub fn error_s(&self) -> ERROR_S_R {
        ERROR_S_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bus error status bit (sticky) - This status bit is automatically reset when the I2C_STATUS register is read."]
    #[inline(always)]
    pub fn bus_error_s(&self) -> BUS_ERROR_S_R {
        BUS_ERROR_S_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master frame start pending status bit"]
    #[inline(always)]
    pub fn start_pending(&self) -> START_PENDING_R {
        START_PENDING_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Master mode status bit"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicate if the I2C interface is currently requesting DMA data"]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicate if an I2C stop bit has been detected"]
    #[inline(always)]
    pub fn stop_detect(&self) -> STOP_DETECT_R {
        STOP_DETECT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicate that I2C interface either needs data to transmit or has received data"]
    #[inline(always)]
    pub fn data_event(&self) -> DATA_EVENT_R {
        DATA_EVENT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Same as BUS_ERROR: Indicate if the I2C interface has detected a bus error (automatically cleared when a stop condition is detected)"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicate if the I2C interface has detected a bus error (automatically cleared when a stop condition is detected)"]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicate if the I2C data buffer is full"]
    #[inline(always)]
    pub fn buffer_full(&self) -> BUFFER_FULL_R {
        BUFFER_FULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicate if the I2C interface is holding the clock signal"]
    #[inline(always)]
    pub fn clk_stretch(&self) -> CLK_STRETCH_R {
        CLK_STRETCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicate if the I2C interface bus is free"]
    #[inline(always)]
    pub fn bus_free(&self) -> BUS_FREE_R {
        BUS_FREE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicate if the I2C data register holds an address or data byte"]
    #[inline(always)]
    pub fn addr_data(&self) -> ADDR_DATA_R {
        ADDR_DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicate whether the I2C bus transfer is a read or a write"]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicate whether the I2C bus transfer is using the general call address or another address"]
    #[inline(always)]
    pub fn gen_call(&self) -> GEN_CALL_R {
        GEN_CALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicate whether an acknowledge or a not acknowledge has been received"]
    #[inline(always)]
    pub fn ack_status(&self) -> ACK_STATUS_R {
        ACK_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
