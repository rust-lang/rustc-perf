#[doc = "Register `PTPTSCR` reader"]
pub struct R(crate::R<PTPTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTSCR` writer"]
pub struct W(crate::W<PTPTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSCR_SPEC>;
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
impl From<crate::W<PTPTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSE` reader - TSE"]
pub struct TSE_R(crate::FieldReader<bool, bool>);
impl TSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSE` writer - TSE"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
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
#[doc = "Field `TSFCU` reader - TSFCU"]
pub struct TSFCU_R(crate::FieldReader<bool, bool>);
impl TSFCU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSFCU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSFCU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSFCU` writer - TSFCU"]
pub struct TSFCU_W<'a> {
    w: &'a mut W,
}
impl<'a> TSFCU_W<'a> {
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
#[doc = "Field `TSPTPPSV2E` reader - TSPTPPSV2E"]
pub struct TSPTPPSV2E_R(crate::FieldReader<bool, bool>);
impl TSPTPPSV2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSPTPPSV2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSPTPPSV2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSPTPPSV2E` writer - TSPTPPSV2E"]
pub struct TSPTPPSV2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPTPPSV2E_W<'a> {
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
#[doc = "Field `TSSPTPOEFE` reader - TSSPTPOEFE"]
pub struct TSSPTPOEFE_R(crate::FieldReader<bool, bool>);
impl TSSPTPOEFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSPTPOEFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSPTPOEFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSPTPOEFE` writer - TSSPTPOEFE"]
pub struct TSSPTPOEFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSPTPOEFE_W<'a> {
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
#[doc = "Field `TSSIPV6FE` reader - TSSIPV6FE"]
pub struct TSSIPV6FE_R(crate::FieldReader<bool, bool>);
impl TSSIPV6FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSIPV6FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSIPV6FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSIPV6FE` writer - TSSIPV6FE"]
pub struct TSSIPV6FE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSIPV6FE_W<'a> {
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
#[doc = "Field `TSSIPV4FE` reader - TSSIPV4FE"]
pub struct TSSIPV4FE_R(crate::FieldReader<bool, bool>);
impl TSSIPV4FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSIPV4FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSIPV4FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSIPV4FE` writer - TSSIPV4FE"]
pub struct TSSIPV4FE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSIPV4FE_W<'a> {
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
#[doc = "Field `TSSEME` reader - TSSEME"]
pub struct TSSEME_R(crate::FieldReader<bool, bool>);
impl TSSEME_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSEME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSEME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSEME` writer - TSSEME"]
pub struct TSSEME_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSEME_W<'a> {
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
#[doc = "Field `TSSMRME` reader - TSSMRME"]
pub struct TSSMRME_R(crate::FieldReader<bool, bool>);
impl TSSMRME_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSMRME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSMRME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSMRME` writer - TSSMRME"]
pub struct TSSMRME_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSMRME_W<'a> {
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
#[doc = "Field `TSCNT` reader - TSCNT"]
pub struct TSCNT_R(crate::FieldReader<u8, u8>);
impl TSCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCNT` writer - TSCNT"]
pub struct TSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TSPFFMAE` reader - TSPFFMAE"]
pub struct TSPFFMAE_R(crate::FieldReader<bool, bool>);
impl TSPFFMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSPFFMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSPFFMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSPFFMAE` writer - TSPFFMAE"]
pub struct TSPFFMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPFFMAE_W<'a> {
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
#[doc = "Field `TSSTI` reader - TSSTI"]
pub struct TSSTI_R(crate::FieldReader<bool, bool>);
impl TSSTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSTI` writer - TSSTI"]
pub struct TSSTI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSTI_W<'a> {
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
#[doc = "Field `TSSTU` reader - TSSTU"]
pub struct TSSTU_R(crate::FieldReader<bool, bool>);
impl TSSTU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSTU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSTU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSTU` writer - TSSTU"]
pub struct TSSTU_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSTU_W<'a> {
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
#[doc = "Field `TSITE` reader - TSITE"]
pub struct TSITE_R(crate::FieldReader<bool, bool>);
impl TSITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSITE` writer - TSITE"]
pub struct TSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSITE_W<'a> {
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
#[doc = "Field `TTSARU` reader - TTSARU"]
pub struct TTSARU_R(crate::FieldReader<bool, bool>);
impl TTSARU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTSARU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTSARU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTSARU` writer - TTSARU"]
pub struct TTSARU_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSARU_W<'a> {
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
#[doc = "Field `TSSARFE` reader - TSSARFE"]
pub struct TSSARFE_R(crate::FieldReader<bool, bool>);
impl TSSARFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSARFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSARFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSARFE` writer - TSSARFE"]
pub struct TSSARFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSARFE_W<'a> {
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
#[doc = "Field `TSSSR` reader - TSSSR"]
pub struct TSSSR_R(crate::FieldReader<bool, bool>);
impl TSSSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSSR` writer - TSSSR"]
pub struct TSSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSSR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSFCU"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSPTPPSV2E"]
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> TSPTPPSV2E_R {
        TSPTPPSV2E_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TSSPTPOEFE"]
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TSSPTPOEFE_R {
        TSSPTPOEFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSSIPV6FE"]
    #[inline(always)]
    pub fn tssipv6fe(&self) -> TSSIPV6FE_R {
        TSSIPV6FE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TSSIPV4FE"]
    #[inline(always)]
    pub fn tssipv4fe(&self) -> TSSIPV4FE_R {
        TSSIPV4FE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TSSEME"]
    #[inline(always)]
    pub fn tsseme(&self) -> TSSEME_R {
        TSSEME_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TSSMRME"]
    #[inline(always)]
    pub fn tssmrme(&self) -> TSSMRME_R {
        TSSMRME_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - TSCNT"]
    #[inline(always)]
    pub fn tscnt(&self) -> TSCNT_R {
        TSCNT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - TSPFFMAE"]
    #[inline(always)]
    pub fn tspffmae(&self) -> TSPFFMAE_R {
        TSPFFMAE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TSSTI"]
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSSTU"]
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TSITE"]
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TTSARU"]
    #[inline(always)]
    pub fn ttsaru(&self) -> TTSARU_R {
        TTSARU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TSSARFE"]
    #[inline(always)]
    pub fn tssarfe(&self) -> TSSARFE_R {
        TSSARFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSSSR"]
    #[inline(always)]
    pub fn tsssr(&self) -> TSSSR_R {
        TSSSR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bit 1 - TSFCU"]
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TSFCU_W {
        TSFCU_W { w: self }
    }
    #[doc = "Bit 10 - TSPTPPSV2E"]
    #[inline(always)]
    pub fn tsptppsv2e(&mut self) -> TSPTPPSV2E_W {
        TSPTPPSV2E_W { w: self }
    }
    #[doc = "Bit 11 - TSSPTPOEFE"]
    #[inline(always)]
    pub fn tssptpoefe(&mut self) -> TSSPTPOEFE_W {
        TSSPTPOEFE_W { w: self }
    }
    #[doc = "Bit 12 - TSSIPV6FE"]
    #[inline(always)]
    pub fn tssipv6fe(&mut self) -> TSSIPV6FE_W {
        TSSIPV6FE_W { w: self }
    }
    #[doc = "Bit 13 - TSSIPV4FE"]
    #[inline(always)]
    pub fn tssipv4fe(&mut self) -> TSSIPV4FE_W {
        TSSIPV4FE_W { w: self }
    }
    #[doc = "Bit 14 - TSSEME"]
    #[inline(always)]
    pub fn tsseme(&mut self) -> TSSEME_W {
        TSSEME_W { w: self }
    }
    #[doc = "Bit 15 - TSSMRME"]
    #[inline(always)]
    pub fn tssmrme(&mut self) -> TSSMRME_W {
        TSSMRME_W { w: self }
    }
    #[doc = "Bits 16:17 - TSCNT"]
    #[inline(always)]
    pub fn tscnt(&mut self) -> TSCNT_W {
        TSCNT_W { w: self }
    }
    #[doc = "Bit 18 - TSPFFMAE"]
    #[inline(always)]
    pub fn tspffmae(&mut self) -> TSPFFMAE_W {
        TSPFFMAE_W { w: self }
    }
    #[doc = "Bit 2 - TSSTI"]
    #[inline(always)]
    pub fn tssti(&mut self) -> TSSTI_W {
        TSSTI_W { w: self }
    }
    #[doc = "Bit 3 - TSSTU"]
    #[inline(always)]
    pub fn tsstu(&mut self) -> TSSTU_W {
        TSSTU_W { w: self }
    }
    #[doc = "Bit 4 - TSITE"]
    #[inline(always)]
    pub fn tsite(&mut self) -> TSITE_W {
        TSITE_W { w: self }
    }
    #[doc = "Bit 5 - TTSARU"]
    #[inline(always)]
    pub fn ttsaru(&mut self) -> TTSARU_W {
        TTSARU_W { w: self }
    }
    #[doc = "Bit 8 - TSSARFE"]
    #[inline(always)]
    pub fn tssarfe(&mut self) -> TSSARFE_W {
        TSSARFE_W { w: self }
    }
    #[doc = "Bit 9 - TSSSR"]
    #[inline(always)]
    pub fn tsssr(&mut self) -> TSSSR_W {
        TSSSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptscr](index.html) module"]
pub struct PTPTSCR_SPEC;
impl crate::RegisterSpec for PTPTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptscr::R](R) reader structure"]
impl crate::Readable for PTPTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptscr::W](W) writer structure"]
impl crate::Writable for PTPTSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTSCR to value 0x2000"]
impl crate::Resettable for PTPTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
