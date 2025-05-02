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
#[doc = "Timer Input 4 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ITR1_RMP_A {
    #[doc = "0: TIM8 trigger output is connected to TIM2_ITR1 input"]
    Tim8Trgout = 0,
    #[doc = "1: Ethernet PTP clock is connected to TIM2_ITR1 input"]
    Ptp = 1,
    #[doc = "2: OTG FS SOF is connected to the TIM2_ITR1 input"]
    OtgFsSof = 2,
    #[doc = "3: OTG HS SOF is connected to the TIM2_ITR1 input"]
    OtgHsSof = 3,
}
impl From<ITR1_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ITR1_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ITR1_RMP` reader - Timer Input 4 remap"]
pub type ITR1_RMP_R = crate::FieldReader<u8, ITR1_RMP_A>;
impl ITR1_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITR1_RMP_A {
        match self.bits {
            0 => ITR1_RMP_A::Tim8Trgout,
            1 => ITR1_RMP_A::Ptp,
            2 => ITR1_RMP_A::OtgFsSof,
            3 => ITR1_RMP_A::OtgHsSof,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Tim8Trgout`"]
    #[inline(always)]
    pub fn is_tim8_trgout(&self) -> bool {
        *self == ITR1_RMP_A::Tim8Trgout
    }
    #[doc = "Checks if the value of the field is `Ptp`"]
    #[inline(always)]
    pub fn is_ptp(&self) -> bool {
        *self == ITR1_RMP_A::Ptp
    }
    #[doc = "Checks if the value of the field is `OtgFsSof`"]
    #[inline(always)]
    pub fn is_otg_fs_sof(&self) -> bool {
        *self == ITR1_RMP_A::OtgFsSof
    }
    #[doc = "Checks if the value of the field is `OtgHsSof`"]
    #[inline(always)]
    pub fn is_otg_hs_sof(&self) -> bool {
        *self == ITR1_RMP_A::OtgHsSof
    }
}
#[doc = "Field `ITR1_RMP` writer - Timer Input 4 remap"]
pub type ITR1_RMP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OR_SPEC, u8, ITR1_RMP_A, 2, O>;
impl<'a, const O: u8> ITR1_RMP_W<'a, O> {
    #[doc = "TIM8 trigger output is connected to TIM2_ITR1 input"]
    #[inline(always)]
    pub fn tim8_trgout(self) -> &'a mut W {
        self.variant(ITR1_RMP_A::Tim8Trgout)
    }
    #[doc = "Ethernet PTP clock is connected to TIM2_ITR1 input"]
    #[inline(always)]
    pub fn ptp(self) -> &'a mut W {
        self.variant(ITR1_RMP_A::Ptp)
    }
    #[doc = "OTG FS SOF is connected to the TIM2_ITR1 input"]
    #[inline(always)]
    pub fn otg_fs_sof(self) -> &'a mut W {
        self.variant(ITR1_RMP_A::OtgFsSof)
    }
    #[doc = "OTG HS SOF is connected to the TIM2_ITR1 input"]
    #[inline(always)]
    pub fn otg_hs_sof(self) -> &'a mut W {
        self.variant(ITR1_RMP_A::OtgHsSof)
    }
}
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
