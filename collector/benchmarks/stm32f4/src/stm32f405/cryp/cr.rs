#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALGODIR`"]
pub type ALGODIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALGODIR`"]
pub struct ALGODIR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGODIR_W<'a> {
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
#[doc = "Reader of field `ALGOMODE0`"]
pub type ALGOMODE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALGOMODE0`"]
pub struct ALGOMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGOMODE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `DATATYPE`"]
pub type DATATYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATATYPE`"]
pub struct DATATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `KEYSIZE`"]
pub type KEYSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEYSIZE`"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `FFLUSH`"]
pub struct FFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CRYPEN`"]
pub type CRYPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPEN`"]
pub struct CRYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `GCM_CCMPH`"]
pub type GCM_CCMPH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GCM_CCMPH`"]
pub struct GCM_CCMPH_W<'a> {
    w: &'a mut W,
}
impl<'a> GCM_CCMPH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALGOMODE3`"]
pub type ALGOMODE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALGOMODE3`"]
pub struct ALGOMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGOMODE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode0(&self) -> ALGOMODE0_R {
        ALGOMODE0_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GCM_CCMPH_R {
        GCM_CCMPH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode3(&self) -> ALGOMODE3_R {
        ALGOMODE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&mut self) -> ALGODIR_W {
        ALGODIR_W { w: self }
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode0(&mut self) -> ALGOMODE0_W {
        ALGOMODE0_W { w: self }
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W {
        DATATYPE_W { w: self }
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W { w: self }
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W {
        CRYPEN_W { w: self }
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&mut self) -> GCM_CCMPH_W {
        GCM_CCMPH_W { w: self }
    }
    #[doc = "Bit 19 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode3(&mut self) -> ALGOMODE3_W {
        ALGOMODE3_W { w: self }
    }
}
