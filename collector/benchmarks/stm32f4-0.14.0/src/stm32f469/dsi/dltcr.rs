#[doc = "Register `DLTCR` reader"]
pub struct R(crate::R<DLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLTCR` writer"]
pub struct W(crate::W<DLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HS2LP_TIME` reader - High-Speed To Low-Power Time"]
pub struct HS2LP_TIME_R(crate::FieldReader<u8, u8>);
impl HS2LP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        HS2LP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS2LP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS2LP_TIME` writer - High-Speed To Low-Power Time"]
pub struct HS2LP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> HS2LP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `LP2HS_TIME` reader - Low-Power To High-Speed Time"]
pub struct LP2HS_TIME_R(crate::FieldReader<u8, u8>);
impl LP2HS_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        LP2HS_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LP2HS_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP2HS_TIME` writer - Low-Power To High-Speed Time"]
pub struct LP2HS_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LP2HS_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MRD_TIME` reader - Maximum Read Time"]
pub struct MRD_TIME_R(crate::FieldReader<u16, u16>);
impl MRD_TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        MRD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRD_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRD_TIME` writer - Maximum Read Time"]
pub struct MRD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - High-Speed To Low-Power Time"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Low-Power To High-Speed Time"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:14 - Maximum Read Time"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - High-Speed To Low-Power Time"]
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W {
        HS2LP_TIME_W { w: self }
    }
    #[doc = "Bits 16:23 - Low-Power To High-Speed Time"]
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W {
        LP2HS_TIME_W { w: self }
    }
    #[doc = "Bits 0:14 - Maximum Read Time"]
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MRD_TIME_W {
        MRD_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Data Lane Timer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dltcr](index.html) module"]
pub struct DLTCR_SPEC;
impl crate::RegisterSpec for DLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dltcr::R](R) reader structure"]
impl crate::Readable for DLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dltcr::W](W) writer structure"]
impl crate::Writable for DLTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLTCR to value 0"]
impl crate::Resettable for DLTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
