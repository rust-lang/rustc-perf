#[doc = "Register `MACPMTCSR` reader"]
pub struct R(crate::R<MACPMTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPMTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPMTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPMTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPMTCSR` writer"]
pub struct W(crate::W<MACPMTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPMTCSR_SPEC>;
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
impl From<crate::W<MACPMTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPMTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - PD"]
pub struct PD_R(crate::FieldReader<bool, bool>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - PD"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
#[doc = "Field `MPE` reader - MPE"]
pub struct MPE_R(crate::FieldReader<bool, bool>);
impl MPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPE` writer - MPE"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
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
#[doc = "Field `WFE` reader - WFE"]
pub struct WFE_R(crate::FieldReader<bool, bool>);
impl WFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFE` writer - WFE"]
pub struct WFE_W<'a> {
    w: &'a mut W,
}
impl<'a> WFE_W<'a> {
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
#[doc = "Field `MPR` reader - MPR"]
pub struct MPR_R(crate::FieldReader<bool, bool>);
impl MPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPR` writer - MPR"]
pub struct MPR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPR_W<'a> {
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
#[doc = "Field `WFR` reader - WFR"]
pub struct WFR_R(crate::FieldReader<bool, bool>);
impl WFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFR` writer - WFR"]
pub struct WFR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFR_W<'a> {
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
#[doc = "Field `GU` reader - GU"]
pub struct GU_R(crate::FieldReader<bool, bool>);
impl GU_R {
    pub(crate) fn new(bits: bool) -> Self {
        GU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GU` writer - GU"]
pub struct GU_W<'a> {
    w: &'a mut W,
}
impl<'a> GU_W<'a> {
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
#[doc = "Field `WFFRPR` reader - WFFRPR"]
pub struct WFFRPR_R(crate::FieldReader<bool, bool>);
impl WFFRPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFFRPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFFRPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFFRPR` writer - WFFRPR"]
pub struct WFFRPR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFFRPR_W<'a> {
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
    #[doc = "Bit 0 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPE"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WFE"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WFR"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GU"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PD"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 1 - MPE"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Bit 2 - WFE"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W {
        WFE_W { w: self }
    }
    #[doc = "Bit 5 - MPR"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W {
        MPR_W { w: self }
    }
    #[doc = "Bit 6 - WFR"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W {
        WFR_W { w: self }
    }
    #[doc = "Bit 9 - GU"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W {
        GU_W { w: self }
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W {
        WFFRPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PMT control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpmtcsr](index.html) module"]
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macpmtcsr::R](R) reader structure"]
impl crate::Readable for MACPMTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macpmtcsr::W](W) writer structure"]
impl crate::Writable for MACPMTCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MACPMTCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
