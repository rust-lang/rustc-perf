#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CE-ATA command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CE_ATACMD_A {
    #[doc = "0: CE-ATA command disabled"]
    DISABLED = 0,
    #[doc = "1: CE-ATA command enabled"]
    ENABLED = 1,
}
impl From<CE_ATACMD_A> for bool {
    #[inline(always)]
    fn from(variant: CE_ATACMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CE_ATACMD`"]
pub type CE_ATACMD_R = crate::R<bool, CE_ATACMD_A>;
impl CE_ATACMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CE_ATACMD_A {
        match self.bits {
            false => CE_ATACMD_A::DISABLED,
            true => CE_ATACMD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CE_ATACMD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CE_ATACMD_A::ENABLED
    }
}
#[doc = "Write proxy for field `CE_ATACMD`"]
pub struct CE_ATACMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_ATACMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CE_ATACMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CE-ATA command disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CE_ATACMD_A::DISABLED)
    }
    #[doc = "CE-ATA command enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CE_ATACMD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "not Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIEN_A {
    #[doc = "0: Interrupts to the CE-ATA not disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt to the CE-ATA are disabled"]
    ENABLED = 1,
}
impl From<NIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `nIEN`"]
pub type NIEN_R = crate::R<bool, NIEN_A>;
impl NIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIEN_A {
        match self.bits {
            false => NIEN_A::DISABLED,
            true => NIEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NIEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NIEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `nIEN`"]
pub struct NIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts to the CE-ATA not disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NIEN_A::DISABLED)
    }
    #[doc = "Interrupt to the CE-ATA are disabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable CMD completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCMDCOMPL_A {
    #[doc = "0: Command complete signal disabled"]
    DISABLED = 0,
    #[doc = "1: Command complete signal enabled"]
    ENABLED = 1,
}
impl From<ENCMDCOMPL_A> for bool {
    #[inline(always)]
    fn from(variant: ENCMDCOMPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENCMDcompl`"]
pub type ENCMDCOMPL_R = crate::R<bool, ENCMDCOMPL_A>;
impl ENCMDCOMPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCMDCOMPL_A {
        match self.bits {
            false => ENCMDCOMPL_A::DISABLED,
            true => ENCMDCOMPL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENCMDCOMPL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENCMDCOMPL_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENCMDcompl`"]
pub struct ENCMDCOMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCMDCOMPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCMDCOMPL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command complete signal disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENCMDCOMPL_A::DISABLED)
    }
    #[doc = "Command complete signal enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENCMDCOMPL_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "SD I/O suspend command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOSUSPEND_A {
    #[doc = "0: Next command is not a SDIO suspend command"]
    DISABLED = 0,
    #[doc = "1: Next command send is a SDIO suspend command"]
    ENABLED = 1,
}
impl From<SDIOSUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOSUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDIOSuspend`"]
pub type SDIOSUSPEND_R = crate::R<bool, SDIOSUSPEND_A>;
impl SDIOSUSPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOSUSPEND_A {
        match self.bits {
            false => SDIOSUSPEND_A::DISABLED,
            true => SDIOSUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOSUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOSUSPEND_A::ENABLED
    }
}
#[doc = "Write proxy for field `SDIOSuspend`"]
pub struct SDIOSUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOSUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOSUSPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Next command is not a SDIO suspend command"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIOSUSPEND_A::DISABLED)
    }
    #[doc = "Next command send is a SDIO suspend command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIOSUSPEND_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Command path state machine (CPSM) Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPSMEN_A {
    #[doc = "0: Command path state machine disabled"]
    DISABLED = 0,
    #[doc = "1: Command path state machine enabled"]
    ENABLED = 1,
}
impl From<CPSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPSMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPSMEN`"]
pub type CPSMEN_R = crate::R<bool, CPSMEN_A>;
impl CPSMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPSMEN_A {
        match self.bits {
            false => CPSMEN_A::DISABLED,
            true => CPSMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPSMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPSMEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CPSMEN`"]
pub struct CPSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPSMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command path state machine disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPSMEN_A::DISABLED)
    }
    #[doc = "Command path state machine enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPSMEN_A::ENABLED)
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
#[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITPEND_A {
    #[doc = "0: Don't wait for data end"]
    DISABLED = 0,
    #[doc = "1: Wait for end of data transfer signal before sending command"]
    ENABLED = 1,
}
impl From<WAITPEND_A> for bool {
    #[inline(always)]
    fn from(variant: WAITPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAITPEND`"]
pub type WAITPEND_R = crate::R<bool, WAITPEND_A>;
impl WAITPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITPEND_A {
        match self.bits {
            false => WAITPEND_A::DISABLED,
            true => WAITPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITPEND_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAITPEND`"]
pub struct WAITPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Don't wait for data end"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITPEND_A::DISABLED)
    }
    #[doc = "Wait for end of data transfer signal before sending command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITPEND_A::ENABLED)
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
#[doc = "CPSM waits for interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITINT_A {
    #[doc = "0: Don't wait for interrupt request"]
    DISABLED = 0,
    #[doc = "1: Wait for interrupt request"]
    ENABLED = 1,
}
impl From<WAITINT_A> for bool {
    #[inline(always)]
    fn from(variant: WAITINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAITINT`"]
pub type WAITINT_R = crate::R<bool, WAITINT_A>;
impl WAITINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITINT_A {
        match self.bits {
            false => WAITINT_A::DISABLED,
            true => WAITINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITINT_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAITINT`"]
pub struct WAITINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Don't wait for interrupt request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITINT_A::DISABLED)
    }
    #[doc = "Wait for interrupt request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITINT_A::ENABLED)
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
#[doc = "Wait for response bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAITRESP_A {
    #[doc = "0: No response"]
    NORESPONSE = 0,
    #[doc = "1: Short response"]
    SHORTRESPONSE = 1,
    #[doc = "2: No reponse"]
    NORESPONSE2 = 2,
    #[doc = "3: Long reponse"]
    LONGRESPONSE = 3,
}
impl From<WAITRESP_A> for u8 {
    #[inline(always)]
    fn from(variant: WAITRESP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAITRESP`"]
pub type WAITRESP_R = crate::R<u8, WAITRESP_A>;
impl WAITRESP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITRESP_A {
        match self.bits {
            0 => WAITRESP_A::NORESPONSE,
            1 => WAITRESP_A::SHORTRESPONSE,
            2 => WAITRESP_A::NORESPONSE2,
            3 => WAITRESP_A::LONGRESPONSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORESPONSE`"]
    #[inline(always)]
    pub fn is_no_response(&self) -> bool {
        *self == WAITRESP_A::NORESPONSE
    }
    #[doc = "Checks if the value of the field is `SHORTRESPONSE`"]
    #[inline(always)]
    pub fn is_short_response(&self) -> bool {
        *self == WAITRESP_A::SHORTRESPONSE
    }
    #[doc = "Checks if the value of the field is `NORESPONSE2`"]
    #[inline(always)]
    pub fn is_no_response2(&self) -> bool {
        *self == WAITRESP_A::NORESPONSE2
    }
    #[doc = "Checks if the value of the field is `LONGRESPONSE`"]
    #[inline(always)]
    pub fn is_long_response(&self) -> bool {
        *self == WAITRESP_A::LONGRESPONSE
    }
}
#[doc = "Write proxy for field `WAITRESP`"]
pub struct WAITRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITRESP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITRESP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn no_response(self) -> &'a mut W {
        self.variant(WAITRESP_A::NORESPONSE)
    }
    #[doc = "Short response"]
    #[inline(always)]
    pub fn short_response(self) -> &'a mut W {
        self.variant(WAITRESP_A::SHORTRESPONSE)
    }
    #[doc = "No reponse"]
    #[inline(always)]
    pub fn no_response2(self) -> &'a mut W {
        self.variant(WAITRESP_A::NORESPONSE2)
    }
    #[doc = "Long reponse"]
    #[inline(always)]
    pub fn long_response(self) -> &'a mut W {
        self.variant(WAITRESP_A::LONGRESPONSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CMDINDEX`"]
pub type CMDINDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDINDEX`"]
pub struct CMDINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - CE-ATA command"]
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CE_ATACMD_R {
        CE_ATACMD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - not Interrupt Enable"]
    #[inline(always)]
    pub fn n_ien(&self) -> NIEN_R {
        NIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable CMD completion"]
    #[inline(always)]
    pub fn encmdcompl(&self) -> ENCMDCOMPL_R {
        ENCMDCOMPL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - CE-ATA command"]
    #[inline(always)]
    pub fn ce_atacmd(&mut self) -> CE_ATACMD_W {
        CE_ATACMD_W { w: self }
    }
    #[doc = "Bit 13 - not Interrupt Enable"]
    #[inline(always)]
    pub fn n_ien(&mut self) -> NIEN_W {
        NIEN_W { w: self }
    }
    #[doc = "Bit 12 - Enable CMD completion"]
    #[inline(always)]
    pub fn encmdcompl(&mut self) -> ENCMDCOMPL_W {
        ENCMDCOMPL_W { w: self }
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W {
        SDIOSUSPEND_W { w: self }
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W {
        CPSMEN_W { w: self }
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W {
        WAITPEND_W { w: self }
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W {
        WAITINT_W { w: self }
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W {
        WAITRESP_W { w: self }
    }
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W {
        CMDINDEX_W { w: self }
    }
}
