#[doc = "Register `TCCR3` reader"]
pub struct R(crate::R<TCCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR3` writer"]
pub struct W(crate::W<TCCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR3_SPEC>;
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
impl From<crate::W<TCCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPRD_TOCNT` reader - Low-Power Read Timeout Counter"]
pub struct LPRD_TOCNT_R(crate::FieldReader<u16, u16>);
impl LPRD_TOCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        LPRD_TOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPRD_TOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRD_TOCNT` writer - Low-Power Read Timeout Counter"]
pub struct LPRD_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRD_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low-Power Read Timeout Counter"]
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LPRD_TOCNT_R {
        LPRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-Power Read Timeout Counter"]
    #[inline(always)]
    pub fn lprd_tocnt(&mut self) -> LPRD_TOCNT_W {
        LPRD_TOCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3](index.html) module"]
pub struct TCCR3_SPEC;
impl crate::RegisterSpec for TCCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr3::R](R) reader structure"]
impl crate::Readable for TCCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr3::W](W) writer structure"]
impl crate::Writable for TCCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCR3 to value 0"]
impl crate::Resettable for TCCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
