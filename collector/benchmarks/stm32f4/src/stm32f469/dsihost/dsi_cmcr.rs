#[doc = "Reader of register DSI_CMCR"]
pub type R = crate::R<u32, super::DSI_CMCR>;
#[doc = "Writer for register DSI_CMCR"]
pub type W = crate::W<u32, super::DSI_CMCR>;
#[doc = "Register DSI_CMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_CMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MRDPS`"]
pub type MRDPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MRDPS`"]
pub struct MRDPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MRDPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DLWTX`"]
pub type DLWTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLWTX`"]
pub struct DLWTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DLWTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DSR0TX`"]
pub type DSR0TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSR0TX`"]
pub struct DSR0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR0TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DSW1TX`"]
pub type DSW1TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSW1TX`"]
pub struct DSW1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSW1TX_W<'a> {
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
#[doc = "Reader of field `DSW0TX`"]
pub type DSW0TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSW0TX`"]
pub struct DSW0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSW0TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `GLWTX`"]
pub type GLWTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLWTX`"]
pub struct GLWTX_W<'a> {
    w: &'a mut W,
}
impl<'a> GLWTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `GSR2TX`"]
pub type GSR2TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSR2TX`"]
pub struct GSR2TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSR2TX_W<'a> {
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
#[doc = "Reader of field `GSR1TX`"]
pub type GSR1TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSR1TX`"]
pub struct GSR1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSR1TX_W<'a> {
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
#[doc = "Reader of field `GSR0TX`"]
pub type GSR0TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSR0TX`"]
pub struct GSR0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSR0TX_W<'a> {
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
#[doc = "Reader of field `GSW2TX`"]
pub type GSW2TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSW2TX`"]
pub struct GSW2TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSW2TX_W<'a> {
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
#[doc = "Reader of field `GSW1TX`"]
pub type GSW1TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSW1TX`"]
pub struct GSW1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSW1TX_W<'a> {
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
#[doc = "Reader of field `GSW0TX`"]
pub type GSW0TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSW0TX`"]
pub struct GSW0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSW0TX_W<'a> {
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
#[doc = "Reader of field `ARE`"]
pub type ARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARE`"]
pub struct ARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARE_W<'a> {
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
#[doc = "Reader of field `TEARE`"]
pub type TEARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEARE`"]
pub struct TEARE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEARE_W<'a> {
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
    #[doc = "Bit 24 - Maximum Read Packet Size"]
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCS Long Write Transmission"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCS Short Read Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DCS Short Read One parameter Transmission"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DCS Short Write Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Generic Long Write Transmission"]
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Generic Short Read Two parameters Transmission"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Generic Short Read One parameters Transmission"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic Short Read Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Short Write Two parameters Transmission"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Short Write One parameters Transmission"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Short Write Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Maximum Read Packet Size"]
    #[inline(always)]
    pub fn mrdps(&mut self) -> MRDPS_W {
        MRDPS_W { w: self }
    }
    #[doc = "Bit 19 - DCS Long Write Transmission"]
    #[inline(always)]
    pub fn dlwtx(&mut self) -> DLWTX_W {
        DLWTX_W { w: self }
    }
    #[doc = "Bit 18 - DCS Short Read Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsr0tx(&mut self) -> DSR0TX_W {
        DSR0TX_W { w: self }
    }
    #[doc = "Bit 17 - DCS Short Read One parameter Transmission"]
    #[inline(always)]
    pub fn dsw1tx(&mut self) -> DSW1TX_W {
        DSW1TX_W { w: self }
    }
    #[doc = "Bit 16 - DCS Short Write Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsw0tx(&mut self) -> DSW0TX_W {
        DSW0TX_W { w: self }
    }
    #[doc = "Bit 14 - Generic Long Write Transmission"]
    #[inline(always)]
    pub fn glwtx(&mut self) -> GLWTX_W {
        GLWTX_W { w: self }
    }
    #[doc = "Bit 13 - Generic Short Read Two parameters Transmission"]
    #[inline(always)]
    pub fn gsr2tx(&mut self) -> GSR2TX_W {
        GSR2TX_W { w: self }
    }
    #[doc = "Bit 12 - Generic Short Read One parameters Transmission"]
    #[inline(always)]
    pub fn gsr1tx(&mut self) -> GSR1TX_W {
        GSR1TX_W { w: self }
    }
    #[doc = "Bit 11 - Generic Short Read Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsr0tx(&mut self) -> GSR0TX_W {
        GSR0TX_W { w: self }
    }
    #[doc = "Bit 10 - Generic Short Write Two parameters Transmission"]
    #[inline(always)]
    pub fn gsw2tx(&mut self) -> GSW2TX_W {
        GSW2TX_W { w: self }
    }
    #[doc = "Bit 9 - Generic Short Write One parameters Transmission"]
    #[inline(always)]
    pub fn gsw1tx(&mut self) -> GSW1TX_W {
        GSW1TX_W { w: self }
    }
    #[doc = "Bit 8 - Generic Short Write Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsw0tx(&mut self) -> GSW0TX_W {
        GSW0TX_W { w: self }
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn are(&mut self) -> ARE_W {
        ARE_W { w: self }
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn teare(&mut self) -> TEARE_W {
        TEARE_W { w: self }
    }
}
