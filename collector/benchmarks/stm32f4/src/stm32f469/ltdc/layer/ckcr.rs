#[doc = "Reader of register CKCR"]
pub type R = crate::R<u32, super::CKCR>;
#[doc = "Writer for register CKCR"]
pub type W = crate::W<u32, super::CKCR>;
#[doc = "Register CKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKRED`"]
pub type CKRED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKRED`"]
pub struct CKRED_W<'a> {
    w: &'a mut W,
}
impl<'a> CKRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CKGREEN`"]
pub type CKGREEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKGREEN`"]
pub struct CKGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CKBLUE`"]
pub type CKBLUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKBLUE`"]
pub struct CKBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W {
        CKRED_W { w: self }
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W {
        CKGREEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W {
        CKBLUE_W { w: self }
    }
}
