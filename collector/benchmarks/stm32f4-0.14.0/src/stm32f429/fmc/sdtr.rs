#[doc = "Register `SDTR%s` reader"]
pub struct R(crate::R<SDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDTR%s` writer"]
pub struct W(crate::W<SDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDTR_SPEC>;
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
impl From<crate::W<SDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRD` reader - Load Mode Register to Active"]
pub struct TMRD_R(crate::FieldReader<u8, u8>);
impl TMRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRD` writer - Load Mode Register to Active"]
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TXSR` reader - Exit self-refresh delay"]
pub struct TXSR_R(crate::FieldReader<u8, u8>);
impl TXSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSR` writer - Exit self-refresh delay"]
pub struct TXSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TRAS` reader - Self refresh time"]
pub struct TRAS_R(crate::FieldReader<u8, u8>);
impl TRAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRAS` writer - Self refresh time"]
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TRC` reader - Row cycle delay"]
pub struct TRC_R(crate::FieldReader<u8, u8>);
impl TRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRC` writer - Row cycle delay"]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `TWR` reader - Recovery delay"]
pub struct TWR_R(crate::FieldReader<u8, u8>);
impl TWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWR` writer - Recovery delay"]
pub struct TWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TRP` reader - Row precharge delay"]
pub struct TRP_R(crate::FieldReader<u8, u8>);
impl TRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRP` writer - Row precharge delay"]
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `TRCD` reader - Row to column delay"]
pub struct TRCD_R(crate::FieldReader<u8, u8>);
impl TRCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCD` writer - Row to column delay"]
pub struct TRCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Mode Register to Active"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit self-refresh delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row cycle delay"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Recovery delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register to Active"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
    #[doc = "Bits 4:7 - Exit self-refresh delay"]
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W {
        TXSR_W { w: self }
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
    #[doc = "Bits 12:15 - Row cycle delay"]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
    #[doc = "Bits 16:19 - Recovery delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W {
        TWR_W { w: self }
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W {
        TRCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdtr](index.html) module"]
pub struct SDTR_SPEC;
impl crate::RegisterSpec for SDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdtr::R](R) reader structure"]
impl crate::Readable for SDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdtr::W](W) writer structure"]
impl crate::Writable for SDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDTR%s to value 0x0fff_ffff"]
impl crate::Resettable for SDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
