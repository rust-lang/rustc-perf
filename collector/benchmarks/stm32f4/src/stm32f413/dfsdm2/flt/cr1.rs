#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AWFSEL`"]
pub type AWFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AWFSEL`"]
pub struct AWFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `FAST`"]
pub type FAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAST`"]
pub struct FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RCH`"]
pub type RCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCH`"]
pub struct RCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `RDMAEN`"]
pub type RDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDMAEN`"]
pub struct RDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RSYNC`"]
pub type RSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSYNC`"]
pub struct RSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNC_W<'a> {
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
#[doc = "Reader of field `RCONT`"]
pub type RCONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCONT`"]
pub struct RCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCONT_W<'a> {
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
#[doc = "Reader of field `RSWSTART`"]
pub type RSWSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSWSTART`"]
pub struct RSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RSWSTART_W<'a> {
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
#[doc = "Reader of field `JEXTEN`"]
pub type JEXTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JEXTEN`"]
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `JEXTSEL`"]
pub type JEXTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JEXTSEL`"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `JDMAEN`"]
pub type JDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JDMAEN`"]
pub struct JDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDMAEN_W<'a> {
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
#[doc = "Reader of field `JSCAN`"]
pub type JSCAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSCAN`"]
pub struct JSCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> JSCAN_W<'a> {
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
#[doc = "Reader of field `JSYNC`"]
pub type JSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSYNC`"]
pub struct JSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> JSYNC_W<'a> {
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
#[doc = "Reader of field `JSWSTART`"]
pub type JSWSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSWSTART`"]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
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
#[doc = "Reader of field `DFEN`"]
pub type DFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFEN`"]
pub struct DFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEN_W<'a> {
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
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&mut self) -> AWFSEL_W {
        AWFSEL_W { w: self }
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W { w: self }
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W {
        RCH_W { w: self }
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W {
        RDMAEN_W { w: self }
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W {
        RSYNC_W { w: self }
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W {
        RCONT_W { w: self }
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W {
        RSWSTART_W { w: self }
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W {
        JDMAEN_W { w: self }
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&mut self) -> JSCAN_W {
        JSCAN_W { w: self }
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W {
        JSYNC_W { w: self }
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W {
        DFEN_W { w: self }
    }
}
