#[doc = "Reader of register M1AR"]
pub type R = crate::R<u32, super::M1AR>;
#[doc = "Writer for register M1AR"]
pub type W = crate::W<u32, super::M1AR>;
#[doc = "Register M1AR `reset()`'s with value 0"]
impl crate::ResetValue for super::M1AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `M1A`"]
pub type M1A_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `M1A`"]
pub struct M1A_W<'a> {
    w: &'a mut W,
}
impl<'a> M1A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&mut self) -> M1A_W {
        M1A_W { w: self }
    }
}
