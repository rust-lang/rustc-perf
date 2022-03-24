#[doc = "Register `DIEPTSIZ1` reader"]
pub struct R(crate::R<DIEPTSIZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ1` writer"]
pub struct W(crate::W<DIEPTSIZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ1_SPEC>;
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
impl From<crate::W<DIEPTSIZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub struct XFRSIZ_R(crate::FieldReader<u32, u32>);
impl XFRSIZ_R {
    pub(crate) fn new(bits: u32) -> Self {
        XFRSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRSIZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub struct XFRSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
#[doc = "Field `PKTCNT` reader - Packet count"]
pub struct PKTCNT_R(crate::FieldReader<u16, u16>);
impl PKTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTCNT` writer - Packet count"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | ((value as u32 & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Field `MCNT` reader - Multi count"]
pub struct MCNT_R(crate::FieldReader<u8, u8>);
impl MCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCNT` writer - Multi count"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W { w: self }
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz1](index.html) module"]
pub struct DIEPTSIZ1_SPEC;
impl crate::RegisterSpec for DIEPTSIZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz1::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz1::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTSIZ1 to value 0"]
impl crate::Resettable for DIEPTSIZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
