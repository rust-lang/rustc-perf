#[doc = "Reader of register CEC_CFGR"]
pub type R = crate::R<u32, super::CEC_CFGR>;
#[doc = "Writer for register CEC_CFGR"]
pub type W = crate::W<u32, super::CEC_CFGR>;
#[doc = "Register CEC_CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CEC_CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSTN`"]
pub type LSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTN`"]
pub struct LSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTN_W<'a> {
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
#[doc = "Reader of field `OAR`"]
pub type OAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OAR`"]
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SFTOP`"]
pub type SFTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTOP`"]
pub struct SFTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTOP_W<'a> {
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
#[doc = "Reader of field `BRDNOGEN`"]
pub type BRDNOGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRDNOGEN`"]
pub struct BRDNOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDNOGEN_W<'a> {
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
#[doc = "Reader of field `LBPEGEN`"]
pub type LBPEGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBPEGEN`"]
pub struct LBPEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEGEN_W<'a> {
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
#[doc = "Reader of field `BREGEN`"]
pub type BREGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BREGEN`"]
pub struct BREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREGEN_W<'a> {
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
#[doc = "Reader of field `BRESTP`"]
pub type BRESTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRESTP`"]
pub struct BRESTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRESTP_W<'a> {
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
#[doc = "Reader of field `RXTOL`"]
pub type RXTOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTOL`"]
pub struct RXTOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOL_W<'a> {
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
#[doc = "Reader of field `SFT`"]
pub type SFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SFT`"]
pub struct SFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W {
        LSTN_W { w: self }
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W {
        SFTOP_W { w: self }
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W {
        BRDNOGEN_W { w: self }
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W {
        LBPEGEN_W { w: self }
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W {
        BREGEN_W { w: self }
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W {
        BRESTP_W { w: self }
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W {
        RXTOL_W { w: self }
    }
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W {
        SFT_W { w: self }
    }
}
