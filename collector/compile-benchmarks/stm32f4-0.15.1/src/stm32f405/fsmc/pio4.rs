#[doc = "Register `PIO4` reader"]
pub struct R(crate::R<PIO4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO4` writer"]
pub struct W(crate::W<PIO4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO4_SPEC>;
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
impl From<crate::W<PIO4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOHIZx` reader - IOHIZx"]
pub type IOHIZX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOHIZx` writer - IOHIZx"]
pub type IOHIZX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO4_SPEC, u8, u8, 8, O>;
#[doc = "Field `IOHOLDx` reader - IOHOLDx"]
pub type IOHOLDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOHOLDx` writer - IOHOLDx"]
pub type IOHOLDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO4_SPEC, u8, u8, 8, O>;
#[doc = "Field `IOWAITx` reader - IOWAITx"]
pub type IOWAITX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOWAITx` writer - IOWAITx"]
pub type IOWAITX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO4_SPEC, u8, u8, 8, O>;
#[doc = "Field `IOSETx` reader - IOSETx"]
pub type IOSETX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOSETx` writer - IOSETx"]
pub type IOSETX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&self) -> IOHIZX_R {
        IOHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&self) -> IOHOLDX_R {
        IOHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&self) -> IOWAITX_R {
        IOWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&self) -> IOSETX_R {
        IOSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&mut self) -> IOHIZX_W<24> {
        IOHIZX_W::new(self)
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&mut self) -> IOHOLDX_W<16> {
        IOHOLDX_W::new(self)
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&mut self) -> IOWAITX_W<8> {
        IOWAITX_W::new(self)
    }
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&mut self) -> IOSETX_W<0> {
        IOSETX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio4](index.html) module"]
pub struct PIO4_SPEC;
impl crate::RegisterSpec for PIO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio4::R](R) reader structure"]
impl crate::Readable for PIO4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio4::W](W) writer structure"]
impl crate::Writable for PIO4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PIO4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
