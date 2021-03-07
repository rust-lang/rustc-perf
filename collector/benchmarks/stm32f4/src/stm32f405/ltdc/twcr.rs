#[doc = "Reader of register TWCR"]
pub type R = crate::R<u32, super::TWCR>;
#[doc = "Writer for register TWCR"]
pub type W = crate::W<u32, super::TWCR>;
#[doc = "Register TWCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TWCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOTALW`"]
pub type TOTALW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOTALW`"]
pub struct TOTALW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOTALH`"]
pub type TOTALH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOTALH`"]
pub struct TOTALH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn totalw(&mut self) -> TOTALW_W {
        TOTALW_W { w: self }
    }
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn totalh(&mut self) -> TOTALH_W {
        TOTALH_W { w: self }
    }
}
