#[doc = "Reader of register CEC_IER"]
pub type R = crate::R<u32, super::CEC_IER>;
#[doc = "Writer for register CEC_IER"]
pub type W = crate::W<u32, super::CEC_IER>;
#[doc = "Register CEC_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::CEC_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXACKIE`"]
pub type TXACKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXACKIE`"]
pub struct TXACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXERRIE`"]
pub type TXERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXERRIE`"]
pub struct TXERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TXUDRIE`"]
pub type TXUDRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUDRIE`"]
pub struct TXUDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDRIE_W<'a> {
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
#[doc = "Reader of field `TXENDIE`"]
pub type TXENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENDIE`"]
pub struct TXENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIE_W<'a> {
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
#[doc = "Reader of field `TXBRIE`"]
pub type TXBRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBRIE`"]
pub struct TXBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ARBLSTIE`"]
pub type ARBLSTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLSTIE`"]
pub struct ARBLSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLSTIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RXACKIE`"]
pub type RXACKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXACKIE`"]
pub struct RXACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACKIE_W<'a> {
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
#[doc = "Reader of field `LBPEIE`"]
pub type LBPEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBPEIE`"]
pub struct LBPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEIE_W<'a> {
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
#[doc = "Reader of field `SBPEIE`"]
pub type SBPEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBPEIE`"]
pub struct SBPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBPEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BREIE`"]
pub type BREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BREIE`"]
pub struct BREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BREIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RXOVRIE`"]
pub type RXOVRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVRIE`"]
pub struct RXOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRIE_W<'a> {
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
#[doc = "Reader of field `RXENDIE`"]
pub type RXENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENDIE`"]
pub struct RXENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDIE_W<'a> {
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
#[doc = "Reader of field `RXBRIE`"]
pub type RXBRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBRIE`"]
pub struct RXBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRIE_W<'a> {
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
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable"]
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable"]
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable"]
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable"]
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn txackie(&mut self) -> TXACKIE_W {
        TXACKIE_W { w: self }
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable"]
    #[inline(always)]
    pub fn txerrie(&mut self) -> TXERRIE_W {
        TXERRIE_W { w: self }
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn txudrie(&mut self) -> TXUDRIE_W {
        TXUDRIE_W { w: self }
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W {
        TXENDIE_W { w: self }
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable"]
    #[inline(always)]
    pub fn txbrie(&mut self) -> TXBRIE_W {
        TXBRIE_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblstie(&mut self) -> ARBLSTIE_W {
        ARBLSTIE_W { w: self }
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxackie(&mut self) -> RXACKIE_W {
        RXACKIE_W { w: self }
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LBPEIE_W {
        LBPEIE_W { w: self }
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SBPEIE_W {
        SBPEIE_W { w: self }
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable"]
    #[inline(always)]
    pub fn breie(&mut self) -> BREIE_W {
        BREIE_W { w: self }
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W {
        RXOVRIE_W { w: self }
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable"]
    #[inline(always)]
    pub fn rxendie(&mut self) -> RXENDIE_W {
        RXENDIE_W { w: self }
    }
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RXBRIE_W {
        RXBRIE_W { w: self }
    }
}
