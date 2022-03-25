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
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Presp Mode"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `HSWR_TOCNT` reader - High-Speed Write Timeout Counter"]
pub struct HSWR_TOCNT_R(crate::FieldReader<u16, u16>);
impl HSWR_TOCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSWR_TOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSWR_TOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSWR_TOCNT` writer - High-Speed Write Timeout Counter"]
pub struct HSWR_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSWR_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Presp Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 0x01) != 0)
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
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bits 0:15 - High-Speed Write Timeout Counter"]
    #[inline(always)]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W {
        HSWR_TOCNT_W { w: self }
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
