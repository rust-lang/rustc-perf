#[doc = "Reader of register PMC"]
pub type R = crate::R<u32, super::PMC>;
#[doc = "Writer for register PMC"]
pub type W = crate::W<u32, super::PMC>;
#[doc = "Register PMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MII_RMII_SEL`"]
pub type MII_RMII_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MII_RMII_SEL`"]
pub struct MII_RMII_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_RMII_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W {
        MII_RMII_SEL_W { w: self }
    }
}
