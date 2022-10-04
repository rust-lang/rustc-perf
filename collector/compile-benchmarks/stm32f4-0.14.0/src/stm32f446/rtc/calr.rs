#[doc = "Register `CALR` reader"]
pub struct R(crate::R<CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALR` writer"]
pub struct W(crate::W<CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALR_SPEC>;
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
impl From<crate::W<CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Increase frequency of RTC by 488.5 ppm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALP_A {
    #[doc = "0: No RTCCLK pulses are added"]
    NOCHANGE = 0,
    #[doc = "1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
    INCREASEFREQ = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm"]
pub struct CALP_R(crate::FieldReader<bool, CALP_A>);
impl CALP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::NOCHANGE,
            true => CALP_A::INCREASEFREQ,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == CALP_A::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `INCREASEFREQ`"]
    #[inline(always)]
    pub fn is_increase_freq(&self) -> bool {
        **self == CALP_A::INCREASEFREQ
    }
}
impl core::ops::Deref for CALP_R {
    type Target = crate::FieldReader<bool, CALP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm"]
pub struct CALP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No RTCCLK pulses are added"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CALP_A::NOCHANGE)
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
    #[inline(always)]
    pub fn increase_freq(self) -> &'a mut W {
        self.variant(CALP_A::INCREASEFREQ)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Use an 8-second calibration cycle period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALW8_A {
    #[doc = "1: When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
    EIGHT_SECOND = 1,
}
impl From<CALW8_A> for bool {
    #[inline(always)]
    fn from(variant: CALW8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period"]
pub struct CALW8_R(crate::FieldReader<bool, CALW8_A>);
impl CALW8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALW8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALW8_A> {
        match self.bits {
            true => Some(CALW8_A::EIGHT_SECOND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EIGHT_SECOND`"]
    #[inline(always)]
    pub fn is_eight_second(&self) -> bool {
        **self == CALW8_A::EIGHT_SECOND
    }
}
impl core::ops::Deref for CALW8_R {
    type Target = crate::FieldReader<bool, CALW8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period"]
pub struct CALW8_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALW8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
    #[inline(always)]
    pub fn eight_second(self) -> &'a mut W {
        self.variant(CALW8_A::EIGHT_SECOND)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Use a 16-second calibration cycle period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALW16_A {
    #[doc = "1: When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
    SIXTEEN_SECOND = 1,
}
impl From<CALW16_A> for bool {
    #[inline(always)]
    fn from(variant: CALW16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period"]
pub struct CALW16_R(crate::FieldReader<bool, CALW16_A>);
impl CALW16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALW16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALW16_A> {
        match self.bits {
            true => Some(CALW16_A::SIXTEEN_SECOND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_SECOND`"]
    #[inline(always)]
    pub fn is_sixteen_second(&self) -> bool {
        **self == CALW16_A::SIXTEEN_SECOND
    }
}
impl core::ops::Deref for CALW16_R {
    type Target = crate::FieldReader<bool, CALW16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period"]
pub struct CALW16_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALW16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
    #[inline(always)]
    pub fn sixteen_second(self) -> &'a mut W {
        self.variant(CALW16_A::SIXTEEN_SECOND)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CALM` reader - Calibration minus"]
pub struct CALM_R(crate::FieldReader<u16, u16>);
impl CALM_R {
    pub(crate) fn new(bits: u16) -> Self {
        CALM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALM` writer - Calibration minus"]
pub struct CALM_W<'a> {
    w: &'a mut W,
}
impl<'a> CALM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Calibration minus"]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W {
        CALP_W { w: self }
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W {
        CALW8_W { w: self }
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W {
        CALW16_W { w: self }
    }
    #[doc = "Bits 0:8 - Calibration minus"]
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W {
        CALM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](index.html) module"]
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calr::R](R) reader structure"]
impl crate::Readable for CALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calr::W](W) writer structure"]
impl crate::Writable for CALR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
