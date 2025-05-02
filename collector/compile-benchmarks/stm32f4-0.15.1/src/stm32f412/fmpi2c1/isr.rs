#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TXE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: TXDR register not empty"]
    NotEmpty = 0,
    #[doc = "1: TXDR register empty"]
    Empty = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - TXE"]
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
#[doc = "Field `TXE` writer - TXE"]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, TXE_A, O>;
impl<'a, const O: u8> TXE_W<'a, O> {
    #[doc = "TXDR register not empty"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXE_A::NotEmpty)
    }
    #[doc = "TXDR register empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXE_A::Empty)
    }
}
#[doc = "TXIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIS_A {
    #[doc = "0: The TXDR register is not empty"]
    NotEmpty = 0,
    #[doc = "1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    Empty = 1,
}
impl From<TXIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIS` reader - TXIS"]
pub type TXIS_R = crate::BitReader<TXIS_A>;
impl TXIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIS_A {
        match self.bits {
            false => TXIS_A::NotEmpty,
            true => TXIS_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXIS_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXIS_A::Empty
    }
}
#[doc = "Field `TXIS` writer - TXIS"]
pub type TXIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, TXIS_A, O>;
impl<'a, const O: u8> TXIS_W<'a, O> {
    #[doc = "The TXDR register is not empty"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXIS_A::NotEmpty)
    }
    #[doc = "The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXIS_A::Empty)
    }
}
#[doc = "RXNE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    #[doc = "0: The RXDR register is empty"]
    Empty = 0,
    #[doc = "1: Received data is copied into the RXDR register, and is ready to be read"]
    NotEmpty = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - RXNE"]
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
#[doc = "ADDR\n\nValue on reset: 0"]
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
#[doc = "Field `ADDR` reader - ADDR"]
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
#[doc = "NACKF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKF_A {
    #[doc = "0: No NACK has been received"]
    NoNack = 0,
    #[doc = "1: NACK has been received"]
    Nack = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKF` reader - NACKF"]
pub type NACKF_R = crate::BitReader<NACKF_A>;
impl NACKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::NoNack,
            true => NACKF_A::Nack,
        }
    }
    #[doc = "Checks if the value of the field is `NoNack`"]
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKF_A::NoNack
    }
    #[doc = "Checks if the value of the field is `Nack`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKF_A::Nack
    }
}
#[doc = "STOPF\n\nValue on reset: 0"]
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
#[doc = "Field `STOPF` reader - STOPF"]
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
#[doc = "TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "0: Transfer is not complete"]
    NotComplete = 0,
    #[doc = "1: NBYTES has been transfered"]
    Complete = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - TC"]
pub type TC_R = crate::BitReader<TC_A>;
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::NotComplete,
            true => TC_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TC_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC_A::Complete
    }
}
#[doc = "TCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR_A {
    #[doc = "0: Transfer is not complete"]
    NotComplete = 0,
    #[doc = "1: NBYTES has been transfered"]
    Complete = 1,
}
impl From<TCR_A> for bool {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCR` reader - TCR"]
pub type TCR_R = crate::BitReader<TCR_A>;
impl TCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCR_A {
        match self.bits {
            false => TCR_A::NotComplete,
            true => TCR_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCR_A::Complete
    }
}
#[doc = "BERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    #[doc = "0: No bus error"]
    NoError = 0,
    #[doc = "1: Misplaced Start and Stop condition is detected"]
    Error = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` reader - BERR"]
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
#[doc = "ARLO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLO_A {
    #[doc = "0: No arbitration lost"]
    NotLost = 0,
    #[doc = "1: Arbitration lost"]
    Lost = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLO` reader - ARLO"]
pub type ARLO_R = crate::BitReader<ARLO_A>;
impl ARLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NotLost,
            true => ARLO_A::Lost,
        }
    }
    #[doc = "Checks if the value of the field is `NotLost`"]
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLO_A::NotLost
    }
    #[doc = "Checks if the value of the field is `Lost`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO_A::Lost
    }
}
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun/underrun error occurs"]
    NoOverrun = 0,
    #[doc = "1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs"]
    Overrun = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - OVR"]
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
#[doc = "PECERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    #[doc = "0: Received PEC does match with PEC register"]
    Match = 0,
    #[doc = "1: Received PEC does not match with PEC register"]
    NoMatch = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PECERR"]
pub type PECERR_R = crate::BitReader<PECERR_A>;
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::Match,
            true => PECERR_A::NoMatch,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == PECERR_A::Match
    }
    #[doc = "Checks if the value of the field is `NoMatch`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == PECERR_A::NoMatch
    }
}
#[doc = "TIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: No timeout occured"]
    NoTimeout = 0,
    #[doc = "1: Timeout occured"]
    Timeout = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - TIMEOUT"]
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
#[doc = "ALERT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERT_A {
    #[doc = "0: SMBA alert is not detected"]
    NoAlert = 0,
    #[doc = "1: SMBA alert event is detected on SMBA pin"]
    Alert = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERT` reader - ALERT"]
pub type ALERT_R = crate::BitReader<ALERT_A>;
impl ALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::NoAlert,
            true => ALERT_A::Alert,
        }
    }
    #[doc = "Checks if the value of the field is `NoAlert`"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERT_A::NoAlert
    }
    #[doc = "Checks if the value of the field is `Alert`"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERT_A::Alert
    }
}
#[doc = "BUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: No communication is in progress on the bus"]
    NotBusy = 0,
    #[doc = "1: A communication is in progress on the bus"]
    Busy = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NotBusy,
            true => BUSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `NotBusy`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NotBusy
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::Busy
    }
}
#[doc = "DIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Write transfer, slave enters receiver mode"]
    Write = 0,
    #[doc = "1: Read transfer, slave enters transmitter mode"]
    Read = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::Write,
            true => DIR_A::Read,
        }
    }
    #[doc = "Checks if the value of the field is `Write`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DIR_A::Write
    }
    #[doc = "Checks if the value of the field is `Read`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DIR_A::Read
    }
}
#[doc = "Field `ADDCODE` reader - ADDCODE"]
pub type ADDCODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACKF"]
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCR"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARLO"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PECERR"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ALERT"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - ADDCODE"]
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W<0> {
        TXE_W::new(self)
    }
    #[doc = "Bit 1 - TXIS"]
    #[inline(always)]
    pub fn txis(&mut self) -> TXIS_W<1> {
        TXIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0x01"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
