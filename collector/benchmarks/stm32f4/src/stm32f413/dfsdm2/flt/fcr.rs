#[doc = "Reader of register FCR"]
pub type R = crate::R<u32, super::FCR>;
#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORD`"]
pub type FORD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORD`"]
pub struct FORD_W<'a> {
    w: &'a mut W,
}
impl<'a> FORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `FOSR`"]
pub type FOSR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FOSR`"]
pub struct FOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `IOSR`"]
pub type IOSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSR`"]
pub struct IOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    pub fn ford(&mut self) -> FORD_W {
        FORD_W { w: self }
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    pub fn fosr(&mut self) -> FOSR_W {
        FOSR_W { w: self }
    }
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    pub fn iosr(&mut self) -> IOSR_W {
        IOSR_W { w: self }
    }
}
