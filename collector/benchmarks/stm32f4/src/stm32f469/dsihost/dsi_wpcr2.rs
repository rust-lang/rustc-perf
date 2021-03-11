#[doc = "Reader of register DSI_WPCR2"]
pub type R = crate::R<u32, super::DSI_WPCR2>;
#[doc = "Writer for register DSI_WPCR2"]
pub type W = crate::W<u32, super::DSI_WPCR2>;
#[doc = "Register DSI_WPCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WPCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPRXFT`"]
pub type LPRXFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPRXFT`"]
pub struct LPRXFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRXFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `FLPRXLPM`"]
pub type FLPRXLPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLPRXLPM`"]
pub struct FLPRXLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPRXLPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `HSTXSRCDL`"]
pub type HSTXSRCDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXSRCDL`"]
pub struct HSTXSRCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `HSTXSRCCL`"]
pub type HSTXSRCCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXSRCCL`"]
pub struct HSTXSRCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDCC`"]
pub type SDCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDCC`"]
pub struct SDCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCC_W<'a> {
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
#[doc = "Reader of field `LPSRDL`"]
pub type LPSRDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPSRDL`"]
pub struct LPSRDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LPSRCL`"]
pub type LPSRCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPSRCL`"]
pub struct LPSRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `HSTXDLL`"]
pub type HSTXDLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXDLL`"]
pub struct HSTXDLL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `HSTXDCL`"]
pub type HSTXDCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXDCL`"]
pub struct HSTXDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline(always)]
    pub fn sdcc(&self) -> SDCC_R {
        SDCC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline(always)]
    pub fn lpsrdl(&self) -> LPSRDL_R {
        LPSRDL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline(always)]
    pub fn lpsrcl(&self) -> LPSRCL_R {
        LPSRCL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline(always)]
    pub fn hstxdll(&self) -> HSTXDLL_R {
        HSTXDLL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline(always)]
    pub fn lprxft(&mut self) -> LPRXFT_W {
        LPRXFT_W { w: self }
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W {
        FLPRXLPM_W { w: self }
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W {
        HSTXSRCDL_W { w: self }
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W {
        HSTXSRCCL_W { w: self }
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline(always)]
    pub fn sdcc(&mut self) -> SDCC_W {
        SDCC_W { w: self }
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline(always)]
    pub fn lpsrdl(&mut self) -> LPSRDL_W {
        LPSRDL_W { w: self }
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline(always)]
    pub fn lpsrcl(&mut self) -> LPSRCL_W {
        LPSRCL_W { w: self }
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline(always)]
    pub fn hstxdll(&mut self) -> HSTXDLL_W {
        HSTXDLL_W { w: self }
    }
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W {
        HSTXDCL_W { w: self }
    }
}
