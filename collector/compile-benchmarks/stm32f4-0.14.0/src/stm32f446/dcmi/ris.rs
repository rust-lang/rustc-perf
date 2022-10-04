#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINE_RIS` reader - Line raw interrupt status"]
pub struct LINE_RIS_R(crate::FieldReader<bool, bool>);
impl LINE_RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINE_RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC_RIS` reader - VSYNC raw interrupt status"]
pub struct VSYNC_RIS_R(crate::FieldReader<bool, bool>);
impl VSYNC_RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_RIS` reader - Synchronization error raw interrupt status"]
pub struct ERR_RIS_R(crate::FieldReader<bool, bool>);
impl ERR_RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RIS` reader - Overrun raw interrupt status"]
pub struct OVR_RIS_R(crate::FieldReader<bool, bool>);
impl OVR_RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_RIS` reader - Capture complete raw interrupt status"]
pub struct FRAME_RIS_R(crate::FieldReader<bool, bool>);
impl FRAME_RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Line raw interrupt status"]
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VSYNC raw interrupt status"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization error raw interrupt status"]
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture complete raw interrupt status"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
