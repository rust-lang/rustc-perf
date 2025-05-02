#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable"]
pub type DOWNIE_R = crate::BitReader<bool>;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable"]
pub type DOWNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable"]
pub type UPIE_R = crate::BitReader<bool>;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable"]
pub type UPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader<bool>;
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `CMPOKIE` reader - Compare register update OK Interrupt Enable"]
pub type CMPOKIE_R = crate::BitReader<bool>;
#[doc = "Field `CMPOKIE` writer - Compare register update OK Interrupt Enable"]
pub type CMPOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader<bool>;
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader<bool>;
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `CMPMIE` reader - Compare match Interrupt Enable"]
pub type CMPMIE_R = crate::BitReader<bool>;
#[doc = "Field `CMPMIE` writer - Compare match Interrupt Enable"]
pub type CMPMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<6> {
        DOWNIE_W::new(self)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<5> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<4> {
        ARROKIE_W::new(self)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<3> {
        CMPOKIE_W::new(self)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<2> {
        EXTTRIGIE_W::new(self)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<1> {
        ARRMIE_W::new(self)
    }
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CMPMIE_W<0> {
        CMPMIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
