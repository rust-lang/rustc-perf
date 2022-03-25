#[doc = "Register `GPSR` reader"]
pub struct R(crate::R<GPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPSR` writer"]
pub struct W(crate::W<GPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSR_SPEC>;
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
impl From<crate::W<GPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB` reader - RCB"]
pub struct RCB_R(crate::FieldReader<bool, bool>);
impl RCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB` writer - RCB"]
pub struct RCB_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PRDFF` reader - PRDFF"]
pub struct PRDFF_R(crate::FieldReader<bool, bool>);
impl PRDFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRDFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRDFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDFF` writer - PRDFF"]
pub struct PRDFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PRDFE` reader - PRDFE"]
pub struct PRDFE_R(crate::FieldReader<bool, bool>);
impl PRDFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRDFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRDFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDFE` writer - PRDFE"]
pub struct PRDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PWRFF` reader - PWRFF"]
pub struct PWRFF_R(crate::FieldReader<bool, bool>);
impl PWRFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRFF` writer - PWRFF"]
pub struct PWRFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PWRFE` reader - PWRFE"]
pub struct PWRFE_R(crate::FieldReader<bool, bool>);
impl PWRFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRFE` writer - PWRFE"]
pub struct PWRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRFE_W<'a> {
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
#[doc = "Field `CMDFF` reader - Acknowledge Request Enable"]
pub struct CMDFF_R(crate::FieldReader<bool, bool>);
impl CMDFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDFF` writer - Acknowledge Request Enable"]
pub struct CMDFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDFF_W<'a> {
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
#[doc = "Field `CMDFE` reader - Tearing Effect Acknowledge Request Enable"]
pub struct CMDFE_R(crate::FieldReader<bool, bool>);
impl CMDFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDFE` writer - Tearing Effect Acknowledge Request Enable"]
pub struct CMDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDFE_W<'a> {
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
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&mut self) -> RCB_W {
        RCB_W { w: self }
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&mut self) -> PRDFF_W {
        PRDFF_W { w: self }
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&mut self) -> PRDFE_W {
        PRDFE_W { w: self }
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&mut self) -> PWRFF_W {
        PWRFF_W { w: self }
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&mut self) -> PWRFE_W {
        PWRFE_W { w: self }
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdff(&mut self) -> CMDFF_W {
        CMDFF_W { w: self }
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdfe(&mut self) -> CMDFE_W {
        CMDFE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Generic Packet Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpsr](index.html) module"]
pub struct GPSR_SPEC;
impl crate::RegisterSpec for GPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpsr::R](R) reader structure"]
impl crate::Readable for GPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpsr::W](W) writer structure"]
impl crate::Writable for GPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPSR to value 0"]
impl crate::Resettable for GPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
