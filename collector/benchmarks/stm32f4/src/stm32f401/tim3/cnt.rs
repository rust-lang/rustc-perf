#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Writer for register CNT"]
pub type W = crate::W<u32, super::CNT>;
#[doc = "Register CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_H`"]
pub type CNT_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_H`"]
pub struct CNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT`"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High counter value"]
    #[inline(always)]
    pub fn cnt_h(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High counter value"]
    #[inline(always)]
    pub fn cnt_h(&mut self) -> CNT_H_W {
        CNT_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
