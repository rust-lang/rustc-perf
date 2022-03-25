#[doc = "Register `TCCR5` reader"]
pub struct R(crate::R<TCCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR5` writer"]
pub struct W(crate::W<TCCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR5_SPEC>;
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
impl From<crate::W<TCCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSWR_TOCNT` reader - Low-Power Write Timeout Counter"]
pub struct LSWR_TOCNT_R(crate::FieldReader<u16, u16>);
impl LSWR_TOCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        LSWR_TOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSWR_TOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSWR_TOCNT` writer - Low-Power Write Timeout Counter"]
pub struct LSWR_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSWR_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low-Power Write Timeout Counter"]
    #[inline(always)]
    pub fn lswr_tocnt(&self) -> LSWR_TOCNT_R {
        LSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-Power Write Timeout Counter"]
    #[inline(always)]
    pub fn lswr_tocnt(&mut self) -> LSWR_TOCNT_W {
        LSWR_TOCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr5](index.html) module"]
pub struct TCCR5_SPEC;
impl crate::RegisterSpec for TCCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr5::R](R) reader structure"]
impl crate::Readable for TCCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr5::W](W) writer structure"]
impl crate::Writable for TCCR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCR5 to value 0"]
impl crate::Resettable for TCCR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
