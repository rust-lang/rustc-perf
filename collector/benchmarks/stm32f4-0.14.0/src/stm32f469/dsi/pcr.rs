#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCRXE` reader - CRC Reception Enable"]
pub struct CRCRXE_R(crate::FieldReader<bool, bool>);
impl CRCRXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCRXE` writer - CRC Reception Enable"]
pub struct CRCRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRXE_W<'a> {
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
#[doc = "Field `ECCRXE` reader - ECC Reception Enable"]
pub struct ECCRXE_R(crate::FieldReader<bool, bool>);
impl ECCRXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCRXE` writer - ECC Reception Enable"]
pub struct ECCRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCRXE_W<'a> {
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
#[doc = "Field `BTAE` reader - Bus Turn Around Enable"]
pub struct BTAE_R(crate::FieldReader<bool, bool>);
impl BTAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTAE` writer - Bus Turn Around Enable"]
pub struct BTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTAE_W<'a> {
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
#[doc = "Field `ETRXE` reader - EoTp Reception Enable"]
pub struct ETRXE_R(crate::FieldReader<bool, bool>);
impl ETRXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRXE` writer - EoTp Reception Enable"]
pub struct ETRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRXE_W<'a> {
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
#[doc = "Field `ETTXE` reader - EoTp Transmission Enable"]
pub struct ETTXE_R(crate::FieldReader<bool, bool>);
impl ETTXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETTXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETTXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETTXE` writer - EoTp Transmission Enable"]
pub struct ETTXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETTXE_W<'a> {
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
    #[doc = "Bit 4 - CRC Reception Enable"]
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC Reception Enable"]
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Turn Around Enable"]
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - EoTp Reception Enable"]
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EoTp Transmission Enable"]
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC Reception Enable"]
    #[inline(always)]
    pub fn crcrxe(&mut self) -> CRCRXE_W {
        CRCRXE_W { w: self }
    }
    #[doc = "Bit 3 - ECC Reception Enable"]
    #[inline(always)]
    pub fn eccrxe(&mut self) -> ECCRXE_W {
        ECCRXE_W { w: self }
    }
    #[doc = "Bit 2 - Bus Turn Around Enable"]
    #[inline(always)]
    pub fn btae(&mut self) -> BTAE_W {
        BTAE_W { w: self }
    }
    #[doc = "Bit 1 - EoTp Reception Enable"]
    #[inline(always)]
    pub fn etrxe(&mut self) -> ETRXE_W {
        ETRXE_W { w: self }
    }
    #[doc = "Bit 0 - EoTp Transmission Enable"]
    #[inline(always)]
    pub fn ettxe(&mut self) -> ETTXE_W {
        ETTXE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Protocol Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
