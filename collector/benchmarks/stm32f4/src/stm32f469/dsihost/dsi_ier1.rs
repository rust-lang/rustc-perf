#[doc = "Reader of register DSI_IER1"]
pub type R = crate::R<u32, super::DSI_IER1>;
#[doc = "Writer for register DSI_IER1"]
pub type W = crate::W<u32, super::DSI_IER1>;
#[doc = "Register DSI_IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_IER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPRXEIE`"]
pub type GPRXEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPRXEIE`"]
pub struct GPRXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRXEIE_W<'a> {
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
#[doc = "Reader of field `GPRDEIE`"]
pub type GPRDEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPRDEIE`"]
pub struct GPRDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRDEIE_W<'a> {
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
#[doc = "Reader of field `GPTXEIE`"]
pub type GPTXEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTXEIE`"]
pub struct GPTXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTXEIE_W<'a> {
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
#[doc = "Reader of field `GPWREIE`"]
pub type GPWREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPWREIE`"]
pub struct GPWREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWREIE_W<'a> {
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
#[doc = "Reader of field `GCWREIE`"]
pub type GCWREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCWREIE`"]
pub struct GCWREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCWREIE_W<'a> {
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
#[doc = "Reader of field `LPWREIE`"]
pub type LPWREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPWREIE`"]
pub struct LPWREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWREIE_W<'a> {
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
#[doc = "Reader of field `EOTPEIE`"]
pub type EOTPEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTPEIE`"]
pub struct EOTPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTPEIE_W<'a> {
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
#[doc = "Reader of field `PSEIE`"]
pub type PSEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSEIE`"]
pub struct PSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEIE_W<'a> {
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
#[doc = "Reader of field `CRCEIE`"]
pub type CRCEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEIE`"]
pub struct CRCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEIE_W<'a> {
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
#[doc = "Reader of field `ECCMEIE`"]
pub type ECCMEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCMEIE`"]
pub struct ECCMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCMEIE_W<'a> {
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
#[doc = "Reader of field `ECCSEIE`"]
pub type ECCSEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCSEIE`"]
pub struct ECCSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSEIE_W<'a> {
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
#[doc = "Reader of field `TOLPRXIE`"]
pub type TOLPRXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOLPRXIE`"]
pub struct TOLPRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOLPRXIE_W<'a> {
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
#[doc = "Reader of field `TOHSTXIE`"]
pub type TOHSTXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOHSTXIE`"]
pub struct TOHSTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOHSTXIE_W<'a> {
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
    #[doc = "Bit 12 - Generic Payload Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic Payload Read Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Command Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LTDC Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EoTp Error Interrupt Enable"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC Multi-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC Single-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception Interrupt Enable"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Generic Payload Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprxeie(&mut self) -> GPRXEIE_W {
        GPRXEIE_W { w: self }
    }
    #[doc = "Bit 11 - Generic Payload Read Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprdeie(&mut self) -> GPRDEIE_W {
        GPRDEIE_W { w: self }
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn gptxeie(&mut self) -> GPTXEIE_W {
        GPTXEIE_W { w: self }
    }
    #[doc = "Bit 9 - Generic Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gpwreie(&mut self) -> GPWREIE_W {
        GPWREIE_W { w: self }
    }
    #[doc = "Bit 8 - Generic Command Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gcwreie(&mut self) -> GCWREIE_W {
        GCWREIE_W { w: self }
    }
    #[doc = "Bit 7 - LTDC Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn lpwreie(&mut self) -> LPWREIE_W {
        LPWREIE_W { w: self }
    }
    #[doc = "Bit 6 - EoTp Error Interrupt Enable"]
    #[inline(always)]
    pub fn eotpeie(&mut self) -> EOTPEIE_W {
        EOTPEIE_W { w: self }
    }
    #[doc = "Bit 5 - Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn pseie(&mut self) -> PSEIE_W {
        PSEIE_W { w: self }
    }
    #[doc = "Bit 4 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W {
        CRCEIE_W { w: self }
    }
    #[doc = "Bit 3 - ECC Multi-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccmeie(&mut self) -> ECCMEIE_W {
        ECCMEIE_W { w: self }
    }
    #[doc = "Bit 2 - ECC Single-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W {
        ECCSEIE_W { w: self }
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception Interrupt Enable"]
    #[inline(always)]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W {
        TOLPRXIE_W { w: self }
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W {
        TOHSTXIE_W { w: self }
    }
}
