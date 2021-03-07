#[doc = "Reader of register GOTGINT"]
pub type R = crate::R<u32, super::GOTGINT>;
#[doc = "Writer for register GOTGINT"]
pub type W = crate::W<u32, super::GOTGINT>;
#[doc = "Register GOTGINT `reset()`'s with value 0"]
impl crate::ResetValue for super::GOTGINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEDET`"]
pub type SEDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEDET`"]
pub struct SEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDET_W<'a> {
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
#[doc = "Reader of field `SRSSCHG`"]
pub type SRSSCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRSSCHG`"]
pub struct SRSSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSSCHG_W<'a> {
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
#[doc = "Reader of field `HNSSCHG`"]
pub type HNSSCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNSSCHG`"]
pub struct HNSSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> HNSSCHG_W<'a> {
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
#[doc = "Reader of field `HNGDET`"]
pub type HNGDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNGDET`"]
pub struct HNGDET_W<'a> {
    w: &'a mut W,
}
impl<'a> HNGDET_W<'a> {
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
#[doc = "Reader of field `ADTOCHG`"]
pub type ADTOCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADTOCHG`"]
pub struct ADTOCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTOCHG_W<'a> {
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
#[doc = "Reader of field `DBCDNE`"]
pub type DBCDNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBCDNE`"]
pub struct DBCDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCDNE_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&mut self) -> SEDET_W {
        SEDET_W { w: self }
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&mut self) -> SRSSCHG_W {
        SRSSCHG_W { w: self }
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&mut self) -> HNSSCHG_W {
        HNSSCHG_W { w: self }
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&mut self) -> HNGDET_W {
        HNGDET_W { w: self }
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&mut self) -> ADTOCHG_W {
        ADTOCHG_W { w: self }
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&mut self) -> DBCDNE_W {
        DBCDNE_W { w: self }
    }
}
