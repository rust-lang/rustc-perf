#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMPI2C1_SCL`"]
pub type FMPI2C1_SCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMPI2C1_SCL`"]
pub struct FMPI2C1_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1_SCL_W<'a> {
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
#[doc = "Reader of field `FMPI2C1_SDA`"]
pub type FMPI2C1_SDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMPI2C1_SDA`"]
pub struct FMPI2C1_SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1_SDA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - FMPI2C1_SCL"]
    #[inline(always)]
    pub fn fmpi2c1_scl(&self) -> FMPI2C1_SCL_R {
        FMPI2C1_SCL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FMPI2C1_SDA"]
    #[inline(always)]
    pub fn fmpi2c1_sda(&self) -> FMPI2C1_SDA_R {
        FMPI2C1_SDA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMPI2C1_SCL"]
    #[inline(always)]
    pub fn fmpi2c1_scl(&mut self) -> FMPI2C1_SCL_W {
        FMPI2C1_SCL_W { w: self }
    }
    #[doc = "Bit 1 - FMPI2C1_SDA"]
    #[inline(always)]
    pub fn fmpi2c1_sda(&mut self) -> FMPI2C1_SDA_W {
        FMPI2C1_SDA_W { w: self }
    }
}
