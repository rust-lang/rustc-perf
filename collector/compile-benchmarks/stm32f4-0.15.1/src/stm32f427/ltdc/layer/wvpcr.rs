#[doc = "Register `WVPCR` reader"]
pub struct R(crate::R<WVPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WVPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WVPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WVPCR` writer"]
pub struct W(crate::W<WVPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WVPCR_SPEC>;
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
impl From<crate::W<WVPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WVPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSPPOS` reader - Window Vertical Stop Position"]
pub type WVSPPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSPPOS` writer - Window Vertical Stop Position"]
pub type WVSPPOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WVPCR_SPEC, u16, u16, 11, O>;
#[doc = "Field `WVSTPOS` reader - Window Vertical Start Position"]
pub type WVSTPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSTPOS` writer - Window Vertical Start Position"]
pub type WVSTPOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WVPCR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<16> {
        WVSPPOS_W::new(self)
    }
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<0> {
        WVSTPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Window Vertical Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvpcr](index.html) module"]
pub struct WVPCR_SPEC;
impl crate::RegisterSpec for WVPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvpcr::R](R) reader structure"]
impl crate::Readable for WVPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wvpcr::W](W) writer structure"]
impl crate::Writable for WVPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WVPCR to value 0"]
impl crate::Resettable for WVPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
