#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `LINE_MIS`"]
pub type LINE_MIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSYNC_MIS`"]
pub type VSYNC_MIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR_MIS`"]
pub type ERR_MIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR_MIS`"]
pub type OVR_MIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRAME_MIS`"]
pub type FRAME_MIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - Line masked interrupt status"]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VSYNC masked interrupt status"]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization error masked interrupt status"]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun masked interrupt status"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture complete masked interrupt status"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 0x01) != 0)
    }
}
