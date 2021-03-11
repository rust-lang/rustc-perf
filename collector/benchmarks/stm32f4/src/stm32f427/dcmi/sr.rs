#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `FNE`"]
pub type FNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSYNC`"]
pub type VSYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSYNC`"]
pub type HSYNC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - FIFO not empty"]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - VSYNC"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - HSYNC"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x01) != 0)
    }
}
