#[doc = "Reader of register DSI_WCFGR"]
pub type R = crate::R<u32, super::DSI_WCFGR>;
#[doc = "Writer for register DSI_WCFGR"]
pub type W = crate::W<u32, super::DSI_WCFGR>;
#[doc = "Register DSI_WCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSPOL`"]
pub type VSPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSPOL`"]
pub struct VSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPOL_W<'a> {
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
#[doc = "Reader of field `AR`"]
pub type AR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR`"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
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
#[doc = "Reader of field `TEPOL`"]
pub type TEPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEPOL`"]
pub struct TEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEPOL_W<'a> {
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
#[doc = "Reader of field `TESRC`"]
pub type TESRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESRC`"]
pub struct TESRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TESRC_W<'a> {
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
#[doc = "Reader of field `COLMUX`"]
pub type COLMUX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COLMUX`"]
pub struct COLMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> COLMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `DSIM`"]
pub type DSIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSIM`"]
pub struct DSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIM_W<'a> {
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
    #[doc = "Bit 7 - VSync Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Automatic Refresh"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TE Polarity"]
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TE Source"]
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Color Multiplexing"]
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - DSI Mode"]
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - VSync Polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W {
        VSPOL_W { w: self }
    }
    #[doc = "Bit 6 - Automatic Refresh"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
    #[doc = "Bit 5 - TE Polarity"]
    #[inline(always)]
    pub fn tepol(&mut self) -> TEPOL_W {
        TEPOL_W { w: self }
    }
    #[doc = "Bit 4 - TE Source"]
    #[inline(always)]
    pub fn tesrc(&mut self) -> TESRC_W {
        TESRC_W { w: self }
    }
    #[doc = "Bits 1:3 - Color Multiplexing"]
    #[inline(always)]
    pub fn colmux(&mut self) -> COLMUX_W {
        COLMUX_W { w: self }
    }
    #[doc = "Bit 0 - DSI Mode"]
    #[inline(always)]
    pub fn dsim(&mut self) -> DSIM_W {
        DSIM_W { w: self }
    }
}
