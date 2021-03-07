#[doc = "Reader of register BCCR"]
pub type R = crate::R<u32, super::BCCR>;
#[doc = "Writer for register BCCR"]
pub type W = crate::W<u32, super::BCCR>;
#[doc = "Register BCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BC`"]
pub type BC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BC`"]
pub struct BC_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W {
        BC_W { w: self }
    }
}
