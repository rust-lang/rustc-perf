#[doc = "Reader of register TRISE"]
pub type R = crate::R<u32, super::TRISE>;
#[doc = "Writer for register TRISE"]
pub type W = crate::W<u32, super::TRISE>;
#[doc = "Register TRISE `reset()`'s with value 0x02"]
impl crate::ResetValue for super::TRISE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `TRISE`"]
pub type TRISE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRISE`"]
pub struct TRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn trise(&self) -> TRISE_R {
        TRISE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn trise(&mut self) -> TRISE_W {
        TRISE_W { w: self }
    }
}
