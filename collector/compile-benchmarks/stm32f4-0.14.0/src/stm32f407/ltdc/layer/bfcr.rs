#[doc = "Register `BFCR` reader"]
pub struct R(crate::R<BFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFCR` writer"]
pub struct W(crate::W<BFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFCR_SPEC>;
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
impl From<crate::W<BFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Blending Factor 1\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BF1_A {
    #[doc = "4: BF1 = constant alpha"]
    CONSTANT = 4,
    #[doc = "6: BF1 = pixel alpha * constant alpha"]
    PIXEL = 6,
}
impl From<BF1_A> for u8 {
    #[inline(always)]
    fn from(variant: BF1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BF1` reader - Blending Factor 1"]
pub struct BF1_R(crate::FieldReader<u8, BF1_A>);
impl BF1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BF1_A> {
        match self.bits {
            4 => Some(BF1_A::CONSTANT),
            6 => Some(BF1_A::PIXEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT`"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        **self == BF1_A::CONSTANT
    }
    #[doc = "Checks if the value of the field is `PIXEL`"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        **self == BF1_A::PIXEL
    }
}
impl core::ops::Deref for BF1_R {
    type Target = crate::FieldReader<u8, BF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BF1` writer - Blending Factor 1"]
pub struct BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BF1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BF1 = constant alpha"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(BF1_A::CONSTANT)
    }
    #[doc = "BF1 = pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(BF1_A::PIXEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Blending Factor 2\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BF2_A {
    #[doc = "5: BF2 = 1 - constant alpha"]
    CONSTANT = 5,
    #[doc = "7: BF2 = 1 - pixel alpha * constant alpha"]
    PIXEL = 7,
}
impl From<BF2_A> for u8 {
    #[inline(always)]
    fn from(variant: BF2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BF2` reader - Blending Factor 2"]
pub struct BF2_R(crate::FieldReader<u8, BF2_A>);
impl BF2_R {
    pub(crate) fn new(bits: u8) -> Self {
        BF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BF2_A> {
        match self.bits {
            5 => Some(BF2_A::CONSTANT),
            7 => Some(BF2_A::PIXEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT`"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        **self == BF2_A::CONSTANT
    }
    #[doc = "Checks if the value of the field is `PIXEL`"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        **self == BF2_A::PIXEL
    }
}
impl core::ops::Deref for BF2_R {
    type Target = crate::FieldReader<u8, BF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BF2` writer - Blending Factor 2"]
pub struct BF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BF2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BF2 = 1 - constant alpha"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(BF2_A::CONSTANT)
    }
    #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(BF2_A::PIXEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W {
        BF1_W { w: self }
    }
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W {
        BF2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Blending Factors Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfcr](index.html) module"]
pub struct BFCR_SPEC;
impl crate::RegisterSpec for BFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfcr::R](R) reader structure"]
impl crate::Readable for BFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfcr::W](W) writer structure"]
impl crate::Writable for BFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BFCR to value 0x0607"]
impl crate::Resettable for BFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0607
    }
}
