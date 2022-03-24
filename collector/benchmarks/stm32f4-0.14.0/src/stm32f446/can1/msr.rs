#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSR` writer"]
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX` reader - RX"]
pub struct RX_R(crate::FieldReader<bool, bool>);
impl RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMP` reader - SAMP"]
pub struct SAMP_R(crate::FieldReader<bool, bool>);
impl SAMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXM` reader - RXM"]
pub struct RXM_R(crate::FieldReader<bool, bool>);
impl RXM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXM` reader - TXM"]
pub struct TXM_R(crate::FieldReader<bool, bool>);
impl TXM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAKI` reader - SLAKI"]
pub struct SLAKI_R(crate::FieldReader<bool, bool>);
impl SLAKI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAKI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAKI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAKI` writer - SLAKI"]
pub struct SLAKI_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAKI_W<'a> {
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
#[doc = "Field `WKUI` reader - WKUI"]
pub struct WKUI_R(crate::FieldReader<bool, bool>);
impl WKUI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUI` writer - WKUI"]
pub struct WKUI_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUI_W<'a> {
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
#[doc = "Field `ERRI` reader - ERRI"]
pub struct ERRI_R(crate::FieldReader<bool, bool>);
impl ERRI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRI` writer - ERRI"]
pub struct ERRI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRI_W<'a> {
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
#[doc = "Field `SLAK` reader - SLAK"]
pub struct SLAK_R(crate::FieldReader<bool, bool>);
impl SLAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INAK` reader - INAK"]
pub struct INAK_R(crate::FieldReader<bool, bool>);
impl INAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        INAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 11 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SAMP"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXM"]
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TXM"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SLAK"]
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - INAK"]
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&mut self) -> SLAKI_W {
        SLAKI_W { w: self }
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&mut self) -> WKUI_W {
        WKUI_W { w: self }
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&mut self) -> ERRI_W {
        ERRI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "master status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msr::W](W) writer structure"]
impl crate::Writable for MSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSR to value 0x0c02"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c02
    }
}
