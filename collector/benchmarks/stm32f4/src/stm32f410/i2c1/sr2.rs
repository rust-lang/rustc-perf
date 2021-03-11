#[doc = "Reader of register SR2"]
pub type R = crate::R<u32, super::SR2>;
#[doc = "Reader of field `PEC`"]
pub type PEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `DUALF`"]
pub type DUALF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBHOST`"]
pub type SMBHOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBDEFAULT`"]
pub type SMBDEFAULT_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENCALL`"]
pub type GENCALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRA`"]
pub type TRA_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSL`"]
pub type MSL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 8:15 - acket error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dualf(&self) -> DUALF_R {
        DUALF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn smbhost(&self) -> SMBHOST_R {
        SMBHOST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn smbdefault(&self) -> SMBDEFAULT_R {
        SMBDEFAULT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tra(&self) -> TRA_R {
        TRA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 0x01) != 0)
    }
}
