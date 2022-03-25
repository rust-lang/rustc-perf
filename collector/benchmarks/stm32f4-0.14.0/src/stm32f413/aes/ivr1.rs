#[doc = "Register `IVR1` reader"]
pub struct R(crate::R<IVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVR1` writer"]
pub struct W(crate::W<IVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR1_SPEC>;
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
impl From<crate::W<IVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_IVR1` reader - Initialization Vector Register (IVR \\[63:32\\])"]
pub struct AES_IVR1_R(crate::FieldReader<u32, u32>);
impl AES_IVR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        AES_IVR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_IVR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_IVR1` writer - Initialization Vector Register (IVR \\[63:32\\])"]
pub struct AES_IVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_IVR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_ivr1(&self) -> AES_IVR1_R {
        AES_IVR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_ivr1(&mut self) -> AES_IVR1_W {
        AES_IVR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization vector register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr1](index.html) module"]
pub struct IVR1_SPEC;
impl crate::RegisterSpec for IVR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivr1::R](R) reader structure"]
impl crate::Readable for IVR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivr1::W](W) writer structure"]
impl crate::Writable for IVR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVR1 to value 0"]
impl crate::Resettable for IVR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
