#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Writer for register IDR"]
pub type W = crate::W<u32, super::IDR>;
#[doc = "Register IDR `reset()`'s with value 0"]
impl crate::ResetValue for super::IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDR`"]
pub type IDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDR`"]
pub struct IDR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Independent Data register"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Independent Data register"]
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W {
        IDR_W { w: self }
    }
}
