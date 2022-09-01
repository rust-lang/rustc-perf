#[doc = "Register `DMARDLAR` reader"]
pub struct R(crate::R<DMARDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARDLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARDLAR` writer"]
pub struct W(crate::W<DMARDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARDLAR_SPEC>;
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
impl From<crate::W<DMARDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARDLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRL` reader - SRL"]
pub struct SRL_R(crate::FieldReader<u32, u32>);
impl SRL_R {
    pub(crate) fn new(bits: u32) -> Self {
        SRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRL` writer - SRL"]
pub struct SRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SRL"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRL"]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W {
        SRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA receive descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmardlar](index.html) module"]
pub struct DMARDLAR_SPEC;
impl crate::RegisterSpec for DMARDLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmardlar::R](R) reader structure"]
impl crate::Readable for DMARDLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmardlar::W](W) writer structure"]
impl crate::Writable for DMARDLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMARDLAR to value 0"]
impl crate::Resettable for DMARDLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
