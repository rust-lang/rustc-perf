#[doc = "Reader of register DSI_ISR1"]
pub type R = crate::R<u32, super::DSI_ISR1>;
#[doc = "Reader of field `GPRXE`"]
pub type GPRXE_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPRDE`"]
pub type GPRDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPTXE`"]
pub type GPTXE_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPWRE`"]
pub type GPWRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `GCWRE`"]
pub type GCWRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPWRE`"]
pub type LPWRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOTPE`"]
pub type EOTPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSE`"]
pub type PSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRCE`"]
pub type CRCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCME`"]
pub type ECCME_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCSE`"]
pub type ECCSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOLPRX`"]
pub type TOLPRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOHSTX`"]
pub type TOHSTX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 12 - Generic Payload Receive Error"]
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic Payload Read Error"]
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Payload Write Error"]
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Command Write Error"]
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LTDC Payload Write Error"]
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EoTp Error"]
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet Size Error"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC Error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC Multi-bit Error"]
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC Single-bit Error"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 0x01) != 0)
    }
}
