#[doc = "Reader of register MACPMTCSR"]
pub type R = crate::R<u32, super::MACPMTCSR>;
#[doc = "Writer for register MACPMTCSR"]
pub type W = crate::W<u32, super::MACPMTCSR>;
#[doc = "Register MACPMTCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPMTCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MPE`"]
pub type MPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPE`"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `WFE`"]
pub type WFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFE`"]
pub struct WFE_W<'a> {
    w: &'a mut W,
}
impl<'a> WFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MPR`"]
pub type MPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPR`"]
pub struct MPR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WFR`"]
pub type WFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFR`"]
pub struct WFR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GU`"]
pub type GU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GU`"]
pub struct GU_W<'a> {
    w: &'a mut W,
}
impl<'a> GU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `WFFRPR`"]
pub type WFFRPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFFRPR`"]
pub struct WFFRPR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFFRPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPE"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WFE"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WFR"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GU"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PD"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 1 - MPE"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Bit 2 - WFE"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W {
        WFE_W { w: self }
    }
    #[doc = "Bit 5 - MPR"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W {
        MPR_W { w: self }
    }
    #[doc = "Bit 6 - WFR"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W {
        WFR_W { w: self }
    }
    #[doc = "Bit 9 - GU"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W {
        GU_W { w: self }
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W {
        WFFRPR_W { w: self }
    }
}
