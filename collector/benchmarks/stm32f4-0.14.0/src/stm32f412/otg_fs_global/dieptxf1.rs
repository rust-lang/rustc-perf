#[doc = "Register `DIEPTXF1` reader"]
pub struct R(crate::R<DIEPTXF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF1` writer"]
pub struct W(crate::W<DIEPTXF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF1_SPEC>;
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
impl From<crate::W<DIEPTXF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFO2 transmit RAM start address"]
pub struct INEPTXSA_R(crate::FieldReader<u16, u16>);
impl INEPTXSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTXSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPTXSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFO2 transmit RAM start address"]
pub struct INEPTXSA_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub struct INEPTXFD_R(crate::FieldReader<u16, u16>);
impl INEPTXFD_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTXFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPTXFD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub struct INEPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W {
        INEPTXSA_W { w: self }
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W {
        INEPTXFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf1](index.html) module"]
pub struct DIEPTXF1_SPEC;
impl crate::RegisterSpec for DIEPTXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf1::R](R) reader structure"]
impl crate::Readable for DIEPTXF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](W) writer structure"]
impl crate::Writable for DIEPTXF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTXF1 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
