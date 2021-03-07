#[doc = "Reader of register ARR"]
pub type R = crate::R<u32, super::ARR>;
#[doc = "Writer for register ARR"]
pub type W = crate::W<u32, super::ARR>;
#[doc = "Register ARR `reset()`'s with value 0"]
impl crate::ResetValue for super::ARR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARR_H`"]
pub type ARR_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARR_H`"]
pub struct ARR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ARR`"]
pub type ARR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARR`"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arr_h(&self) -> ARR_H_R {
        ARR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arr_h(&mut self) -> ARR_H_W {
        ARR_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
}
