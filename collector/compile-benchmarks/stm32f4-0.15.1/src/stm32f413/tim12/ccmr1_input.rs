#[doc = "Register `CCMR1_Input` reader"]
pub struct R(crate::R<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR1_Input` writer"]
pub struct W(crate::W<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_INPUT_SPEC>;
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
impl From<crate::W<CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type IC2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type IC2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_INPUT_SPEC, u8, u8, 4, O>;
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler"]
pub type IC2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler"]
pub type IC2PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection"]
pub type CC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection"]
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type IC1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type IC1F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_INPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler"]
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler"]
pub type IC1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_INPUT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W<12> {
        IC2F_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W<10> {
        IC2PSC_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    #[doc = "Bits 4:6 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<4> {
        IC1F_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<2> {
        IC1PSC_W::new(self)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_input](index.html) module"]
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr1_input::R](R) reader structure"]
impl crate::Readable for CCMR1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr1_input::W](W) writer structure"]
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
