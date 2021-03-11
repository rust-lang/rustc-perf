#[doc = "Reader of register M0AR"]
pub type R = crate::R<u32, super::M0AR>;
#[doc = "Writer for register M0AR"]
pub type W = crate::W<u32, super::M0AR>;
#[doc = "Register M0AR `reset()`'s with value 0"]
impl crate::ResetValue for super::M0AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `M0A`"]
pub type M0A_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `M0A`"]
pub struct M0A_W<'a> {
    w: &'a mut W,
}
impl<'a> M0A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W {
        M0A_W { w: self }
    }
}
