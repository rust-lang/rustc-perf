#[doc = "Register `KEYR3` reader"]
pub struct R(crate::R<KEYR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR3` writer"]
pub struct W(crate::W<KEYR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR3_SPEC>;
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
impl From<crate::W<KEYR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_KEYR3` reader - AES key register (MSB key \\[127:96\\])"]
pub type AES_KEYR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_KEYR3` writer - AES key register (MSB key \\[127:96\\])"]
pub type AES_KEYR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn aes_keyr3(&self) -> AES_KEYR3_R {
        AES_KEYR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn aes_keyr3(&mut self) -> AES_KEYR3_W<0> {
        AES_KEYR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr3](index.html) module"]
pub struct KEYR3_SPEC;
impl crate::RegisterSpec for KEYR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr3::R](R) reader structure"]
impl crate::Readable for KEYR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr3::W](W) writer structure"]
impl crate::Writable for KEYR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR3 to value 0"]
impl crate::Resettable for KEYR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
