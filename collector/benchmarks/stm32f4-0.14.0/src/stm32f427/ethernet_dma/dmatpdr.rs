#[doc = "Register `DMATPDR` reader"]
pub struct R(crate::R<DMATPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATPDR` writer"]
pub struct W(crate::W<DMATPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATPDR_SPEC>;
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
impl From<crate::W<DMATPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit poll demand\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TPD_A {
    #[doc = "0: Poll the transmit descriptor list"]
    POLL = 0,
}
impl From<TPD_A> for u32 {
    #[inline(always)]
    fn from(variant: TPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub struct TPD_R(crate::FieldReader<u32, TPD_A>);
impl TPD_R {
    pub(crate) fn new(bits: u32) -> Self {
        TPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPD_A> {
        match self.bits {
            0 => Some(TPD_A::POLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POLL`"]
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        **self == TPD_A::POLL
    }
}
impl core::ops::Deref for TPD_R {
    type Target = crate::FieldReader<u32, TPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub struct TPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Poll the transmit descriptor list"]
    #[inline(always)]
    pub fn poll(self) -> &'a mut W {
        self.variant(TPD_A::POLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W {
        TPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatpdr](index.html) module"]
pub struct DMATPDR_SPEC;
impl crate::RegisterSpec for DMATPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatpdr::R](R) reader structure"]
impl crate::Readable for DMATPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatpdr::W](W) writer structure"]
impl crate::Writable for DMATPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATPDR to value 0"]
impl crate::Resettable for DMATPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
