#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSR` writer"]
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW2` reader - Lowest priority flag for mailbox 2"]
pub struct LOW2_R(crate::FieldReader<bool, bool>);
impl LOW2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOW2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW1` reader - Lowest priority flag for mailbox 1"]
pub struct LOW1_R(crate::FieldReader<bool, bool>);
impl LOW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW0` reader - Lowest priority flag for mailbox 0"]
pub struct LOW0_R(crate::FieldReader<bool, bool>);
impl LOW0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TME2` reader - Lowest priority flag for mailbox 2"]
pub struct TME2_R(crate::FieldReader<bool, bool>);
impl TME2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TME2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TME2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TME1` reader - Lowest priority flag for mailbox 1"]
pub struct TME1_R(crate::FieldReader<bool, bool>);
impl TME1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TME1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TME1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TME0` reader - Lowest priority flag for mailbox 0"]
pub struct TME0_R(crate::FieldReader<bool, bool>);
impl TME0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TME0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TME0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODE` reader - CODE"]
pub struct CODE_R(crate::FieldReader<u8, u8>);
impl CODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRQ2` reader - ABRQ2"]
pub struct ABRQ2_R(crate::FieldReader<bool, bool>);
impl ABRQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRQ2` writer - ABRQ2"]
pub struct ABRQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRQ2_W<'a> {
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
#[doc = "Field `TERR2` reader - TERR2"]
pub struct TERR2_R(crate::FieldReader<bool, bool>);
impl TERR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR2` writer - TERR2"]
pub struct TERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR2_W<'a> {
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
#[doc = "Field `ALST2` reader - ALST2"]
pub struct ALST2_R(crate::FieldReader<bool, bool>);
impl ALST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALST2` writer - ALST2"]
pub struct ALST2_W<'a> {
    w: &'a mut W,
}
impl<'a> ALST2_W<'a> {
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
#[doc = "Field `TXOK2` reader - TXOK2"]
pub struct TXOK2_R(crate::FieldReader<bool, bool>);
impl TXOK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOK2` writer - TXOK2"]
pub struct TXOK2_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK2_W<'a> {
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
#[doc = "Field `RQCP2` reader - RQCP2"]
pub struct RQCP2_R(crate::FieldReader<bool, bool>);
impl RQCP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RQCP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQCP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQCP2` writer - RQCP2"]
pub struct RQCP2_W<'a> {
    w: &'a mut W,
}
impl<'a> RQCP2_W<'a> {
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
#[doc = "Field `ABRQ1` reader - ABRQ1"]
pub struct ABRQ1_R(crate::FieldReader<bool, bool>);
impl ABRQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRQ1` writer - ABRQ1"]
pub struct ABRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRQ1_W<'a> {
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
#[doc = "Field `TERR1` reader - TERR1"]
pub struct TERR1_R(crate::FieldReader<bool, bool>);
impl TERR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR1` writer - TERR1"]
pub struct TERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR1_W<'a> {
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
#[doc = "Field `ALST1` reader - ALST1"]
pub struct ALST1_R(crate::FieldReader<bool, bool>);
impl ALST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALST1` writer - ALST1"]
pub struct ALST1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALST1_W<'a> {
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
#[doc = "Field `TXOK1` reader - TXOK1"]
pub struct TXOK1_R(crate::FieldReader<bool, bool>);
impl TXOK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOK1` writer - TXOK1"]
pub struct TXOK1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK1_W<'a> {
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
#[doc = "Field `RQCP1` reader - RQCP1"]
pub struct RQCP1_R(crate::FieldReader<bool, bool>);
impl RQCP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RQCP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQCP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQCP1` writer - RQCP1"]
pub struct RQCP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RQCP1_W<'a> {
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
#[doc = "Field `ABRQ0` reader - ABRQ0"]
pub struct ABRQ0_R(crate::FieldReader<bool, bool>);
impl ABRQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRQ0` writer - ABRQ0"]
pub struct ABRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRQ0_W<'a> {
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
#[doc = "Field `TERR0` reader - TERR0"]
pub struct TERR0_R(crate::FieldReader<bool, bool>);
impl TERR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR0` writer - TERR0"]
pub struct TERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR0_W<'a> {
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
#[doc = "Field `ALST0` reader - ALST0"]
pub struct ALST0_R(crate::FieldReader<bool, bool>);
impl ALST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALST0` writer - ALST0"]
pub struct ALST0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALST0_W<'a> {
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
#[doc = "Field `TXOK0` reader - TXOK0"]
pub struct TXOK0_R(crate::FieldReader<bool, bool>);
impl TXOK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOK0` writer - TXOK0"]
pub struct TXOK0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK0_W<'a> {
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
#[doc = "Field `RQCP0` reader - RQCP0"]
pub struct RQCP0_R(crate::FieldReader<bool, bool>);
impl RQCP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RQCP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQCP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQCP0` writer - RQCP0"]
pub struct RQCP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RQCP0_W<'a> {
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
    #[doc = "Bit 31 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn low2(&self) -> LOW2_R {
        LOW2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn low1(&self) -> LOW1_R {
        LOW1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn low0(&self) -> LOW0_R {
        LOW0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - CODE"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23 - ABRQ2"]
    #[inline(always)]
    pub fn abrq2(&self) -> ABRQ2_R {
        ABRQ2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TERR2"]
    #[inline(always)]
    pub fn terr2(&self) -> TERR2_R {
        TERR2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ALST2"]
    #[inline(always)]
    pub fn alst2(&self) -> ALST2_R {
        ALST2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TXOK2"]
    #[inline(always)]
    pub fn txok2(&self) -> TXOK2_R {
        TXOK2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RQCP2"]
    #[inline(always)]
    pub fn rqcp2(&self) -> RQCP2_R {
        RQCP2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ABRQ1"]
    #[inline(always)]
    pub fn abrq1(&self) -> ABRQ1_R {
        ABRQ1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TERR1"]
    #[inline(always)]
    pub fn terr1(&self) -> TERR1_R {
        TERR1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ALST1"]
    #[inline(always)]
    pub fn alst1(&self) -> ALST1_R {
        ALST1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TXOK1"]
    #[inline(always)]
    pub fn txok1(&self) -> TXOK1_R {
        TXOK1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RQCP1"]
    #[inline(always)]
    pub fn rqcp1(&self) -> RQCP1_R {
        RQCP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ABRQ0"]
    #[inline(always)]
    pub fn abrq0(&self) -> ABRQ0_R {
        ABRQ0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TERR0"]
    #[inline(always)]
    pub fn terr0(&self) -> TERR0_R {
        TERR0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ALST0"]
    #[inline(always)]
    pub fn alst0(&self) -> ALST0_R {
        ALST0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXOK0"]
    #[inline(always)]
    pub fn txok0(&self) -> TXOK0_R {
        TXOK0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RQCP0"]
    #[inline(always)]
    pub fn rqcp0(&self) -> RQCP0_R {
        RQCP0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - ABRQ2"]
    #[inline(always)]
    pub fn abrq2(&mut self) -> ABRQ2_W {
        ABRQ2_W { w: self }
    }
    #[doc = "Bit 19 - TERR2"]
    #[inline(always)]
    pub fn terr2(&mut self) -> TERR2_W {
        TERR2_W { w: self }
    }
    #[doc = "Bit 18 - ALST2"]
    #[inline(always)]
    pub fn alst2(&mut self) -> ALST2_W {
        ALST2_W { w: self }
    }
    #[doc = "Bit 17 - TXOK2"]
    #[inline(always)]
    pub fn txok2(&mut self) -> TXOK2_W {
        TXOK2_W { w: self }
    }
    #[doc = "Bit 16 - RQCP2"]
    #[inline(always)]
    pub fn rqcp2(&mut self) -> RQCP2_W {
        RQCP2_W { w: self }
    }
    #[doc = "Bit 15 - ABRQ1"]
    #[inline(always)]
    pub fn abrq1(&mut self) -> ABRQ1_W {
        ABRQ1_W { w: self }
    }
    #[doc = "Bit 11 - TERR1"]
    #[inline(always)]
    pub fn terr1(&mut self) -> TERR1_W {
        TERR1_W { w: self }
    }
    #[doc = "Bit 10 - ALST1"]
    #[inline(always)]
    pub fn alst1(&mut self) -> ALST1_W {
        ALST1_W { w: self }
    }
    #[doc = "Bit 9 - TXOK1"]
    #[inline(always)]
    pub fn txok1(&mut self) -> TXOK1_W {
        TXOK1_W { w: self }
    }
    #[doc = "Bit 8 - RQCP1"]
    #[inline(always)]
    pub fn rqcp1(&mut self) -> RQCP1_W {
        RQCP1_W { w: self }
    }
    #[doc = "Bit 7 - ABRQ0"]
    #[inline(always)]
    pub fn abrq0(&mut self) -> ABRQ0_W {
        ABRQ0_W { w: self }
    }
    #[doc = "Bit 3 - TERR0"]
    #[inline(always)]
    pub fn terr0(&mut self) -> TERR0_W {
        TERR0_W { w: self }
    }
    #[doc = "Bit 2 - ALST0"]
    #[inline(always)]
    pub fn alst0(&mut self) -> ALST0_W {
        ALST0_W { w: self }
    }
    #[doc = "Bit 1 - TXOK0"]
    #[inline(always)]
    pub fn txok0(&mut self) -> TXOK0_W {
        TXOK0_W { w: self }
    }
    #[doc = "Bit 0 - RQCP0"]
    #[inline(always)]
    pub fn rqcp0(&mut self) -> RQCP0_W {
        RQCP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "transmit status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsr::W](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSR to value 0x1c00_0000"]
impl crate::Resettable for TSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c00_0000
    }
}
