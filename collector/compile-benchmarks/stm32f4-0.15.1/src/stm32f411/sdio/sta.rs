#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CE-ATA command completion signal received for CMD61\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATAEND_A {
    #[doc = "0: Completion signal not received"]
    NotReceived = 0,
    #[doc = "1: CE-ATA command completion signal received for CMD61"]
    Received = 1,
}
impl From<CEATAEND_A> for bool {
    #[inline(always)]
    fn from(variant: CEATAEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATAEND` reader - CE-ATA command completion signal received for CMD61"]
pub type CEATAEND_R = crate::BitReader<CEATAEND_A>;
impl CEATAEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEATAEND_A {
        match self.bits {
            false => CEATAEND_A::NotReceived,
            true => CEATAEND_A::Received,
        }
    }
    #[doc = "Checks if the value of the field is `NotReceived`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == CEATAEND_A::NotReceived
    }
    #[doc = "Checks if the value of the field is `Received`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == CEATAEND_A::Received
    }
}
#[doc = "SDIO interrupt received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOIT_A {
    #[doc = "0: SDIO interrupt not receieved"]
    NotReceived = 0,
    #[doc = "1: SDIO interrupt received"]
    Received = 1,
}
impl From<SDIOIT_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub type SDIOIT_R = crate::BitReader<SDIOIT_A>;
impl SDIOIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOIT_A {
        match self.bits {
            false => SDIOIT_A::NotReceived,
            true => SDIOIT_A::Received,
        }
    }
    #[doc = "Checks if the value of the field is `NotReceived`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == SDIOIT_A::NotReceived
    }
    #[doc = "Checks if the value of the field is `Received`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == SDIOIT_A::Received
    }
}
#[doc = "Data available in receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDAVL_A {
    #[doc = "0: Data not available in receive FIFO"]
    NotAvailable = 0,
    #[doc = "1: Data available in receive FIFO"]
    Available = 1,
}
impl From<RXDAVL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDAVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDAVL` reader - Data available in receive FIFO"]
pub type RXDAVL_R = crate::BitReader<RXDAVL_A>;
impl RXDAVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDAVL_A {
        match self.bits {
            false => RXDAVL_A::NotAvailable,
            true => RXDAVL_A::Available,
        }
    }
    #[doc = "Checks if the value of the field is `NotAvailable`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == RXDAVL_A::NotAvailable
    }
    #[doc = "Checks if the value of the field is `Available`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXDAVL_A::Available
    }
}
#[doc = "Data available in transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDAVL_A {
    #[doc = "0: Data not available in transmit FIFO"]
    NotAvailable = 0,
    #[doc = "1: Data available in transmit FIFO"]
    Available = 1,
}
impl From<TXDAVL_A> for bool {
    #[inline(always)]
    fn from(variant: TXDAVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDAVL` reader - Data available in transmit FIFO"]
pub type TXDAVL_R = crate::BitReader<TXDAVL_A>;
impl TXDAVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDAVL_A {
        match self.bits {
            false => TXDAVL_A::NotAvailable,
            true => TXDAVL_A::Available,
        }
    }
    #[doc = "Checks if the value of the field is `NotAvailable`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TXDAVL_A::NotAvailable
    }
    #[doc = "Checks if the value of the field is `Available`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TXDAVL_A::Available
    }
}
#[doc = "Receive FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOE_A {
    #[doc = "0: Receive FIFO not empty"]
    NotEmpty = 0,
    #[doc = "1: Receive FIFO empty"]
    Empty = 1,
}
impl From<RXFIFOE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub type RXFIFOE_R = crate::BitReader<RXFIFOE_A>;
impl RXFIFOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOE_A {
        match self.bits {
            false => RXFIFOE_A::NotEmpty,
            true => RXFIFOE_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXFIFOE_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXFIFOE_A::Empty
    }
}
#[doc = "Transmit FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOE_A {
    #[doc = "0: Transmit FIFO not empty"]
    NotEmpty = 0,
    #[doc = "1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words."]
    Empty = 1,
}
impl From<TXFIFOE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub type TXFIFOE_R = crate::BitReader<TXFIFOE_A>;
impl TXFIFOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOE_A {
        match self.bits {
            false => TXFIFOE_A::NotEmpty,
            true => TXFIFOE_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFIFOE_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFIFOE_A::Empty
    }
}
#[doc = "Receive FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOF_A {
    #[doc = "0: Transmit FIFO not full"]
    NotFull = 0,
    #[doc = "1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full."]
    Full = 1,
}
impl From<RXFIFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub type RXFIFOF_R = crate::BitReader<RXFIFOF_A>;
impl RXFIFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOF_A {
        match self.bits {
            false => RXFIFOF_A::NotFull,
            true => RXFIFOF_A::Full,
        }
    }
    #[doc = "Checks if the value of the field is `NotFull`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFIFOF_A::NotFull
    }
    #[doc = "Checks if the value of the field is `Full`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFIFOF_A::Full
    }
}
#[doc = "Transmit FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOF_A {
    #[doc = "0: Transmit FIFO not full"]
    NotFull = 0,
    #[doc = "1: Transmit FIFO full"]
    Full = 1,
}
impl From<TXFIFOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub type TXFIFOF_R = crate::BitReader<TXFIFOF_A>;
impl TXFIFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOF_A {
        match self.bits {
            false => TXFIFOF_A::NotFull,
            true => TXFIFOF_A::Full,
        }
    }
    #[doc = "Checks if the value of the field is `NotFull`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFIFOF_A::NotFull
    }
    #[doc = "Checks if the value of the field is `Full`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFIFOF_A::Full
    }
}
#[doc = "Receive FIFO half full: there are at least 8 words in the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOHF_A {
    #[doc = "0: Receive FIFO not half full"]
    NotHalfFull = 0,
    #[doc = "1: Receive FIFO half full. At least 8 words in the FIFO"]
    HalfFull = 1,
}
impl From<RXFIFOHF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub type RXFIFOHF_R = crate::BitReader<RXFIFOHF_A>;
impl RXFIFOHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOHF_A {
        match self.bits {
            false => RXFIFOHF_A::NotHalfFull,
            true => RXFIFOHF_A::HalfFull,
        }
    }
    #[doc = "Checks if the value of the field is `NotHalfFull`"]
    #[inline(always)]
    pub fn is_not_half_full(&self) -> bool {
        *self == RXFIFOHF_A::NotHalfFull
    }
    #[doc = "Checks if the value of the field is `HalfFull`"]
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == RXFIFOHF_A::HalfFull
    }
}
#[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOHE_A {
    #[doc = "0: Transmit FIFO not half empty"]
    NotHalfEmpty = 0,
    #[doc = "1: Transmit FIFO half empty. At least 8 words can be written into the FIFO"]
    HalfEmpty = 1,
}
impl From<TXFIFOHE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub type TXFIFOHE_R = crate::BitReader<TXFIFOHE_A>;
impl TXFIFOHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOHE_A {
        match self.bits {
            false => TXFIFOHE_A::NotHalfEmpty,
            true => TXFIFOHE_A::HalfEmpty,
        }
    }
    #[doc = "Checks if the value of the field is `NotHalfEmpty`"]
    #[inline(always)]
    pub fn is_not_half_empty(&self) -> bool {
        *self == TXFIFOHE_A::NotHalfEmpty
    }
    #[doc = "Checks if the value of the field is `HalfEmpty`"]
    #[inline(always)]
    pub fn is_half_empty(&self) -> bool {
        *self == TXFIFOHE_A::HalfEmpty
    }
}
#[doc = "Data receive in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXACT_A {
    #[doc = "0: Data receive not in progress"]
    NotInProgress = 0,
    #[doc = "1: Data receive in progress"]
    InProgress = 1,
}
impl From<RXACT_A> for bool {
    #[inline(always)]
    fn from(variant: RXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXACT` reader - Data receive in progress"]
pub type RXACT_R = crate::BitReader<RXACT_A>;
impl RXACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXACT_A {
        match self.bits {
            false => RXACT_A::NotInProgress,
            true => RXACT_A::InProgress,
        }
    }
    #[doc = "Checks if the value of the field is `NotInProgress`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RXACT_A::NotInProgress
    }
    #[doc = "Checks if the value of the field is `InProgress`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RXACT_A::InProgress
    }
}
#[doc = "Data transmit in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACT_A {
    #[doc = "0: Data transmit is not in progress"]
    NotInProgress = 0,
    #[doc = "1: Data transmit in progress"]
    InProgress = 1,
}
impl From<TXACT_A> for bool {
    #[inline(always)]
    fn from(variant: TXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACT` reader - Data transmit in progress"]
pub type TXACT_R = crate::BitReader<TXACT_A>;
impl TXACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACT_A {
        match self.bits {
            false => TXACT_A::NotInProgress,
            true => TXACT_A::InProgress,
        }
    }
    #[doc = "Checks if the value of the field is `NotInProgress`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == TXACT_A::NotInProgress
    }
    #[doc = "Checks if the value of the field is `InProgress`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == TXACT_A::InProgress
    }
}
#[doc = "Command transfer in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACT_A {
    #[doc = "0: Command transfer not in progress"]
    NotInProgress = 0,
    #[doc = "1: Command tranfer in progress"]
    InProgress = 1,
}
impl From<CMDACT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDACT` reader - Command transfer in progress"]
pub type CMDACT_R = crate::BitReader<CMDACT_A>;
impl CMDACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDACT_A {
        match self.bits {
            false => CMDACT_A::NotInProgress,
            true => CMDACT_A::InProgress,
        }
    }
    #[doc = "Checks if the value of the field is `NotInProgress`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == CMDACT_A::NotInProgress
    }
    #[doc = "Checks if the value of the field is `InProgress`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CMDACT_A::InProgress
    }
}
#[doc = "Data block sent/received (CRC check passed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBCKEND_A {
    #[doc = "0: Data block not sent/received (CRC check failed)"]
    NotTransferred = 0,
    #[doc = "1: Data block sent/received (CRC check passed)"]
    Transferred = 1,
}
impl From<DBCKEND_A> for bool {
    #[inline(always)]
    fn from(variant: DBCKEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBCKEND` reader - Data block sent/received (CRC check passed)"]
pub type DBCKEND_R = crate::BitReader<DBCKEND_A>;
impl DBCKEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBCKEND_A {
        match self.bits {
            false => DBCKEND_A::NotTransferred,
            true => DBCKEND_A::Transferred,
        }
    }
    #[doc = "Checks if the value of the field is `NotTransferred`"]
    #[inline(always)]
    pub fn is_not_transferred(&self) -> bool {
        *self == DBCKEND_A::NotTransferred
    }
    #[doc = "Checks if the value of the field is `Transferred`"]
    #[inline(always)]
    pub fn is_transferred(&self) -> bool {
        *self == DBCKEND_A::Transferred
    }
}
#[doc = "Start bit not detected on all data signals in wide bus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBITERR_A {
    #[doc = "0: No start bit detected error"]
    Detected = 0,
    #[doc = "1: Start bit not detected error"]
    NotDetected = 1,
}
impl From<STBITERR_A> for bool {
    #[inline(always)]
    fn from(variant: STBITERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBITERR` reader - Start bit not detected on all data signals in wide bus mode"]
pub type STBITERR_R = crate::BitReader<STBITERR_A>;
impl STBITERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBITERR_A {
        match self.bits {
            false => STBITERR_A::Detected,
            true => STBITERR_A::NotDetected,
        }
    }
    #[doc = "Checks if the value of the field is `Detected`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == STBITERR_A::Detected
    }
    #[doc = "Checks if the value of the field is `NotDetected`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == STBITERR_A::NotDetected
    }
}
#[doc = "Data end (data counter, SDIDCOUNT, is zero)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAEND_A {
    #[doc = "0: Not done"]
    NotDone = 0,
    #[doc = "1: Data end (DCOUNT, is zero)"]
    Done = 1,
}
impl From<DATAEND_A> for bool {
    #[inline(always)]
    fn from(variant: DATAEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub type DATAEND_R = crate::BitReader<DATAEND_A>;
impl DATAEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAEND_A {
        match self.bits {
            false => DATAEND_A::NotDone,
            true => DATAEND_A::Done,
        }
    }
    #[doc = "Checks if the value of the field is `NotDone`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == DATAEND_A::NotDone
    }
    #[doc = "Checks if the value of the field is `Done`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == DATAEND_A::Done
    }
}
#[doc = "Command sent (no response required)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDSENT_A {
    #[doc = "0: Command not sent"]
    NotSent = 0,
    #[doc = "1: Command sent (no response required)"]
    Sent = 1,
}
impl From<CMDSENT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDSENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub type CMDSENT_R = crate::BitReader<CMDSENT_A>;
impl CMDSENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDSENT_A {
        match self.bits {
            false => CMDSENT_A::NotSent,
            true => CMDSENT_A::Sent,
        }
    }
    #[doc = "Checks if the value of the field is `NotSent`"]
    #[inline(always)]
    pub fn is_not_sent(&self) -> bool {
        *self == CMDSENT_A::NotSent
    }
    #[doc = "Checks if the value of the field is `Sent`"]
    #[inline(always)]
    pub fn is_sent(&self) -> bool {
        *self == CMDSENT_A::Sent
    }
}
#[doc = "Command response received (CRC check passed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDREND_A {
    #[doc = "0: Command not done"]
    NotDone = 0,
    #[doc = "1: Command response received (CRC check passed)"]
    Done = 1,
}
impl From<CMDREND_A> for bool {
    #[inline(always)]
    fn from(variant: CMDREND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub type CMDREND_R = crate::BitReader<CMDREND_A>;
impl CMDREND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDREND_A {
        match self.bits {
            false => CMDREND_A::NotDone,
            true => CMDREND_A::Done,
        }
    }
    #[doc = "Checks if the value of the field is `NotDone`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == CMDREND_A::NotDone
    }
    #[doc = "Checks if the value of the field is `Done`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == CMDREND_A::Done
    }
}
#[doc = "Received FIFO overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVERR_A {
    #[doc = "0: No FIFO overrun error"]
    NoOverrun = 0,
    #[doc = "1: Receive FIFO overrun error"]
    Overrun = 1,
}
impl From<RXOVERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub type RXOVERR_R = crate::BitReader<RXOVERR_A>;
impl RXOVERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVERR_A {
        match self.bits {
            false => RXOVERR_A::NoOverrun,
            true => RXOVERR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXOVERR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXOVERR_A::Overrun
    }
}
#[doc = "Transmit FIFO underrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUNDERR_A {
    #[doc = "0: No transmit FIFO underrun error"]
    NoUnderrun = 0,
    #[doc = "1: Transmit FIFO underrun error"]
    Underrun = 1,
}
impl From<TXUNDERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXUNDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub type TXUNDERR_R = crate::BitReader<TXUNDERR_A>;
impl TXUNDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUNDERR_A {
        match self.bits {
            false => TXUNDERR_A::NoUnderrun,
            true => TXUNDERR_A::Underrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoUnderrun`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXUNDERR_A::NoUnderrun
    }
    #[doc = "Checks if the value of the field is `Underrun`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXUNDERR_A::Underrun
    }
}
#[doc = "Data timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTIMEOUT_A {
    #[doc = "0: No data timeout"]
    NoTimeout = 0,
    #[doc = "1: Data timeout"]
    Timeout = 1,
}
impl From<DTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub type DTIMEOUT_R = crate::BitReader<DTIMEOUT_A>;
impl DTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTIMEOUT_A {
        match self.bits {
            false => DTIMEOUT_A::NoTimeout,
            true => DTIMEOUT_A::Timeout,
        }
    }
    #[doc = "Checks if the value of the field is `NoTimeout`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == DTIMEOUT_A::NoTimeout
    }
    #[doc = "Checks if the value of the field is `Timeout`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DTIMEOUT_A::Timeout
    }
}
#[doc = "Command response timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMEOUT_A {
    #[doc = "0: No Command timeout"]
    NoTimeout = 0,
    #[doc = "1: Command timeout"]
    Timeout = 1,
}
impl From<CTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub type CTIMEOUT_R = crate::BitReader<CTIMEOUT_A>;
impl CTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMEOUT_A {
        match self.bits {
            false => CTIMEOUT_A::NoTimeout,
            true => CTIMEOUT_A::Timeout,
        }
    }
    #[doc = "Checks if the value of the field is `NoTimeout`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == CTIMEOUT_A::NoTimeout
    }
    #[doc = "Checks if the value of the field is `Timeout`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == CTIMEOUT_A::Timeout
    }
}
#[doc = "Data block sent/received (CRC check failed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCRCFAIL_A {
    #[doc = "0: No Data block sent/received crc check fail"]
    NotFailed = 0,
    #[doc = "1: Data block sent/received crc failed"]
    Failed = 1,
}
impl From<DCRCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: DCRCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub type DCRCFAIL_R = crate::BitReader<DCRCFAIL_A>;
impl DCRCFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCRCFAIL_A {
        match self.bits {
            false => DCRCFAIL_A::NotFailed,
            true => DCRCFAIL_A::Failed,
        }
    }
    #[doc = "Checks if the value of the field is `NotFailed`"]
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == DCRCFAIL_A::NotFailed
    }
    #[doc = "Checks if the value of the field is `Failed`"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == DCRCFAIL_A::Failed
    }
}
#[doc = "Command response received (CRC check failed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRCFAIL_A {
    #[doc = "0: Command response received, crc check passed"]
    NotFailed = 0,
    #[doc = "1: Command response received, crc check failed"]
    Failed = 1,
}
impl From<CCRCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub type CCRCFAIL_R = crate::BitReader<CCRCFAIL_A>;
impl CCRCFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRCFAIL_A {
        match self.bits {
            false => CCRCFAIL_A::NotFailed,
            true => CCRCFAIL_A::Failed,
        }
    }
    #[doc = "Checks if the value of the field is `NotFailed`"]
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == CCRCFAIL_A::NotFailed
    }
    #[doc = "Checks if the value of the field is `Failed`"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == CCRCFAIL_A::Failed
    }
}
impl R {
    #[doc = "Bit 23 - CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode"]
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
