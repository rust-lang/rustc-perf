#[doc = "Register `PUCR` reader"]
pub struct R(crate::R<PUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCR` writer"]
pub struct W(crate::W<PUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_SPEC>;
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
impl From<crate::W<PUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEDL` reader - ULPS Exit on Data Lane"]
pub struct UEDL_R(crate::FieldReader<bool, bool>);
impl UEDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UEDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UEDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UEDL` writer - ULPS Exit on Data Lane"]
pub struct UEDL_W<'a> {
    w: &'a mut W,
}
impl<'a> UEDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `URDL` reader - ULPS Request on Data Lane"]
pub struct URDL_R(crate::FieldReader<bool, bool>);
impl URDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        URDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URDL` writer - ULPS Request on Data Lane"]
pub struct URDL_W<'a> {
    w: &'a mut W,
}
impl<'a> URDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `UECL` reader - ULPS Exit on Clock Lane"]
pub struct UECL_R(crate::FieldReader<bool, bool>);
impl UECL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UECL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UECL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UECL` writer - ULPS Exit on Clock Lane"]
pub struct UECL_W<'a> {
    w: &'a mut W,
}
impl<'a> UECL_W<'a> {
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
#[doc = "Field `URCL` reader - ULPS Request on Clock Lane"]
pub struct URCL_R(crate::FieldReader<bool, bool>);
impl URCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        URCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URCL` writer - ULPS Request on Clock Lane"]
pub struct URCL_W<'a> {
    w: &'a mut W,
}
impl<'a> URCL_W<'a> {
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
    #[doc = "Bit 3 - ULPS Exit on Data Lane"]
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ULPS Request on Data Lane"]
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ULPS Exit on Clock Lane"]
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ULPS Request on Clock Lane"]
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ULPS Exit on Data Lane"]
    #[inline(always)]
    pub fn uedl(&mut self) -> UEDL_W {
        UEDL_W { w: self }
    }
    #[doc = "Bit 2 - ULPS Request on Data Lane"]
    #[inline(always)]
    pub fn urdl(&mut self) -> URDL_W {
        URDL_W { w: self }
    }
    #[doc = "Bit 1 - ULPS Exit on Clock Lane"]
    #[inline(always)]
    pub fn uecl(&mut self) -> UECL_W {
        UECL_W { w: self }
    }
    #[doc = "Bit 0 - ULPS Request on Clock Lane"]
    #[inline(always)]
    pub fn urcl(&mut self) -> URCL_W {
        URCL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY ULPS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr](index.html) module"]
pub struct PUCR_SPEC;
impl crate::RegisterSpec for PUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr::R](R) reader structure"]
impl crate::Readable for PUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr::W](W) writer structure"]
impl crate::Writable for PUCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCR to value 0"]
impl crate::Resettable for PUCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
