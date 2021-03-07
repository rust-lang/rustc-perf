#[doc = "Reader of register CEC_CR"]
pub type R = crate::R<u32, super::CEC_CR>;
#[doc = "Writer for register CEC_CR"]
pub type W = crate::W<u32, super::CEC_CR>;
#[doc = "Register CEC_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CEC_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXEOM`"]
pub type TXEOM_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSOM`"]
pub type TXSOM_R = crate::R<bool, bool>;
#[doc = "Reader of field `CECEN`"]
pub type CECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CECEN`"]
pub struct CECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECEN_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Tx End Of Message"]
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx Start Of Message"]
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CEC Enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC Enable"]
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W {
        CECEN_W { w: self }
    }
}
