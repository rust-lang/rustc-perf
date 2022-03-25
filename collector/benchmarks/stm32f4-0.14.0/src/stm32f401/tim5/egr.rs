#[doc = "Register `EGR` writer"]
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TG_AW {
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    TRIGGER = 1,
}
impl From<TG_AW> for bool {
    #[inline(always)]
    fn from(variant: TG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG` writer - Trigger generation"]
pub struct TG_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TG_AW::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Capture/compare 4 generation"]
pub type CC4G_AW = CC1G_AW;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation"]
pub struct CC4G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4G_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC4G_AW::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Capture/compare 3 generation"]
pub type CC3G_AW = CC1G_AW;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation"]
pub struct CC3G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3G_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC3G_AW::TRIGGER)
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
#[doc = "Capture/compare 2 generation"]
pub type CC2G_AW = CC1G_AW;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub struct CC2G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2G_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC2G_AW::TRIGGER)
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
#[doc = "Capture/compare 1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1G_AW {
    #[doc = "1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    TRIGGER = 1,
}
impl From<CC1G_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1G_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub struct CC1G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1G_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1G_AW::TRIGGER)
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
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_AW {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    UPDATE = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::UPDATE)
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
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W { w: self }
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W {
        CC4G_W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W {
        CC3G_W { w: self }
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W {
        CC2G_W { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W {
        CC1G_W { w: self }
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [egr::W](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
