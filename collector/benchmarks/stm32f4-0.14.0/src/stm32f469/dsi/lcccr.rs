#[doc = "Register `LCCCR` reader"]
pub struct R(crate::R<LCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCCCR` writer"]
pub struct W(crate::W<LCCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCCCR_SPEC>;
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
impl From<crate::W<LCCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPE` reader - Loosely Packed Enable"]
pub struct LPE_R(crate::FieldReader<bool, bool>);
impl LPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPE` writer - Loosely Packed Enable"]
pub struct LPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `COLC` reader - Color Coding"]
pub struct COLC_R(crate::FieldReader<u8, u8>);
impl COLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        COLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLC` writer - Color Coding"]
pub struct COLC_W<'a> {
    w: &'a mut W,
}
impl<'a> COLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Loosely Packed Enable"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Color Coding"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Loosely Packed Enable"]
    #[inline(always)]
    pub fn lpe(&mut self) -> LPE_W {
        LPE_W { w: self }
    }
    #[doc = "Bits 0:3 - Color Coding"]
    #[inline(always)]
    pub fn colc(&mut self) -> COLC_W {
        COLC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC Current Color Coding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcccr](index.html) module"]
pub struct LCCCR_SPEC;
impl crate::RegisterSpec for LCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcccr::R](R) reader structure"]
impl crate::Readable for LCCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcccr::W](W) writer structure"]
impl crate::Writable for LCCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCCCR to value 0"]
impl crate::Resettable for LCCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
