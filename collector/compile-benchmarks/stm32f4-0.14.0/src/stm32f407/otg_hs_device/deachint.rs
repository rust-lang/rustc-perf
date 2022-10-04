#[doc = "Register `DEACHINT` reader"]
pub struct R(crate::R<DEACHINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEACHINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEACHINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEACHINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEACHINT` writer"]
pub struct W(crate::W<DEACHINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEACHINT_SPEC>;
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
impl From<crate::W<DEACHINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEACHINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEP1INT` reader - IN endpoint 1interrupt bit"]
pub struct IEP1INT_R(crate::FieldReader<bool, bool>);
impl IEP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEP1INT` writer - IN endpoint 1interrupt bit"]
pub struct IEP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> IEP1INT_W<'a> {
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
#[doc = "Field `OEP1INT` reader - OUT endpoint 1 interrupt bit"]
pub struct OEP1INT_R(crate::FieldReader<bool, bool>);
impl OEP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEP1INT` writer - OUT endpoint 1 interrupt bit"]
pub struct OEP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> OEP1INT_W<'a> {
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
impl R {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&mut self) -> IEP1INT_W {
        IEP1INT_W { w: self }
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&mut self) -> OEP1INT_W {
        OEP1INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device each endpoint interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deachint](index.html) module"]
pub struct DEACHINT_SPEC;
impl crate::RegisterSpec for DEACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deachint::R](R) reader structure"]
impl crate::Readable for DEACHINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deachint::W](W) writer structure"]
impl crate::Writable for DEACHINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DEACHINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
