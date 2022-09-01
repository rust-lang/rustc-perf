#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBF` reader - DBF"]
pub struct DBF_R(crate::FieldReader<bool, bool>);
impl DBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBF` writer - DBF"]
pub struct DBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBF_W<'a> {
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
#[doc = "Field `RESET` reader - RESET"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - RESET"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Field `TTCM` reader - TTCM"]
pub struct TTCM_R(crate::FieldReader<bool, bool>);
impl TTCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTCM` writer - TTCM"]
pub struct TTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TTCM_W<'a> {
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
#[doc = "Field `ABOM` reader - ABOM"]
pub struct ABOM_R(crate::FieldReader<bool, bool>);
impl ABOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABOM` writer - ABOM"]
pub struct ABOM_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOM_W<'a> {
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
#[doc = "Field `AWUM` reader - AWUM"]
pub struct AWUM_R(crate::FieldReader<bool, bool>);
impl AWUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWUM` writer - AWUM"]
pub struct AWUM_W<'a> {
    w: &'a mut W,
}
impl<'a> AWUM_W<'a> {
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
#[doc = "Field `NART` reader - NART"]
pub struct NART_R(crate::FieldReader<bool, bool>);
impl NART_R {
    pub(crate) fn new(bits: bool) -> Self {
        NART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NART` writer - NART"]
pub struct NART_W<'a> {
    w: &'a mut W,
}
impl<'a> NART_W<'a> {
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
#[doc = "Field `RFLM` reader - RFLM"]
pub struct RFLM_R(crate::FieldReader<bool, bool>);
impl RFLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFLM` writer - RFLM"]
pub struct RFLM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFLM_W<'a> {
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
#[doc = "Field `TXFP` reader - TXFP"]
pub struct TXFP_R(crate::FieldReader<bool, bool>);
impl TXFP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFP` writer - TXFP"]
pub struct TXFP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFP_W<'a> {
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
#[doc = "Field `SLEEP` reader - SLEEP"]
pub struct SLEEP_R(crate::FieldReader<bool, bool>);
impl SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - SLEEP"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
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
#[doc = "Field `INRQ` reader - INRQ"]
pub struct INRQ_R(crate::FieldReader<bool, bool>);
impl INRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        INRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INRQ` writer - INRQ"]
pub struct INRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INRQ_W<'a> {
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
    #[doc = "Bit 16 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AWUM"]
    #[inline(always)]
    pub fn awum(&self) -> AWUM_R {
        AWUM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - INRQ"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - DBF"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W {
        DBF_W { w: self }
    }
    #[doc = "Bit 15 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&mut self) -> TTCM_W {
        TTCM_W { w: self }
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&mut self) -> ABOM_W {
        ABOM_W { w: self }
    }
    #[doc = "Bit 5 - AWUM"]
    #[inline(always)]
    pub fn awum(&mut self) -> AWUM_W {
        AWUM_W { w: self }
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&mut self) -> NART_W {
        NART_W { w: self }
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&mut self) -> RFLM_W {
        RFLM_W { w: self }
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&mut self) -> TXFP_W {
        TXFP_W { w: self }
    }
    #[doc = "Bit 1 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 0 - INRQ"]
    #[inline(always)]
    pub fn inrq(&mut self) -> INRQ_W {
        INRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "master control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0x0001_0002"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0002
    }
}
