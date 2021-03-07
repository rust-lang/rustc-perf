#[doc = "Reader of register DSI_GPDR"]
pub type R = crate::R<u32, super::DSI_GPDR>;
#[doc = "Writer for register DSI_GPDR"]
pub type W = crate::W<u32, super::DSI_GPDR>;
#[doc = "Register DSI_GPDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_GPDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA4`"]
pub type DATA4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA4`"]
pub struct DATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `DATA3`"]
pub type DATA3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA3`"]
pub struct DATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA2`"]
pub type DATA2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA2`"]
pub struct DATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA1`"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Payload Byte 4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Payload Byte 3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Payload Byte 2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Payload Byte 1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Payload Byte 4"]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W { w: self }
    }
    #[doc = "Bits 16:23 - Payload Byte 3"]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W { w: self }
    }
    #[doc = "Bits 8:15 - Payload Byte 2"]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W { w: self }
    }
    #[doc = "Bits 0:7 - Payload Byte 1"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
}
