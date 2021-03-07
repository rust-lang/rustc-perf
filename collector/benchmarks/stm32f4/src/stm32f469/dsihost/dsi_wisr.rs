#[doc = "Reader of register DSI_WISR"]
pub type R = crate::R<u32, super::DSI_WISR>;
#[doc = "Reader of field `RRIF`"]
pub type RRIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRS`"]
pub type RRS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLUIF`"]
pub type PLLUIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLLIF`"]
pub type PLLLIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLLS`"]
pub type PLLLS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERIF`"]
pub type ERIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF`"]
pub type TEIF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 13 - Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Regulator Ready Status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL Lock Status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
}
