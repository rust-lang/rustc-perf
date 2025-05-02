#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITR1_RMP` reader - Timer Input 4 remap"]
pub type ITR1_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITR1_RMP` writer - Timer Input 4 remap"]
pub type ITR1_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 10:11 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<10> {
        ITR1_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
