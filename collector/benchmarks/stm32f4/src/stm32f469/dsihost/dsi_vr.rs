#[doc = "Reader of register DSI_VR"]
pub type R = crate::R<u32, super::DSI_VR>;
#[doc = "Writer for register DSI_VR"]
pub type W = crate::W<u32, super::DSI_VR>;
#[doc = "Register DSI_VR `reset()`'s with value 0x3133_302a"]
impl crate::ResetValue for super::DSI_VR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3133_302a
    }
}
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VERSION`"]
pub struct VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Version of the DSI Host"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Version of the DSI Host"]
    #[inline(always)]
    pub fn version(&mut self) -> VERSION_W {
        VERSION_W { w: self }
    }
}
