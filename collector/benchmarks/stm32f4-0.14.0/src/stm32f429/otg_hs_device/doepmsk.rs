#[doc = "Register `DOEPMSK` reader"]
pub struct R(crate::R<DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPMSK` writer"]
pub struct W(crate::W<DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPMSK_SPEC>;
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
impl From<crate::W<DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub struct XFRCM_R(crate::FieldReader<bool, bool>);
impl XFRCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub struct XFRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRCM_W<'a> {
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
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub struct EPDM_R(crate::FieldReader<bool, bool>);
impl EPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub struct EPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDM_W<'a> {
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
#[doc = "Field `STUPM` reader - SETUP phase done mask"]
pub struct STUPM_R(crate::FieldReader<bool, bool>);
impl STUPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STUPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUPM` writer - SETUP phase done mask"]
pub struct STUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> STUPM_W<'a> {
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
#[doc = "Field `OTEPDM` reader - OUT token received when endpoint disabled mask"]
pub struct OTEPDM_R(crate::FieldReader<bool, bool>);
impl OTEPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTEPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTEPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTEPDM` writer - OUT token received when endpoint disabled mask"]
pub struct OTEPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> OTEPDM_W<'a> {
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
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received mask"]
pub struct B2BSTUP_R(crate::FieldReader<bool, bool>);
impl B2BSTUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2BSTUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2BSTUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received mask"]
pub struct B2BSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> B2BSTUP_W<'a> {
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
#[doc = "Field `OPEM` reader - OUT packet error mask"]
pub struct OPEM_R(crate::FieldReader<bool, bool>);
impl OPEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPEM` writer - OUT packet error mask"]
pub struct OPEM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPEM_W<'a> {
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
#[doc = "Field `BOIM` reader - BNA interrupt mask"]
pub struct BOIM_R(crate::FieldReader<bool, bool>);
impl BOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOIM` writer - BNA interrupt mask"]
pub struct BOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOIM_W<'a> {
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
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn opem(&self) -> OPEM_R {
        OPEM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn boim(&self) -> BOIM_R {
        BOIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W {
        EPDM_W { w: self }
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W {
        STUPM_W { w: self }
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W {
        OTEPDM_W { w: self }
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W {
        B2BSTUP_W { w: self }
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn opem(&mut self) -> OPEM_W {
        OPEM_W { w: self }
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn boim(&mut self) -> BOIM_W {
        BOIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device OUT endpoint common interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](index.html) module"]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepmsk::R](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
