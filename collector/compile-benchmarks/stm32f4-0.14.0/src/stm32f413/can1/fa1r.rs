#[doc = "Register `FA1R` reader"]
pub struct R(crate::R<FA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FA1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FA1R` writer"]
pub struct W(crate::W<FA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FA1R_SPEC>;
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
impl From<crate::W<FA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FA1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FACT0` reader - Filter active"]
pub struct FACT0_R(crate::FieldReader<bool, bool>);
impl FACT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT0` writer - Filter active"]
pub struct FACT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT0_W<'a> {
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
#[doc = "Field `FACT1` reader - Filter active"]
pub struct FACT1_R(crate::FieldReader<bool, bool>);
impl FACT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT1` writer - Filter active"]
pub struct FACT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT1_W<'a> {
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
#[doc = "Field `FACT2` reader - Filter active"]
pub struct FACT2_R(crate::FieldReader<bool, bool>);
impl FACT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT2` writer - Filter active"]
pub struct FACT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT2_W<'a> {
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
#[doc = "Field `FACT3` reader - Filter active"]
pub struct FACT3_R(crate::FieldReader<bool, bool>);
impl FACT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT3` writer - Filter active"]
pub struct FACT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT3_W<'a> {
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
#[doc = "Field `FACT4` reader - Filter active"]
pub struct FACT4_R(crate::FieldReader<bool, bool>);
impl FACT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT4` writer - Filter active"]
pub struct FACT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT4_W<'a> {
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
#[doc = "Field `FACT5` reader - Filter active"]
pub struct FACT5_R(crate::FieldReader<bool, bool>);
impl FACT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT5` writer - Filter active"]
pub struct FACT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT5_W<'a> {
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
#[doc = "Field `FACT6` reader - Filter active"]
pub struct FACT6_R(crate::FieldReader<bool, bool>);
impl FACT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT6` writer - Filter active"]
pub struct FACT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT6_W<'a> {
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
#[doc = "Field `FACT7` reader - Filter active"]
pub struct FACT7_R(crate::FieldReader<bool, bool>);
impl FACT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT7` writer - Filter active"]
pub struct FACT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT7_W<'a> {
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
#[doc = "Field `FACT8` reader - Filter active"]
pub struct FACT8_R(crate::FieldReader<bool, bool>);
impl FACT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT8` writer - Filter active"]
pub struct FACT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT8_W<'a> {
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
#[doc = "Field `FACT9` reader - Filter active"]
pub struct FACT9_R(crate::FieldReader<bool, bool>);
impl FACT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT9` writer - Filter active"]
pub struct FACT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT9_W<'a> {
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
#[doc = "Field `FACT10` reader - Filter active"]
pub struct FACT10_R(crate::FieldReader<bool, bool>);
impl FACT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT10` writer - Filter active"]
pub struct FACT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT10_W<'a> {
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
#[doc = "Field `FACT11` reader - Filter active"]
pub struct FACT11_R(crate::FieldReader<bool, bool>);
impl FACT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT11` writer - Filter active"]
pub struct FACT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT11_W<'a> {
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
#[doc = "Field `FACT12` reader - Filter active"]
pub struct FACT12_R(crate::FieldReader<bool, bool>);
impl FACT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT12` writer - Filter active"]
pub struct FACT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT12_W<'a> {
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
#[doc = "Field `FACT13` reader - Filter active"]
pub struct FACT13_R(crate::FieldReader<bool, bool>);
impl FACT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT13` writer - Filter active"]
pub struct FACT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT13_W<'a> {
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
#[doc = "Field `FACT14` reader - Filter active"]
pub struct FACT14_R(crate::FieldReader<bool, bool>);
impl FACT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT14` writer - Filter active"]
pub struct FACT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT14_W<'a> {
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
#[doc = "Field `FACT15` reader - Filter active"]
pub struct FACT15_R(crate::FieldReader<bool, bool>);
impl FACT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT15` writer - Filter active"]
pub struct FACT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT15_W<'a> {
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
#[doc = "Field `FACT16` reader - Filter active"]
pub struct FACT16_R(crate::FieldReader<bool, bool>);
impl FACT16_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT16` writer - Filter active"]
pub struct FACT16_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT16_W<'a> {
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
#[doc = "Field `FACT17` reader - Filter active"]
pub struct FACT17_R(crate::FieldReader<bool, bool>);
impl FACT17_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT17` writer - Filter active"]
pub struct FACT17_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT17_W<'a> {
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
#[doc = "Field `FACT18` reader - Filter active"]
pub struct FACT18_R(crate::FieldReader<bool, bool>);
impl FACT18_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT18` writer - Filter active"]
pub struct FACT18_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT18_W<'a> {
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
#[doc = "Field `FACT19` reader - Filter active"]
pub struct FACT19_R(crate::FieldReader<bool, bool>);
impl FACT19_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT19` writer - Filter active"]
pub struct FACT19_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT19_W<'a> {
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
#[doc = "Field `FACT20` reader - Filter active"]
pub struct FACT20_R(crate::FieldReader<bool, bool>);
impl FACT20_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT20` writer - Filter active"]
pub struct FACT20_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT20_W<'a> {
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
#[doc = "Field `FACT21` reader - Filter active"]
pub struct FACT21_R(crate::FieldReader<bool, bool>);
impl FACT21_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT21` writer - Filter active"]
pub struct FACT21_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT21_W<'a> {
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
#[doc = "Field `FACT22` reader - Filter active"]
pub struct FACT22_R(crate::FieldReader<bool, bool>);
impl FACT22_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT22` writer - Filter active"]
pub struct FACT22_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT22_W<'a> {
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
#[doc = "Field `FACT23` reader - Filter active"]
pub struct FACT23_R(crate::FieldReader<bool, bool>);
impl FACT23_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT23` writer - Filter active"]
pub struct FACT23_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT23_W<'a> {
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
#[doc = "Field `FACT24` reader - Filter active"]
pub struct FACT24_R(crate::FieldReader<bool, bool>);
impl FACT24_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT24` writer - Filter active"]
pub struct FACT24_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT24_W<'a> {
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
#[doc = "Field `FACT25` reader - Filter active"]
pub struct FACT25_R(crate::FieldReader<bool, bool>);
impl FACT25_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT25` writer - Filter active"]
pub struct FACT25_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT25_W<'a> {
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
#[doc = "Field `FACT26` reader - Filter active"]
pub struct FACT26_R(crate::FieldReader<bool, bool>);
impl FACT26_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT26` writer - Filter active"]
pub struct FACT26_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT26_W<'a> {
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
#[doc = "Field `FACT27` reader - Filter active"]
pub struct FACT27_R(crate::FieldReader<bool, bool>);
impl FACT27_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACT27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACT27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACT27` writer - Filter active"]
pub struct FACT27_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT27_W<'a> {
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
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&self) -> FACT0_R {
        FACT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&self) -> FACT1_R {
        FACT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&self) -> FACT2_R {
        FACT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&self) -> FACT3_R {
        FACT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&self) -> FACT4_R {
        FACT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&self) -> FACT5_R {
        FACT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&self) -> FACT6_R {
        FACT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&self) -> FACT7_R {
        FACT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&self) -> FACT8_R {
        FACT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&self) -> FACT9_R {
        FACT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&self) -> FACT10_R {
        FACT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&self) -> FACT11_R {
        FACT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&self) -> FACT12_R {
        FACT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&self) -> FACT13_R {
        FACT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&self) -> FACT14_R {
        FACT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&self) -> FACT15_R {
        FACT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&self) -> FACT16_R {
        FACT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&self) -> FACT17_R {
        FACT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&self) -> FACT18_R {
        FACT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&self) -> FACT19_R {
        FACT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&self) -> FACT20_R {
        FACT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&self) -> FACT21_R {
        FACT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&self) -> FACT22_R {
        FACT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&self) -> FACT23_R {
        FACT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&self) -> FACT24_R {
        FACT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&self) -> FACT25_R {
        FACT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&self) -> FACT26_R {
        FACT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&self) -> FACT27_R {
        FACT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&mut self) -> FACT0_W {
        FACT0_W { w: self }
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&mut self) -> FACT1_W {
        FACT1_W { w: self }
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&mut self) -> FACT2_W {
        FACT2_W { w: self }
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&mut self) -> FACT3_W {
        FACT3_W { w: self }
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&mut self) -> FACT4_W {
        FACT4_W { w: self }
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&mut self) -> FACT5_W {
        FACT5_W { w: self }
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&mut self) -> FACT6_W {
        FACT6_W { w: self }
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&mut self) -> FACT7_W {
        FACT7_W { w: self }
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&mut self) -> FACT8_W {
        FACT8_W { w: self }
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&mut self) -> FACT9_W {
        FACT9_W { w: self }
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&mut self) -> FACT10_W {
        FACT10_W { w: self }
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&mut self) -> FACT11_W {
        FACT11_W { w: self }
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&mut self) -> FACT12_W {
        FACT12_W { w: self }
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&mut self) -> FACT13_W {
        FACT13_W { w: self }
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&mut self) -> FACT14_W {
        FACT14_W { w: self }
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&mut self) -> FACT15_W {
        FACT15_W { w: self }
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&mut self) -> FACT16_W {
        FACT16_W { w: self }
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&mut self) -> FACT17_W {
        FACT17_W { w: self }
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&mut self) -> FACT18_W {
        FACT18_W { w: self }
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&mut self) -> FACT19_W {
        FACT19_W { w: self }
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&mut self) -> FACT20_W {
        FACT20_W { w: self }
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&mut self) -> FACT21_W {
        FACT21_W { w: self }
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&mut self) -> FACT22_W {
        FACT22_W { w: self }
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&mut self) -> FACT23_W {
        FACT23_W { w: self }
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&mut self) -> FACT24_W {
        FACT24_W { w: self }
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&mut self) -> FACT25_W {
        FACT25_W { w: self }
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&mut self) -> FACT26_W {
        FACT26_W { w: self }
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&mut self) -> FACT27_W {
        FACT27_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter activation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fa1r](index.html) module"]
pub struct FA1R_SPEC;
impl crate::RegisterSpec for FA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fa1r::R](R) reader structure"]
impl crate::Readable for FA1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fa1r::W](W) writer structure"]
impl crate::Writable for FA1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FA1R to value 0"]
impl crate::Resettable for FA1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
