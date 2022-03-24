#[doc = "Register `DMAR` reader"]
pub struct R(crate::R<DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAR` writer"]
pub struct W(crate::W<DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAR_SPEC>;
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
impl From<crate::W<DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAB` reader - DMA register for burst accesses"]
pub struct DMAB_R(crate::FieldReader<u16, u16>);
impl DMAB_R {
    pub(crate) fn new(bits: u16) -> Self {
        DMAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAB` writer - DMA register for burst accesses"]
pub struct DMAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W {
        DMAB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmar](index.html) module"]
pub struct DMAR_SPEC;
impl crate::RegisterSpec for DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmar::R](R) reader structure"]
impl crate::Readable for DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmar::W](W) writer structure"]
impl crate::Writable for DMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
