#[doc = "Reader of register STA"]
pub type R = crate::R<u32, super::STA>;
#[doc = "CE-ATA command completion signal received for CMD61\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATAEND_A {
    #[doc = "0: Completion signal not received"]
    NOTRECEIVED = 0,
    #[doc = "1: CE-ATA command completion signal received for CMD61"]
    RECEIVED = 1,
}
impl From<CEATAEND_A> for bool {
    #[inline(always)]
    fn from(variant: CEATAEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEATAEND`"]
pub type CEATAEND_R = crate::R<bool, CEATAEND_A>;
impl CEATAEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEATAEND_A {
        match self.bits {
            false => CEATAEND_A::NOTRECEIVED,
            true => CEATAEND_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == CEATAEND_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == CEATAEND_A::RECEIVED
    }
}
#[doc = "SDIO interrupt received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOIT_A {
    #[doc = "0: SDIO interrupt not receieved"]
    NOTRECEIVED = 0,
    #[doc = "1: SDIO interrupt received"]
    RECEIVED = 1,
}
impl From<SDIOIT_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDIOIT`"]
pub type SDIOIT_R = crate::R<bool, SDIOIT_A>;
impl SDIOIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOIT_A {
        match self.bits {
            false => SDIOIT_A::NOTRECEIVED,
            true => SDIOIT_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == SDIOIT_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == SDIOIT_A::RECEIVED
    }
}
#[doc = "Data available in receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDAVL_A {
    #[doc = "0: Data not available in receive FIFO"]
    NOTAVAILABLE = 0,
    #[doc = "1: Data available in receive FIFO"]
    AVAILABLE = 1,
}
impl From<RXDAVL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDAVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDAVL`"]
pub type RXDAVL_R = crate::R<bool, RXDAVL_A>;
impl RXDAVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDAVL_A {
        match self.bits {
            false => RXDAVL_A::NOTAVAILABLE,
            true => RXDAVL_A::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTAVAILABLE`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == RXDAVL_A::NOTAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXDAVL_A::AVAILABLE
    }
}
#[doc = "Data available in transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDAVL_A {
    #[doc = "0: Data not available in transmit FIFO"]
    NOTAVAILABLE = 0,
    #[doc = "1: Data available in transmit FIFO"]
    AVAILABLE = 1,
}
impl From<TXDAVL_A> for bool {
    #[inline(always)]
    fn from(variant: TXDAVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDAVL`"]
pub type TXDAVL_R = crate::R<bool, TXDAVL_A>;
impl TXDAVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDAVL_A {
        match self.bits {
            false => TXDAVL_A::NOTAVAILABLE,
            true => TXDAVL_A::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTAVAILABLE`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TXDAVL_A::NOTAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TXDAVL_A::AVAILABLE
    }
}
#[doc = "Receive FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOE_A {
    #[doc = "0: Receive FIFO not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Receive FIFO empty"]
    EMPTY = 1,
}
impl From<RXFIFOE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFIFOE`"]
pub type RXFIFOE_R = crate::R<bool, RXFIFOE_A>;
impl RXFIFOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOE_A {
        match self.bits {
            false => RXFIFOE_A::NOTEMPTY,
            true => RXFIFOE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXFIFOE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXFIFOE_A::EMPTY
    }
}
#[doc = "Transmit FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOE_A {
    #[doc = "0: Transmit FIFO not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words."]
    EMPTY = 1,
}
impl From<TXFIFOE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFIFOE`"]
pub type TXFIFOE_R = crate::R<bool, TXFIFOE_A>;
impl TXFIFOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOE_A {
        match self.bits {
            false => TXFIFOE_A::NOTEMPTY,
            true => TXFIFOE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFIFOE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFIFOE_A::EMPTY
    }
}
#[doc = "Receive FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOF_A {
    #[doc = "0: Transmit FIFO not full"]
    NOTFULL = 0,
    #[doc = "1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full."]
    FULL = 1,
}
impl From<RXFIFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFIFOF`"]
pub type RXFIFOF_R = crate::R<bool, RXFIFOF_A>;
impl RXFIFOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOF_A {
        match self.bits {
            false => RXFIFOF_A::NOTFULL,
            true => RXFIFOF_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFIFOF_A::NOTFULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFIFOF_A::FULL
    }
}
#[doc = "Transmit FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOF_A {
    #[doc = "0: Transmit FIFO not full"]
    NOTFULL = 0,
    #[doc = "1: Transmit FIFO full"]
    FULL = 1,
}
impl From<TXFIFOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFIFOF`"]
pub type TXFIFOF_R = crate::R<bool, TXFIFOF_A>;
impl TXFIFOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOF_A {
        match self.bits {
            false => TXFIFOF_A::NOTFULL,
            true => TXFIFOF_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFIFOF_A::NOTFULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFIFOF_A::FULL
    }
}
#[doc = "Receive FIFO half full: there are at least 8 words in the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOHF_A {
    #[doc = "0: Receive FIFO not half full"]
    NOTHALFFULL = 0,
    #[doc = "1: Receive FIFO half full. At least 8 words in the FIFO"]
    HALFFULL = 1,
}
impl From<RXFIFOHF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFIFOHF`"]
pub type RXFIFOHF_R = crate::R<bool, RXFIFOHF_A>;
impl RXFIFOHF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOHF_A {
        match self.bits {
            false => RXFIFOHF_A::NOTHALFFULL,
            true => RXFIFOHF_A::HALFFULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALFFULL`"]
    #[inline(always)]
    pub fn is_not_half_full(&self) -> bool {
        *self == RXFIFOHF_A::NOTHALFFULL
    }
    #[doc = "Checks if the value of the field is `HALFFULL`"]
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == RXFIFOHF_A::HALFFULL
    }
}
#[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOHE_A {
    #[doc = "0: Transmit FIFO not half empty"]
    NOTHALFEMPTY = 0,
    #[doc = "1: Transmit FIFO half empty. At least 8 words can be written into the FIFO"]
    HALFEMPTY = 1,
}
impl From<TXFIFOHE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFIFOHE`"]
pub type TXFIFOHE_R = crate::R<bool, TXFIFOHE_A>;
impl TXFIFOHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOHE_A {
        match self.bits {
            false => TXFIFOHE_A::NOTHALFEMPTY,
            true => TXFIFOHE_A::HALFEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALFEMPTY`"]
    #[inline(always)]
    pub fn is_not_half_empty(&self) -> bool {
        *self == TXFIFOHE_A::NOTHALFEMPTY
    }
    #[doc = "Checks if the value of the field is `HALFEMPTY`"]
    #[inline(always)]
    pub fn is_half_empty(&self) -> bool {
        *self == TXFIFOHE_A::HALFEMPTY
    }
}
#[doc = "Data receive in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXACT_A {
    #[doc = "0: Data receive not in progress"]
    NOTINPROGRESS = 0,
    #[doc = "1: Data receive in progress"]
    INPROGRESS = 1,
}
impl From<RXACT_A> for bool {
    #[inline(always)]
    fn from(variant: RXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXACT`"]
pub type RXACT_R = crate::R<bool, RXACT_A>;
impl RXACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXACT_A {
        match self.bits {
            false => RXACT_A::NOTINPROGRESS,
            true => RXACT_A::INPROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINPROGRESS`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RXACT_A::NOTINPROGRESS
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RXACT_A::INPROGRESS
    }
}
#[doc = "Data transmit in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACT_A {
    #[doc = "0: Data transmit is not in progress"]
    NOTINPROGRESS = 0,
    #[doc = "1: Data transmit in progress"]
    INPROGRESS = 1,
}
impl From<TXACT_A> for bool {
    #[inline(always)]
    fn from(variant: TXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXACT`"]
pub type TXACT_R = crate::R<bool, TXACT_A>;
impl TXACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACT_A {
        match self.bits {
            false => TXACT_A::NOTINPROGRESS,
            true => TXACT_A::INPROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINPROGRESS`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == TXACT_A::NOTINPROGRESS
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == TXACT_A::INPROGRESS
    }
}
#[doc = "Command transfer in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACT_A {
    #[doc = "0: Command transfer not in progress"]
    NOTINPROGRESS = 0,
    #[doc = "1: Command tranfer in progress"]
    INPROGRESS = 1,
}
impl From<CMDACT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDACT`"]
pub type CMDACT_R = crate::R<bool, CMDACT_A>;
impl CMDACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDACT_A {
        match self.bits {
            false => CMDACT_A::NOTINPROGRESS,
            true => CMDACT_A::INPROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINPROGRESS`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == CMDACT_A::NOTINPROGRESS
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CMDACT_A::INPROGRESS
    }
}
#[doc = "Data block sent/received (CRC check passed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBCKEND_A {
    #[doc = "0: Data block not sent/received (CRC check failed)"]
    NOTTRANSFERRED = 0,
    #[doc = "1: Data block sent/received (CRC check passed)"]
    TRANSFERRED = 1,
}
impl From<DBCKEND_A> for bool {
    #[inline(always)]
    fn from(variant: DBCKEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBCKEND`"]
pub type DBCKEND_R = crate::R<bool, DBCKEND_A>;
impl DBCKEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBCKEND_A {
        match self.bits {
            false => DBCKEND_A::NOTTRANSFERRED,
            true => DBCKEND_A::TRANSFERRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTTRANSFERRED`"]
    #[inline(always)]
    pub fn is_not_transferred(&self) -> bool {
        *self == DBCKEND_A::NOTTRANSFERRED
    }
    #[doc = "Checks if the value of the field is `TRANSFERRED`"]
    #[inline(always)]
    pub fn is_transferred(&self) -> bool {
        *self == DBCKEND_A::TRANSFERRED
    }
}
#[doc = "Start bit not detected on all data signals in wide bus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBITERR_A {
    #[doc = "0: No start bit detected error"]
    DETECTED = 0,
    #[doc = "1: Start bit not detected error"]
    NOTDETECTED = 1,
}
impl From<STBITERR_A> for bool {
    #[inline(always)]
    fn from(variant: STBITERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STBITERR`"]
pub type STBITERR_R = crate::R<bool, STBITERR_A>;
impl STBITERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBITERR_A {
        match self.bits {
            false => STBITERR_A::DETECTED,
            true => STBITERR_A::NOTDETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == STBITERR_A::DETECTED
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == STBITERR_A::NOTDETECTED
    }
}
#[doc = "Data end (data counter, SDIDCOUNT, is zero)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAEND_A {
    #[doc = "1: Data end (DCOUNT, is zero)"]
    DONE = 1,
    #[doc = "0: Not done"]
    NOTDONE = 0,
}
impl From<DATAEND_A> for bool {
    #[inline(always)]
    fn from(variant: DATAEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATAEND`"]
pub type DATAEND_R = crate::R<bool, DATAEND_A>;
impl DATAEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAEND_A {
        match self.bits {
            true => DATAEND_A::DONE,
            false => DATAEND_A::NOTDONE,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == DATAEND_A::DONE
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == DATAEND_A::NOTDONE
    }
}
#[doc = "Command sent (no response required)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDSENT_A {
    #[doc = "0: Command not sent"]
    NOTSENT = 0,
    #[doc = "1: Command sent (no response required)"]
    SENT = 1,
}
impl From<CMDSENT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDSENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDSENT`"]
pub type CMDSENT_R = crate::R<bool, CMDSENT_A>;
impl CMDSENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDSENT_A {
        match self.bits {
            false => CMDSENT_A::NOTSENT,
            true => CMDSENT_A::SENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSENT`"]
    #[inline(always)]
    pub fn is_not_sent(&self) -> bool {
        *self == CMDSENT_A::NOTSENT
    }
    #[doc = "Checks if the value of the field is `SENT`"]
    #[inline(always)]
    pub fn is_sent(&self) -> bool {
        *self == CMDSENT_A::SENT
    }
}
#[doc = "Command response received (CRC check passed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDREND_A {
    #[doc = "0: Command not done"]
    NOTDONE = 0,
    #[doc = "1: Command response received (CRC check passed)"]
    DONE = 1,
}
impl From<CMDREND_A> for bool {
    #[inline(always)]
    fn from(variant: CMDREND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDREND`"]
pub type CMDREND_R = crate::R<bool, CMDREND_A>;
impl CMDREND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDREND_A {
        match self.bits {
            false => CMDREND_A::NOTDONE,
            true => CMDREND_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == CMDREND_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == CMDREND_A::DONE
    }
}
#[doc = "Received FIFO overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVERR_A {
    #[doc = "0: No FIFO overrun error"]
    NOOVERRUN = 0,
    #[doc = "1: Receive FIFO overrun error"]
    OVERRUN = 1,
}
impl From<RXOVERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXOVERR`"]
pub type RXOVERR_R = crate::R<bool, RXOVERR_A>;
impl RXOVERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVERR_A {
        match self.bits {
            false => RXOVERR_A::NOOVERRUN,
            true => RXOVERR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXOVERR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXOVERR_A::OVERRUN
    }
}
#[doc = "Transmit FIFO underrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUNDERR_A {
    #[doc = "0: No transmit FIFO underrun error"]
    NOUNDERRUN = 0,
    #[doc = "1: Transmit FIFO underrun error"]
    UNDERRUN = 1,
}
impl From<TXUNDERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXUNDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXUNDERR`"]
pub type TXUNDERR_R = crate::R<bool, TXUNDERR_A>;
impl TXUNDERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUNDERR_A {
        match self.bits {
            false => TXUNDERR_A::NOUNDERRUN,
            true => TXUNDERR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXUNDERR_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXUNDERR_A::UNDERRUN
    }
}
#[doc = "Data timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTIMEOUT_A {
    #[doc = "0: No data timeout"]
    NOTIMEOUT = 0,
    #[doc = "1: Data timeout"]
    TIMEOUT = 1,
}
impl From<DTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTIMEOUT`"]
pub type DTIMEOUT_R = crate::R<bool, DTIMEOUT_A>;
impl DTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTIMEOUT_A {
        match self.bits {
            false => DTIMEOUT_A::NOTIMEOUT,
            true => DTIMEOUT_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == DTIMEOUT_A::NOTIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DTIMEOUT_A::TIMEOUT
    }
}
#[doc = "Command response timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMEOUT_A {
    #[doc = "0: No Command timeout"]
    NOTIMEOUT = 0,
    #[doc = "1: Command timeout"]
    TIMEOUT = 1,
}
impl From<CTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTIMEOUT`"]
pub type CTIMEOUT_R = crate::R<bool, CTIMEOUT_A>;
impl CTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMEOUT_A {
        match self.bits {
            false => CTIMEOUT_A::NOTIMEOUT,
            true => CTIMEOUT_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == CTIMEOUT_A::NOTIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == CTIMEOUT_A::TIMEOUT
    }
}
#[doc = "Data block sent/received (CRC check failed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCRCFAIL_A {
    #[doc = "0: No Data block sent/received crc check fail"]
    NOTFAILED = 0,
    #[doc = "1: Data block sent/received crc failed"]
    FAILED = 1,
}
impl From<DCRCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: DCRCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCRCFAIL`"]
pub type DCRCFAIL_R = crate::R<bool, DCRCFAIL_A>;
impl DCRCFAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCRCFAIL_A {
        match self.bits {
            false => DCRCFAIL_A::NOTFAILED,
            true => DCRCFAIL_A::FAILED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFAILED`"]
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == DCRCFAIL_A::NOTFAILED
    }
    #[doc = "Checks if the value of the field is `FAILED`"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == DCRCFAIL_A::FAILED
    }
}
#[doc = "Command response received (CRC check failed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRCFAIL_A {
    #[doc = "0: Command response received, crc check passed"]
    NOTFAILED = 0,
    #[doc = "1: Command response received, crc check failed"]
    FAILED = 1,
}
impl From<CCRCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCRCFAIL`"]
pub type CCRCFAIL_R = crate::R<bool, CCRCFAIL_A>;
impl CCRCFAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRCFAIL_A {
        match self.bits {
            false => CCRCFAIL_A::NOTFAILED,
            true => CCRCFAIL_A::FAILED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFAILED`"]
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == CCRCFAIL_A::NOTFAILED
    }
    #[doc = "Checks if the value of the field is `FAILED`"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == CCRCFAIL_A::FAILED
    }
}
impl R {
    #[doc = "Bit 23 - CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode"]
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 0x01) != 0)
    }
}
