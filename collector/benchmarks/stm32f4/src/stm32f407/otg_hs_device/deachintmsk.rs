#[doc = "Reader of register DEACHINTMSK"]
pub type R = crate::R<u32, super::DEACHINTMSK>;
#[doc = "Writer for register DEACHINTMSK"]
pub type W = crate::W<u32, super::DEACHINTMSK>;
#[doc = "Register DEACHINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DEACHINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEP1INTM`"]
pub type IEP1INTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEP1INTM`"]
pub struct IEP1INTM_W<'a> {
    w: &'a mut W,
}
impl<'a> IEP1INTM_W<'a> {
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
#[doc = "Reader of field `OEP1INTM`"]
pub type OEP1INTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEP1INTM`"]
pub struct OEP1INTM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEP1INTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&mut self) -> IEP1INTM_W {
        IEP1INTM_W { w: self }
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&mut self) -> OEP1INTM_W {
        OEP1INTM_W { w: self }
    }
}
