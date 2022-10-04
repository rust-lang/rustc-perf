#[doc = "Register `AHB2LPENR` reader"]
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2LPENR` writer"]
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG FS clock enable during Sleep mode"]
pub type OTGFSLPEN_A = DCMILPEN_A;
#[doc = "Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode"]
pub type OTGFSLPEN_R = DCMILPEN_R;
#[doc = "Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode"]
pub struct OTGFSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGFSLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
#[doc = "Random number generator clock enable during Sleep mode"]
pub type RNGLPEN_A = DCMILPEN_A;
#[doc = "Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode"]
pub type RNGLPEN_R = DCMILPEN_R;
#[doc = "Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode"]
pub struct RNGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(RNGLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(RNGLPEN_A::ENABLEDINSLEEP)
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
#[doc = "Hash modules clock enable during Sleep mode"]
pub type HASHLPEN_A = DCMILPEN_A;
#[doc = "Field `HASHLPEN` reader - Hash modules clock enable during Sleep mode"]
pub type HASHLPEN_R = DCMILPEN_R;
#[doc = "Field `HASHLPEN` writer - Hash modules clock enable during Sleep mode"]
pub struct HASHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(HASHLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(HASHLPEN_A::ENABLEDINSLEEP)
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
#[doc = "Cryptography modules clock enable during Sleep mode"]
pub type CRYPLPEN_A = DCMILPEN_A;
#[doc = "Field `CRYPLPEN` reader - Cryptography modules clock enable during Sleep mode"]
pub type CRYPLPEN_R = DCMILPEN_R;
#[doc = "Field `CRYPLPEN` writer - Cryptography modules clock enable during Sleep mode"]
pub struct CRYPLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(CRYPLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(CRYPLPEN_A::ENABLEDINSLEEP)
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
#[doc = "Camera interface enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMILPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP = 1,
}
impl From<DCMILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMILPEN` reader - Camera interface enable during Sleep mode"]
pub struct DCMILPEN_R(crate::FieldReader<bool, DCMILPEN_A>);
impl DCMILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMILPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMILPEN_A {
        match self.bits {
            false => DCMILPEN_A::DISABLEDINSLEEP,
            true => DCMILPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        **self == DCMILPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        **self == DCMILPEN_A::ENABLEDINSLEEP
    }
}
impl core::ops::Deref for DCMILPEN_R {
    type Target = crate::FieldReader<bool, DCMILPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMILPEN` writer - Camera interface enable during Sleep mode"]
pub struct DCMILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DCMILPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DCMILPEN_A::ENABLEDINSLEEP)
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
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cryplpen(&self) -> CRYPLPEN_R {
        CRYPLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W {
        OTGFSLPEN_W { w: self }
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W {
        RNGLPEN_W { w: self }
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W {
        HASHLPEN_W { w: self }
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cryplpen(&mut self) -> CRYPLPEN_W {
        CRYPLPEN_W { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W {
        DCMILPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](index.html) module"]
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2lpenr::R](R) reader structure"]
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure"]
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0xf1"]
impl crate::Resettable for AHB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf1
    }
}
