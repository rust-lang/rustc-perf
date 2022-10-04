#[doc = "Register `DOEPINT6` reader"]
pub struct R(crate::R<DOEPINT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT6` writer"]
pub struct W(crate::W<DOEPINT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT6_SPEC>;
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
impl From<crate::W<DOEPINT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub struct XFRC_R(crate::FieldReader<bool, bool>);
impl XFRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub struct XFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRC_W<'a> {
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
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub struct EPDISD_R(crate::FieldReader<bool, bool>);
impl EPDISD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDISD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDISD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub struct EPDISD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISD_W<'a> {
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
#[doc = "Field `STUP` reader - SETUP phase done"]
pub struct STUP_R(crate::FieldReader<bool, bool>);
impl STUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUP` writer - SETUP phase done"]
pub struct STUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STUP_W<'a> {
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
#[doc = "Field `OTEPDIS` reader - OUT token received when endpoint disabled"]
pub struct OTEPDIS_R(crate::FieldReader<bool, bool>);
impl OTEPDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTEPDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTEPDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTEPDIS` writer - OUT token received when endpoint disabled"]
pub struct OTEPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTEPDIS_W<'a> {
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
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
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
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
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
#[doc = "Field `NYET` reader - NYET interrupt"]
pub struct NYET_R(crate::FieldReader<bool, bool>);
impl NYET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYET` writer - NYET interrupt"]
pub struct NYET_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W { w: self }
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W {
        STUP_W { w: self }
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W {
        OTEPDIS_W { w: self }
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W {
        B2BSTUP_W { w: self }
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W {
        NYET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device endpoint-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint6](index.html) module"]
pub struct DOEPINT6_SPEC;
impl crate::RegisterSpec for DOEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint6::R](R) reader structure"]
impl crate::Readable for DOEPINT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint6::W](W) writer structure"]
impl crate::Writable for DOEPINT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT6 to value 0"]
impl crate::Resettable for DOEPINT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
