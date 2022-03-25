#[doc = "Register `CRCPR` reader"]
pub struct R(crate::R<CRCPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCPR` writer"]
pub struct W(crate::W<CRCPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCPR_SPEC>;
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
impl From<crate::W<CRCPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCPOLY` reader - CRC polynomial register"]
pub struct CRCPOLY_R(crate::FieldReader<u16, u16>);
impl CRCPOLY_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRCPOLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCPOLY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCPOLY` writer - CRC polynomial register"]
pub struct CRCPOLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPOLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcpr](index.html) module"]
pub struct CRCPR_SPEC;
impl crate::RegisterSpec for CRCPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcpr::R](R) reader structure"]
impl crate::Readable for CRCPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcpr::W](W) writer structure"]
impl crate::Writable for CRCPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCPR to value 0x07"]
impl crate::Resettable for CRCPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
