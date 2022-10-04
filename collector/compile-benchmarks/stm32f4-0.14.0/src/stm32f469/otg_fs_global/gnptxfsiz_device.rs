#[doc = "Register `GNPTXFSIZ_Device` reader"]
pub struct R(crate::R<GNPTXFSIZ_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXFSIZ_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXFSIZ_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXFSIZ_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GNPTXFSIZ_Device` writer"]
pub struct W(crate::W<GNPTXFSIZ_DEVICE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GNPTXFSIZ_DEVICE_SPEC>;
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
impl From<crate::W<GNPTXFSIZ_DEVICE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GNPTXFSIZ_DEVICE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub struct TX0FSA_R(crate::FieldReader<u16, u16>);
impl TX0FSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX0FSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX0FSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub struct TX0FSA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX0FSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub struct TX0FD_R(crate::FieldReader<u16, u16>);
impl TX0FD_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX0FD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX0FD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub struct TX0FD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX0FD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> TX0FSA_W {
        TX0FSA_W { w: self }
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&mut self) -> TX0FD_W {
        TX0FD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxfsiz_device](index.html) module"]
pub struct GNPTXFSIZ_DEVICE_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxfsiz_device::R](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_DEVICE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz_device::W](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_DEVICE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GNPTXFSIZ_Device to value 0x0200"]
impl crate::Resettable for GNPTXFSIZ_DEVICE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
