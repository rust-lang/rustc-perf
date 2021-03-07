#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LINE_ISC`"]
pub struct LINE_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_ISC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `VSYNC_ISC`"]
pub struct VSYNC_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_ISC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `ERR_ISC`"]
pub struct ERR_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_ISC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `OVR_ISC`"]
pub struct OVR_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_ISC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `FRAME_ISC`"]
pub struct FRAME_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ISC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 4 - line interrupt status clear"]
    #[inline(always)]
    pub fn line_isc(&mut self) -> LINE_ISC_W {
        LINE_ISC_W { w: self }
    }
    #[doc = "Bit 3 - Vertical synch interrupt status clear"]
    #[inline(always)]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W {
        VSYNC_ISC_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization error interrupt status clear"]
    #[inline(always)]
    pub fn err_isc(&mut self) -> ERR_ISC_W {
        ERR_ISC_W { w: self }
    }
    #[doc = "Bit 1 - Overrun interrupt status clear"]
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W {
        OVR_ISC_W { w: self }
    }
    #[doc = "Bit 0 - Capture complete interrupt status clear"]
    #[inline(always)]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W {
        FRAME_ISC_W { w: self }
    }
}
