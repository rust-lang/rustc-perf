#[doc = "Register `CKCR` reader"]
pub struct R(crate::R<CKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCR` writer"]
pub struct W(crate::W<CKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCR_SPEC>;
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
impl From<crate::W<CKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKRED` reader - Color Key Red value"]
pub type CKRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKRED` writer - Color Key Red value"]
pub type CKRED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKGREEN` reader - Color Key Green value"]
pub type CKGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKGREEN` writer - Color Key Green value"]
pub type CKGREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKBLUE` reader - Color Key Blue value"]
pub type CKBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKBLUE` writer - Color Key Blue value"]
pub type CKBLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CKCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W<16> {
        CKRED_W::new(self)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W<8> {
        CKGREEN_W::new(self)
    }
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W<0> {
        CKBLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Color Keying Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcr](index.html) module"]
pub struct CKCR_SPEC;
impl crate::RegisterSpec for CKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcr::R](R) reader structure"]
impl crate::Readable for CKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcr::W](W) writer structure"]
impl crate::Writable for CKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKCR to value 0"]
impl crate::Resettable for CKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
