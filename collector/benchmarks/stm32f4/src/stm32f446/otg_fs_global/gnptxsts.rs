#[doc = "Reader of register GNPTXSTS"]
pub type R = crate::R<u32, super::GNPTXSTS>;
#[doc = "Reader of field `NPTXFSAV`"]
pub type NPTXFSAV_R = crate::R<u16, u16>;
#[doc = "Reader of field `NPTQXSAV`"]
pub type NPTQXSAV_R = crate::R<u8, u8>;
#[doc = "Reader of field `NPTXQTOP`"]
pub type NPTXQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
