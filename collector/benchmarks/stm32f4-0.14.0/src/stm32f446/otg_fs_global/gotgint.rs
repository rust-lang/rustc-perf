#[doc = "Register `GOTGINT` reader"]
pub struct R(crate::R<GOTGINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGINT` writer"]
pub struct W(crate::W<GOTGINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGINT_SPEC>;
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
impl From<crate::W<GOTGINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEDET` reader - Session end detected"]
pub struct SEDET_R(crate::FieldReader<bool, bool>);
impl SEDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEDET` writer - Session end detected"]
pub struct SEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDET_W<'a> {
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
#[doc = "Field `SRSSCHG` reader - Session request success status change"]
pub struct SRSSCHG_R(crate::FieldReader<bool, bool>);
impl SRSSCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRSSCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRSSCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRSSCHG` writer - Session request success status change"]
pub struct SRSSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSSCHG_W<'a> {
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
#[doc = "Field `HNSSCHG` reader - Host negotiation success status change"]
pub struct HNSSCHG_R(crate::FieldReader<bool, bool>);
impl HNSSCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNSSCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNSSCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNSSCHG` writer - Host negotiation success status change"]
pub struct HNSSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> HNSSCHG_W<'a> {
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
#[doc = "Field `HNGDET` reader - Host negotiation detected"]
pub struct HNGDET_R(crate::FieldReader<bool, bool>);
impl HNGDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNGDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNGDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNGDET` writer - Host negotiation detected"]
pub struct HNGDET_W<'a> {
    w: &'a mut W,
}
impl<'a> HNGDET_W<'a> {
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
#[doc = "Field `ADTOCHG` reader - A-device timeout change"]
pub struct ADTOCHG_R(crate::FieldReader<bool, bool>);
impl ADTOCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADTOCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADTOCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADTOCHG` writer - A-device timeout change"]
pub struct ADTOCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTOCHG_W<'a> {
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
#[doc = "Field `DBCDNE` reader - Debounce done"]
pub struct DBCDNE_R(crate::FieldReader<bool, bool>);
impl DBCDNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCDNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCDNE` writer - Debounce done"]
pub struct DBCDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCDNE_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&mut self) -> SEDET_W {
        SEDET_W { w: self }
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&mut self) -> SRSSCHG_W {
        SRSSCHG_W { w: self }
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&mut self) -> HNSSCHG_W {
        HNSSCHG_W { w: self }
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&mut self) -> HNGDET_W {
        HNGDET_W { w: self }
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&mut self) -> ADTOCHG_W {
        ADTOCHG_W { w: self }
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&mut self) -> DBCDNE_W {
        DBCDNE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgint](index.html) module"]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgint::R](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgint::W](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
