#[doc = "Reader of register DSI_VMCR"]
pub type R = crate::R<u32, super::DSI_VMCR>;
#[doc = "Writer for register DSI_VMCR"]
pub type W = crate::W<u32, super::DSI_VMCR>;
#[doc = "Register DSI_VMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGO`"]
pub type PGO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGO`"]
pub struct PGO_W<'a> {
    w: &'a mut W,
}
impl<'a> PGO_W<'a> {
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
#[doc = "Reader of field `PGM`"]
pub type PGM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGM`"]
pub struct PGM_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PGE`"]
pub type PGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE`"]
pub struct PGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE_W<'a> {
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
#[doc = "Reader of field `LPCE`"]
pub type LPCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCE`"]
pub struct LPCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FBTAAE`"]
pub type FBTAAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBTAAE`"]
pub struct FBTAAE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBTAAE_W<'a> {
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
#[doc = "Reader of field `LPHFE`"]
pub type LPHFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPHFE`"]
pub struct LPHFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPHFE_W<'a> {
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
#[doc = "Reader of field `LPHBPE`"]
pub type LPHBPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPHBPE`"]
pub struct LPHBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPHBPE_W<'a> {
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
#[doc = "Reader of field `LVAE`"]
pub type LVAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVAE`"]
pub struct LVAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVAE_W<'a> {
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
#[doc = "Reader of field `LPVFPE`"]
pub type LPVFPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPVFPE`"]
pub struct LPVFPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVFPE_W<'a> {
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
#[doc = "Reader of field `LPVBPE`"]
pub type LPVBPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPVBPE`"]
pub struct LPVBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVBPE_W<'a> {
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
#[doc = "Reader of field `LPVSAE`"]
pub type LPVSAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPVSAE`"]
pub struct LPVSAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVSAE_W<'a> {
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
#[doc = "Reader of field `VMT`"]
pub type VMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VMT`"]
pub struct VMT_W<'a> {
    w: &'a mut W,
}
impl<'a> VMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Pattern Generator Orientation"]
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pattern Generator Mode"]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pattern Generator Enable"]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Frame Bus-Turn-Around Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Low-Power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&self) -> LVAE_R {
        LVAE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync Active Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Pattern Generator Orientation"]
    #[inline(always)]
    pub fn pgo(&mut self) -> PGO_W {
        PGO_W { w: self }
    }
    #[doc = "Bit 20 - Pattern Generator Mode"]
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W {
        PGM_W { w: self }
    }
    #[doc = "Bit 16 - Pattern Generator Enable"]
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W {
        PGE_W { w: self }
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W {
        LPCE_W { w: self }
    }
    #[doc = "Bit 14 - Frame Bus-Turn-Around Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W {
        FBTAAE_W { w: self }
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&mut self) -> LPHFE_W {
        LPHFE_W { w: self }
    }
    #[doc = "Bit 12 - Low-Power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W {
        LPHBPE_W { w: self }
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&mut self) -> LVAE_W {
        LVAE_W { w: self }
    }
    #[doc = "Bit 10 - Low-power Vertical Front-porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W {
        LPVFPE_W { w: self }
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W {
        LPVBPE_W { w: self }
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync Active Enable"]
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W {
        LPVSAE_W { w: self }
    }
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W {
        VMT_W { w: self }
    }
}
