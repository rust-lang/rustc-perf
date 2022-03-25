#[doc = "Register `MACA1LR` reader"]
pub struct R(crate::R<MACA1LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA1LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA1LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA1LR` writer"]
pub struct W(crate::W<MACA1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA1LR_SPEC>;
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
impl From<crate::W<MACA1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA1L` reader - MACA1LR"]
pub struct MACA1L_R(crate::FieldReader<u32, u32>);
impl MACA1L_R {
    pub(crate) fn new(bits: u32) -> Self {
        MACA1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACA1L_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACA1L` writer - MACA1LR"]
pub struct MACA1L_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA1L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1l(&self) -> MACA1L_R {
        MACA1L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1l(&mut self) -> MACA1L_W {
        MACA1L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address1 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca1lr](index.html) module"]
pub struct MACA1LR_SPEC;
impl crate::RegisterSpec for MACA1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca1lr::R](R) reader structure"]
impl crate::Readable for MACA1LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca1lr::W](W) writer structure"]
impl crate::Writable for MACA1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for MACA1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
