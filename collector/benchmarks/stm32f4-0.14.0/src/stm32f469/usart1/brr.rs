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
pub struct DIV_MANTISSA_R(crate::FieldReader<u16, u16>);
impl DIV_MANTISSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIV_MANTISSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_MANTISSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_Mantissa` writer - mantissa of USARTDIV"]
pub struct DIV_MANTISSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_MANTISSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Field `DIV_Fraction` reader - fraction of USARTDIV"]
pub struct DIV_FRACTION_R(crate::FieldReader<u8, u8>);
impl DIV_FRACTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_FRACTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_FRACTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_Fraction` writer - fraction of USARTDIV"]
pub struct DIV_FRACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_FRACTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
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
    pub fn div_mantissa(&mut self) -> DIV_MANTISSA_W {
        DIV_MANTISSA_W { w: self }
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W {
        DIV_FRACTION_W { w: self }
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
