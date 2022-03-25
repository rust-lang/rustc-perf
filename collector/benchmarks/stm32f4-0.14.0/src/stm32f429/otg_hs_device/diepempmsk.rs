#[doc = "Register `DIEPEMPMSK` reader"]
pub struct R(crate::R<DIEPEMPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPEMPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPEMPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPEMPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPEMPMSK` writer"]
pub struct W(crate::W<DIEPEMPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPEMPMSK_SPEC>;
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
impl From<crate::W<DIEPEMPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPEMPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTXFEM` reader - IN EP Tx FIFO empty interrupt mask bits"]
pub struct INEPTXFEM_R(crate::FieldReader<u16, u16>);
impl INEPTXFEM_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTXFEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPTXFEM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPTXFEM` writer - IN EP Tx FIFO empty interrupt mask bits"]
pub struct INEPTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXFEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W {
        INEPTXFEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepempmsk](index.html) module"]
pub struct DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for DIEPEMPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepempmsk::R](R) reader structure"]
impl crate::Readable for DIEPEMPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](W) writer structure"]
impl crate::Writable for DIEPEMPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DIEPEMPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
