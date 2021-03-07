#[doc = "Reader of register DSI_PCR"]
pub type R = crate::R<u32, super::DSI_PCR>;
#[doc = "Writer for register DSI_PCR"]
pub type W = crate::W<u32, super::DSI_PCR>;
#[doc = "Register DSI_PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCRXE`"]
pub type CRCRXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCRXE`"]
pub struct CRCRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRXE_W<'a> {
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
#[doc = "Reader of field `ECCRXE`"]
pub type ECCRXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCRXE`"]
pub struct ECCRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCRXE_W<'a> {
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
#[doc = "Reader of field `BTAE`"]
pub type BTAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTAE`"]
pub struct BTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTAE_W<'a> {
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
#[doc = "Reader of field `ETRXE`"]
pub type ETRXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETRXE`"]
pub struct ETRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRXE_W<'a> {
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
#[doc = "Reader of field `ETTXE`"]
pub type ETTXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETTXE`"]
pub struct ETTXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETTXE_W<'a> {
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
    #[doc = "Bit 4 - CRC Reception Enable"]
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC Reception Enable"]
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Turn Around Enable"]
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - EoTp Reception Enable"]
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EoTp Transmission Enable"]
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC Reception Enable"]
    #[inline(always)]
    pub fn crcrxe(&mut self) -> CRCRXE_W {
        CRCRXE_W { w: self }
    }
    #[doc = "Bit 3 - ECC Reception Enable"]
    #[inline(always)]
    pub fn eccrxe(&mut self) -> ECCRXE_W {
        ECCRXE_W { w: self }
    }
    #[doc = "Bit 2 - Bus Turn Around Enable"]
    #[inline(always)]
    pub fn btae(&mut self) -> BTAE_W {
        BTAE_W { w: self }
    }
    #[doc = "Bit 1 - EoTp Reception Enable"]
    #[inline(always)]
    pub fn etrxe(&mut self) -> ETRXE_W {
        ETRXE_W { w: self }
    }
    #[doc = "Bit 0 - EoTp Transmission Enable"]
    #[inline(always)]
    pub fn ettxe(&mut self) -> ETTXE_W {
        ETTXE_W { w: self }
    }
}
