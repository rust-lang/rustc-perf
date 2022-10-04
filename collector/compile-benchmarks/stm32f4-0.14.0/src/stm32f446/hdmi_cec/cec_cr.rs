#[doc = "Register `CEC_CR` reader"]
pub struct R(crate::R<CEC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_CR` writer"]
pub struct W(crate::W<CEC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CR_SPEC>;
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
impl From<crate::W<CEC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEOM` reader - Tx End Of Message"]
pub struct TXEOM_R(crate::FieldReader<bool, bool>);
impl TXEOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSOM` reader - Tx Start Of Message"]
pub struct TXSOM_R(crate::FieldReader<bool, bool>);
impl TXSOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECEN` reader - CEC Enable"]
pub struct CECEN_R(crate::FieldReader<bool, bool>);
impl CECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECEN` writer - CEC Enable"]
pub struct CECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Tx End Of Message"]
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx Start Of Message"]
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CEC Enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC Enable"]
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W {
        CECEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cr](index.html) module"]
pub struct CEC_CR_SPEC;
impl crate::RegisterSpec for CEC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_cr::R](R) reader structure"]
impl crate::Readable for CEC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_cr::W](W) writer structure"]
impl crate::Writable for CEC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_CR to value 0"]
impl crate::Resettable for CEC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
