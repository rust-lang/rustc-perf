#[doc = "Register `ACTRL` reader"]
pub struct R(crate::R<ACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTRL` writer"]
pub struct W(crate::W<ACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTRL_SPEC>;
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
impl From<crate::W<ACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMCYCINT` reader - DISMCYCINT"]
pub struct DISMCYCINT_R(crate::FieldReader<bool, bool>);
impl DISMCYCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISMCYCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISMCYCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISMCYCINT` writer - DISMCYCINT"]
pub struct DISMCYCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMCYCINT_W<'a> {
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
#[doc = "Field `DISDEFWBUF` reader - DISDEFWBUF"]
pub struct DISDEFWBUF_R(crate::FieldReader<bool, bool>);
impl DISDEFWBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISDEFWBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISDEFWBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISDEFWBUF` writer - DISDEFWBUF"]
pub struct DISDEFWBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDEFWBUF_W<'a> {
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
#[doc = "Field `DISFOLD` reader - DISFOLD"]
pub struct DISFOLD_R(crate::FieldReader<bool, bool>);
impl DISFOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISFOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISFOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISFOLD` writer - DISFOLD"]
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
#[doc = "Field `DISFPCA` reader - DISFPCA"]
pub struct DISFPCA_R(crate::FieldReader<bool, bool>);
impl DISFPCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISFPCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISFPCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISFPCA` writer - DISFPCA"]
pub struct DISFPCA_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFPCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DISOOFP` reader - DISOOFP"]
pub struct DISOOFP_R(crate::FieldReader<bool, bool>);
impl DISOOFP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISOOFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISOOFP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISOOFP` writer - DISOOFP"]
pub struct DISOOFP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISOOFP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DISMCYCINT"]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DISDEFWBUF"]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DISFPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DISOOFP"]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DISMCYCINT"]
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W {
        DISMCYCINT_W { w: self }
    }
    #[doc = "Bit 1 - DISDEFWBUF"]
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W {
        DISDEFWBUF_W { w: self }
    }
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    #[doc = "Bit 8 - DISFPCA"]
    #[inline(always)]
    pub fn disfpca(&mut self) -> DISFPCA_W {
        DISFPCA_W { w: self }
    }
    #[doc = "Bit 9 - DISOOFP"]
    #[inline(always)]
    pub fn disoofp(&mut self) -> DISOOFP_W {
        DISOOFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actrl](index.html) module"]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actrl::R](R) reader structure"]
impl crate::Readable for ACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actrl::W](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
