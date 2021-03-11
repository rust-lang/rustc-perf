#[doc = "Reader of register CDSR"]
pub type R = crate::R<u32, super::CDSR>;
#[doc = "Reader of field `HSYNCS`"]
pub type HSYNCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSYNCS`"]
pub type VSYNCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HDES`"]
pub type HDES_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDES`"]
pub type VDES_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 0x01) != 0)
    }
}
