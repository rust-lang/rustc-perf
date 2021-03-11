#[doc = "Reader of register MACDBGR"]
pub type R = crate::R<u32, super::MACDBGR>;
#[doc = "Reader of field `TFF`"]
pub type TFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFNE`"]
pub type TFNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFWA`"]
pub type TFWA_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFRS`"]
pub type TFRS_R = crate::R<u8, u8>;
#[doc = "Reader of field `MTP`"]
pub type MTP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MTFCS`"]
pub type MTFCS_R = crate::R<u8, u8>;
#[doc = "Reader of field `MMTEA`"]
pub type MMTEA_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFFL`"]
pub type RFFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFRCS`"]
pub type RFRCS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFWRA`"]
pub type RFWRA_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSFRWCS`"]
pub type MSFRWCS_R = crate::R<u8, u8>;
#[doc = "Reader of field `MMRPEA`"]
pub type MMRPEA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 25 - Tx FIFO full"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tx FIFO not empty"]
    #[inline(always)]
    pub fn tfne(&self) -> TFNE_R {
        TFNE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Tx FIFO write active"]
    #[inline(always)]
    pub fn tfwa(&self) -> TFWA_R {
        TFWA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Tx FIFO read status"]
    #[inline(always)]
    pub fn tfrs(&self) -> TFRS_R {
        TFRS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - MAC transmitter in pause"]
    #[inline(always)]
    pub fn mtp(&self) -> MTP_R {
        MTP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC transmit frame controller status"]
    #[inline(always)]
    pub fn mtfcs(&self) -> MTFCS_R {
        MTFCS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII transmit engine active"]
    #[inline(always)]
    pub fn mmtea(&self) -> MMTEA_R {
        MMTEA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Rx FIFO fill level"]
    #[inline(always)]
    pub fn rffl(&self) -> RFFL_R {
        RFFL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Rx FIFO read controller status"]
    #[inline(always)]
    pub fn rfrcs(&self) -> RFRCS_R {
        RFRCS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Rx FIFO write controller active"]
    #[inline(always)]
    pub fn rfwra(&self) -> RFWRA_R {
        RFWRA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC small FIFO read/write controllers status"]
    #[inline(always)]
    pub fn msfrwcs(&self) -> MSFRWCS_R {
        MSFRWCS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - MAC MII receive protocol engine active"]
    #[inline(always)]
    pub fn mmrpea(&self) -> MMRPEA_R {
        MMRPEA_R::new((self.bits & 0x01) != 0)
    }
}
