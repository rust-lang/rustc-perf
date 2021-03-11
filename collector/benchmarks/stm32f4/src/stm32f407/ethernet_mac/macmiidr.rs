#[doc = "Reader of register MACMIIDR"]
pub type R = crate::R<u32, super::MACMIIDR>;
#[doc = "Writer for register MACMIIDR"]
pub type W = crate::W<u32, super::MACMIIDR>;
#[doc = "Register MACMIIDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACMIIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MD`"]
pub type MD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MD`"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MII data read from/written to the PHY"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data read from/written to the PHY"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
}
