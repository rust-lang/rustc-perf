#[doc = "Reader of register MMCTIR"]
pub type R = crate::R<u32, super::MMCTIR>;
#[doc = "Reader of field `TGFSCS`"]
pub type TGFSCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TGFMSCS`"]
pub type TGFMSCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TGFS`"]
pub type TGFS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
