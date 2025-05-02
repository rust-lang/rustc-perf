#[doc = "Register `BGCLUT` reader"]
pub struct R(crate::R<BGCLUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCLUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCLUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCLUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCLUT` writer"]
pub struct W(crate::W<BGCLUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCLUT_SPEC>;
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
impl From<crate::W<BGCLUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCLUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APLHA` reader - APLHA"]
pub type APLHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APLHA` writer - APLHA"]
pub type APLHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGCLUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED` reader - RED"]
pub type RED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED` writer - RED"]
pub type RED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGCLUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `GREEN` reader - GREEN"]
pub type GREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GREEN` writer - GREEN"]
pub type GREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGCLUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `BLUE` reader - BLUE"]
pub type BLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLUE` writer - BLUE"]
pub type BLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGCLUT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - APLHA"]
    #[inline(always)]
    pub fn aplha(&self) -> APLHA_R {
        APLHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - APLHA"]
    #[inline(always)]
    pub fn aplha(&mut self) -> APLHA_W<24> {
        APLHA_W::new(self)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BGCLUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgclut](index.html) module"]
pub struct BGCLUT_SPEC;
impl crate::RegisterSpec for BGCLUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgclut::R](R) reader structure"]
impl crate::Readable for BGCLUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgclut::W](W) writer structure"]
impl crate::Writable for BGCLUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGCLUT to value 0"]
impl crate::Resettable for BGCLUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
