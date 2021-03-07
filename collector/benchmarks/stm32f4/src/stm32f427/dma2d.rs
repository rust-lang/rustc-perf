#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    pub dma2d_cr: DMA2D_CR,
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    pub dma2d_isr: DMA2D_ISR,
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    pub dma2d_ifcr: DMA2D_IFCR,
    #[doc = "0x0c - DMA2D foreground memory address register"]
    pub dma2d_fgmar: DMA2D_FGMAR,
    #[doc = "0x10 - DMA2D foreground offset register"]
    pub dma2d_fgor: DMA2D_FGOR,
    #[doc = "0x14 - DMA2D background memory address register"]
    pub dma2d_bgmar: DMA2D_BGMAR,
    #[doc = "0x18 - DMA2D background offset register"]
    pub dma2d_bgor: DMA2D_BGOR,
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    pub dma2d_fgpfccr: DMA2D_FGPFCCR,
    #[doc = "0x20 - DMA2D foreground color register"]
    pub dma2d_fgcolr: DMA2D_FGCOLR,
    #[doc = "0x24 - DMA2D background PFC control register"]
    pub dma2d_bgpfccr: DMA2D_BGPFCCR,
    #[doc = "0x28 - DMA2D background color register"]
    pub dma2d_bgcolr: DMA2D_BGCOLR,
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    pub dma2d_fgcmar: DMA2D_FGCMAR,
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    pub dma2d_bgcmar: DMA2D_BGCMAR,
    #[doc = "0x34 - DMA2D output PFC control register"]
    pub dma2d_opfccr: DMA2D_OPFCCR,
    #[doc = "0x38 - DMA2D output color register"]
    pub dma2d_ocolr: DMA2D_OCOLR,
    #[doc = "0x3c - DMA2D output memory address register"]
    pub dma2d_omar: DMA2D_OMAR,
    #[doc = "0x40 - DMA2D output offset register"]
    pub dma2d_oor: DMA2D_OOR,
    #[doc = "0x44 - DMA2D number of line register"]
    pub dma2d_nlr: DMA2D_NLR,
    #[doc = "0x48 - DMA2D line watermark register"]
    pub dma2d_lwr: DMA2D_LWR,
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    pub dma2d_amtcr: DMA2D_AMTCR,
}
#[doc = "DMA2D control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_cr](dma2d_cr) module"]
pub type DMA2D_CR = crate::Reg<u32, _DMA2D_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_CR;
#[doc = "`read()` method returns [dma2d_cr::R](dma2d_cr::R) reader structure"]
impl crate::Readable for DMA2D_CR {}
#[doc = "`write(|w| ..)` method takes [dma2d_cr::W](dma2d_cr::W) writer structure"]
impl crate::Writable for DMA2D_CR {}
#[doc = "DMA2D control register"]
pub mod dma2d_cr;
#[doc = "DMA2D Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_isr](dma2d_isr) module"]
pub type DMA2D_ISR = crate::Reg<u32, _DMA2D_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_ISR;
#[doc = "`read()` method returns [dma2d_isr::R](dma2d_isr::R) reader structure"]
impl crate::Readable for DMA2D_ISR {}
#[doc = "DMA2D Interrupt Status Register"]
pub mod dma2d_isr;
#[doc = "DMA2D interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_ifcr](dma2d_ifcr) module"]
pub type DMA2D_IFCR = crate::Reg<u32, _DMA2D_IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_IFCR;
#[doc = "`read()` method returns [dma2d_ifcr::R](dma2d_ifcr::R) reader structure"]
impl crate::Readable for DMA2D_IFCR {}
#[doc = "`write(|w| ..)` method takes [dma2d_ifcr::W](dma2d_ifcr::W) writer structure"]
impl crate::Writable for DMA2D_IFCR {}
#[doc = "DMA2D interrupt flag clear register"]
pub mod dma2d_ifcr;
#[doc = "DMA2D foreground memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_fgmar](dma2d_fgmar) module"]
pub type DMA2D_FGMAR = crate::Reg<u32, _DMA2D_FGMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_FGMAR;
#[doc = "`read()` method returns [dma2d_fgmar::R](dma2d_fgmar::R) reader structure"]
impl crate::Readable for DMA2D_FGMAR {}
#[doc = "`write(|w| ..)` method takes [dma2d_fgmar::W](dma2d_fgmar::W) writer structure"]
impl crate::Writable for DMA2D_FGMAR {}
#[doc = "DMA2D foreground memory address register"]
pub mod dma2d_fgmar;
#[doc = "DMA2D foreground offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_fgor](dma2d_fgor) module"]
pub type DMA2D_FGOR = crate::Reg<u32, _DMA2D_FGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_FGOR;
#[doc = "`read()` method returns [dma2d_fgor::R](dma2d_fgor::R) reader structure"]
impl crate::Readable for DMA2D_FGOR {}
#[doc = "`write(|w| ..)` method takes [dma2d_fgor::W](dma2d_fgor::W) writer structure"]
impl crate::Writable for DMA2D_FGOR {}
#[doc = "DMA2D foreground offset register"]
pub mod dma2d_fgor;
#[doc = "DMA2D background memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_bgmar](dma2d_bgmar) module"]
pub type DMA2D_BGMAR = crate::Reg<u32, _DMA2D_BGMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_BGMAR;
#[doc = "`read()` method returns [dma2d_bgmar::R](dma2d_bgmar::R) reader structure"]
impl crate::Readable for DMA2D_BGMAR {}
#[doc = "`write(|w| ..)` method takes [dma2d_bgmar::W](dma2d_bgmar::W) writer structure"]
impl crate::Writable for DMA2D_BGMAR {}
#[doc = "DMA2D background memory address register"]
pub mod dma2d_bgmar;
#[doc = "DMA2D background offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_bgor](dma2d_bgor) module"]
pub type DMA2D_BGOR = crate::Reg<u32, _DMA2D_BGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_BGOR;
#[doc = "`read()` method returns [dma2d_bgor::R](dma2d_bgor::R) reader structure"]
impl crate::Readable for DMA2D_BGOR {}
#[doc = "`write(|w| ..)` method takes [dma2d_bgor::W](dma2d_bgor::W) writer structure"]
impl crate::Writable for DMA2D_BGOR {}
#[doc = "DMA2D background offset register"]
pub mod dma2d_bgor;
#[doc = "DMA2D foreground PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_fgpfccr](dma2d_fgpfccr) module"]
pub type DMA2D_FGPFCCR = crate::Reg<u32, _DMA2D_FGPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_FGPFCCR;
#[doc = "`read()` method returns [dma2d_fgpfccr::R](dma2d_fgpfccr::R) reader structure"]
impl crate::Readable for DMA2D_FGPFCCR {}
#[doc = "`write(|w| ..)` method takes [dma2d_fgpfccr::W](dma2d_fgpfccr::W) writer structure"]
impl crate::Writable for DMA2D_FGPFCCR {}
#[doc = "DMA2D foreground PFC control register"]
pub mod dma2d_fgpfccr;
#[doc = "DMA2D foreground color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_fgcolr](dma2d_fgcolr) module"]
pub type DMA2D_FGCOLR = crate::Reg<u32, _DMA2D_FGCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_FGCOLR;
#[doc = "`read()` method returns [dma2d_fgcolr::R](dma2d_fgcolr::R) reader structure"]
impl crate::Readable for DMA2D_FGCOLR {}
#[doc = "`write(|w| ..)` method takes [dma2d_fgcolr::W](dma2d_fgcolr::W) writer structure"]
impl crate::Writable for DMA2D_FGCOLR {}
#[doc = "DMA2D foreground color register"]
pub mod dma2d_fgcolr;
#[doc = "DMA2D background PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_bgpfccr](dma2d_bgpfccr) module"]
pub type DMA2D_BGPFCCR = crate::Reg<u32, _DMA2D_BGPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_BGPFCCR;
#[doc = "`read()` method returns [dma2d_bgpfccr::R](dma2d_bgpfccr::R) reader structure"]
impl crate::Readable for DMA2D_BGPFCCR {}
#[doc = "`write(|w| ..)` method takes [dma2d_bgpfccr::W](dma2d_bgpfccr::W) writer structure"]
impl crate::Writable for DMA2D_BGPFCCR {}
#[doc = "DMA2D background PFC control register"]
pub mod dma2d_bgpfccr;
#[doc = "DMA2D background color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_bgcolr](dma2d_bgcolr) module"]
pub type DMA2D_BGCOLR = crate::Reg<u32, _DMA2D_BGCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_BGCOLR;
#[doc = "`read()` method returns [dma2d_bgcolr::R](dma2d_bgcolr::R) reader structure"]
impl crate::Readable for DMA2D_BGCOLR {}
#[doc = "`write(|w| ..)` method takes [dma2d_bgcolr::W](dma2d_bgcolr::W) writer structure"]
impl crate::Writable for DMA2D_BGCOLR {}
#[doc = "DMA2D background color register"]
pub mod dma2d_bgcolr;
#[doc = "DMA2D foreground CLUT memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_fgcmar](dma2d_fgcmar) module"]
pub type DMA2D_FGCMAR = crate::Reg<u32, _DMA2D_FGCMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_FGCMAR;
#[doc = "`read()` method returns [dma2d_fgcmar::R](dma2d_fgcmar::R) reader structure"]
impl crate::Readable for DMA2D_FGCMAR {}
#[doc = "`write(|w| ..)` method takes [dma2d_fgcmar::W](dma2d_fgcmar::W) writer structure"]
impl crate::Writable for DMA2D_FGCMAR {}
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod dma2d_fgcmar;
#[doc = "DMA2D background CLUT memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_bgcmar](dma2d_bgcmar) module"]
pub type DMA2D_BGCMAR = crate::Reg<u32, _DMA2D_BGCMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_BGCMAR;
#[doc = "`read()` method returns [dma2d_bgcmar::R](dma2d_bgcmar::R) reader structure"]
impl crate::Readable for DMA2D_BGCMAR {}
#[doc = "`write(|w| ..)` method takes [dma2d_bgcmar::W](dma2d_bgcmar::W) writer structure"]
impl crate::Writable for DMA2D_BGCMAR {}
#[doc = "DMA2D background CLUT memory address register"]
pub mod dma2d_bgcmar;
#[doc = "DMA2D output PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_opfccr](dma2d_opfccr) module"]
pub type DMA2D_OPFCCR = crate::Reg<u32, _DMA2D_OPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_OPFCCR;
#[doc = "`read()` method returns [dma2d_opfccr::R](dma2d_opfccr::R) reader structure"]
impl crate::Readable for DMA2D_OPFCCR {}
#[doc = "`write(|w| ..)` method takes [dma2d_opfccr::W](dma2d_opfccr::W) writer structure"]
impl crate::Writable for DMA2D_OPFCCR {}
#[doc = "DMA2D output PFC control register"]
pub mod dma2d_opfccr;
#[doc = "DMA2D output color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_ocolr](dma2d_ocolr) module"]
pub type DMA2D_OCOLR = crate::Reg<u32, _DMA2D_OCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_OCOLR;
#[doc = "`read()` method returns [dma2d_ocolr::R](dma2d_ocolr::R) reader structure"]
impl crate::Readable for DMA2D_OCOLR {}
#[doc = "`write(|w| ..)` method takes [dma2d_ocolr::W](dma2d_ocolr::W) writer structure"]
impl crate::Writable for DMA2D_OCOLR {}
#[doc = "DMA2D output color register"]
pub mod dma2d_ocolr;
#[doc = "DMA2D output memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_omar](dma2d_omar) module"]
pub type DMA2D_OMAR = crate::Reg<u32, _DMA2D_OMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_OMAR;
#[doc = "`read()` method returns [dma2d_omar::R](dma2d_omar::R) reader structure"]
impl crate::Readable for DMA2D_OMAR {}
#[doc = "`write(|w| ..)` method takes [dma2d_omar::W](dma2d_omar::W) writer structure"]
impl crate::Writable for DMA2D_OMAR {}
#[doc = "DMA2D output memory address register"]
pub mod dma2d_omar;
#[doc = "DMA2D output offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_oor](dma2d_oor) module"]
pub type DMA2D_OOR = crate::Reg<u32, _DMA2D_OOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_OOR;
#[doc = "`read()` method returns [dma2d_oor::R](dma2d_oor::R) reader structure"]
impl crate::Readable for DMA2D_OOR {}
#[doc = "`write(|w| ..)` method takes [dma2d_oor::W](dma2d_oor::W) writer structure"]
impl crate::Writable for DMA2D_OOR {}
#[doc = "DMA2D output offset register"]
pub mod dma2d_oor;
#[doc = "DMA2D number of line register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_nlr](dma2d_nlr) module"]
pub type DMA2D_NLR = crate::Reg<u32, _DMA2D_NLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_NLR;
#[doc = "`read()` method returns [dma2d_nlr::R](dma2d_nlr::R) reader structure"]
impl crate::Readable for DMA2D_NLR {}
#[doc = "`write(|w| ..)` method takes [dma2d_nlr::W](dma2d_nlr::W) writer structure"]
impl crate::Writable for DMA2D_NLR {}
#[doc = "DMA2D number of line register"]
pub mod dma2d_nlr;
#[doc = "DMA2D line watermark register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_lwr](dma2d_lwr) module"]
pub type DMA2D_LWR = crate::Reg<u32, _DMA2D_LWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_LWR;
#[doc = "`read()` method returns [dma2d_lwr::R](dma2d_lwr::R) reader structure"]
impl crate::Readable for DMA2D_LWR {}
#[doc = "`write(|w| ..)` method takes [dma2d_lwr::W](dma2d_lwr::W) writer structure"]
impl crate::Writable for DMA2D_LWR {}
#[doc = "DMA2D line watermark register"]
pub mod dma2d_lwr;
#[doc = "DMA2D AXI master timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_amtcr](dma2d_amtcr) module"]
pub type DMA2D_AMTCR = crate::Reg<u32, _DMA2D_AMTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2D_AMTCR;
#[doc = "`read()` method returns [dma2d_amtcr::R](dma2d_amtcr::R) reader structure"]
impl crate::Readable for DMA2D_AMTCR {}
#[doc = "`write(|w| ..)` method takes [dma2d_amtcr::W](dma2d_amtcr::W) writer structure"]
impl crate::Writable for DMA2D_AMTCR {}
#[doc = "DMA2D AXI master timer configuration register"]
pub mod dma2d_amtcr;
