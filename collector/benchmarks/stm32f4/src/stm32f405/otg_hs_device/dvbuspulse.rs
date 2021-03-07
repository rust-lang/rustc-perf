#[doc = "Reader of register DVBUSPULSE"]
pub type R = crate::R<u32, super::DVBUSPULSE>;
#[doc = "Writer for register DVBUSPULSE"]
pub type W = crate::W<u32, super::DVBUSPULSE>;
#[doc = "Register DVBUSPULSE `reset()`'s with value 0x05b8"]
impl crate::ResetValue for super::DVBUSPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05b8
    }
}
#[doc = "Reader of field `DVBUSP`"]
pub type DVBUSP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DVBUSP`"]
pub struct DVBUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DVBUSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W {
        DVBUSP_W { w: self }
    }
}
