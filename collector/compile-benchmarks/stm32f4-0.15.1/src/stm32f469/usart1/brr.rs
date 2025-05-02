#[doc = "Register `BRR` reader"]
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_Mantissa` reader - mantissa of USARTDIV"]
pub type DIV_MANTISSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV_Mantissa` writer - mantissa of USARTDIV"]
pub type DIV_MANTISSA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BRR_SPEC, u16, u16, 12, O>;
#[doc = "Field `DIV_Fraction` reader - fraction of USARTDIV"]
pub type DIV_FRACTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV_Fraction` writer - fraction of USARTDIV"]
pub type DIV_FRACTION_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BRR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn div_mantissa(&self) -> DIV_MANTISSA_R {
        DIV_MANTISSA_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DIV_FRACTION_R {
        DIV_FRACTION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn div_mantissa(&mut self) -> DIV_MANTISSA_W<4> {
        DIV_MANTISSA_W::new(self)
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W<0> {
        DIV_FRACTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brr::R](R) reader structure"]
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
