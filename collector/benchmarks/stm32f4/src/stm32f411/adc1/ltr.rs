#[doc = "Reader of register LTR"]
pub type R = crate::R<u32, super::LTR>;
#[doc = "Writer for register LTR"]
pub type W = crate::W<u32, super::LTR>;
#[doc = "Register LTR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LT`"]
pub type LT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LT`"]
pub struct LT_W<'a> {
    w: &'a mut W,
}
impl<'a> LT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W {
        LT_W { w: self }
    }
}
