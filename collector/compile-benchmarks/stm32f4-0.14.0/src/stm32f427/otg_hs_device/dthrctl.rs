#[doc = "Register `DTHRCTL` reader"]
pub struct R(crate::R<DTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTHRCTL` writer"]
pub struct W(crate::W<DTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTHRCTL_SPEC>;
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
impl From<crate::W<DTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONISOTHREN` reader - Nonisochronous IN endpoints threshold enable"]
pub struct NONISOTHREN_R(crate::FieldReader<bool, bool>);
impl NONISOTHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NONISOTHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NONISOTHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NONISOTHREN` writer - Nonisochronous IN endpoints threshold enable"]
pub struct NONISOTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> NONISOTHREN_W<'a> {
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
#[doc = "Field `ISOTHREN` reader - ISO IN endpoint threshold enable"]
pub struct ISOTHREN_R(crate::FieldReader<bool, bool>);
impl ISOTHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOTHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOTHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOTHREN` writer - ISO IN endpoint threshold enable"]
pub struct ISOTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOTHREN_W<'a> {
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
#[doc = "Field `TXTHRLEN` reader - Transmit threshold length"]
pub struct TXTHRLEN_R(crate::FieldReader<u16, u16>);
impl TXTHRLEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXTHRLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTHRLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTHRLEN` writer - Transmit threshold length"]
pub struct TXTHRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHRLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | ((value as u32 & 0x01ff) << 2);
        self.w
    }
}
#[doc = "Field `RXTHREN` reader - Receive threshold enable"]
pub struct RXTHREN_R(crate::FieldReader<bool, bool>);
impl RXTHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTHREN` writer - Receive threshold enable"]
pub struct RXTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHREN_W<'a> {
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
#[doc = "Field `RXTHRLEN` reader - Receive threshold length"]
pub struct RXTHRLEN_R(crate::FieldReader<u16, u16>);
impl RXTHRLEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXTHRLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTHRLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTHRLEN` writer - Receive threshold length"]
pub struct RXTHRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHRLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 17)) | ((value as u32 & 0x01ff) << 17);
        self.w
    }
}
#[doc = "Field `ARPEN` reader - Arbiter parking enable"]
pub struct ARPEN_R(crate::FieldReader<bool, bool>);
impl ARPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARPEN` writer - Arbiter parking enable"]
pub struct ARPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPEN_W<'a> {
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
    #[doc = "Bit 0 - Nonisochronous IN endpoints threshold enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ISO IN endpoint threshold enable"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:10 - Transmit threshold length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Receive threshold enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:25 - Receive threshold length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter parking enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Nonisochronous IN endpoints threshold enable"]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W {
        NONISOTHREN_W { w: self }
    }
    #[doc = "Bit 1 - ISO IN endpoint threshold enable"]
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W {
        ISOTHREN_W { w: self }
    }
    #[doc = "Bits 2:10 - Transmit threshold length"]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W {
        TXTHRLEN_W { w: self }
    }
    #[doc = "Bit 16 - Receive threshold enable"]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W {
        RXTHREN_W { w: self }
    }
    #[doc = "Bits 17:25 - Receive threshold length"]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W {
        RXTHRLEN_W { w: self }
    }
    #[doc = "Bit 27 - Arbiter parking enable"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W {
        ARPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS Device threshold control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dthrctl](index.html) module"]
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dthrctl::R](R) reader structure"]
impl crate::Readable for DTHRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dthrctl::W](W) writer structure"]
impl crate::Writable for DTHRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTHRCTL to value 0"]
impl crate::Resettable for DTHRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
