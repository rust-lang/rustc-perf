#[doc = "Register `VSCR` reader"]
pub struct R(crate::R<VSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VSCR` writer"]
pub struct W(crate::W<VSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VSCR_SPEC>;
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
impl From<crate::W<VSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UR` reader - Update Register"]
pub struct UR_R(crate::FieldReader<bool, bool>);
impl UR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UR` writer - Update Register"]
pub struct UR_W<'a> {
    w: &'a mut W,
}
impl<'a> UR_W<'a> {
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
#[doc = "Field `EN` reader - Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 8 - Update Register"]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Update Register"]
    #[inline(always)]
    pub fn ur(&mut self) -> UR_W {
        UR_W { w: self }
    }
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video Shadow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vscr](index.html) module"]
pub struct VSCR_SPEC;
impl crate::RegisterSpec for VSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vscr::R](R) reader structure"]
impl crate::Readable for VSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vscr::W](W) writer structure"]
impl crate::Writable for VSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VSCR to value 0"]
impl crate::Resettable for VSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
