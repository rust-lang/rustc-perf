#[doc = "Register `ESCR` reader"]
pub struct R(crate::R<ESCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCR` writer"]
pub struct W(crate::W<ESCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCR_SPEC>;
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
impl From<crate::W<ESCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEC` reader - Frame end delimiter code"]
pub type FEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEC` writer - Frame end delimiter code"]
pub type FEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LEC` reader - Line end delimiter code"]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - Line end delimiter code"]
pub type LEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LSC` reader - Line start delimiter code"]
pub type LSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSC` writer - Line start delimiter code"]
pub type LSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FSC` reader - Frame start delimiter code"]
pub type FSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSC` writer - Frame start delimiter code"]
pub type FSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - Frame end delimiter code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter code"]
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Frame start delimiter code"]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Frame end delimiter code"]
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<24> {
        FEC_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end delimiter code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<16> {
        LEC_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start delimiter code"]
    #[inline(always)]
    pub fn lsc(&mut self) -> LSC_W<8> {
        LSC_W::new(self)
    }
    #[doc = "Bits 0:7 - Frame start delimiter code"]
    #[inline(always)]
    pub fn fsc(&mut self) -> FSC_W<0> {
        FSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "embedded synchronization code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [escr](index.html) module"]
pub struct ESCR_SPEC;
impl crate::RegisterSpec for ESCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [escr::R](R) reader structure"]
impl crate::Readable for ESCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [escr::W](W) writer structure"]
impl crate::Writable for ESCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESCR to value 0"]
impl crate::Resettable for ESCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
