#[doc = "Register `APB1_FZ` reader"]
pub struct R(crate::R<APB1_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1_FZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1_FZ` writer"]
pub struct W(crate::W<APB1_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1_FZ_SPEC>;
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
impl From<crate::W<APB1_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1_FZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP"]
pub struct DBG_TIM2_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM2_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM2_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM2_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP"]
pub struct DBG_TIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM2_STOP_W<'a> {
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
#[doc = "Field `DBG_TIM3_STOP` reader - DBG_TIM3 _STOP"]
pub struct DBG_TIM3_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM3_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM3_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM3_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM3_STOP` writer - DBG_TIM3 _STOP"]
pub struct DBG_TIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM3_STOP_W<'a> {
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
#[doc = "Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP"]
pub struct DBG_TIM4_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM4_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM4_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM4_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP"]
pub struct DBG_TIM4_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM4_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DBG_TIM12_STOP` reader - DBG_TIM12_STOP"]
pub struct DBG_TIM12_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM12_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM12_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM12_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM12_STOP` writer - DBG_TIM12_STOP"]
pub struct DBG_TIM12_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM12_STOP_W<'a> {
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
#[doc = "Field `DBG_TIM13_STOP` reader - DBG_TIM13_STOP"]
pub struct DBG_TIM13_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM13_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM13_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM13_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM13_STOP` writer - DBG_TIM13_STOP"]
pub struct DBG_TIM13_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM13_STOP_W<'a> {
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
#[doc = "Field `DBG_TIM14_STOP` reader - DBG_TIM14_STOP"]
pub struct DBG_TIM14_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM14_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM14_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM14_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM14_STOP` writer - DBG_TIM14_STOP"]
pub struct DBG_TIM14_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM14_STOP_W<'a> {
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
#[doc = "Field `DBG_RTC_STOP` reader - RTC stopped when Core is halted"]
pub struct DBG_RTC_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_RTC_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_RTC_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_RTC_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - RTC stopped when Core is halted"]
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
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
#[doc = "Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP"]
pub struct DBG_WWDG_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_WWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WWDG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_WWDG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP"]
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
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
#[doc = "Field `DBG_IWDG_STOP` reader - DBG_IWDEG_STOP"]
pub struct DBG_IWDG_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_IWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_IWDG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_IWDG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - DBG_IWDEG_STOP"]
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
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
#[doc = "Field `DBG_J2C1_SMBUS_TIMEOUT` reader - DBG_J2C1_SMBUS_TIMEOUT"]
pub struct DBG_J2C1_SMBUS_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl DBG_J2C1_SMBUS_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_J2C1_SMBUS_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_J2C1_SMBUS_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_J2C1_SMBUS_TIMEOUT` writer - DBG_J2C1_SMBUS_TIMEOUT"]
pub struct DBG_J2C1_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_J2C1_SMBUS_TIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `DBG_J2C2_SMBUS_TIMEOUT` reader - DBG_J2C2_SMBUS_TIMEOUT"]
pub struct DBG_J2C2_SMBUS_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl DBG_J2C2_SMBUS_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_J2C2_SMBUS_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_J2C2_SMBUS_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_J2C2_SMBUS_TIMEOUT` writer - DBG_J2C2_SMBUS_TIMEOUT"]
pub struct DBG_J2C2_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_J2C2_SMBUS_TIMEOUT_W<'a> {
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
#[doc = "Field `DBG_J2C3SMBUS_TIMEOUT` reader - DBG_J2C3SMBUS_TIMEOUT"]
pub struct DBG_J2C3SMBUS_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl DBG_J2C3SMBUS_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_J2C3SMBUS_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_J2C3SMBUS_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_J2C3SMBUS_TIMEOUT` writer - DBG_J2C3SMBUS_TIMEOUT"]
pub struct DBG_J2C3SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_J2C3SMBUS_TIMEOUT_W<'a> {
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
#[doc = "Field `DBG_I2CFMP_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub struct DBG_I2CFMP_SMBUS_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl DBG_I2CFMP_SMBUS_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2CFMP_SMBUS_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2CFMP_SMBUS_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2CFMP_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub struct DBG_I2CFMP_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2CFMP_SMBUS_TIMEOUT_W<'a> {
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
#[doc = "Field `DBG_CAN1_STOP` reader - DBG_CAN1_STOP"]
pub struct DBG_CAN1_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_CAN1_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_CAN1_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_CAN1_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_CAN1_STOP` writer - DBG_CAN1_STOP"]
pub struct DBG_CAN1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_CAN1_STOP_W<'a> {
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
#[doc = "Field `DBG_CAN2_STOP` reader - DBG_CAN2_STOP"]
pub struct DBG_CAN2_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_CAN2_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_CAN2_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_CAN2_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_CAN2_STOP` writer - DBG_CAN2_STOP"]
pub struct DBG_CAN2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_CAN2_STOP_W<'a> {
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
    #[doc = "Bit 0 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBG_TIM3 _STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DBG_TIM12_STOP"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DBG_TIM13_STOP"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DBG_TIM14_STOP"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DBG_IWDEG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_j2c1_smbus_timeout(&self) -> DBG_J2C1_SMBUS_TIMEOUT_R {
        DBG_J2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_j2c2_smbus_timeout(&self) -> DBG_J2C2_SMBUS_TIMEOUT_R {
        DBG_J2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_j2c3smbus_timeout(&self) -> DBG_J2C3SMBUS_TIMEOUT_R {
        DBG_J2C3SMBUS_TIMEOUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_i2cfmp_smbus_timeout(&self) -> DBG_I2CFMP_SMBUS_TIMEOUT_R {
        DBG_I2CFMP_SMBUS_TIMEOUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DBG_CAN1_STOP"]
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DBG_CAN1_STOP_R {
        DBG_CAN1_STOP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DBG_CAN2_STOP"]
    #[inline(always)]
    pub fn dbg_can2_stop(&self) -> DBG_CAN2_STOP_R {
        DBG_CAN2_STOP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W {
        DBG_TIM2_STOP_W { w: self }
    }
    #[doc = "Bit 1 - DBG_TIM3 _STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W {
        DBG_TIM3_STOP_W { w: self }
    }
    #[doc = "Bit 2 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W {
        DBG_TIM4_STOP_W { w: self }
    }
    #[doc = "Bit 3 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W {
        DBG_TIM5_STOP_W { w: self }
    }
    #[doc = "Bit 4 - DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W {
        DBG_TIM6_STOP_W { w: self }
    }
    #[doc = "Bit 5 - DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W {
        DBG_TIM7_STOP_W { w: self }
    }
    #[doc = "Bit 6 - DBG_TIM12_STOP"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W {
        DBG_TIM12_STOP_W { w: self }
    }
    #[doc = "Bit 7 - DBG_TIM13_STOP"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W {
        DBG_TIM13_STOP_W { w: self }
    }
    #[doc = "Bit 8 - DBG_TIM14_STOP"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W {
        DBG_TIM14_STOP_W { w: self }
    }
    #[doc = "Bit 10 - RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    #[doc = "Bit 11 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    #[doc = "Bit 12 - DBG_IWDEG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    #[doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_j2c1_smbus_timeout(&mut self) -> DBG_J2C1_SMBUS_TIMEOUT_W {
        DBG_J2C1_SMBUS_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_j2c2_smbus_timeout(&mut self) -> DBG_J2C2_SMBUS_TIMEOUT_W {
        DBG_J2C2_SMBUS_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_j2c3smbus_timeout(&mut self) -> DBG_J2C3SMBUS_TIMEOUT_W {
        DBG_J2C3SMBUS_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 24 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_i2cfmp_smbus_timeout(&mut self) -> DBG_I2CFMP_SMBUS_TIMEOUT_W {
        DBG_I2CFMP_SMBUS_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 25 - DBG_CAN1_STOP"]
    #[inline(always)]
    pub fn dbg_can1_stop(&mut self) -> DBG_CAN1_STOP_W {
        DBG_CAN1_STOP_W { w: self }
    }
    #[doc = "Bit 26 - DBG_CAN2_STOP"]
    #[inline(always)]
    pub fn dbg_can2_stop(&mut self) -> DBG_CAN2_STOP_W {
        DBG_CAN2_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug MCU APB1 Freeze registe\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1_fz](index.html) module"]
pub struct APB1_FZ_SPEC;
impl crate::RegisterSpec for APB1_FZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1_fz::R](R) reader structure"]
impl crate::Readable for APB1_FZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1_fz::W](W) writer structure"]
impl crate::Writable for APB1_FZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1_FZ to value 0"]
impl crate::Resettable for APB1_FZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
