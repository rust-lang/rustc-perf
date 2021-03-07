#[doc = "Reader of register DMA2D_ISR"]
pub type R = crate::R<u32, super::DMA2D_ISR>;
#[doc = "Reader of field `TEIF`"]
pub type TEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF`"]
pub type TCIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TWIF`"]
pub type TWIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAEIF`"]
pub type CAEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF`"]
pub type CTCIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEIF`"]
pub type CEIF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transfer error interrupt flag This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading)."]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt flag This bit is set when a DMA2D transfer operation is complete (data transfer only)."]
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer watermark interrupt flag This bit is set when the last pixel of the watermarked line has been transferred."]
    #[inline(always)]
    pub fn twif(&self) -> TWIF_R {
        TWIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLUT access error interrupt flag This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D."]
    #[inline(always)]
    pub fn caeif(&self) -> CAEIF_R {
        CAEIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLUT transfer complete interrupt flag This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete."]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configuration error interrupt flag This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed."]
    #[inline(always)]
    pub fn ceif(&self) -> CEIF_R {
        CEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
