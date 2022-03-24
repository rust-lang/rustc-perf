#[doc = "Register `PTPTSLUR` reader"]
pub struct R(crate::R<PTPTSLUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSLUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSLUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSLUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTSLUR` writer"]
pub struct W(crate::W<PTPTSLUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSLUR_SPEC>;
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
impl From<crate::W<PTPTSLUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSLUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSUSS` reader - TSUSS"]
pub struct TSUSS_R(crate::FieldReader<u32, u32>);
impl TSUSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSUSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUSS` writer - TSUSS"]
pub struct TSUSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `TSUPNS` reader - TSUSS"]
pub struct TSUPNS_R(crate::FieldReader<bool, bool>);
impl TSUPNS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSUPNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUPNS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUPNS` writer - TSUSS"]
pub struct TSUPNS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPNS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&self) -> TSUSS_R {
        TSUSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - TSUSS"]
    #[inline(always)]
    pub fn tsupns(&self) -> TSUPNS_R {
        TSUPNS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&mut self) -> TSUSS_W {
        TSUSS_W { w: self }
    }
    #[doc = "Bit 31 - TSUSS"]
    #[inline(always)]
    pub fn tsupns(&mut self) -> TSUPNS_W {
        TSUPNS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp low update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptslur](index.html) module"]
pub struct PTPTSLUR_SPEC;
impl crate::RegisterSpec for PTPTSLUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptslur::R](R) reader structure"]
impl crate::Readable for PTPTSLUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptslur::W](W) writer structure"]
impl crate::Writable for PTPTSLUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTSLUR to value 0"]
impl crate::Resettable for PTPTSLUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
