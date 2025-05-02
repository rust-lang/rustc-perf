#[doc = "Register `ARG` reader"]
pub struct R(crate::R<ARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARG` writer"]
pub struct W(crate::W<ARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARG_SPEC>;
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
impl From<crate::W<ARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDARG` reader - Command argument"]
pub type CMDARG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDARG` writer - Command argument"]
pub type CMDARG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ARG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W<0> {
        CMDARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg](index.html) module"]
pub struct ARG_SPEC;
impl crate::RegisterSpec for ARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arg::R](R) reader structure"]
impl crate::Readable for ARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arg::W](W) writer structure"]
impl crate::Writable for ARG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARG to value 0"]
impl crate::Resettable for ARG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
