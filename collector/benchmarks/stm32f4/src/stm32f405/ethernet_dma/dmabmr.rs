#[doc = "Reader of register DMABMR"]
pub type R = crate::R<u32, super::DMABMR>;
#[doc = "Writer for register DMABMR"]
pub type W = crate::W<u32, super::DMABMR>;
#[doc = "Register DMABMR `reset()`'s with value 0x2101"]
impl crate::ResetValue for super::DMABMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2101
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
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
#[doc = "Reader of field `DA`"]
pub type DA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DA`"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
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
#[doc = "Reader of field `DSL`"]
pub type DSL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSL`"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `EDFE`"]
pub type EDFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDFE`"]
pub struct EDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFE_W<'a> {
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
#[doc = "Reader of field `PBL`"]
pub type PBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBL`"]
pub struct PBL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTPR`"]
pub type RTPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTPR`"]
pub struct RTPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `FB`"]
pub type FB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB`"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
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
#[doc = "Reader of field `RDP`"]
pub type RDP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDP`"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | (((value as u32) & 0x3f) << 17);
        self.w
    }
}
#[doc = "Reader of field `USP`"]
pub type USP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USP`"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FPM`"]
pub type FPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPM`"]
pub struct FPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPM_W<'a> {
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
#[doc = "Reader of field `AAB`"]
pub type AAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AAB`"]
pub struct AAB_W<'a> {
    w: &'a mut W,
}
impl<'a> AAB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `MB`"]
pub type MB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MB`"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SR"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DA"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - DSL"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - EDFE"]
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - PBL"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - RTPR"]
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - FB"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:22 - RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - USP"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FPM"]
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AAB"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MB"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SR"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 1 - DA"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Bits 2:6 - DSL"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    #[doc = "Bit 7 - EDFE"]
    #[inline(always)]
    pub fn edfe(&mut self) -> EDFE_W {
        EDFE_W { w: self }
    }
    #[doc = "Bits 8:13 - PBL"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W {
        PBL_W { w: self }
    }
    #[doc = "Bits 14:15 - RTPR"]
    #[inline(always)]
    pub fn rtpr(&mut self) -> RTPR_W {
        RTPR_W { w: self }
    }
    #[doc = "Bit 16 - FB"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Bits 17:22 - RDP"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 23 - USP"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    #[doc = "Bit 24 - FPM"]
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W {
        FPM_W { w: self }
    }
    #[doc = "Bit 25 - AAB"]
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W {
        AAB_W { w: self }
    }
    #[doc = "Bit 26 - MB"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
}
