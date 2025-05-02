#[doc = "Register `DCCR` reader"]
pub struct R(crate::R<DCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCCR` writer"]
pub struct W(crate::W<DCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCR_SPEC>;
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
impl From<crate::W<DCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCALPHA` reader - Default Color Alpha"]
pub type DCALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCALPHA` writer - Default Color Alpha"]
pub type DCALPHA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DCRED` reader - Default Color Red"]
pub type DCRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCRED` writer - Default Color Red"]
pub type DCRED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DCGREEN` reader - Default Color Green"]
pub type DCGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCGREEN` writer - Default Color Green"]
pub type DCGREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DCBLUE` reader - Default Color Blue"]
pub type DCBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCBLUE` writer - Default Color Blue"]
pub type DCBLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - Default Color Alpha"]
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Default Color Red"]
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Default Color Green"]
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Default Color Blue"]
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Default Color Alpha"]
    #[inline(always)]
    pub fn dcalpha(&mut self) -> DCALPHA_W<24> {
        DCALPHA_W::new(self)
    }
    #[doc = "Bits 16:23 - Default Color Red"]
    #[inline(always)]
    pub fn dcred(&mut self) -> DCRED_W<16> {
        DCRED_W::new(self)
    }
    #[doc = "Bits 8:15 - Default Color Green"]
    #[inline(always)]
    pub fn dcgreen(&mut self) -> DCGREEN_W<8> {
        DCGREEN_W::new(self)
    }
    #[doc = "Bits 0:7 - Default Color Blue"]
    #[inline(always)]
    pub fn dcblue(&mut self) -> DCBLUE_W<0> {
        DCBLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Default Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccr](index.html) module"]
pub struct DCCR_SPEC;
impl crate::RegisterSpec for DCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dccr::R](R) reader structure"]
impl crate::Readable for DCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dccr::W](W) writer structure"]
impl crate::Writable for DCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCCR to value 0"]
impl crate::Resettable for DCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
