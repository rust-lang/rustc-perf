#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clears Register Reload Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRRIF_AW {
    #[doc = "1: Clears the RRIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CRRIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CRRIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRRIF` writer - Clears Register Reload Interrupt Flag"]
pub struct CRRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRRIF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the RRIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRRIF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Clears the Transfer Error Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTERRIF_AW {
    #[doc = "1: Clears the TERRIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CTERRIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTERRIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTERRIF` writer - Clears the Transfer Error Interrupt Flag"]
pub struct CTERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTERRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTERRIF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the TERRIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTERRIF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clears the FIFO Underrun Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFUIF_AW {
    #[doc = "1: Clears the FUIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CFUIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CFUIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFUIF` writer - Clears the FIFO Underrun Interrupt flag"]
pub struct CFUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFUIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFUIF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the FUIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFUIF_AW::CLEAR)
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
#[doc = "Clears the Line Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLIF_AW {
    #[doc = "1: Clears the LIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CLIF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLIF` writer - Clears the Line Interrupt Flag"]
pub struct CLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLIF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the LIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLIF_AW::CLEAR)
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
impl W {
    #[doc = "Bit 3 - Clears Register Reload Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W {
        CRRIF_W { w: self }
    }
    #[doc = "Bit 2 - Clears the Transfer Error Interrupt Flag"]
    #[inline(always)]
    pub fn cterrif(&mut self) -> CTERRIF_W {
        CTERRIF_W { w: self }
    }
    #[doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag"]
    #[inline(always)]
    pub fn cfuif(&mut self) -> CFUIF_W {
        CFUIF_W { w: self }
    }
    #[doc = "Bit 0 - Clears the Line Interrupt Flag"]
    #[inline(always)]
    pub fn clif(&mut self) -> CLIF_W {
        CLIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
