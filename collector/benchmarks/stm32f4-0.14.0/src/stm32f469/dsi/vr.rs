#[doc = "Register `VR` reader"]
pub struct R(crate::R<VR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VR` writer"]
pub struct W(crate::W<VR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VR_SPEC>;
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
impl From<crate::W<VR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION` reader - Version of the DSI Host"]
pub struct VERSION_R(crate::FieldReader<u32, u32>);
impl VERSION_R {
    pub(crate) fn new(bits: u32) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VERSION` writer - Version of the DSI Host"]
pub struct VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Version of the DSI Host"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Version of the DSI Host"]
    #[inline(always)]
    pub fn version(&mut self) -> VERSION_W {
        VERSION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Version Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vr](index.html) module"]
pub struct VR_SPEC;
impl crate::RegisterSpec for VR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vr::R](R) reader structure"]
impl crate::Readable for VR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vr::W](W) writer structure"]
impl crate::Writable for VR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VR to value 0x3133_302a"]
impl crate::Resettable for VR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3133_302a
    }
}
