#[doc = "Register `CCMR2_Input` reader"]
pub struct R(crate::R<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR2_Input` writer"]
pub struct W(crate::W<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_INPUT_SPEC>;
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
impl From<crate::W<CCMR2_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC4F` reader - Input capture 4 filter"]
pub struct IC4F_R(crate::FieldReader<u8, u8>);
impl IC4F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4F` writer - Input capture 4 filter"]
pub struct IC4F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler"]
pub struct IC4PSC_R(crate::FieldReader<u8, u8>);
impl IC4PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler"]
pub struct IC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Capture/Compare 4 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC4S_A {
    #[doc = "1: CC4 channel is configured as input, IC4 is mapped on TI4"]
    TI4 = 1,
    #[doc = "2: CC4 channel is configured as input, IC4 is mapped on TI3"]
    TI3 = 2,
    #[doc = "3: CC4 channel is configured as input, IC4 is mapped on TRC"]
    TRC = 3,
}
impl From<CC4S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection"]
pub struct CC4S_R(crate::FieldReader<u8, CC4S_A>);
impl CC4S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC4S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CC4S_A> {
        match self.bits {
            1 => Some(CC4S_A::TI4),
            2 => Some(CC4S_A::TI3),
            3 => Some(CC4S_A::TRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TI4`"]
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        **self == CC4S_A::TI4
    }
    #[doc = "Checks if the value of the field is `TI3`"]
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        **self == CC4S_A::TI3
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        **self == CC4S_A::TRC
    }
}
impl core::ops::Deref for CC4S_R {
    type Target = crate::FieldReader<u8, CC4S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection"]
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC4S_A::TI4)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC4S_A::TI3)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC4S_A::TRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC3F` reader - Input capture 3 filter"]
pub struct IC3F_R(crate::FieldReader<u8, u8>);
impl IC3F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3F` writer - Input capture 3 filter"]
pub struct IC3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler"]
pub struct IC3PSC_R(crate::FieldReader<u8, u8>);
impl IC3PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler"]
pub struct IC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Capture/compare 3 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC3S_A {
    #[doc = "1: CC3 channel is configured as input, IC3 is mapped on TI3"]
    TI3 = 1,
    #[doc = "2: CC3 channel is configured as input, IC3 is mapped on TI4"]
    TI4 = 2,
    #[doc = "3: CC3 channel is configured as input, IC3 is mapped on TRC"]
    TRC = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CC3S` reader - Capture/compare 3 selection"]
pub struct CC3S_R(crate::FieldReader<u8, CC3S_A>);
impl CC3S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC3S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CC3S_A> {
        match self.bits {
            1 => Some(CC3S_A::TI3),
            2 => Some(CC3S_A::TI4),
            3 => Some(CC3S_A::TRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TI3`"]
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        **self == CC3S_A::TI3
    }
    #[doc = "Checks if the value of the field is `TI4`"]
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        **self == CC3S_A::TI4
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        **self == CC3S_A::TRC
    }
}
impl core::ops::Deref for CC3S_R {
    type Target = crate::FieldReader<u8, CC3S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3S` writer - Capture/compare 3 selection"]
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC3S_A::TI3)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC3S_A::TI4)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC3S_A::TRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W {
        IC4F_W { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W {
        IC4PSC_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W {
        IC3F_W { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W {
        IC3PSC_W { w: self }
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 2 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_input](index.html) module"]
pub struct CCMR2_INPUT_SPEC;
impl crate::RegisterSpec for CCMR2_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr2_input::R](R) reader structure"]
impl crate::Readable for CCMR2_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr2_input::W](W) writer structure"]
impl crate::Writable for CCMR2_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMR2_Input to value 0"]
impl crate::Resettable for CCMR2_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
