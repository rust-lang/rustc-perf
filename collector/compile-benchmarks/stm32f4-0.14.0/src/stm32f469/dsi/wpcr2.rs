#[doc = "Register `WPCR2` reader"]
pub struct R(crate::R<WPCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR2` writer"]
pub struct W(crate::W<WPCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR2_SPEC>;
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
impl From<crate::W<WPCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPRXFT` reader - Low-Power RX low-pass Filtering Tuning"]
pub struct LPRXFT_R(crate::FieldReader<u8, u8>);
impl LPRXFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPRXFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPRXFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRXFT` writer - Low-Power RX low-pass Filtering Tuning"]
pub struct LPRXFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRXFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `FLPRXLPM` reader - Forces LP Receiver in Low-Power Mode"]
pub struct FLPRXLPM_R(crate::FieldReader<bool, bool>);
impl FLPRXLPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLPRXLPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLPRXLPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLPRXLPM` writer - Forces LP Receiver in Low-Power Mode"]
pub struct FLPRXLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPRXLPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `HSTXSRCDL` reader - High-Speed Transmission Slew Rate Control on Data Lanes"]
pub struct HSTXSRCDL_R(crate::FieldReader<u8, u8>);
impl HSTXSRCDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXSRCDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRCDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRCDL` writer - High-Speed Transmission Slew Rate Control on Data Lanes"]
pub struct HSTXSRCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `HSTXSRCCL` reader - High-Speed Transmission Slew Rate Control on Clock Lane"]
pub struct HSTXSRCCL_R(crate::FieldReader<u8, u8>);
impl HSTXSRCCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXSRCCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRCCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRCCL` writer - High-Speed Transmission Slew Rate Control on Clock Lane"]
pub struct HSTXSRCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SDCC` reader - SDD Control"]
pub struct SDCC_R(crate::FieldReader<bool, bool>);
impl SDCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCC` writer - SDD Control"]
pub struct SDCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCC_W<'a> {
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
#[doc = "Field `LPSRDL` reader - Low-Power transmission Slew Rate Compensation on Data Lanes"]
pub struct LPSRDL_R(crate::FieldReader<u8, u8>);
impl LPSRDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPSRDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSRDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSRDL` writer - Low-Power transmission Slew Rate Compensation on Data Lanes"]
pub struct LPSRDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `LPSRCL` reader - Low-Power transmission Slew Rate Compensation on Clock Lane"]
pub struct LPSRCL_R(crate::FieldReader<u8, u8>);
impl LPSRCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPSRCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSRCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSRCL` writer - Low-Power transmission Slew Rate Compensation on Clock Lane"]
pub struct LPSRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `HSTXDLL` reader - High-Speed Transmission Delay on Data Lanes"]
pub struct HSTXDLL_R(crate::FieldReader<u8, u8>);
impl HSTXDLL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXDLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXDLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXDLL` writer - High-Speed Transmission Delay on Data Lanes"]
pub struct HSTXDLL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `HSTXDCL` reader - High-Speed Transmission Delay on Clock Lane"]
pub struct HSTXDCL_R(crate::FieldReader<u8, u8>);
impl HSTXDCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTXDCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXDCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXDCL` writer - High-Speed Transmission Delay on Clock Lane"]
pub struct HSTXDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline(always)]
    pub fn sdcc(&self) -> SDCC_R {
        SDCC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline(always)]
    pub fn lpsrdl(&self) -> LPSRDL_R {
        LPSRDL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline(always)]
    pub fn lpsrcl(&self) -> LPSRCL_R {
        LPSRCL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline(always)]
    pub fn hstxdll(&self) -> HSTXDLL_R {
        HSTXDLL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline(always)]
    pub fn lprxft(&mut self) -> LPRXFT_W {
        LPRXFT_W { w: self }
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W {
        FLPRXLPM_W { w: self }
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W {
        HSTXSRCDL_W { w: self }
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W {
        HSTXSRCCL_W { w: self }
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline(always)]
    pub fn sdcc(&mut self) -> SDCC_W {
        SDCC_W { w: self }
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline(always)]
    pub fn lpsrdl(&mut self) -> LPSRDL_W {
        LPSRDL_W { w: self }
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline(always)]
    pub fn lpsrcl(&mut self) -> LPSRCL_W {
        LPSRCL_W { w: self }
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline(always)]
    pub fn hstxdll(&mut self) -> HSTXDLL_W {
        HSTXDLL_W { w: self }
    }
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W {
        HSTXDCL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr2](index.html) module"]
pub struct WPCR2_SPEC;
impl crate::RegisterSpec for WPCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr2::R](R) reader structure"]
impl crate::Readable for WPCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr2::W](W) writer structure"]
impl crate::Writable for WPCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR2 to value 0"]
impl crate::Resettable for WPCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
