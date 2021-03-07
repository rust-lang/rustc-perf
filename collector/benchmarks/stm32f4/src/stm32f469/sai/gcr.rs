#[doc = "Reader of register GCR"]
pub type R = crate::R<u32, super::GCR>;
#[doc = "Writer for register GCR"]
pub type W = crate::W<u32, super::GCR>;
#[doc = "Register GCR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNCOUT`"]
pub type SYNCOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCOUT`"]
pub struct SYNCOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Synchronization outputs"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronization outputs"]
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W {
        SYNCOUT_W { w: self }
    }
}
