#[doc = "Reader of register DSI_MCR"]
pub type R = crate::R<u32, super::DSI_MCR>;
#[doc = "Writer for register DSI_MCR"]
pub type W = crate::W<u32, super::DSI_MCR>;
#[doc = "Register DSI_MCR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DSI_MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `CMDM`"]
pub type CMDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDM`"]
pub struct CMDM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDM_W<'a> {
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
    #[doc = "Bit 0 - Command mode"]
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command mode"]
    #[inline(always)]
    pub fn cmdm(&mut self) -> CMDM_W {
        CMDM_W { w: self }
    }
}
