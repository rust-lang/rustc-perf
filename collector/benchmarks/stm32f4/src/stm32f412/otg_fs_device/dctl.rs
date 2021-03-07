#[doc = "Reader of register DCTL"]
pub type R = crate::R<u32, super::DCTL>;
#[doc = "Writer for register DCTL"]
pub type W = crate::W<u32, super::DCTL>;
#[doc = "Register DCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RWUSIG`"]
pub type RWUSIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWUSIG`"]
pub struct RWUSIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUSIG_W<'a> {
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
#[doc = "Reader of field `SDIS`"]
pub type SDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIS`"]
pub struct SDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIS_W<'a> {
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
#[doc = "Reader of field `GINSTS`"]
pub type GINSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `GONSTS`"]
pub type GONSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCTL`"]
pub type TCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCTL`"]
pub struct TCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SGINAK`"]
pub type SGINAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SGINAK`"]
pub struct SGINAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGINAK_W<'a> {
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
#[doc = "Reader of field `CGINAK`"]
pub type CGINAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CGINAK`"]
pub struct CGINAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGINAK_W<'a> {
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
#[doc = "Reader of field `SGONAK`"]
pub type SGONAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SGONAK`"]
pub struct SGONAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGONAK_W<'a> {
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
#[doc = "Reader of field `CGONAK`"]
pub type CGONAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CGONAK`"]
pub struct CGONAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGONAK_W<'a> {
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
#[doc = "Reader of field `POPRGDNE`"]
pub type POPRGDNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POPRGDNE`"]
pub struct POPRGDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> POPRGDNE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&self) -> SGINAK_R {
        SGINAK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&self) -> CGINAK_R {
        CGINAK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&self) -> SGONAK_R {
        SGONAK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&self) -> CGONAK_R {
        CGONAK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W {
        RWUSIG_W { w: self }
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W {
        SDIS_W { w: self }
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W {
        TCTL_W { w: self }
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W {
        SGINAK_W { w: self }
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W {
        CGINAK_W { w: self }
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W {
        SGONAK_W { w: self }
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W {
        CGONAK_W { w: self }
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W {
        POPRGDNE_W { w: self }
    }
}
