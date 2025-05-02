#[doc = "Register `TCCR4` reader"]
pub struct R(crate::R<TCCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4` writer"]
pub struct W(crate::W<TCCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4_SPEC>;
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
impl From<crate::W<TCCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - Presp Mode"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Presp Mode"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCCR4_SPEC, bool, O>;
#[doc = "Field `HSWR_TOCNT` reader - High-Speed Write Timeout Counter"]
pub type HSWR_TOCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSWR_TOCNT` writer - High-Speed Write Timeout Counter"]
pub type HSWR_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 24 - Presp Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 0:15 - High-Speed Write Timeout Counter"]
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 24 - Presp Mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<24> {
        PM_W::new(self)
    }
    #[doc = "Bits 0:15 - High-Speed Write Timeout Counter"]
    #[inline(always)]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<0> {
        HSWR_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4](index.html) module"]
pub struct TCCR4_SPEC;
impl crate::RegisterSpec for TCCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr4::R](R) reader structure"]
impl crate::Readable for TCCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4::W](W) writer structure"]
impl crate::Writable for TCCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCR4 to value 0"]
impl crate::Resettable for TCCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
