#[doc = "Writer for register KRR"]
pub type W = crate::W<u32, super::KRR>;
#[doc = "Register KRR `reset()`'s with value 0"]
impl crate::ResetValue for super::KRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `b`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - b192"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
}
