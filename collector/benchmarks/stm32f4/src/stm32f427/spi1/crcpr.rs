#[doc = "Reader of register CRCPR"]
pub type R = crate::R<u32, super::CRCPR>;
#[doc = "Writer for register CRCPR"]
pub type W = crate::W<u32, super::CRCPR>;
#[doc = "Register CRCPR `reset()`'s with value 0x07"]
impl crate::ResetValue for super::CRCPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `CRCPOLY`"]
pub type CRCPOLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRCPOLY`"]
pub struct CRCPOLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPOLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W { w: self }
    }
}
