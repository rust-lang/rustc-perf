#[doc = "Reader of register MMCRIMR"]
pub type R = crate::R<u32, super::MMCRIMR>;
#[doc = "Writer for register MMCRIMR"]
pub type W = crate::W<u32, super::MMCRIMR>;
#[doc = "Register MMCRIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCRIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFCEM`"]
pub type RFCEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFCEM`"]
pub struct RFCEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCEM_W<'a> {
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
#[doc = "Reader of field `RFAEM`"]
pub type RFAEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFAEM`"]
pub struct RFAEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFAEM_W<'a> {
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
#[doc = "Reader of field `RGUFM`"]
pub type RGUFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RGUFM`"]
pub struct RGUFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RGUFM_W<'a> {
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
impl R {
    #[doc = "Bit 5 - RFCEM"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RFAEM"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RGUFM"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RFCEM"]
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W {
        RFCEM_W { w: self }
    }
    #[doc = "Bit 6 - RFAEM"]
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W {
        RFAEM_W { w: self }
    }
    #[doc = "Bit 17 - RGUFM"]
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W {
        RGUFM_W { w: self }
    }
}
