#[doc = "Register `AHB2RSTR` reader"]
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RSTR` writer"]
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG FS module reset"]
pub type OTGFSRST_A = DCMIRST_A;
#[doc = "Field `OTGFSRST` reader - USB OTG FS module reset"]
pub type OTGFSRST_R = DCMIRST_R;
#[doc = "Field `OTGFSRST` writer - USB OTG FS module reset"]
pub struct OTGFSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGFSRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::RESET)
    }
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
#[doc = "Random number generator module reset"]
pub type RNGRST_A = DCMIRST_A;
#[doc = "Field `RNGRST` reader - Random number generator module reset"]
pub type RNGRST_R = DCMIRST_R;
#[doc = "Field `RNGRST` writer - Random number generator module reset"]
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RNGRST_A::RESET)
    }
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
#[doc = "Hash module reset"]
pub type HSAHRST_A = DCMIRST_A;
#[doc = "Field `HSAHRST` reader - Hash module reset"]
pub type HSAHRST_R = DCMIRST_R;
#[doc = "Field `HSAHRST` writer - Hash module reset"]
pub struct HSAHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSAHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSAHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSAHRST_A::RESET)
    }
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
#[doc = "Cryptographic module reset"]
pub type CRYPRST_A = DCMIRST_A;
#[doc = "Field `CRYPRST` reader - Cryptographic module reset"]
pub type CRYPRST_R = DCMIRST_R;
#[doc = "Field `CRYPRST` writer - Cryptographic module reset"]
pub struct CRYPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRYPRST_A::RESET)
    }
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
#[doc = "Camera interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMIRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<DCMIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIRST` reader - Camera interface reset"]
pub struct DCMIRST_R(crate::FieldReader<bool, DCMIRST_A>);
impl DCMIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMIRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCMIRST_A> {
        match self.bits {
            true => Some(DCMIRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == DCMIRST_A::RESET
    }
}
impl core::ops::Deref for DCMIRST_R {
    type Target = crate::FieldReader<bool, DCMIRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMIRST` writer - Camera interface reset"]
pub struct DCMIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMIRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCMIRST_A::RESET)
    }
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
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hash module reset"]
    #[inline(always)]
    pub fn hsahrst(&self) -> HSAHRST_R {
        HSAHRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cryptographic module reset"]
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W {
        OTGFSRST_W { w: self }
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W {
        RNGRST_W { w: self }
    }
    #[doc = "Bit 5 - Hash module reset"]
    #[inline(always)]
    pub fn hsahrst(&mut self) -> HSAHRST_W {
        HSAHRST_W { w: self }
    }
    #[doc = "Bit 4 - Cryptographic module reset"]
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W {
        CRYPRST_W { w: self }
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W {
        DCMIRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rstr::R](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
