#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Writer for register SR1"]
pub type W = crate::W<u32, super::SR1>;
#[doc = "Register SR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBALERT_A {
    #[doc = "0: No SMBALERT occured"]
    NOALERT = 0,
    #[doc = "1: SMBALERT occurred"]
    ALERT = 1,
}
impl From<SMBALERT_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMBALERT`"]
pub type SMBALERT_R = crate::R<bool, SMBALERT_A>;
impl SMBALERT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBALERT_A {
        match self.bits {
            false => SMBALERT_A::NOALERT,
            true => SMBALERT_A::ALERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOALERT`"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALERT_A::NOALERT
    }
    #[doc = "Checks if the value of the field is `ALERT`"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == SMBALERT_A::ALERT
    }
}
#[doc = "Write proxy for field `SMBALERT`"]
pub struct SMBALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBALERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMBALERT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SMBALERT occured"]
    #[inline(always)]
    pub fn no_alert(self) -> &'a mut W {
        self.variant(SMBALERT_A::NOALERT)
    }
    #[doc = "SMBALERT occurred"]
    #[inline(always)]
    pub fn alert(self) -> &'a mut W {
        self.variant(SMBALERT_A::ALERT)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Timeout or Tlow error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: No Timeout error"]
    NOTIMEOUT = 0,
    #[doc = "1: SCL remained LOW for 25 ms"]
    TIMEOUT = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::NOTIMEOUT,
            true => TIMEOUT_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT_A::NOTIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT_A::TIMEOUT
    }
}
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(TIMEOUT_A::NOTIMEOUT)
    }
    #[doc = "SCL remained LOW for 25 ms"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(TIMEOUT_A::TIMEOUT)
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
#[doc = "PEC Error in reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    #[doc = "0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    NOERROR = 0,
    #[doc = "1: PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    ERROR = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PECERR`"]
pub type PECERR_R = crate::R<bool, PECERR_A>;
impl PECERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::NOERROR,
            true => PECERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERR_A::ERROR
    }
}
#[doc = "Write proxy for field `PECERR`"]
pub struct PECERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PECERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PECERR_A::NOERROR)
    }
    #[doc = "PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PECERR_A::ERROR)
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
#[doc = "Overrun/Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun/underrun occured"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun/underrun occured"]
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVR_A::NOOVERRUN)
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVR_A::OVERRUN)
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
#[doc = "Acknowledge failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AF_A {
    #[doc = "0: No acknowledge failure"]
    NOFAILURE = 0,
    #[doc = "1: Acknowledge failure"]
    FAILURE = 1,
}
impl From<AF_A> for bool {
    #[inline(always)]
    fn from(variant: AF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AF`"]
pub type AF_R = crate::R<bool, AF_A>;
impl AF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AF_A {
        match self.bits {
            false => AF_A::NOFAILURE,
            true => AF_A::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAILURE`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == AF_A::NOFAILURE
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == AF_A::FAILURE
    }
}
#[doc = "Write proxy for field `AF`"]
pub struct AF_W<'a> {
    w: &'a mut W,
}
impl<'a> AF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No acknowledge failure"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(AF_A::NOFAILURE)
    }
    #[doc = "Acknowledge failure"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(AF_A::FAILURE)
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
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLO_A {
    #[doc = "0: No Arbitration Lost detected"]
    NOLOST = 0,
    #[doc = "1: Arbitration Lost detected"]
    LOST = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARLO`"]
pub type ARLO_R = crate::R<bool, ARLO_A>;
impl ARLO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NOLOST,
            true => ARLO_A::LOST,
        }
    }
    #[doc = "Checks if the value of the field is `NOLOST`"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == ARLO_A::NOLOST
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO_A::LOST
    }
}
#[doc = "Write proxy for field `ARLO`"]
pub struct ARLO_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARLO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn no_lost(self) -> &'a mut W {
        self.variant(ARLO_A::NOLOST)
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut W {
        self.variant(ARLO_A::LOST)
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
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    #[doc = "0: No misplaced Start or Stop condition"]
    NOERROR = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    ERROR = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, BERR_A>;
impl BERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NOERROR,
            true => BERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::ERROR
    }
}
#[doc = "Write proxy for field `BERR`"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(BERR_A::NOERROR)
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(BERR_A::ERROR)
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
#[doc = "Data register empty (transmitters)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: Data register not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Data register empty"]
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TxE`"]
pub type TXE_R = crate::R<bool, TXE_A>;
impl TXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOTEMPTY,
            true => TXE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::EMPTY
    }
}
#[doc = "Data register not empty (receivers)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    #[doc = "0: Data register empty"]
    EMPTY = 0,
    #[doc = "1: Data register not empty"]
    NOTEMPTY = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RxNE`"]
pub type RXNE_R = crate::R<bool, RXNE_A>;
impl RXNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::EMPTY,
            true => RXNE_A::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NOTEMPTY
    }
}
#[doc = "Stop detection (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPF_A {
    #[doc = "0: No Stop condition detected"]
    NOSTOP = 0,
    #[doc = "1: Stop condition detected"]
    STOP = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPF`"]
pub type STOPF_R = crate::R<bool, STOPF_A>;
impl STOPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NOSTOP,
            true => STOPF_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::STOP
    }
}
#[doc = "Reader of field `ADD10`"]
pub type ADD10_R = crate::R<bool, bool>;
#[doc = "Byte transfer finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTF_A {
    #[doc = "0: Data byte transfer not done"]
    NOTFINISHED = 0,
    #[doc = "1: Data byte transfer successful"]
    FINISHED = 1,
}
impl From<BTF_A> for bool {
    #[inline(always)]
    fn from(variant: BTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTF`"]
pub type BTF_R = crate::R<bool, BTF_A>;
impl BTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTF_A {
        match self.bits {
            false => BTF_A::NOTFINISHED,
            true => BTF_A::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFINISHED`"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == BTF_A::NOTFINISHED
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == BTF_A::FINISHED
    }
}
#[doc = "Address sent (master mode)/matched (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_A {
    #[doc = "0: Adress mismatched or not received"]
    NOTMATCH = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    MATCH = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<bool, ADDR_A>;
impl ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NOTMATCH,
            true => ADDR_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMATCH`"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR_A::NOTMATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == ADDR_A::MATCH
    }
}
#[doc = "Start bit (Master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SB_A {
    #[doc = "0: No Start condition"]
    NOSTART = 0,
    #[doc = "1: Start condition generated"]
    START = 1,
}
impl From<SB_A> for bool {
    #[inline(always)]
    fn from(variant: SB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SB`"]
pub type SB_R = crate::R<bool, SB_A>;
impl SB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB_A {
        match self.bits {
            false => SB_A::NOSTART,
            true => SB_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == SB_A::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SB_A::START
    }
}
impl R {
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W {
        SMBALERT_W { w: self }
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W {
        PECERR_W { w: self }
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&mut self) -> AF_W {
        AF_W { w: self }
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ARLO_W {
        ARLO_W { w: self }
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
}
