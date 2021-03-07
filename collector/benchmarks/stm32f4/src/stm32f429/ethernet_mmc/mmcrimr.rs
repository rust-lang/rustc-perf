#[doc = "Reader of register MMCRIMR"]
pub type R = crate::R<u32, super::MMCRIMR>;
#[doc = "Writer for register MMCRIMR"]
pub type W = crate::W<u32, super::MMCRIMR>;
#[doc = "Register MMCRIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCRIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Received frame CRC error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCEM_A {
    #[doc = "0: Received-crc-error counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Received-crc-error counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<RFCEM_A> for bool {
    #[inline(always)]
    fn from(variant: RFCEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFCEM`"]
pub type RFCEM_R = crate::R<bool, RFCEM_A>;
impl RFCEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCEM_A {
        match self.bits {
            false => RFCEM_A::UNMASKED,
            true => RFCEM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFCEM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFCEM_A::MASKED
    }
}
#[doc = "Write proxy for field `RFCEM`"]
pub struct RFCEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFCEM_A::UNMASKED)
    }
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFCEM_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Received frames alignment error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFAEM_A {
    #[doc = "0: Received-alignment-error counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Received-alignment-error counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<RFAEM_A> for bool {
    #[inline(always)]
    fn from(variant: RFAEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFAEM`"]
pub type RFAEM_R = crate::R<bool, RFAEM_A>;
impl RFAEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFAEM_A {
        match self.bits {
            false => RFAEM_A::UNMASKED,
            true => RFAEM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFAEM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFAEM_A::MASKED
    }
}
#[doc = "Write proxy for field `RFAEM`"]
pub struct RFAEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFAEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFAEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFAEM_A::UNMASKED)
    }
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFAEM_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Received good Unicast frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGUFM_A {
    #[doc = "0: Received-good-unicast counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Received-good-unicast counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<RGUFM_A> for bool {
    #[inline(always)]
    fn from(variant: RGUFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGUFM`"]
pub type RGUFM_R = crate::R<bool, RGUFM_A>;
impl RGUFM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGUFM_A {
        match self.bits {
            false => RGUFM_A::UNMASKED,
            true => RGUFM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RGUFM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RGUFM_A::MASKED
    }
}
#[doc = "Write proxy for field `RGUFM`"]
pub struct RGUFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RGUFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGUFM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RGUFM_A::UNMASKED)
    }
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RGUFM_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W {
        RFCEM_W { w: self }
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W {
        RFAEM_W { w: self }
    }
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W {
        RGUFM_W { w: self }
    }
}
