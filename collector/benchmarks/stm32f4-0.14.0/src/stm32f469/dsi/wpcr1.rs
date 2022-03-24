#[doc = "Register `WPCR1` reader"]
pub struct R(crate::R<WPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR1` writer"]
pub struct W(crate::W<WPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR1_SPEC>;
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
impl From<crate::W<WPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCLKPOSTEN` reader - custom time for tCLK-POST Enable"]
pub struct TCLKPOSTEN_R(crate::FieldReader<bool, bool>);
impl TCLKPOSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCLKPOSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLKPOSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLKPOSTEN` writer - custom time for tCLK-POST Enable"]
pub struct TCLKPOSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPOSTEN_W<'a> {
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
#[doc = "Field `TLPXCEN` reader - custom time for tLPX for Clock lane Enable"]
pub struct TLPXCEN_R(crate::FieldReader<bool, bool>);
impl TLPXCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPXCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPXCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPXCEN` writer - custom time for tLPX for Clock lane Enable"]
pub struct TLPXCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXCEN_W<'a> {
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
#[doc = "Field `THSEXITEN` reader - custom time for tHS-EXIT Enable"]
pub struct THSEXITEN_R(crate::FieldReader<bool, bool>);
impl THSEXITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        THSEXITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSEXITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSEXITEN` writer - custom time for tHS-EXIT Enable"]
pub struct THSEXITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSEXITEN_W<'a> {
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
#[doc = "Field `TLPXDEN` reader - custom time for tLPX for Data lanes Enable"]
pub struct TLPXDEN_R(crate::FieldReader<bool, bool>);
impl TLPXDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPXDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPXDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPXDEN` writer - custom time for tLPX for Data lanes Enable"]
pub struct TLPXDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXDEN_W<'a> {
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
#[doc = "Field `THSZEROEN` reader - custom time for tHS-ZERO Enable"]
pub struct THSZEROEN_R(crate::FieldReader<bool, bool>);
impl THSZEROEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        THSZEROEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSZEROEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSZEROEN` writer - custom time for tHS-ZERO Enable"]
pub struct THSZEROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSZEROEN_W<'a> {
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
#[doc = "Field `THSTRAILEN` reader - custom time for tHS-TRAIL Enable"]
pub struct THSTRAILEN_R(crate::FieldReader<bool, bool>);
impl THSTRAILEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        THSTRAILEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSTRAILEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSTRAILEN` writer - custom time for tHS-TRAIL Enable"]
pub struct THSTRAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSTRAILEN_W<'a> {
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
#[doc = "Field `THSPREPEN` reader - custom time for tHS-PREPARE Enable"]
pub struct THSPREPEN_R(crate::FieldReader<bool, bool>);
impl THSPREPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        THSPREPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSPREPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSPREPEN` writer - custom time for tHS-PREPARE Enable"]
pub struct THSPREPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSPREPEN_W<'a> {
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
#[doc = "Field `TCLKZEROEN` reader - custom time for tCLK-ZERO Enable"]
pub struct TCLKZEROEN_R(crate::FieldReader<bool, bool>);
impl TCLKZEROEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCLKZEROEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLKZEROEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLKZEROEN` writer - custom time for tCLK-ZERO Enable"]
pub struct TCLKZEROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKZEROEN_W<'a> {
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
#[doc = "Field `TCLKPREPEN` reader - custom time for tCLK-PREPARE Enable"]
pub struct TCLKPREPEN_R(crate::FieldReader<bool, bool>);
impl TCLKPREPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCLKPREPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLKPREPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLKPREPEN` writer - custom time for tCLK-PREPARE Enable"]
pub struct TCLKPREPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPREPEN_W<'a> {
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
#[doc = "Field `PDEN` reader - Pull-Down Enable"]
pub struct PDEN_R(crate::FieldReader<bool, bool>);
impl PDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN` writer - Pull-Down Enable"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
#[doc = "Field `TDDL` reader - Turn Disable Data Lanes"]
pub struct TDDL_R(crate::FieldReader<bool, bool>);
impl TDDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDDL` writer - Turn Disable Data Lanes"]
pub struct TDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDDL_W<'a> {
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
#[doc = "Field `CDOFFDL` reader - Contention Detection OFF on Data Lanes"]
pub struct CDOFFDL_R(crate::FieldReader<bool, bool>);
impl CDOFFDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDOFFDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDOFFDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDOFFDL` writer - Contention Detection OFF on Data Lanes"]
pub struct CDOFFDL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFFDL_W<'a> {
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
#[doc = "Field `FTXSMDL` reader - Force in TX Stop Mode the Data Lanes"]
pub struct FTXSMDL_R(crate::FieldReader<bool, bool>);
impl FTXSMDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTXSMDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTXSMDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTXSMDL` writer - Force in TX Stop Mode the Data Lanes"]
pub struct FTXSMDL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMDL_W<'a> {
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
#[doc = "Field `FTXSMCL` reader - Force in TX Stop Mode the Clock Lane"]
pub struct FTXSMCL_R(crate::FieldReader<bool, bool>);
impl FTXSMCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTXSMCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTXSMCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTXSMCL` writer - Force in TX Stop Mode the Clock Lane"]
pub struct FTXSMCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMCL_W<'a> {
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
#[doc = "Field `HSIDL1` reader - Invert the High-Speed data signal on Data Lane 1"]
pub struct HSIDL1_R(crate::FieldReader<bool, bool>);
impl HSIDL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIDL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDL1` writer - Invert the High-Speed data signal on Data Lane 1"]
pub struct HSIDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL1_W<'a> {
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
#[doc = "Field `HSIDL0` reader - Invert the Hight-Speed data signal on Data Lane 0"]
pub struct HSIDL0_R(crate::FieldReader<bool, bool>);
impl HSIDL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIDL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDL0` writer - Invert the Hight-Speed data signal on Data Lane 0"]
pub struct HSIDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL0_W<'a> {
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
#[doc = "Field `HSICL` reader - Invert Hight-Speed data signal on Clock Lane"]
pub struct HSICL_R(crate::FieldReader<bool, bool>);
impl HSICL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSICL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSICL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSICL` writer - Invert Hight-Speed data signal on Clock Lane"]
pub struct HSICL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICL_W<'a> {
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
#[doc = "Field `SWDL1` reader - Swap Data Lane 1 pins"]
pub struct SWDL1_R(crate::FieldReader<bool, bool>);
impl SWDL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWDL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWDL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDL1` writer - Swap Data Lane 1 pins"]
pub struct SWDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL1_W<'a> {
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
#[doc = "Field `SWDL0` reader - Swap Data Lane 0 pins"]
pub struct SWDL0_R(crate::FieldReader<bool, bool>);
impl SWDL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWDL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWDL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDL0` writer - Swap Data Lane 0 pins"]
pub struct SWDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL0_W<'a> {
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
#[doc = "Field `SWCL` reader - Swap Clock Lane pins"]
pub struct SWCL_R(crate::FieldReader<bool, bool>);
impl SWCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWCL` writer - Swap Clock Lane pins"]
pub struct SWCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCL_W<'a> {
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
#[doc = "Field `UIX4` reader - Unit Interval multiplied by 4"]
pub struct UIX4_R(crate::FieldReader<u8, u8>);
impl UIX4_R {
    pub(crate) fn new(bits: u8) -> Self {
        UIX4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIX4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIX4` writer - Unit Interval multiplied by 4"]
pub struct UIX4_W<'a> {
    w: &'a mut W,
}
impl<'a> UIX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - custom time for tCLK-POST Enable"]
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - custom time for tLPX for Clock lane Enable"]
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - custom time for tHS-EXIT Enable"]
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - custom time for tLPX for Data lanes Enable"]
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - custom time for tHS-ZERO Enable"]
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - custom time for tHS-TRAIL Enable"]
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - custom time for tHS-PREPARE Enable"]
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - custom time for tCLK-ZERO Enable"]
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - custom time for tCLK-PREPARE Enable"]
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Turn Disable Data Lanes"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Contention Detection OFF on Data Lanes"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force in TX Stop Mode the Data Lanes"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force in TX Stop Mode the Clock Lane"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Invert the High-Speed data signal on Data Lane 1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invert the Hight-Speed data signal on Data Lane 0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Invert Hight-Speed data signal on Clock Lane"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Swap Data Lane 1 pins"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Swap Data Lane 0 pins"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Swap Clock Lane pins"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Unit Interval multiplied by 4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 27 - custom time for tCLK-POST Enable"]
    #[inline(always)]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W {
        TCLKPOSTEN_W { w: self }
    }
    #[doc = "Bit 26 - custom time for tLPX for Clock lane Enable"]
    #[inline(always)]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W {
        TLPXCEN_W { w: self }
    }
    #[doc = "Bit 25 - custom time for tHS-EXIT Enable"]
    #[inline(always)]
    pub fn thsexiten(&mut self) -> THSEXITEN_W {
        THSEXITEN_W { w: self }
    }
    #[doc = "Bit 24 - custom time for tLPX for Data lanes Enable"]
    #[inline(always)]
    pub fn tlpxden(&mut self) -> TLPXDEN_W {
        TLPXDEN_W { w: self }
    }
    #[doc = "Bit 23 - custom time for tHS-ZERO Enable"]
    #[inline(always)]
    pub fn thszeroen(&mut self) -> THSZEROEN_W {
        THSZEROEN_W { w: self }
    }
    #[doc = "Bit 22 - custom time for tHS-TRAIL Enable"]
    #[inline(always)]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W {
        THSTRAILEN_W { w: self }
    }
    #[doc = "Bit 21 - custom time for tHS-PREPARE Enable"]
    #[inline(always)]
    pub fn thsprepen(&mut self) -> THSPREPEN_W {
        THSPREPEN_W { w: self }
    }
    #[doc = "Bit 20 - custom time for tCLK-ZERO Enable"]
    #[inline(always)]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W {
        TCLKZEROEN_W { w: self }
    }
    #[doc = "Bit 19 - custom time for tCLK-PREPARE Enable"]
    #[inline(always)]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W {
        TCLKPREPEN_W { w: self }
    }
    #[doc = "Bit 18 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 16 - Turn Disable Data Lanes"]
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W {
        TDDL_W { w: self }
    }
    #[doc = "Bit 14 - Contention Detection OFF on Data Lanes"]
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W {
        CDOFFDL_W { w: self }
    }
    #[doc = "Bit 13 - Force in TX Stop Mode the Data Lanes"]
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W {
        FTXSMDL_W { w: self }
    }
    #[doc = "Bit 12 - Force in TX Stop Mode the Clock Lane"]
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W {
        FTXSMCL_W { w: self }
    }
    #[doc = "Bit 11 - Invert the High-Speed data signal on Data Lane 1"]
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W {
        HSIDL1_W { w: self }
    }
    #[doc = "Bit 10 - Invert the Hight-Speed data signal on Data Lane 0"]
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W {
        HSIDL0_W { w: self }
    }
    #[doc = "Bit 9 - Invert Hight-Speed data signal on Clock Lane"]
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W {
        HSICL_W { w: self }
    }
    #[doc = "Bit 8 - Swap Data Lane 1 pins"]
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W {
        SWDL1_W { w: self }
    }
    #[doc = "Bit 7 - Swap Data Lane 0 pins"]
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W {
        SWDL0_W { w: self }
    }
    #[doc = "Bit 6 - Swap Clock Lane pins"]
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W {
        SWCL_W { w: self }
    }
    #[doc = "Bits 0:5 - Unit Interval multiplied by 4"]
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W {
        UIX4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr1](index.html) module"]
pub struct WPCR1_SPEC;
impl crate::RegisterSpec for WPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr1::R](R) reader structure"]
impl crate::Readable for WPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr1::W](W) writer structure"]
impl crate::Writable for WPCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
