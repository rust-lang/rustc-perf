#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR1` writer"]
pub struct W(crate::W<SR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBALERT_A {
    #[doc = "0: No SMBALERT occured"]
    NoAlert = 0,
    #[doc = "1: SMBALERT occurred"]
    Alert = 1,
}
impl From<SMBALERT_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBALERT` reader - SMBus alert"]
pub type SMBALERT_R = crate::BitReader<SMBALERT_A>;
impl SMBALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBALERT_A {
        match self.bits {
            false => SMBALERT_A::NoAlert,
            true => SMBALERT_A::Alert,
        }
    }
    #[doc = "Checks if the value of the field is `NoAlert`"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALERT_A::NoAlert
    }
    #[doc = "Checks if the value of the field is `Alert`"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == SMBALERT_A::Alert
    }
}
#[doc = "Field `SMBALERT` writer - SMBus alert"]
pub type SMBALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, SMBALERT_A, O>;
impl<'a, const O: u8> SMBALERT_W<'a, O> {
    #[doc = "No SMBALERT occured"]
    #[inline(always)]
    pub fn no_alert(self) -> &'a mut W {
        self.variant(SMBALERT_A::NoAlert)
    }
    #[doc = "SMBALERT occurred"]
    #[inline(always)]
    pub fn alert(self) -> &'a mut W {
        self.variant(SMBALERT_A::Alert)
    }
}
#[doc = "Timeout or Tlow error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: No Timeout error"]
    NoTimeout = 0,
    #[doc = "1: SCL remained LOW for 25 ms"]
    Timeout = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Timeout or Tlow error"]
pub type TIMEOUT_R = crate::BitReader<TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::NoTimeout,
            true => TIMEOUT_A::Timeout,
        }
    }
    #[doc = "Checks if the value of the field is `NoTimeout`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT_A::NoTimeout
    }
    #[doc = "Checks if the value of the field is `Timeout`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT_A::Timeout
    }
}
#[doc = "Field `TIMEOUT` writer - Timeout or Tlow error"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, TIMEOUT_A, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(TIMEOUT_A::NoTimeout)
    }
    #[doc = "SCL remained LOW for 25 ms"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(TIMEOUT_A::Timeout)
    }
}
#[doc = "PEC Error in reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    #[doc = "0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    NoError = 0,
    #[doc = "1: PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    Error = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader<PECERR_A>;
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::NoError,
            true => PECERR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERR_A::Error
    }
}
#[doc = "Field `PECERR` writer - PEC Error in reception"]
pub type PECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, PECERR_A, O>;
impl<'a, const O: u8> PECERR_W<'a, O> {
    #[doc = "no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PECERR_A::NoError)
    }
    #[doc = "PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PECERR_A::Error)
    }
}
#[doc = "Overrun/Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun/underrun occured"]
    NoOverrun = 0,
    #[doc = "1: Overrun/underrun occured"]
    Overrun = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun/Underrun"]
pub type OVR_R = crate::BitReader<OVR_A>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NoOverrun,
            true => OVR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::Overrun
    }
}
#[doc = "Field `OVR` writer - Overrun/Underrun"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, OVR_A, O>;
impl<'a, const O: u8> OVR_W<'a, O> {
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVR_A::NoOverrun)
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVR_A::Overrun)
    }
}
#[doc = "Acknowledge failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AF_A {
    #[doc = "0: No acknowledge failure"]
    NoFailure = 0,
    #[doc = "1: Acknowledge failure"]
    Failure = 1,
}
impl From<AF_A> for bool {
    #[inline(always)]
    fn from(variant: AF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AF` reader - Acknowledge failure"]
pub type AF_R = crate::BitReader<AF_A>;
impl AF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AF_A {
        match self.bits {
            false => AF_A::NoFailure,
            true => AF_A::Failure,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == AF_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `Failure`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == AF_A::Failure
    }
}
#[doc = "Field `AF` writer - Acknowledge failure"]
pub type AF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, AF_A, O>;
impl<'a, const O: u8> AF_W<'a, O> {
    #[doc = "No acknowledge failure"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(AF_A::NoFailure)
    }
    #[doc = "Acknowledge failure"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(AF_A::Failure)
    }
}
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLO_A {
    #[doc = "0: No Arbitration Lost detected"]
    NoLost = 0,
    #[doc = "1: Arbitration Lost detected"]
    Lost = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLO` reader - Arbitration lost (master mode)"]
pub type ARLO_R = crate::BitReader<ARLO_A>;
impl ARLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NoLost,
            true => ARLO_A::Lost,
        }
    }
    #[doc = "Checks if the value of the field is `NoLost`"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == ARLO_A::NoLost
    }
    #[doc = "Checks if the value of the field is `Lost`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO_A::Lost
    }
}
#[doc = "Field `ARLO` writer - Arbitration lost (master mode)"]
pub type ARLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, ARLO_A, O>;
impl<'a, const O: u8> ARLO_W<'a, O> {
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn no_lost(self) -> &'a mut W {
        self.variant(ARLO_A::NoLost)
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut W {
        self.variant(ARLO_A::Lost)
    }
}
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    #[doc = "0: No misplaced Start or Stop condition"]
    NoError = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    Error = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<BERR_A>;
impl BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NoError,
            true => BERR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::Error
    }
}
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, BERR_A, O>;
impl<'a, const O: u8> BERR_W<'a, O> {
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(BERR_A::NoError)
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(BERR_A::Error)
    }
}
#[doc = "Data register empty (transmitters)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: Data register not empty"]
    NotEmpty = 0,
    #[doc = "1: Data register empty"]
    Empty = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxE` reader - Data register empty (transmitters)"]
pub type TXE_R = crate::BitReader<TXE_A>;
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NotEmpty,
            true => TXE_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::Empty
    }
}
#[doc = "Data register not empty (receivers)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    #[doc = "0: Data register empty"]
    Empty = 0,
    #[doc = "1: Data register not empty"]
    NotEmpty = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RxNE` reader - Data register not empty (receivers)"]
pub type RXNE_R = crate::BitReader<RXNE_A>;
impl RXNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::Empty,
            true => RXNE_A::NotEmpty,
        }
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::Empty
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NotEmpty
    }
}
#[doc = "Stop detection (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPF_A {
    #[doc = "0: No Stop condition detected"]
    NoStop = 0,
    #[doc = "1: Stop condition detected"]
    Stop = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader<STOPF_A>;
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NoStop,
            true => STOPF_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `NoStop`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NoStop
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::Stop
    }
}
#[doc = "Field `ADD10` reader - 10-bit header sent (Master mode)"]
pub type ADD10_R = crate::BitReader<bool>;
#[doc = "Byte transfer finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTF_A {
    #[doc = "0: Data byte transfer not done"]
    NotFinished = 0,
    #[doc = "1: Data byte transfer successful"]
    Finished = 1,
}
impl From<BTF_A> for bool {
    #[inline(always)]
    fn from(variant: BTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTF` reader - Byte transfer finished"]
pub type BTF_R = crate::BitReader<BTF_A>;
impl BTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTF_A {
        match self.bits {
            false => BTF_A::NotFinished,
            true => BTF_A::Finished,
        }
    }
    #[doc = "Checks if the value of the field is `NotFinished`"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == BTF_A::NotFinished
    }
    #[doc = "Checks if the value of the field is `Finished`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == BTF_A::Finished
    }
}
#[doc = "Address sent (master mode)/matched (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_A {
    #[doc = "0: Adress mismatched or not received"]
    NotMatch = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    Match = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR_R = crate::BitReader<ADDR_A>;
impl ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NotMatch,
            true => ADDR_A::Match,
        }
    }
    #[doc = "Checks if the value of the field is `NotMatch`"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR_A::NotMatch
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR_A::Match
    }
}
#[doc = "Start bit (Master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SB_A {
    #[doc = "0: No Start condition"]
    NoStart = 0,
    #[doc = "1: Start condition generated"]
    Start = 1,
}
impl From<SB_A> for bool {
    #[inline(always)]
    fn from(variant: SB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SB` reader - Start bit (Master mode)"]
pub type SB_R = crate::BitReader<SB_A>;
impl SB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB_A {
        match self.bits {
            false => SB_A::NoStart,
            true => SB_A::Start,
        }
    }
    #[doc = "Checks if the value of the field is `NoStart`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == SB_A::NoStart
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SB_A::Start
    }
}
impl R {
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W<15> {
        SMBALERT_W::new(self)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<14> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W<12> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<11> {
        OVR_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&mut self) -> AF_W<10> {
        AF_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ARLO_W<9> {
        ARLO_W::new(self)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<8> {
        BERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr1::W](W) writer structure"]
impl crate::Writable for SR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
