#[doc = "Register `PFCR` reader"]
pub struct R(crate::R<PFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFCR` writer"]
pub struct W(crate::W<PFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFCR_SPEC>;
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
impl From<crate::W<PFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pixel Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PF_A {
    #[doc = "0: ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: RGB888"]
    Rgb888 = 1,
    #[doc = "2: RGB565"]
    Rgb565 = 2,
    #[doc = "3: ARGB1555"]
    Argb1555 = 3,
    #[doc = "4: ARGB4444"]
    Argb4444 = 4,
    #[doc = "5: L8 (8-bit luminance)"]
    L8 = 5,
    #[doc = "6: AL44 (4-bit alpha, 4-bit luminance)"]
    Al44 = 6,
    #[doc = "7: AL88 (8-bit alpha, 8-bit luminance)"]
    Al88 = 7,
}
impl From<PF_A> for u8 {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PF` reader - Pixel Format"]
pub type PF_R = crate::FieldReader<u8, PF_A>;
impl PF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            0 => PF_A::Argb8888,
            1 => PF_A::Rgb888,
            2 => PF_A::Rgb565,
            3 => PF_A::Argb1555,
            4 => PF_A::Argb4444,
            5 => PF_A::L8,
            6 => PF_A::Al44,
            7 => PF_A::Al88,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Argb8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PF_A::Argb8888
    }
    #[doc = "Checks if the value of the field is `Rgb888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PF_A::Rgb888
    }
    #[doc = "Checks if the value of the field is `Rgb565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PF_A::Rgb565
    }
    #[doc = "Checks if the value of the field is `Argb1555`"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == PF_A::Argb1555
    }
    #[doc = "Checks if the value of the field is `Argb4444`"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == PF_A::Argb4444
    }
    #[doc = "Checks if the value of the field is `L8`"]
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == PF_A::L8
    }
    #[doc = "Checks if the value of the field is `Al44`"]
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == PF_A::Al44
    }
    #[doc = "Checks if the value of the field is `Al88`"]
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == PF_A::Al88
    }
}
#[doc = "Field `PF` writer - Pixel Format"]
pub type PF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFCR_SPEC, u8, PF_A, 3, O>;
impl<'a, const O: u8> PF_W<'a, O> {
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(PF_A::Argb8888)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(PF_A::Rgb888)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(PF_A::Rgb565)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(PF_A::Argb1555)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(PF_A::Argb4444)
    }
    #[doc = "L8 (8-bit luminance)"]
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(PF_A::L8)
    }
    #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(PF_A::Al44)
    }
    #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(PF_A::Al88)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W<0> {
        PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Pixel Format Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfcr](index.html) module"]
pub struct PFCR_SPEC;
impl crate::RegisterSpec for PFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfcr::R](R) reader structure"]
impl crate::Readable for PFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfcr::W](W) writer structure"]
impl crate::Writable for PFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFCR to value 0"]
impl crate::Resettable for PFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
