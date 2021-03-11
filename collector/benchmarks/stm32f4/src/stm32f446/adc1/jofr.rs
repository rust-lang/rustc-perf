#[doc = "Reader of register JOFR%s"]
pub type R = crate::R<u32, super::JOFR>;
#[doc = "Writer for register JOFR%s"]
pub type W = crate::W<u32, super::JOFR>;
#[doc = "Register JOFR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::JOFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JOFFSET`"]
pub type JOFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `JOFFSET`"]
pub struct JOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> JOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset(&self) -> JOFFSET_R {
        JOFFSET_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset(&mut self) -> JOFFSET_W {
        JOFFSET_W { w: self }
    }
}
