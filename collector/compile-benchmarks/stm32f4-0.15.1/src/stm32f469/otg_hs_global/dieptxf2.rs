#[doc = "Register `DIEPTXF2` reader"]
pub struct R(crate::R<DIEPTXF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF2` writer"]
pub struct W(crate::W<DIEPTXF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF2_SPEC>;
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
impl From<crate::W<DIEPTXF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF2_SPEC, u16, u16, 16, O>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
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
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<0> {
        INEPTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<16> {
        INEPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf2](index.html) module"]
pub struct DIEPTXF2_SPEC;
impl crate::RegisterSpec for DIEPTXF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf2::R](R) reader structure"]
impl crate::Readable for DIEPTXF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf2::W](W) writer structure"]
impl crate::Writable for DIEPTXF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTXF2 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
