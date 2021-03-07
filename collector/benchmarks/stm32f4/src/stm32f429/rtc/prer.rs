#[doc = "Reader of register PRER"]
pub type R = crate::R<u32, super::PRER>;
#[doc = "Writer for register PRER"]
pub type W = crate::W<u32, super::PRER>;
#[doc = "Register PRER `reset()`'s with value 0x007f_00ff"]
impl crate::ResetValue for super::PRER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x007f_00ff
    }
}
#[doc = "Reader of field `PREDIV_A`"]
pub type PREDIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PREDIV_A`"]
pub struct PREDIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PREDIV_S`"]
pub type PREDIV_S_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PREDIV_S`"]
pub struct PREDIV_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&self) -> PREDIV_A_R {
        PREDIV_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&self) -> PREDIV_S_R {
        PREDIV_S_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&mut self) -> PREDIV_A_W {
        PREDIV_A_W { w: self }
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&mut self) -> PREDIV_S_W {
        PREDIV_S_W { w: self }
    }
}
