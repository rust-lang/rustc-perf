#[doc = "Register `EXTICR1` reader"]
pub struct R(crate::R<EXTICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR1` writer"]
pub struct W(crate::W<EXTICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR1_SPEC>;
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
impl From<crate::W<EXTICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI3` reader - EXTI x configuration (x = 0 to 3)"]
pub type EXTI3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI3` writer - EXTI x configuration (x = 0 to 3)"]
pub type EXTI3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI2` reader - EXTI x configuration (x = 0 to 3)"]
pub type EXTI2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI2` writer - EXTI x configuration (x = 0 to 3)"]
pub type EXTI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI1` reader - EXTI x configuration (x = 0 to 3)"]
pub type EXTI1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI1` writer - EXTI x configuration (x = 0 to 3)"]
pub type EXTI1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI0` reader - EXTI x configuration (x = 0 to 3)"]
pub type EXTI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI0` writer - EXTI x configuration (x = 0 to 3)"]
pub type EXTI0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W<12> {
        EXTI3_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W<8> {
        EXTI2_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W<4> {
        EXTI1_W::new(self)
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W<0> {
        EXTI0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](index.html) module"]
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr1::R](R) reader structure"]
impl crate::Readable for EXTICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr1::W](W) writer structure"]
impl crate::Writable for EXTICR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
