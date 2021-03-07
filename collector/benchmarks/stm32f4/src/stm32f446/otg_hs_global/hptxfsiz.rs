#[doc = "Reader of register HPTXFSIZ"]
pub type R = crate::R<u32, super::HPTXFSIZ>;
#[doc = "Writer for register HPTXFSIZ"]
pub type W = crate::W<u32, super::HPTXFSIZ>;
#[doc = "Register HPTXFSIZ `reset()`'s with value 0x0200_0600"]
impl crate::ResetValue for super::HPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0600
    }
}
#[doc = "Reader of field `PTXSA`"]
pub type PTXSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTXSA`"]
pub struct PTXSA_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PTXFD`"]
pub type PTXFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTXFD`"]
pub struct PTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&self) -> PTXFD_R {
        PTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W {
        PTXSA_W { w: self }
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PTXFD_W {
        PTXFD_W { w: self }
    }
}
