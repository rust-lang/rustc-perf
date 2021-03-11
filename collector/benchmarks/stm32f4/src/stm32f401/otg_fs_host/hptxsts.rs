#[doc = "Reader of register HPTXSTS"]
pub type R = crate::R<u32, super::HPTXSTS>;
#[doc = "Writer for register HPTXSTS"]
pub type W = crate::W<u32, super::HPTXSTS>;
#[doc = "Register HPTXSTS `reset()`'s with value 0x0008_0100"]
impl crate::ResetValue for super::HPTXSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0100
    }
}
#[doc = "Reader of field `PTXFSAVL`"]
pub type PTXFSAVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTXFSAVL`"]
pub struct PTXFSAVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSAVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PTXQSAV`"]
pub type PTXQSAV_R = crate::R<u8, u8>;
#[doc = "Reader of field `PTXQTOP`"]
pub type PTXQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfsavl(&mut self) -> PTXFSAVL_W {
        PTXFSAVL_W { w: self }
    }
}
