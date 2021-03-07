#[doc = "Reader of register CWSIZE"]
pub type R = crate::R<u32, super::CWSIZE>;
#[doc = "Writer for register CWSIZE"]
pub type W = crate::W<u32, super::CWSIZE>;
#[doc = "Register CWSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::CWSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLINE`"]
pub type VLINE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLINE`"]
pub struct VLINE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CAPCNT`"]
pub type CAPCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPCNT`"]
pub struct CAPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&mut self) -> VLINE_W {
        VLINE_W { w: self }
    }
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&mut self) -> CAPCNT_W {
        CAPCNT_W { w: self }
    }
}
