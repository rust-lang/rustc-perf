#[doc = "Reader of register DSI_WIFCR"]
pub type R = crate::R<u32, super::DSI_WIFCR>;
#[doc = "Writer for register DSI_WIFCR"]
pub type W = crate::W<u32, super::DSI_WIFCR>;
#[doc = "Register DSI_WIFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WIFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRRIF`"]
pub type CRRIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRRIF`"]
pub struct CRRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRRIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CPLLUIF`"]
pub type CPLLUIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPLLUIF`"]
pub struct CPLLUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPLLUIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CPLLLIF`"]
pub type CPLLLIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPLLLIF`"]
pub struct CPLLLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPLLLIF_W<'a> {
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
#[doc = "Reader of field `CERIF`"]
pub type CERIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CERIF`"]
pub struct CERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CERIF_W<'a> {
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
#[doc = "Reader of field `CTEIF`"]
pub type CTEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF`"]
pub struct CTEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF_W<'a> {
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
    #[doc = "Bit 13 - Clear Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&self) -> CRRIF_R {
        CRRIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clear PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn cplluif(&self) -> CPLLUIF_R {
        CPLLUIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn cplllif(&self) -> CPLLLIF_R {
        CPLLLIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn cerif(&self) -> CERIF_R {
        CERIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Clear Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W {
        CRRIF_W { w: self }
    }
    #[doc = "Bit 10 - Clear PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn cplluif(&mut self) -> CPLLUIF_W {
        CPLLUIF_W { w: self }
    }
    #[doc = "Bit 9 - Clear PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn cplllif(&mut self) -> CPLLLIF_W {
        CPLLLIF_W { w: self }
    }
    #[doc = "Bit 1 - Clear End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn cerif(&mut self) -> CERIF_W {
        CERIF_W { w: self }
    }
    #[doc = "Bit 0 - Clear Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
    }
}
