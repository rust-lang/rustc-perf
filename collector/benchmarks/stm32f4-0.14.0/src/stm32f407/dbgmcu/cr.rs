#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_SLEEP` reader - DBG_SLEEP"]
pub struct DBG_SLEEP_R(crate::FieldReader<bool, bool>);
impl DBG_SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_SLEEP` writer - DBG_SLEEP"]
pub struct DBG_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_SLEEP_W<'a> {
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
#[doc = "Field `DBG_STOP` reader - DBG_STOP"]
pub struct DBG_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_STOP` writer - DBG_STOP"]
pub struct DBG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STOP_W<'a> {
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
#[doc = "Field `DBG_STANDBY` reader - DBG_STANDBY"]
pub struct DBG_STANDBY_R(crate::FieldReader<bool, bool>);
impl DBG_STANDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_STANDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_STANDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_STANDBY` writer - DBG_STANDBY"]
pub struct DBG_STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STANDBY_W<'a> {
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
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub struct TRACE_IOEN_R(crate::FieldReader<bool, bool>);
impl TRACE_IOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRACE_IOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACE_IOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub struct TRACE_IOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_IOEN_W<'a> {
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
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub struct TRACE_MODE_R(crate::FieldReader<u8, u8>);
impl TRACE_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRACE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub struct TRACE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_I2C2_SMBUS_TIMEOUT"]
pub struct DBG_I2C2_SMBUS_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl DBG_I2C2_SMBUS_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C2_SMBUS_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C2_SMBUS_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - DBG_I2C2_SMBUS_TIMEOUT"]
pub struct DBG_I2C2_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C2_SMBUS_TIMEOUT_W<'a> {
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
#[doc = "Field `DBG_TIM8_STOP` reader - DBG_TIM8_STOP"]
pub struct DBG_TIM8_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM8_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM8_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM8_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM8_STOP` writer - DBG_TIM8_STOP"]
pub struct DBG_TIM8_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM8_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP"]
pub struct DBG_TIM5_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM5_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM5_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM5_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP"]
pub struct DBG_TIM5_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM5_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `DBG_TIM6_STOP` reader - DBG_TIM6_STOP"]
pub struct DBG_TIM6_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM6_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM6_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM6_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM6_STOP` writer - DBG_TIM6_STOP"]
pub struct DBG_TIM6_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM6_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DBG_TIM7_STOP` reader - DBG_TIM7_STOP"]
pub struct DBG_TIM7_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM7_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM7_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM7_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM7_STOP` writer - DBG_TIM7_STOP"]
pub struct DBG_TIM7_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM7_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DBG_TIM8_STOP"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W {
        DBG_SLEEP_W { w: self }
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W {
        DBG_STOP_W { w: self }
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W {
        DBG_STANDBY_W { w: self }
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W {
        TRACE_IOEN_W { w: self }
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W {
        TRACE_MODE_W { w: self }
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W {
        DBG_I2C2_SMBUS_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 17 - DBG_TIM8_STOP"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W {
        DBG_TIM8_STOP_W { w: self }
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W {
        DBG_TIM5_STOP_W { w: self }
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W {
        DBG_TIM6_STOP_W { w: self }
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W {
        DBG_TIM7_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
