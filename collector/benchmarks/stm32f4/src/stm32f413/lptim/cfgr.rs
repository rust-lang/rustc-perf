#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENC`"]
pub type ENC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENC`"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
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
#[doc = "Reader of field `COUNTMODE`"]
pub type COUNTMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COUNTMODE`"]
pub struct COUNTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTMODE_W<'a> {
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
#[doc = "Reader of field `PRELOAD`"]
pub type PRELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRELOAD`"]
pub struct PRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELOAD_W<'a> {
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
#[doc = "Reader of field `WAVPOL`"]
pub type WAVPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVPOL`"]
pub struct WAVPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVPOL_W<'a> {
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
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
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
#[doc = "Reader of field `TIMOUT`"]
pub type TIMOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMOUT`"]
pub struct TIMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUT_W<'a> {
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
#[doc = "Reader of field `TRIGEN`"]
pub type TRIGEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGEN`"]
pub struct TRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `TRIGSEL`"]
pub type TRIGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGSEL`"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `TRGFLT`"]
pub type TRGFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGFLT`"]
pub struct TRGFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CKFLT`"]
pub type CKFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKFLT`"]
pub struct CKFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `CKPOL`"]
pub type CKPOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPOL`"]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `CKSEL`"]
pub type CKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSEL`"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
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
    #[doc = "Bit 24 - Encoder mode enable"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Waveform shape"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timeout enable"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Clock selector"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Encoder mode enable"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 23 - counter mode enabled"]
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W {
        COUNTMODE_W { w: self }
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W {
        PRELOAD_W { w: self }
    }
    #[doc = "Bit 21 - Waveform shape polarity"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WAVPOL_W {
        WAVPOL_W { w: self }
    }
    #[doc = "Bit 20 - Waveform shape"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bit 19 - Timeout enable"]
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W {
        TIMOUT_W { w: self }
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W {
        TRIGEN_W { w: self }
    }
    #[doc = "Bits 13:15 - Trigger selector"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger"]
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W {
        TRGFLT_W { w: self }
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock"]
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W {
        CKFLT_W { w: self }
    }
    #[doc = "Bits 1:2 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bit 0 - Clock selector"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
}
