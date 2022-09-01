#[doc = "Register `PCTLR` reader"]
pub struct R(crate::R<PCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCTLR` writer"]
pub struct W(crate::W<PCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCTLR_SPEC>;
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
impl From<crate::W<PCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKE` reader - Clock Enable"]
pub struct CKE_R(crate::FieldReader<bool, bool>);
impl CKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKE` writer - Clock Enable"]
pub struct CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DEN` reader - Digital Enable"]
pub struct DEN_R(crate::FieldReader<bool, bool>);
impl DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEN` writer - Digital Enable"]
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Digital Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W {
        CKE_W { w: self }
    }
    #[doc = "Bit 1 - Digital Enable"]
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pctlr](index.html) module"]
pub struct PCTLR_SPEC;
impl crate::RegisterSpec for PCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pctlr::R](R) reader structure"]
impl crate::Readable for PCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pctlr::W](W) writer structure"]
impl crate::Writable for PCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCTLR to value 0"]
impl crate::Resettable for PCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
