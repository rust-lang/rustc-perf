#[doc = "Register `DMABMR` reader"]
pub struct R(crate::R<DMABMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABMR` writer"]
pub struct W(crate::W<DMABMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABMR_SPEC>;
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
impl From<crate::W<DMABMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - SR"]
pub struct SR_R(crate::FieldReader<bool, bool>);
impl SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - SR"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
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
#[doc = "Field `DA` reader - DA"]
pub struct DA_R(crate::FieldReader<bool, bool>);
impl DA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - DA"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
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
#[doc = "Field `DSL` reader - DSL"]
pub struct DSL_R(crate::FieldReader<u8, u8>);
impl DSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSL` writer - DSL"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `EDFE` reader - EDFE"]
pub struct EDFE_R(crate::FieldReader<bool, bool>);
impl EDFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDFE` writer - EDFE"]
pub struct EDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PBL` reader - PBL"]
pub struct PBL_R(crate::FieldReader<u8, u8>);
impl PBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBL` writer - PBL"]
pub struct PBL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `RTPR` reader - RTPR"]
pub struct RTPR_R(crate::FieldReader<u8, u8>);
impl RTPR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTPR` writer - RTPR"]
pub struct RTPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `FB` reader - FB"]
pub struct FB_R(crate::FieldReader<bool, bool>);
impl FB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB` writer - FB"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
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
#[doc = "Field `RDP` reader - RDP"]
pub struct RDP_R(crate::FieldReader<u8, u8>);
impl RDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDP` writer - RDP"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | ((value as u32 & 0x3f) << 17);
        self.w
    }
}
#[doc = "Field `USP` reader - USP"]
pub struct USP_R(crate::FieldReader<bool, bool>);
impl USP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USP` writer - USP"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `FPM` reader - FPM"]
pub struct FPM_R(crate::FieldReader<bool, bool>);
impl FPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPM` writer - FPM"]
pub struct FPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPM_W<'a> {
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
#[doc = "Field `AAB` reader - AAB"]
pub struct AAB_R(crate::FieldReader<bool, bool>);
impl AAB_R {
    pub(crate) fn new(bits: bool) -> Self {
        AAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAB` writer - AAB"]
pub struct AAB_W<'a> {
    w: &'a mut W,
}
impl<'a> AAB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `MB` reader - MB"]
pub struct MB_R(crate::FieldReader<bool, bool>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - MB"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SR"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DA"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - DSL"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - EDFE"]
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - PBL"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - RTPR"]
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - FB"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:22 - RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - USP"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FPM"]
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AAB"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MB"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SR"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 1 - DA"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Bits 2:6 - DSL"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    #[doc = "Bit 7 - EDFE"]
    #[inline(always)]
    pub fn edfe(&mut self) -> EDFE_W {
        EDFE_W { w: self }
    }
    #[doc = "Bits 8:13 - PBL"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W {
        PBL_W { w: self }
    }
    #[doc = "Bits 14:15 - RTPR"]
    #[inline(always)]
    pub fn rtpr(&mut self) -> RTPR_W {
        RTPR_W { w: self }
    }
    #[doc = "Bit 16 - FB"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Bits 17:22 - RDP"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 23 - USP"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    #[doc = "Bit 24 - FPM"]
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W {
        FPM_W { w: self }
    }
    #[doc = "Bit 25 - AAB"]
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W {
        AAB_W { w: self }
    }
    #[doc = "Bit 26 - MB"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabmr](index.html) module"]
pub struct DMABMR_SPEC;
impl crate::RegisterSpec for DMABMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabmr::R](R) reader structure"]
impl crate::Readable for DMABMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabmr::W](W) writer structure"]
impl crate::Writable for DMABMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABMR to value 0x2101"]
impl crate::Resettable for DMABMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101
    }
}
