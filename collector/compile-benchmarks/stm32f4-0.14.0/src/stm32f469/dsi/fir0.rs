#[doc = "Register `FIR0` reader"]
pub struct R(crate::R<FIR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIR0` writer"]
pub struct W(crate::W<FIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR0_SPEC>;
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
impl From<crate::W<FIR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPE4` reader - Force PHY Error 4"]
pub struct FPE4_R(crate::FieldReader<bool, bool>);
impl FPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE4` writer - Force PHY Error 4"]
pub struct FPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE4_W<'a> {
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
#[doc = "Field `FPE3` reader - Force PHY Error 3"]
pub struct FPE3_R(crate::FieldReader<bool, bool>);
impl FPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE3` writer - Force PHY Error 3"]
pub struct FPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE3_W<'a> {
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
#[doc = "Field `FPE2` reader - Force PHY Error 2"]
pub struct FPE2_R(crate::FieldReader<bool, bool>);
impl FPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE2` writer - Force PHY Error 2"]
pub struct FPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE2_W<'a> {
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
#[doc = "Field `FPE1` reader - Force PHY Error 1"]
pub struct FPE1_R(crate::FieldReader<bool, bool>);
impl FPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE1` writer - Force PHY Error 1"]
pub struct FPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE1_W<'a> {
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
#[doc = "Field `FPE0` reader - Force PHY Error 0"]
pub struct FPE0_R(crate::FieldReader<bool, bool>);
impl FPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE0` writer - Force PHY Error 0"]
pub struct FPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE0_W<'a> {
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
#[doc = "Field `FAE15` reader - Force Acknowledge Error 15"]
pub struct FAE15_R(crate::FieldReader<bool, bool>);
impl FAE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE15` writer - Force Acknowledge Error 15"]
pub struct FAE15_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE15_W<'a> {
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
#[doc = "Field `FAE14` reader - Force Acknowledge Error 14"]
pub struct FAE14_R(crate::FieldReader<bool, bool>);
impl FAE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE14` writer - Force Acknowledge Error 14"]
pub struct FAE14_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE14_W<'a> {
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
#[doc = "Field `FAE13` reader - Force Acknowledge Error 13"]
pub struct FAE13_R(crate::FieldReader<bool, bool>);
impl FAE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE13` writer - Force Acknowledge Error 13"]
pub struct FAE13_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE13_W<'a> {
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
#[doc = "Field `FAE12` reader - Force Acknowledge Error 12"]
pub struct FAE12_R(crate::FieldReader<bool, bool>);
impl FAE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE12` writer - Force Acknowledge Error 12"]
pub struct FAE12_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE12_W<'a> {
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
#[doc = "Field `FAE11` reader - Force Acknowledge Error 11"]
pub struct FAE11_R(crate::FieldReader<bool, bool>);
impl FAE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE11` writer - Force Acknowledge Error 11"]
pub struct FAE11_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE11_W<'a> {
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
#[doc = "Field `FAE10` reader - Force Acknowledge Error 10"]
pub struct FAE10_R(crate::FieldReader<bool, bool>);
impl FAE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE10` writer - Force Acknowledge Error 10"]
pub struct FAE10_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE10_W<'a> {
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
#[doc = "Field `FAE9` reader - Force Acknowledge Error 9"]
pub struct FAE9_R(crate::FieldReader<bool, bool>);
impl FAE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE9` writer - Force Acknowledge Error 9"]
pub struct FAE9_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE9_W<'a> {
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
#[doc = "Field `FAE8` reader - Force Acknowledge Error 8"]
pub struct FAE8_R(crate::FieldReader<bool, bool>);
impl FAE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE8` writer - Force Acknowledge Error 8"]
pub struct FAE8_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE8_W<'a> {
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
#[doc = "Field `FAE7` reader - Force Acknowledge Error 7"]
pub struct FAE7_R(crate::FieldReader<bool, bool>);
impl FAE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE7` writer - Force Acknowledge Error 7"]
pub struct FAE7_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE7_W<'a> {
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
#[doc = "Field `FAE6` reader - Force Acknowledge Error 6"]
pub struct FAE6_R(crate::FieldReader<bool, bool>);
impl FAE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE6` writer - Force Acknowledge Error 6"]
pub struct FAE6_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE6_W<'a> {
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
#[doc = "Field `FAE5` reader - Force Acknowledge Error 5"]
pub struct FAE5_R(crate::FieldReader<bool, bool>);
impl FAE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE5` writer - Force Acknowledge Error 5"]
pub struct FAE5_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE5_W<'a> {
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
#[doc = "Field `FAE4` reader - Force Acknowledge Error 4"]
pub struct FAE4_R(crate::FieldReader<bool, bool>);
impl FAE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE4` writer - Force Acknowledge Error 4"]
pub struct FAE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE4_W<'a> {
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
#[doc = "Field `FAE3` reader - Force Acknowledge Error 3"]
pub struct FAE3_R(crate::FieldReader<bool, bool>);
impl FAE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE3` writer - Force Acknowledge Error 3"]
pub struct FAE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE3_W<'a> {
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
#[doc = "Field `FAE2` reader - Force Acknowledge Error 2"]
pub struct FAE2_R(crate::FieldReader<bool, bool>);
impl FAE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE2` writer - Force Acknowledge Error 2"]
pub struct FAE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE2_W<'a> {
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
#[doc = "Field `FAE1` reader - Force Acknowledge Error 1"]
pub struct FAE1_R(crate::FieldReader<bool, bool>);
impl FAE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE1` writer - Force Acknowledge Error 1"]
pub struct FAE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE1_W<'a> {
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
#[doc = "Field `FAE0` reader - Force Acknowledge Error 0"]
pub struct FAE0_R(crate::FieldReader<bool, bool>);
impl FAE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAE0` writer - Force Acknowledge Error 0"]
pub struct FAE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE0_W<'a> {
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
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    pub fn fpe4(&self) -> FPE4_R {
        FPE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    pub fn fpe3(&self) -> FPE3_R {
        FPE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    pub fn fpe2(&self) -> FPE2_R {
        FPE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    pub fn fpe1(&self) -> FPE1_R {
        FPE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    pub fn fpe0(&self) -> FPE0_R {
        FPE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    pub fn fae15(&self) -> FAE15_R {
        FAE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    pub fn fae14(&self) -> FAE14_R {
        FAE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    pub fn fae13(&self) -> FAE13_R {
        FAE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    pub fn fae12(&self) -> FAE12_R {
        FAE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    pub fn fae11(&self) -> FAE11_R {
        FAE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    pub fn fae10(&self) -> FAE10_R {
        FAE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    pub fn fae9(&self) -> FAE9_R {
        FAE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    pub fn fae8(&self) -> FAE8_R {
        FAE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    pub fn fae7(&self) -> FAE7_R {
        FAE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    pub fn fae6(&self) -> FAE6_R {
        FAE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    pub fn fae5(&self) -> FAE5_R {
        FAE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    pub fn fae4(&self) -> FAE4_R {
        FAE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    pub fn fae3(&self) -> FAE3_R {
        FAE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    pub fn fae2(&self) -> FAE2_R {
        FAE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    pub fn fae1(&self) -> FAE1_R {
        FAE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    pub fn fae0(&self) -> FAE0_R {
        FAE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE4_W {
        FPE4_W { w: self }
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE3_W {
        FPE3_W { w: self }
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE2_W {
        FPE2_W { w: self }
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE1_W {
        FPE1_W { w: self }
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE0_W {
        FPE0_W { w: self }
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    pub fn fae15(&mut self) -> FAE15_W {
        FAE15_W { w: self }
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    pub fn fae14(&mut self) -> FAE14_W {
        FAE14_W { w: self }
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    pub fn fae13(&mut self) -> FAE13_W {
        FAE13_W { w: self }
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    pub fn fae12(&mut self) -> FAE12_W {
        FAE12_W { w: self }
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    pub fn fae11(&mut self) -> FAE11_W {
        FAE11_W { w: self }
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    pub fn fae10(&mut self) -> FAE10_W {
        FAE10_W { w: self }
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    pub fn fae9(&mut self) -> FAE9_W {
        FAE9_W { w: self }
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    pub fn fae8(&mut self) -> FAE8_W {
        FAE8_W { w: self }
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    pub fn fae7(&mut self) -> FAE7_W {
        FAE7_W { w: self }
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    pub fn fae6(&mut self) -> FAE6_W {
        FAE6_W { w: self }
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    pub fn fae5(&mut self) -> FAE5_W {
        FAE5_W { w: self }
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    pub fn fae4(&mut self) -> FAE4_W {
        FAE4_W { w: self }
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    pub fn fae3(&mut self) -> FAE3_W {
        FAE3_W { w: self }
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    pub fn fae2(&mut self) -> FAE2_W {
        FAE2_W { w: self }
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    pub fn fae1(&mut self) -> FAE1_W {
        FAE1_W { w: self }
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    pub fn fae0(&mut self) -> FAE0_W {
        FAE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Force Interrupt Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir0](index.html) module"]
pub struct FIR0_SPEC;
impl crate::RegisterSpec for FIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fir0::R](R) reader structure"]
impl crate::Readable for FIR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fir0::W](W) writer structure"]
impl crate::Writable for FIR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIR0 to value 0"]
impl crate::Resettable for FIR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
