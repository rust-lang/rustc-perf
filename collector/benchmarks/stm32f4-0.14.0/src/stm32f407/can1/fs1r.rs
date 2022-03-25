#[doc = "Register `FS1R` reader"]
pub struct R(crate::R<FS1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS1R` writer"]
pub struct W(crate::W<FS1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS1R_SPEC>;
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
impl From<crate::W<FS1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSC0` reader - Filter scale configuration"]
pub struct FSC0_R(crate::FieldReader<bool, bool>);
impl FSC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC0` writer - Filter scale configuration"]
pub struct FSC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC0_W<'a> {
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
#[doc = "Field `FSC1` reader - Filter scale configuration"]
pub struct FSC1_R(crate::FieldReader<bool, bool>);
impl FSC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC1` writer - Filter scale configuration"]
pub struct FSC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC1_W<'a> {
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
#[doc = "Field `FSC2` reader - Filter scale configuration"]
pub struct FSC2_R(crate::FieldReader<bool, bool>);
impl FSC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC2` writer - Filter scale configuration"]
pub struct FSC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC2_W<'a> {
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
#[doc = "Field `FSC3` reader - Filter scale configuration"]
pub struct FSC3_R(crate::FieldReader<bool, bool>);
impl FSC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC3` writer - Filter scale configuration"]
pub struct FSC3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC3_W<'a> {
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
#[doc = "Field `FSC4` reader - Filter scale configuration"]
pub struct FSC4_R(crate::FieldReader<bool, bool>);
impl FSC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC4` writer - Filter scale configuration"]
pub struct FSC4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC4_W<'a> {
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
#[doc = "Field `FSC5` reader - Filter scale configuration"]
pub struct FSC5_R(crate::FieldReader<bool, bool>);
impl FSC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC5` writer - Filter scale configuration"]
pub struct FSC5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC5_W<'a> {
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
#[doc = "Field `FSC6` reader - Filter scale configuration"]
pub struct FSC6_R(crate::FieldReader<bool, bool>);
impl FSC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC6` writer - Filter scale configuration"]
pub struct FSC6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC6_W<'a> {
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
#[doc = "Field `FSC7` reader - Filter scale configuration"]
pub struct FSC7_R(crate::FieldReader<bool, bool>);
impl FSC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC7` writer - Filter scale configuration"]
pub struct FSC7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC7_W<'a> {
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
#[doc = "Field `FSC8` reader - Filter scale configuration"]
pub struct FSC8_R(crate::FieldReader<bool, bool>);
impl FSC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC8` writer - Filter scale configuration"]
pub struct FSC8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC8_W<'a> {
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
#[doc = "Field `FSC9` reader - Filter scale configuration"]
pub struct FSC9_R(crate::FieldReader<bool, bool>);
impl FSC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC9` writer - Filter scale configuration"]
pub struct FSC9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC9_W<'a> {
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
#[doc = "Field `FSC10` reader - Filter scale configuration"]
pub struct FSC10_R(crate::FieldReader<bool, bool>);
impl FSC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC10` writer - Filter scale configuration"]
pub struct FSC10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC10_W<'a> {
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
#[doc = "Field `FSC11` reader - Filter scale configuration"]
pub struct FSC11_R(crate::FieldReader<bool, bool>);
impl FSC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC11` writer - Filter scale configuration"]
pub struct FSC11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC11_W<'a> {
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
#[doc = "Field `FSC12` reader - Filter scale configuration"]
pub struct FSC12_R(crate::FieldReader<bool, bool>);
impl FSC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC12` writer - Filter scale configuration"]
pub struct FSC12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC12_W<'a> {
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
#[doc = "Field `FSC13` reader - Filter scale configuration"]
pub struct FSC13_R(crate::FieldReader<bool, bool>);
impl FSC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC13` writer - Filter scale configuration"]
pub struct FSC13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC13_W<'a> {
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
#[doc = "Field `FSC14` reader - Filter scale configuration"]
pub struct FSC14_R(crate::FieldReader<bool, bool>);
impl FSC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC14` writer - Filter scale configuration"]
pub struct FSC14_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC14_W<'a> {
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
#[doc = "Field `FSC15` reader - Filter scale configuration"]
pub struct FSC15_R(crate::FieldReader<bool, bool>);
impl FSC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC15` writer - Filter scale configuration"]
pub struct FSC15_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC15_W<'a> {
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
#[doc = "Field `FSC16` reader - Filter scale configuration"]
pub struct FSC16_R(crate::FieldReader<bool, bool>);
impl FSC16_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC16` writer - Filter scale configuration"]
pub struct FSC16_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC16_W<'a> {
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
#[doc = "Field `FSC17` reader - Filter scale configuration"]
pub struct FSC17_R(crate::FieldReader<bool, bool>);
impl FSC17_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC17` writer - Filter scale configuration"]
pub struct FSC17_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC17_W<'a> {
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
#[doc = "Field `FSC18` reader - Filter scale configuration"]
pub struct FSC18_R(crate::FieldReader<bool, bool>);
impl FSC18_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC18` writer - Filter scale configuration"]
pub struct FSC18_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC18_W<'a> {
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
#[doc = "Field `FSC19` reader - Filter scale configuration"]
pub struct FSC19_R(crate::FieldReader<bool, bool>);
impl FSC19_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC19` writer - Filter scale configuration"]
pub struct FSC19_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC19_W<'a> {
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
#[doc = "Field `FSC20` reader - Filter scale configuration"]
pub struct FSC20_R(crate::FieldReader<bool, bool>);
impl FSC20_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC20` writer - Filter scale configuration"]
pub struct FSC20_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC20_W<'a> {
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
#[doc = "Field `FSC21` reader - Filter scale configuration"]
pub struct FSC21_R(crate::FieldReader<bool, bool>);
impl FSC21_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC21` writer - Filter scale configuration"]
pub struct FSC21_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC21_W<'a> {
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
#[doc = "Field `FSC22` reader - Filter scale configuration"]
pub struct FSC22_R(crate::FieldReader<bool, bool>);
impl FSC22_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC22` writer - Filter scale configuration"]
pub struct FSC22_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC22_W<'a> {
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
#[doc = "Field `FSC23` reader - Filter scale configuration"]
pub struct FSC23_R(crate::FieldReader<bool, bool>);
impl FSC23_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC23` writer - Filter scale configuration"]
pub struct FSC23_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC23_W<'a> {
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
#[doc = "Field `FSC24` reader - Filter scale configuration"]
pub struct FSC24_R(crate::FieldReader<bool, bool>);
impl FSC24_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC24` writer - Filter scale configuration"]
pub struct FSC24_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC24_W<'a> {
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
#[doc = "Field `FSC25` reader - Filter scale configuration"]
pub struct FSC25_R(crate::FieldReader<bool, bool>);
impl FSC25_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC25` writer - Filter scale configuration"]
pub struct FSC25_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC25_W<'a> {
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
#[doc = "Field `FSC26` reader - Filter scale configuration"]
pub struct FSC26_R(crate::FieldReader<bool, bool>);
impl FSC26_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC26` writer - Filter scale configuration"]
pub struct FSC26_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC26_W<'a> {
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
#[doc = "Field `FSC27` reader - Filter scale configuration"]
pub struct FSC27_R(crate::FieldReader<bool, bool>);
impl FSC27_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSC27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC27` writer - Filter scale configuration"]
pub struct FSC27_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&self) -> FSC14_R {
        FSC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&self) -> FSC15_R {
        FSC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&self) -> FSC16_R {
        FSC16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&self) -> FSC17_R {
        FSC17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&self) -> FSC18_R {
        FSC18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&self) -> FSC19_R {
        FSC19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&self) -> FSC20_R {
        FSC20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&self) -> FSC21_R {
        FSC21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&self) -> FSC22_R {
        FSC22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&self) -> FSC23_R {
        FSC23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&self) -> FSC24_R {
        FSC24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&self) -> FSC25_R {
        FSC25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&self) -> FSC26_R {
        FSC26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&self) -> FSC27_R {
        FSC27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC0_W {
        FSC0_W { w: self }
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC1_W {
        FSC1_W { w: self }
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC2_W {
        FSC2_W { w: self }
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC3_W {
        FSC3_W { w: self }
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC4_W {
        FSC4_W { w: self }
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC5_W {
        FSC5_W { w: self }
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC6_W {
        FSC6_W { w: self }
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC7_W {
        FSC7_W { w: self }
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC8_W {
        FSC8_W { w: self }
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC9_W {
        FSC9_W { w: self }
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC10_W {
        FSC10_W { w: self }
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC11_W {
        FSC11_W { w: self }
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC12_W {
        FSC12_W { w: self }
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC13_W {
        FSC13_W { w: self }
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&mut self) -> FSC14_W {
        FSC14_W { w: self }
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&mut self) -> FSC15_W {
        FSC15_W { w: self }
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&mut self) -> FSC16_W {
        FSC16_W { w: self }
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&mut self) -> FSC17_W {
        FSC17_W { w: self }
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&mut self) -> FSC18_W {
        FSC18_W { w: self }
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&mut self) -> FSC19_W {
        FSC19_W { w: self }
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&mut self) -> FSC20_W {
        FSC20_W { w: self }
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&mut self) -> FSC21_W {
        FSC21_W { w: self }
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&mut self) -> FSC22_W {
        FSC22_W { w: self }
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&mut self) -> FSC23_W {
        FSC23_W { w: self }
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&mut self) -> FSC24_W {
        FSC24_W { w: self }
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&mut self) -> FSC25_W {
        FSC25_W { w: self }
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&mut self) -> FSC26_W {
        FSC26_W { w: self }
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&mut self) -> FSC27_W {
        FSC27_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter scale register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs1r](index.html) module"]
pub struct FS1R_SPEC;
impl crate::RegisterSpec for FS1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs1r::R](R) reader structure"]
impl crate::Readable for FS1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs1r::W](W) writer structure"]
impl crate::Writable for FS1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FS1R to value 0"]
impl crate::Resettable for FS1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
