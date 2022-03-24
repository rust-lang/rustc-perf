#[doc = "Register `VMCCR` reader"]
pub struct R(crate::R<VMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMCCR` writer"]
pub struct W(crate::W<VMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMCCR_SPEC>;
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
impl From<crate::W<VMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCE` reader - Low-Power Command Enable"]
pub struct LPCE_R(crate::FieldReader<bool, bool>);
impl LPCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCE` writer - Low-Power Command Enable"]
pub struct LPCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `FBTAAE` reader - Frame BTA Acknowledge Enable"]
pub struct FBTAAE_R(crate::FieldReader<bool, bool>);
impl FBTAAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBTAAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBTAAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBTAAE` writer - Frame BTA Acknowledge Enable"]
pub struct FBTAAE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBTAAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `LPHFE` reader - Low-Power Horizontal Front-Porch Enable"]
pub struct LPHFE_R(crate::FieldReader<bool, bool>);
impl LPHFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPHFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPHFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPHFE` writer - Low-Power Horizontal Front-Porch Enable"]
pub struct LPHFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPHFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `LPHBPE` reader - Low-power Horizontal Back-Porch Enable"]
pub struct LPHBPE_R(crate::FieldReader<bool, bool>);
impl LPHBPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPHBPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPHBPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPHBPE` writer - Low-power Horizontal Back-Porch Enable"]
pub struct LPHBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPHBPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `LVAE` reader - Low-Power Vertical Active Enable"]
pub struct LVAE_R(crate::FieldReader<bool, bool>);
impl LVAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVAE` writer - Low-Power Vertical Active Enable"]
pub struct LVAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-Porch Enable"]
pub struct LPVFPE_R(crate::FieldReader<bool, bool>);
impl LPVFPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVFPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVFPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVFPE` writer - Low-power Vertical Front-Porch Enable"]
pub struct LPVFPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVFPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable"]
pub struct LPVBPE_R(crate::FieldReader<bool, bool>);
impl LPVBPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVBPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVBPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVBPE` writer - Low-power Vertical Back-Porch Enable"]
pub struct LPVBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVBPE_W<'a> {
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
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync time Enable"]
pub struct LPVSAE_R(crate::FieldReader<bool, bool>);
impl LPVSAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVSAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVSAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVSAE` writer - Low-Power Vertical Sync time Enable"]
pub struct LPVSAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVSAE_W<'a> {
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
#[doc = "Field `VMT` reader - Video mode Type"]
pub struct VMT_R(crate::FieldReader<u8, u8>);
impl VMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMT` writer - Video mode Type"]
pub struct VMT_W<'a> {
    w: &'a mut W,
}
impl<'a> VMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&self) -> LVAE_R {
        LVAE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W {
        LPCE_W { w: self }
    }
    #[doc = "Bit 14 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W {
        FBTAAE_W { w: self }
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&mut self) -> LPHFE_W {
        LPHFE_W { w: self }
    }
    #[doc = "Bit 12 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W {
        LPHBPE_W { w: self }
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&mut self) -> LVAE_W {
        LVAE_W { w: self }
    }
    #[doc = "Bit 10 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W {
        LPVFPE_W { w: self }
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W {
        LPVBPE_W { w: self }
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W {
        LPVSAE_W { w: self }
    }
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W {
        VMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video mode Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
pub struct VMCCR_SPEC;
impl crate::RegisterSpec for VMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmccr::R](R) reader structure"]
impl crate::Readable for VMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmccr::W](W) writer structure"]
impl crate::Writable for VMCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
