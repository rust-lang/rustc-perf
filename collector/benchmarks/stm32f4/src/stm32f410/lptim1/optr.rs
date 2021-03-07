#[doc = "Reader of register OPTR"]
pub type R = crate::R<u32, super::OPTR>;
#[doc = "Writer for register OPTR"]
pub type W = crate::W<u32, super::OPTR>;
#[doc = "Register OPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OR`"]
pub type OR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OR`"]
pub struct OR_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - OR"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OR"]
    #[inline(always)]
    pub fn or(&mut self) -> OR_W {
        OR_W { w: self }
    }
}
