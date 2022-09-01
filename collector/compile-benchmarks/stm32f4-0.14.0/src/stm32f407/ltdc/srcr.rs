#[doc = "Register `SRCR` reader"]
pub struct R(crate::R<SRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCR` writer"]
pub struct W(crate::W<SRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCR_SPEC>;
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
impl From<crate::W<SRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Vertical Blanking Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBR_A {
    #[doc = "1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    RELOAD = 1,
    #[doc = "0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NOEFFECT = 0,
}
impl From<VBR_A> for bool {
    #[inline(always)]
    fn from(variant: VBR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBR` reader - Vertical Blanking Reload"]
pub struct VBR_R(crate::FieldReader<bool, VBR_A>);
impl VBR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBR_A {
        match self.bits {
            true => VBR_A::RELOAD,
            false => VBR_A::NOEFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        **self == VBR_A::RELOAD
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == VBR_A::NOEFFECT
    }
}
impl core::ops::Deref for VBR_R {
    type Target = crate::FieldReader<bool, VBR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBR` writer - Vertical Blanking Reload"]
pub struct VBR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(VBR_A::RELOAD)
    }
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VBR_A::NOEFFECT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Immediate Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMR_A {
    #[doc = "1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    RELOAD = 1,
    #[doc = "0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NOEFFECT = 0,
}
impl From<IMR_A> for bool {
    #[inline(always)]
    fn from(variant: IMR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMR` reader - Immediate Reload"]
pub struct IMR_R(crate::FieldReader<bool, IMR_A>);
impl IMR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMR_A {
        match self.bits {
            true => IMR_A::RELOAD,
            false => IMR_A::NOEFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        **self == IMR_A::RELOAD
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == IMR_A::NOEFFECT
    }
}
impl core::ops::Deref for IMR_R {
    type Target = crate::FieldReader<bool, IMR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMR` writer - Immediate Reload"]
pub struct IMR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(IMR_A::RELOAD)
    }
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IMR_A::NOEFFECT)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&mut self) -> VBR_W {
        VBR_W { w: self }
    }
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W {
        IMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Reload Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcr](index.html) module"]
pub struct SRCR_SPEC;
impl crate::RegisterSpec for SRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcr::R](R) reader structure"]
impl crate::Readable for SRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcr::W](W) writer structure"]
impl crate::Writable for SRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCR to value 0"]
impl crate::Resettable for SRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
