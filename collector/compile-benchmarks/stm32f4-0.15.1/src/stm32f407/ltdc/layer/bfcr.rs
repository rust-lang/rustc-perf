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
    Constant = 4,
    #[doc = "6: BF1 = pixel alpha * constant alpha"]
    Pixel = 6,
}
impl From<BF1_A> for u8 {
    #[inline(always)]
    fn from(variant: BF1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BF1` reader - Blending Factor 1"]
pub type BF1_R = crate::FieldReader<u8, BF1_A>;
impl BF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BF1_A> {
        match self.bits {
            4 => Some(BF1_A::Constant),
            6 => Some(BF1_A::Pixel),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Constant`"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF1_A::Constant
    }
    #[doc = "Checks if the value of the field is `Pixel`"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF1_A::Pixel
    }
}
#[doc = "Field `BF1` writer - Blending Factor 1"]
pub type BF1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFCR_SPEC, u8, BF1_A, 3, O>;
impl<'a, const O: u8> BF1_W<'a, O> {
    #[doc = "BF1 = constant alpha"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(BF1_A::Constant)
    }
    #[doc = "BF1 = pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(BF1_A::Pixel)
    }
}
#[doc = "Blending Factor 2\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BF2_A {
    #[doc = "5: BF2 = 1 - constant alpha"]
    Constant = 5,
    #[doc = "7: BF2 = 1 - pixel alpha * constant alpha"]
    Pixel = 7,
}
impl From<BF2_A> for u8 {
    #[inline(always)]
    fn from(variant: BF2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BF2` reader - Blending Factor 2"]
pub type BF2_R = crate::FieldReader<u8, BF2_A>;
impl BF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BF2_A> {
        match self.bits {
            5 => Some(BF2_A::Constant),
            7 => Some(BF2_A::Pixel),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Constant`"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF2_A::Constant
    }
    #[doc = "Checks if the value of the field is `Pixel`"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF2_A::Pixel
    }
}
#[doc = "Field `BF2` writer - Blending Factor 2"]
pub type BF2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFCR_SPEC, u8, BF2_A, 3, O>;
impl<'a, const O: u8> BF2_W<'a, O> {
    #[doc = "BF2 = 1 - constant alpha"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(BF2_A::Constant)
    }
    #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(BF2_A::Pixel)
    }
}
impl R {
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W<8> {
        BF1_W::new(self)
    }
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W<0> {
        BF2_W::new(self)
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
