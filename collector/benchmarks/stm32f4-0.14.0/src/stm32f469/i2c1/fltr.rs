#[doc = "Register `FLTR` reader"]
pub struct R(crate::R<FLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTR` writer"]
pub struct W(crate::W<FLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTR_SPEC>;
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
impl From<crate::W<FLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Digital noise filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DNF_A {
    #[doc = "0: Digital filter disabled"]
    NOFILTER = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    FILTER1 = 1,
    #[doc = "2: Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    FILTER2 = 2,
    #[doc = "3: Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    FILTER3 = 3,
    #[doc = "4: Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    FILTER4 = 4,
    #[doc = "5: Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    FILTER5 = 5,
    #[doc = "6: Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    FILTER6 = 6,
    #[doc = "7: Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    FILTER7 = 7,
    #[doc = "8: Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    FILTER8 = 8,
    #[doc = "9: Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    FILTER9 = 9,
    #[doc = "10: Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    FILTER10 = 10,
    #[doc = "11: Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    FILTER11 = 11,
    #[doc = "12: Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    FILTER12 = 12,
    #[doc = "13: Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    FILTER13 = 13,
    #[doc = "14: Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    FILTER14 = 14,
    #[doc = "15: Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    FILTER15 = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DNF` reader - Digital noise filter"]
pub struct DNF_R(crate::FieldReader<u8, DNF_A>);
impl DNF_R {
    pub(crate) fn new(bits: u8) -> Self {
        DNF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNF_A {
        match self.bits {
            0 => DNF_A::NOFILTER,
            1 => DNF_A::FILTER1,
            2 => DNF_A::FILTER2,
            3 => DNF_A::FILTER3,
            4 => DNF_A::FILTER4,
            5 => DNF_A::FILTER5,
            6 => DNF_A::FILTER6,
            7 => DNF_A::FILTER7,
            8 => DNF_A::FILTER8,
            9 => DNF_A::FILTER9,
            10 => DNF_A::FILTER10,
            11 => DNF_A::FILTER11,
            12 => DNF_A::FILTER12,
            13 => DNF_A::FILTER13,
            14 => DNF_A::FILTER14,
            15 => DNF_A::FILTER15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        **self == DNF_A::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FILTER1`"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        **self == DNF_A::FILTER1
    }
    #[doc = "Checks if the value of the field is `FILTER2`"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        **self == DNF_A::FILTER2
    }
    #[doc = "Checks if the value of the field is `FILTER3`"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        **self == DNF_A::FILTER3
    }
    #[doc = "Checks if the value of the field is `FILTER4`"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        **self == DNF_A::FILTER4
    }
    #[doc = "Checks if the value of the field is `FILTER5`"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        **self == DNF_A::FILTER5
    }
    #[doc = "Checks if the value of the field is `FILTER6`"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        **self == DNF_A::FILTER6
    }
    #[doc = "Checks if the value of the field is `FILTER7`"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        **self == DNF_A::FILTER7
    }
    #[doc = "Checks if the value of the field is `FILTER8`"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        **self == DNF_A::FILTER8
    }
    #[doc = "Checks if the value of the field is `FILTER9`"]
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        **self == DNF_A::FILTER9
    }
    #[doc = "Checks if the value of the field is `FILTER10`"]
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        **self == DNF_A::FILTER10
    }
    #[doc = "Checks if the value of the field is `FILTER11`"]
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        **self == DNF_A::FILTER11
    }
    #[doc = "Checks if the value of the field is `FILTER12`"]
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        **self == DNF_A::FILTER12
    }
    #[doc = "Checks if the value of the field is `FILTER13`"]
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        **self == DNF_A::FILTER13
    }
    #[doc = "Checks if the value of the field is `FILTER14`"]
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        **self == DNF_A::FILTER14
    }
    #[doc = "Checks if the value of the field is `FILTER15`"]
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        **self == DNF_A::FILTER15
    }
}
impl core::ops::Deref for DNF_R {
    type Target = crate::FieldReader<u8, DNF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNF` writer - Digital noise filter"]
pub struct DNF_W<'a> {
    w: &'a mut W,
}
impl<'a> DNF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(DNF_A::NOFILTER)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DNF_A::FILTER1)
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DNF_A::FILTER2)
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DNF_A::FILTER3)
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DNF_A::FILTER4)
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DNF_A::FILTER5)
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DNF_A::FILTER6)
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DNF_A::FILTER7)
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DNF_A::FILTER8)
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline(always)]
    pub fn filter9(self) -> &'a mut W {
        self.variant(DNF_A::FILTER9)
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline(always)]
    pub fn filter10(self) -> &'a mut W {
        self.variant(DNF_A::FILTER10)
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline(always)]
    pub fn filter11(self) -> &'a mut W {
        self.variant(DNF_A::FILTER11)
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline(always)]
    pub fn filter12(self) -> &'a mut W {
        self.variant(DNF_A::FILTER12)
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline(always)]
    pub fn filter13(self) -> &'a mut W {
        self.variant(DNF_A::FILTER13)
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline(always)]
    pub fn filter14(self) -> &'a mut W {
        self.variant(DNF_A::FILTER14)
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline(always)]
    pub fn filter15(self) -> &'a mut W {
        self.variant(DNF_A::FILTER15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Analog noise filter OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANOFF_A {
    #[doc = "0: Analog noise filter enabled"]
    ENABLED = 0,
    #[doc = "1: Analog noise filter disabled"]
    DISABLED = 1,
}
impl From<ANOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANOFF` reader - Analog noise filter OFF"]
pub struct ANOFF_R(crate::FieldReader<bool, ANOFF_A>);
impl ANOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANOFF_A {
        match self.bits {
            false => ANOFF_A::ENABLED,
            true => ANOFF_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ANOFF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ANOFF_A::DISABLED
    }
}
impl core::ops::Deref for ANOFF_R {
    type Target = crate::FieldReader<bool, ANOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANOFF` writer - Analog noise filter OFF"]
pub struct ANOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ANOFF_A::ENABLED)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ANOFF_A::DISABLED)
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
impl R {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W {
        DNF_W { w: self }
    }
    #[doc = "Bit 4 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anoff(&mut self) -> ANOFF_W {
        ANOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C FLTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltr](index.html) module"]
pub struct FLTR_SPEC;
impl crate::RegisterSpec for FLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltr::R](R) reader structure"]
impl crate::Readable for FLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltr::W](W) writer structure"]
impl crate::Writable for FLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTR to value 0"]
impl crate::Resettable for FLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
