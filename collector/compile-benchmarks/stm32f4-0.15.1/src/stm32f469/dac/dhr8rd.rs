#[doc = "Register `DHR8RD` reader"]
pub struct R(crate::R<DHR8RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR8RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR8RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR8RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHR8RD` writer"]
pub struct W(crate::W<DHR8RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR8RD_SPEC>;
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
impl From<crate::W<DHR8RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR8RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DHR8RD_SPEC, u8, u8, 8, O>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DHR8RD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<8> {
        DACC2DHR_W::new(self)
    }
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<0> {
        DACC1DHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DUAL DAC 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr8rd](index.html) module"]
pub struct DHR8RD_SPEC;
impl crate::RegisterSpec for DHR8RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhr8rd::R](R) reader structure"]
impl crate::Readable for DHR8RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhr8rd::W](W) writer structure"]
impl crate::Writable for DHR8RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHR8RD to value 0"]
impl crate::Resettable for DHR8RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
