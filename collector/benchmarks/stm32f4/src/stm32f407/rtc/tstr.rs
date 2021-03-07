#[doc = "Reader of register TSTR"]
pub type R = crate::R<u32, super::TSTR>;
#[doc = "Reader of field `ALARMOUTTYPE`"]
pub type ALARMOUTTYPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSINSEL`"]
pub type TSINSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP1INSEL`"]
pub type TAMP1INSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMPIE`"]
pub type TAMPIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP1TRG`"]
pub type TAMP1TRG_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP1E`"]
pub type TAMP1E_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
}
