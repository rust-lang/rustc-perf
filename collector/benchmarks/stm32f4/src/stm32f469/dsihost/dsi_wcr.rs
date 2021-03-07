#[doc = "Reader of register DSI_WCR"]
pub type R = crate::R<u32, super::DSI_WCR>;
#[doc = "Writer for register DSI_WCR"]
pub type W = crate::W<u32, super::DSI_WCR>;
#[doc = "Register DSI_WCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSIEN`"]
pub type DSIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSIEN`"]
pub struct DSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIEN_W<'a> {
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
#[doc = "Reader of field `LTDCEN`"]
pub type LTDCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LTDCEN`"]
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
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
#[doc = "Reader of field `SHTDN`"]
pub type SHTDN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHTDN`"]
pub struct SHTDN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTDN_W<'a> {
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
#[doc = "Reader of field `COLM`"]
pub type COLM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COLM`"]
pub struct COLM_W<'a> {
    w: &'a mut W,
}
impl<'a> COLM_W<'a> {
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
    #[doc = "Bit 3 - DSI Enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LTDC Enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Color Mode"]
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DSI Enable"]
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W {
        DSIEN_W { w: self }
    }
    #[doc = "Bit 2 - LTDC Enable"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&mut self) -> SHTDN_W {
        SHTDN_W { w: self }
    }
    #[doc = "Bit 0 - Color Mode"]
    #[inline(always)]
    pub fn colm(&mut self) -> COLM_W {
        COLM_W { w: self }
    }
}
