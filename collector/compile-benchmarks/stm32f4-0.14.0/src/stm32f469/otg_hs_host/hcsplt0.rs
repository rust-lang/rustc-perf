#[doc = "Register `HCSPLT0` reader"]
pub struct R(crate::R<HCSPLT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCSPLT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCSPLT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCSPLT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCSPLT0` writer"]
pub struct W(crate::W<HCSPLT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCSPLT0_SPEC>;
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
impl From<crate::W<HCSPLT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCSPLT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTADDR` reader - Port address"]
pub struct PRTADDR_R(crate::FieldReader<u8, u8>);
impl PRTADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRTADDR` writer - Port address"]
pub struct PRTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `HUBADDR` reader - Hub address"]
pub struct HUBADDR_R(crate::FieldReader<u8, u8>);
impl HUBADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        HUBADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUBADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HUBADDR` writer - Hub address"]
pub struct HUBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HUBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | ((value as u32 & 0x7f) << 7);
        self.w
    }
}
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub struct XACTPOS_R(crate::FieldReader<u8, u8>);
impl XACTPOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        XACTPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XACTPOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub struct XACTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `COMPLSPLT` reader - Do complete split"]
pub struct COMPLSPLT_R(crate::FieldReader<bool, bool>);
impl COMPLSPLT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPLSPLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPLSPLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPLSPLT` writer - Do complete split"]
pub struct COMPLSPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLSPLT_W<'a> {
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
#[doc = "Field `SPLITEN` reader - Split enable"]
pub struct SPLITEN_R(crate::FieldReader<bool, bool>);
impl SPLITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPLITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLITEN` writer - Split enable"]
pub struct SPLITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W {
        PRTADDR_W { w: self }
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W {
        HUBADDR_W { w: self }
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W {
        XACTPOS_W { w: self }
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&mut self) -> COMPLSPLT_W {
        COMPLSPLT_W { w: self }
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&mut self) -> SPLITEN_W {
        SPLITEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-0 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt0](index.html) module"]
pub struct HCSPLT0_SPEC;
impl crate::RegisterSpec for HCSPLT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcsplt0::R](R) reader structure"]
impl crate::Readable for HCSPLT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcsplt0::W](W) writer structure"]
impl crate::Writable for HCSPLT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCSPLT0 to value 0"]
impl crate::Resettable for HCSPLT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
