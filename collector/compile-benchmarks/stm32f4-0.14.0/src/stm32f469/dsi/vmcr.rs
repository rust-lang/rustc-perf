#[doc = "Register `VMCR` reader"]
pub struct R(crate::R<VMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMCR` writer"]
pub struct W(crate::W<VMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMCR_SPEC>;
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
impl From<crate::W<VMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGO` reader - Pattern Generator Orientation"]
pub struct PGO_R(crate::FieldReader<bool, bool>);
impl PGO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGO` writer - Pattern Generator Orientation"]
pub struct PGO_W<'a> {
    w: &'a mut W,
}
impl<'a> PGO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PGM` reader - Pattern Generator Mode"]
pub struct PGM_R(crate::FieldReader<bool, bool>);
impl PGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM` writer - Pattern Generator Mode"]
pub struct PGM_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PGE` reader - Pattern Generator Enable"]
pub struct PGE_R(crate::FieldReader<bool, bool>);
impl PGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE` writer - Pattern Generator Enable"]
pub struct PGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
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
#[doc = "Field `FBTAAE` reader - Frame Bus-Turn-Around Acknowledge Enable"]
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
#[doc = "Field `FBTAAE` writer - Frame Bus-Turn-Around Acknowledge Enable"]
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
#[doc = "Field `LPHFPE` reader - Low-Power Horizontal Front-Porch Enable"]
pub struct LPHFPE_R(crate::FieldReader<bool, bool>);
impl LPHFPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPHFPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPHFPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPHFPE` writer - Low-Power Horizontal Front-Porch Enable"]
pub struct LPHFPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPHFPE_W<'a> {
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
#[doc = "Field `LPHBPE` reader - Low-Power Horizontal Back-Porch Enable"]
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
#[doc = "Field `LPHBPE` writer - Low-Power Horizontal Back-Porch Enable"]
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
#[doc = "Field `LPVAE` reader - Low-Power Vertical Active Enable"]
pub struct LPVAE_R(crate::FieldReader<bool, bool>);
impl LPVAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVAE` writer - Low-Power Vertical Active Enable"]
pub struct LPVAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVAE_W<'a> {
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
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-porch Enable"]
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
#[doc = "Field `LPVFPE` writer - Low-power Vertical Front-porch Enable"]
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
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync Active Enable"]
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
#[doc = "Field `LPVSAE` writer - Low-Power Vertical Sync Active Enable"]
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
    #[doc = "Bit 24 - Pattern Generator Orientation"]
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pattern Generator Mode"]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pattern Generator Enable"]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Frame Bus-Turn-Around Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Low-Power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync Active Enable"]
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
    #[doc = "Bit 24 - Pattern Generator Orientation"]
    #[inline(always)]
    pub fn pgo(&mut self) -> PGO_W {
        PGO_W { w: self }
    }
    #[doc = "Bit 20 - Pattern Generator Mode"]
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W {
        PGM_W { w: self }
    }
    #[doc = "Bit 16 - Pattern Generator Enable"]
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W {
        PGE_W { w: self }
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W {
        LPCE_W { w: self }
    }
    #[doc = "Bit 14 - Frame Bus-Turn-Around Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W {
        FBTAAE_W { w: self }
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfpe(&mut self) -> LPHFPE_W {
        LPHFPE_W { w: self }
    }
    #[doc = "Bit 12 - Low-Power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W {
        LPHBPE_W { w: self }
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lpvae(&mut self) -> LPVAE_W {
        LPVAE_W { w: self }
    }
    #[doc = "Bit 10 - Low-power Vertical Front-porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W {
        LPVFPE_W { w: self }
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W {
        LPVBPE_W { w: self }
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync Active Enable"]
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
#[doc = "DSI Host Video mode Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmcr](index.html) module"]
pub struct VMCR_SPEC;
impl crate::RegisterSpec for VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmcr::R](R) reader structure"]
impl crate::Readable for VMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmcr::W](W) writer structure"]
impl crate::Writable for VMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
