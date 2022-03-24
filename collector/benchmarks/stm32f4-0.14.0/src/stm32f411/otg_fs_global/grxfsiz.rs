#[doc = "Register `GRXFSIZ` reader"]
pub struct R(crate::R<GRXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRXFSIZ` writer"]
pub struct W(crate::W<GRXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRXFSIZ_SPEC>;
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
impl From<crate::W<GRXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub struct RXFD_R(crate::FieldReader<u16, u16>);
impl RXFD_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub struct RXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&mut self) -> RXFD_W {
        RXFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxfsiz](index.html) module"]
pub struct GRXFSIZ_SPEC;
impl crate::RegisterSpec for GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxfsiz::R](R) reader structure"]
impl crate::Readable for GRXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](W) writer structure"]
impl crate::Writable for GRXFSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0200"]
impl crate::Resettable for GRXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
