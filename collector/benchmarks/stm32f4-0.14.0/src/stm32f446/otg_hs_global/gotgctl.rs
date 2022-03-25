#[doc = "Register `GOTGCTL` reader"]
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGCTL` writer"]
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRQSCS` reader - Session request success"]
pub struct SRQSCS_R(crate::FieldReader<bool, bool>);
impl SRQSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQ` reader - Session request"]
pub struct SRQ_R(crate::FieldReader<bool, bool>);
impl SRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQ` writer - Session request"]
pub struct SRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQ_W<'a> {
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
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub struct HNGSCS_R(crate::FieldReader<bool, bool>);
impl HNGSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNGSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNGSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPRQ` reader - HNP request"]
pub struct HNPRQ_R(crate::FieldReader<bool, bool>);
impl HNPRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNPRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPRQ` writer - HNP request"]
pub struct HNPRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPRQ_W<'a> {
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
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub struct HSHNPEN_R(crate::FieldReader<bool, bool>);
impl HSHNPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSHNPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSHNPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub struct HSHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSHNPEN_W<'a> {
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
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub struct DHNPEN_R(crate::FieldReader<bool, bool>);
impl DHNPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DHNPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHNPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub struct DHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DHNPEN_W<'a> {
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
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub struct CIDSTS_R(crate::FieldReader<bool, bool>);
impl CIDSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIDSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub struct DBCT_R(crate::FieldReader<bool, bool>);
impl DBCT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASVLD` reader - A-session valid"]
pub struct ASVLD_R(crate::FieldReader<bool, bool>);
impl ASVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASVLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASVLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSVLD` reader - B-session valid"]
pub struct BSVLD_R(crate::FieldReader<bool, bool>);
impl BSVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSVLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSVLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W {
        SRQ_W { w: self }
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W {
        HNPRQ_W { w: self }
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W {
        HSHNPEN_W { w: self }
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W {
        DHNPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](index.html) module"]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgctl::R](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0800"]
impl crate::Resettable for GOTGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
