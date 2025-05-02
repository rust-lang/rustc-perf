#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI7` reader - EXTI x configuration (x = 4 to 7)"]
pub type EXTI7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI7` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI6` reader - EXTI x configuration (x = 4 to 7)"]
pub type EXTI6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI6` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI5` reader - EXTI x configuration (x = 4 to 7)"]
pub type EXTI5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI5` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI4` reader - EXTI x configuration (x = 4 to 7)"]
pub type EXTI4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI4` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W<12> {
        EXTI7_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W<8> {
        EXTI6_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W<4> {
        EXTI5_W::new(self)
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W<0> {
        EXTI4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
