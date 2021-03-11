#[doc = "Reader of register DEACHINT"]
pub type R = crate::R<u32, super::DEACHINT>;
#[doc = "Writer for register DEACHINT"]
pub type W = crate::W<u32, super::DEACHINT>;
#[doc = "Register DEACHINT `reset()`'s with value 0"]
impl crate::ResetValue for super::DEACHINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEP1INT`"]
pub type IEP1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEP1INT`"]
pub struct IEP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> IEP1INT_W<'a> {
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
#[doc = "Reader of field `OEP1INT`"]
pub type OEP1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEP1INT`"]
pub struct OEP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> OEP1INT_W<'a> {
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
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&mut self) -> IEP1INT_W {
        IEP1INT_W { w: self }
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&mut self) -> OEP1INT_W {
        OEP1INT_W { w: self }
    }
}
