#[doc = "Reader of register FLASH_CMD_CTRL"]
pub type R = crate::R<u32, super::FLASH_CMD_CTRL>;
#[doc = "Writer for register FLASH_CMD_CTRL"]
pub type W = crate::W<u32, super::FLASH_CMD_CTRL>;
#[doc = "Register FLASH_CMD_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_CMD_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Terminates an active Flash command if possible (e.g. sequential programming sequence)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_END_AW {
    #[doc = "3: Terminates an active Flash command if possible"]
    CMD_END = 3,
}
impl From<CMD_END_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_END_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CMD_END`"]
pub struct CMD_END_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_END_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Terminates an active Flash command if possible"]
    #[inline(always)]
    pub fn cmd_end(self) -> &'a mut W {
        self.variant(CMD_END_AW::CMD_END)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Flash access command only writable when equal to CMD_IDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMMAND_A {
    #[doc = "0: Idle command"]
    CMD_IDLE = 0,
    #[doc = "1: Wake up the Flash"]
    CMD_WAKE_UP = 1,
    #[doc = "2: Load patch & trimming values from NVR4"]
    CMD_LOAD_TRIM = 2,
    #[doc = "5: Execute a read cycle"]
    CMD_READ = 5,
    #[doc = "6: Execute a non-sequential programming cycle"]
    CMD_PROGRAM_NOSEQ = 6,
    #[doc = "7: Starts a sequential programming sequence"]
    CMD_PROGRAM_SEQ = 7,
    #[doc = "8: Execute a sector erase cycle"]
    CMD_SECTOR_ERASE = 8,
    #[doc = "9: Execute a mass erase cycle"]
    CMD_MASS_ERASE = 9,
    #[doc = "10: Wait time to set the LPWR pin"]
    CMD_SET_LOW_POWER = 10,
    #[doc = "11: Wait time to unset the LPWR pin"]
    CMD_UNSET_LOW_POWER = 11,
    #[doc = "12: Wait time to set the RECALL pin"]
    CMD_SET_RECALL = 12,
    #[doc = "13: Wait time to unset the RECALL pin"]
    CMD_UNSET_RECALL = 13,
    #[doc = "14: Wait time to set the VREAD1 pin"]
    CMD_SET_VREAD1 = 14,
    #[doc = "15: Wait time to unset the VREAD1 pin"]
    CMD_UNSET_VREAD1 = 15,
    #[doc = "16: Wait time to set the VREAD0 pin"]
    CMD_SET_VREAD0 = 16,
    #[doc = "17: Wait time to unset the VREAD0 pin"]
    CMD_UNSET_VREAD0 = 17,
    #[doc = "18: Write FLASH_DATA to PATCH2 of NVR4"]
    CMD_WRITE_USER_RED1 = 18,
    #[doc = "19: Write FLASH_DATA to PATCH3 of NVR4"]
    CMD_WRITE_USER_RED2 = 19,
}
impl From<COMMAND_A> for u8 {
    #[inline(always)]
    fn from(variant: COMMAND_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMMAND`"]
pub type COMMAND_R = crate::R<u8, COMMAND_A>;
impl COMMAND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMMAND_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMMAND_A::CMD_IDLE),
            1 => Val(COMMAND_A::CMD_WAKE_UP),
            2 => Val(COMMAND_A::CMD_LOAD_TRIM),
            5 => Val(COMMAND_A::CMD_READ),
            6 => Val(COMMAND_A::CMD_PROGRAM_NOSEQ),
            7 => Val(COMMAND_A::CMD_PROGRAM_SEQ),
            8 => Val(COMMAND_A::CMD_SECTOR_ERASE),
            9 => Val(COMMAND_A::CMD_MASS_ERASE),
            10 => Val(COMMAND_A::CMD_SET_LOW_POWER),
            11 => Val(COMMAND_A::CMD_UNSET_LOW_POWER),
            12 => Val(COMMAND_A::CMD_SET_RECALL),
            13 => Val(COMMAND_A::CMD_UNSET_RECALL),
            14 => Val(COMMAND_A::CMD_SET_VREAD1),
            15 => Val(COMMAND_A::CMD_UNSET_VREAD1),
            16 => Val(COMMAND_A::CMD_SET_VREAD0),
            17 => Val(COMMAND_A::CMD_UNSET_VREAD0),
            18 => Val(COMMAND_A::CMD_WRITE_USER_RED1),
            19 => Val(COMMAND_A::CMD_WRITE_USER_RED2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMD_IDLE`"]
    #[inline(always)]
    pub fn is_cmd_idle(&self) -> bool {
        *self == COMMAND_A::CMD_IDLE
    }
    #[doc = "Checks if the value of the field is `CMD_WAKE_UP`"]
    #[inline(always)]
    pub fn is_cmd_wake_up(&self) -> bool {
        *self == COMMAND_A::CMD_WAKE_UP
    }
    #[doc = "Checks if the value of the field is `CMD_LOAD_TRIM`"]
    #[inline(always)]
    pub fn is_cmd_load_trim(&self) -> bool {
        *self == COMMAND_A::CMD_LOAD_TRIM
    }
    #[doc = "Checks if the value of the field is `CMD_READ`"]
    #[inline(always)]
    pub fn is_cmd_read(&self) -> bool {
        *self == COMMAND_A::CMD_READ
    }
    #[doc = "Checks if the value of the field is `CMD_PROGRAM_NOSEQ`"]
    #[inline(always)]
    pub fn is_cmd_program_noseq(&self) -> bool {
        *self == COMMAND_A::CMD_PROGRAM_NOSEQ
    }
    #[doc = "Checks if the value of the field is `CMD_PROGRAM_SEQ`"]
    #[inline(always)]
    pub fn is_cmd_program_seq(&self) -> bool {
        *self == COMMAND_A::CMD_PROGRAM_SEQ
    }
    #[doc = "Checks if the value of the field is `CMD_SECTOR_ERASE`"]
    #[inline(always)]
    pub fn is_cmd_sector_erase(&self) -> bool {
        *self == COMMAND_A::CMD_SECTOR_ERASE
    }
    #[doc = "Checks if the value of the field is `CMD_MASS_ERASE`"]
    #[inline(always)]
    pub fn is_cmd_mass_erase(&self) -> bool {
        *self == COMMAND_A::CMD_MASS_ERASE
    }
    #[doc = "Checks if the value of the field is `CMD_SET_LOW_POWER`"]
    #[inline(always)]
    pub fn is_cmd_set_low_power(&self) -> bool {
        *self == COMMAND_A::CMD_SET_LOW_POWER
    }
    #[doc = "Checks if the value of the field is `CMD_UNSET_LOW_POWER`"]
    #[inline(always)]
    pub fn is_cmd_unset_low_power(&self) -> bool {
        *self == COMMAND_A::CMD_UNSET_LOW_POWER
    }
    #[doc = "Checks if the value of the field is `CMD_SET_RECALL`"]
    #[inline(always)]
    pub fn is_cmd_set_recall(&self) -> bool {
        *self == COMMAND_A::CMD_SET_RECALL
    }
    #[doc = "Checks if the value of the field is `CMD_UNSET_RECALL`"]
    #[inline(always)]
    pub fn is_cmd_unset_recall(&self) -> bool {
        *self == COMMAND_A::CMD_UNSET_RECALL
    }
    #[doc = "Checks if the value of the field is `CMD_SET_VREAD1`"]
    #[inline(always)]
    pub fn is_cmd_set_vread1(&self) -> bool {
        *self == COMMAND_A::CMD_SET_VREAD1
    }
    #[doc = "Checks if the value of the field is `CMD_UNSET_VREAD1`"]
    #[inline(always)]
    pub fn is_cmd_unset_vread1(&self) -> bool {
        *self == COMMAND_A::CMD_UNSET_VREAD1
    }
    #[doc = "Checks if the value of the field is `CMD_SET_VREAD0`"]
    #[inline(always)]
    pub fn is_cmd_set_vread0(&self) -> bool {
        *self == COMMAND_A::CMD_SET_VREAD0
    }
    #[doc = "Checks if the value of the field is `CMD_UNSET_VREAD0`"]
    #[inline(always)]
    pub fn is_cmd_unset_vread0(&self) -> bool {
        *self == COMMAND_A::CMD_UNSET_VREAD0
    }
    #[doc = "Checks if the value of the field is `CMD_WRITE_USER_RED1`"]
    #[inline(always)]
    pub fn is_cmd_write_user_red1(&self) -> bool {
        *self == COMMAND_A::CMD_WRITE_USER_RED1
    }
    #[doc = "Checks if the value of the field is `CMD_WRITE_USER_RED2`"]
    #[inline(always)]
    pub fn is_cmd_write_user_red2(&self) -> bool {
        *self == COMMAND_A::CMD_WRITE_USER_RED2
    }
}
#[doc = "Write proxy for field `COMMAND`"]
pub struct COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMMAND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Idle command"]
    #[inline(always)]
    pub fn cmd_idle(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_IDLE)
    }
    #[doc = "Wake up the Flash"]
    #[inline(always)]
    pub fn cmd_wake_up(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_WAKE_UP)
    }
    #[doc = "Load patch & trimming values from NVR4"]
    #[inline(always)]
    pub fn cmd_load_trim(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_LOAD_TRIM)
    }
    #[doc = "Execute a read cycle"]
    #[inline(always)]
    pub fn cmd_read(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_READ)
    }
    #[doc = "Execute a non-sequential programming cycle"]
    #[inline(always)]
    pub fn cmd_program_noseq(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_PROGRAM_NOSEQ)
    }
    #[doc = "Starts a sequential programming sequence"]
    #[inline(always)]
    pub fn cmd_program_seq(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_PROGRAM_SEQ)
    }
    #[doc = "Execute a sector erase cycle"]
    #[inline(always)]
    pub fn cmd_sector_erase(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_SECTOR_ERASE)
    }
    #[doc = "Execute a mass erase cycle"]
    #[inline(always)]
    pub fn cmd_mass_erase(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_MASS_ERASE)
    }
    #[doc = "Wait time to set the LPWR pin"]
    #[inline(always)]
    pub fn cmd_set_low_power(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_SET_LOW_POWER)
    }
    #[doc = "Wait time to unset the LPWR pin"]
    #[inline(always)]
    pub fn cmd_unset_low_power(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_UNSET_LOW_POWER)
    }
    #[doc = "Wait time to set the RECALL pin"]
    #[inline(always)]
    pub fn cmd_set_recall(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_SET_RECALL)
    }
    #[doc = "Wait time to unset the RECALL pin"]
    #[inline(always)]
    pub fn cmd_unset_recall(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_UNSET_RECALL)
    }
    #[doc = "Wait time to set the VREAD1 pin"]
    #[inline(always)]
    pub fn cmd_set_vread1(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_SET_VREAD1)
    }
    #[doc = "Wait time to unset the VREAD1 pin"]
    #[inline(always)]
    pub fn cmd_unset_vread1(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_UNSET_VREAD1)
    }
    #[doc = "Wait time to set the VREAD0 pin"]
    #[inline(always)]
    pub fn cmd_set_vread0(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_SET_VREAD0)
    }
    #[doc = "Wait time to unset the VREAD0 pin"]
    #[inline(always)]
    pub fn cmd_unset_vread0(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_UNSET_VREAD0)
    }
    #[doc = "Write FLASH_DATA to PATCH2 of NVR4"]
    #[inline(always)]
    pub fn cmd_write_user_red1(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_WRITE_USER_RED1)
    }
    #[doc = "Write FLASH_DATA to PATCH3 of NVR4"]
    #[inline(always)]
    pub fn cmd_write_user_red2(self) -> &'a mut W {
        self.variant(COMMAND_A::CMD_WRITE_USER_RED2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Flash access command only writable when equal to CMD_IDLE"]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:6 - Terminates an active Flash command if possible (e.g. sequential programming sequence)"]
    #[inline(always)]
    pub fn cmd_end(&mut self) -> CMD_END_W {
        CMD_END_W { w: self }
    }
    #[doc = "Bits 0:4 - Flash access command only writable when equal to CMD_IDLE"]
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W {
        COMMAND_W { w: self }
    }
}
