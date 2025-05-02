#[doc = "Register `AMTCR` reader"]
pub struct R(crate::R<AMTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMTCR` writer"]
pub struct W(crate::W<AMTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMTCR_SPEC>;
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
impl From<crate::W<AMTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT` reader - Dead Time"]
pub type DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT` writer - Dead Time"]
pub type DT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AMTCR_SPEC, u8, u8, 8, O>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disabled AHB/AXI dead-time functionality"]
    Disabled = 0,
    #[doc = "1: Enabled AHB/AXI dead-time functionality"]
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMTCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disabled AHB/AXI dead-time functionality"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "Enabled AHB/AXI dead-time functionality"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 8:15 - Dead Time"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - Dead Time"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<8> {
        DT_W::new(self)
    }
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB master timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amtcr](index.html) module"]
pub struct AMTCR_SPEC;
impl crate::RegisterSpec for AMTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amtcr::R](R) reader structure"]
impl crate::Readable for AMTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amtcr::W](W) writer structure"]
impl crate::Writable for AMTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMTCR to value 0"]
impl crate::Resettable for AMTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
