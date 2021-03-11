#[doc = "Reader of register GNPTXFSIZ_Device"]
pub type R = crate::R<u32, super::GNPTXFSIZ_DEVICE>;
#[doc = "Writer for register GNPTXFSIZ_Device"]
pub type W = crate::W<u32, super::GNPTXFSIZ_DEVICE>;
#[doc = "Register GNPTXFSIZ_Device `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::GNPTXFSIZ_DEVICE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `TX0FSA`"]
pub type TX0FSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX0FSA`"]
pub struct TX0FSA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX0FSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TX0FD`"]
pub type TX0FD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX0FD`"]
pub struct TX0FD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX0FD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> TX0FSA_W {
        TX0FSA_W { w: self }
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&mut self) -> TX0FD_W {
        TX0FD_W { w: self }
    }
}
