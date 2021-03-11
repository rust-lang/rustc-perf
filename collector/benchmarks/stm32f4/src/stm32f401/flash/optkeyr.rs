#[doc = "Writer for register OPTKEYR"]
pub type W = crate::W<u32, super::OPTKEYR>;
#[doc = "Register OPTKEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OPTKEY`"]
pub struct OPTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    pub fn optkey(&mut self) -> OPTKEY_W {
        OPTKEY_W { w: self }
    }
}
