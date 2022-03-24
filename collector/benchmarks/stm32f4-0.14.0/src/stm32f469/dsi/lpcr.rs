#[doc = "Register `LPCR` reader"]
pub struct R(crate::R<LPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCR` writer"]
pub struct W(crate::W<LPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCR_SPEC>;
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
impl From<crate::W<LPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSP` reader - HSYNC Polarity"]
pub struct HSP_R(crate::FieldReader<bool, bool>);
impl HSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSP` writer - HSYNC Polarity"]
pub struct HSP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSP_W<'a> {
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
#[doc = "Field `VSP` reader - VSYNC Polarity"]
pub struct VSP_R(crate::FieldReader<bool, bool>);
impl VSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSP` writer - VSYNC Polarity"]
pub struct VSP_W<'a> {
    w: &'a mut W,
}
impl<'a> VSP_W<'a> {
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
#[doc = "Field `DEP` reader - Data Enable Polarity"]
pub struct DEP_R(crate::FieldReader<bool, bool>);
impl DEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEP` writer - Data Enable Polarity"]
pub struct DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DEP_W<'a> {
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
    #[doc = "Bit 2 - HSYNC Polarity"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data Enable Polarity"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - HSYNC Polarity"]
    #[inline(always)]
    pub fn hsp(&mut self) -> HSP_W {
        HSP_W { w: self }
    }
    #[doc = "Bit 1 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsp(&mut self) -> VSP_W {
        VSP_W { w: self }
    }
    #[doc = "Bit 0 - Data Enable Polarity"]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC Polarity Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](index.html) module"]
pub struct LPCR_SPEC;
impl crate::RegisterSpec for LPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpcr::R](R) reader structure"]
impl crate::Readable for LPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpcr::W](W) writer structure"]
impl crate::Writable for LPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPCR to value 0"]
impl crate::Resettable for LPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
