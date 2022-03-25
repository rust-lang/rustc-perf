#[doc = "Register `CCER` reader"]
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCER` writer"]
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC4NP` reader - Capture/Compare 4 output Polarity"]
pub struct CC4NP_R(crate::FieldReader<bool, bool>);
impl CC4NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC4NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4NP` writer - Capture/Compare 4 output Polarity"]
pub struct CC4NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4NP_W<'a> {
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
#[doc = "Field `CC4P` reader - Capture/Compare 3 output Polarity"]
pub struct CC4P_R(crate::FieldReader<bool, bool>);
impl CC4P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC4P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4P` writer - Capture/Compare 3 output Polarity"]
pub struct CC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4P_W<'a> {
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
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable"]
pub struct CC4E_R(crate::FieldReader<bool, bool>);
impl CC4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC4E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable"]
pub struct CC4E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4E_W<'a> {
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
#[doc = "Field `CC3NP` reader - Capture/Compare 3 output Polarity"]
pub struct CC3NP_R(crate::FieldReader<bool, bool>);
impl CC3NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC3NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3NP` writer - Capture/Compare 3 output Polarity"]
pub struct CC3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3NP_W<'a> {
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
#[doc = "Field `CC3P` reader - Capture/Compare 3 output Polarity"]
pub struct CC3P_R(crate::FieldReader<bool, bool>);
impl CC3P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC3P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3P` writer - Capture/Compare 3 output Polarity"]
pub struct CC3P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3P_W<'a> {
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
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable"]
pub struct CC3E_R(crate::FieldReader<bool, bool>);
impl CC3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable"]
pub struct CC3E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3E_W<'a> {
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
#[doc = "Field `CC2NP` reader - Capture/Compare 2 output Polarity"]
pub struct CC2NP_R(crate::FieldReader<bool, bool>);
impl CC2NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2NP` writer - Capture/Compare 2 output Polarity"]
pub struct CC2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2NP_W<'a> {
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
#[doc = "Field `CC2P` reader - Capture/Compare 2 output Polarity"]
pub struct CC2P_R(crate::FieldReader<bool, bool>);
impl CC2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2P` writer - Capture/Compare 2 output Polarity"]
pub struct CC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2P_W<'a> {
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
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable"]
pub struct CC2E_R(crate::FieldReader<bool, bool>);
impl CC2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable"]
pub struct CC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2E_W<'a> {
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
#[doc = "Field `CC1NP` reader - Capture/Compare 1 output Polarity"]
pub struct CC1NP_R(crate::FieldReader<bool, bool>);
impl CC1NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1NP` writer - Capture/Compare 1 output Polarity"]
pub struct CC1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NP_W<'a> {
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
#[doc = "Field `CC1P` reader - Capture/Compare 1 output Polarity"]
pub struct CC1P_R(crate::FieldReader<bool, bool>);
impl CC1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1P` writer - Capture/Compare 1 output Polarity"]
pub struct CC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1P_W<'a> {
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
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable"]
pub struct CC1E_R(crate::FieldReader<bool, bool>);
impl CC1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable"]
pub struct CC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1E_W<'a> {
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
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W {
        CC4NP_W { w: self }
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W {
        CC4P_W { w: self }
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W {
        CC4E_W { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W {
        CC3NP_W { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W {
        CC3P_W { w: self }
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W {
        CC3E_W { w: self }
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W {
        CC2NP_W { w: self }
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W {
        CC2E_W { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W {
        CC1NP_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W { w: self }
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W {
        CC1E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](index.html) module"]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccer::R](R) reader structure"]
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccer::W](W) writer structure"]
impl crate::Writable for CCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
