#[doc = "Reader of register WHPCR"]
pub type R = crate::R<u32, super::WHPCR>;
#[doc = "Writer for register WHPCR"]
pub type W = crate::W<u32, super::WHPCR>;
#[doc = "Register WHPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WHPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WHSPPOS`"]
pub type WHSPPOS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WHSPPOS`"]
pub struct WHSPPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WHSPPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WHSTPOS`"]
pub type WHSTPOS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WHSTPOS`"]
pub struct WHSTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WHSTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&mut self) -> WHSPPOS_W {
        WHSPPOS_W { w: self }
    }
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&mut self) -> WHSTPOS_W {
        WHSTPOS_W { w: self }
    }
}
