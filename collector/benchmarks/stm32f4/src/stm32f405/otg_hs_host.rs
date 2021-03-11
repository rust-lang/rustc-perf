#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub hcfg: HCFG,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub hfnum: HFNUM,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 36usize],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub hprt: HPRT,
    _reserved7: [u8; 188usize],
    #[doc = "0x100 - OTG_HS host channel-0 characteristics register"]
    pub hcchar0: HCCHAR0,
    #[doc = "0x104 - OTG_HS host channel-0 split control register"]
    pub hcsplt0: HCSPLT0,
    #[doc = "0x108 - OTG_HS host channel-11 interrupt register"]
    pub hcint0: HCINT0,
    #[doc = "0x10c - OTG_HS host channel-11 interrupt mask register"]
    pub hcintmsk0: HCINTMSK0,
    #[doc = "0x110 - OTG_HS host channel-11 transfer size register"]
    pub hctsiz0: HCTSIZ0,
    #[doc = "0x114 - OTG_HS host channel-0 DMA address register"]
    pub hcdma0: HCDMA0,
    _reserved13: [u8; 8usize],
    #[doc = "0x120 - OTG_HS host channel-1 characteristics register"]
    pub hcchar1: HCCHAR1,
    #[doc = "0x124 - OTG_HS host channel-1 split control register"]
    pub hcsplt1: HCSPLT1,
    #[doc = "0x128 - OTG_HS host channel-1 interrupt register"]
    pub hcint1: HCINT1,
    #[doc = "0x12c - OTG_HS host channel-1 interrupt mask register"]
    pub hcintmsk1: HCINTMSK1,
    #[doc = "0x130 - OTG_HS host channel-1 transfer size register"]
    pub hctsiz1: HCTSIZ1,
    #[doc = "0x134 - OTG_HS host channel-1 DMA address register"]
    pub hcdma1: HCDMA1,
    _reserved19: [u8; 8usize],
    #[doc = "0x140 - OTG_HS host channel-2 characteristics register"]
    pub hcchar2: HCCHAR2,
    #[doc = "0x144 - OTG_HS host channel-2 split control register"]
    pub hcsplt2: HCSPLT2,
    #[doc = "0x148 - OTG_HS host channel-2 interrupt register"]
    pub hcint2: HCINT2,
    #[doc = "0x14c - OTG_HS host channel-2 interrupt mask register"]
    pub hcintmsk2: HCINTMSK2,
    #[doc = "0x150 - OTG_HS host channel-2 transfer size register"]
    pub hctsiz2: HCTSIZ2,
    #[doc = "0x154 - OTG_HS host channel-2 DMA address register"]
    pub hcdma2: HCDMA2,
    _reserved25: [u8; 8usize],
    #[doc = "0x160 - OTG_HS host channel-3 characteristics register"]
    pub hcchar3: HCCHAR3,
    #[doc = "0x164 - OTG_HS host channel-3 split control register"]
    pub hcsplt3: HCSPLT3,
    #[doc = "0x168 - OTG_HS host channel-3 interrupt register"]
    pub hcint3: HCINT3,
    #[doc = "0x16c - OTG_HS host channel-3 interrupt mask register"]
    pub hcintmsk3: HCINTMSK3,
    #[doc = "0x170 - OTG_HS host channel-3 transfer size register"]
    pub hctsiz3: HCTSIZ3,
    #[doc = "0x174 - OTG_HS host channel-3 DMA address register"]
    pub hcdma3: HCDMA3,
    _reserved31: [u8; 8usize],
    #[doc = "0x180 - OTG_HS host channel-4 characteristics register"]
    pub hcchar4: HCCHAR4,
    #[doc = "0x184 - OTG_HS host channel-4 split control register"]
    pub hcsplt4: HCSPLT4,
    #[doc = "0x188 - OTG_HS host channel-4 interrupt register"]
    pub hcint4: HCINT4,
    #[doc = "0x18c - OTG_HS host channel-4 interrupt mask register"]
    pub hcintmsk4: HCINTMSK4,
    #[doc = "0x190 - OTG_HS host channel-4 transfer size register"]
    pub hctsiz4: HCTSIZ4,
    #[doc = "0x194 - OTG_HS host channel-4 DMA address register"]
    pub hcdma4: HCDMA4,
    _reserved37: [u8; 8usize],
    #[doc = "0x1a0 - OTG_HS host channel-5 characteristics register"]
    pub hcchar5: HCCHAR5,
    #[doc = "0x1a4 - OTG_HS host channel-5 split control register"]
    pub hcsplt5: HCSPLT5,
    #[doc = "0x1a8 - OTG_HS host channel-5 interrupt register"]
    pub hcint5: HCINT5,
    #[doc = "0x1ac - OTG_HS host channel-5 interrupt mask register"]
    pub hcintmsk5: HCINTMSK5,
    #[doc = "0x1b0 - OTG_HS host channel-5 transfer size register"]
    pub hctsiz5: HCTSIZ5,
    #[doc = "0x1b4 - OTG_HS host channel-5 DMA address register"]
    pub hcdma5: HCDMA5,
    _reserved43: [u8; 8usize],
    #[doc = "0x1c0 - OTG_HS host channel-6 characteristics register"]
    pub hcchar6: HCCHAR6,
    #[doc = "0x1c4 - OTG_HS host channel-6 split control register"]
    pub hcsplt6: HCSPLT6,
    #[doc = "0x1c8 - OTG_HS host channel-6 interrupt register"]
    pub hcint6: HCINT6,
    #[doc = "0x1cc - OTG_HS host channel-6 interrupt mask register"]
    pub hcintmsk6: HCINTMSK6,
    #[doc = "0x1d0 - OTG_HS host channel-6 transfer size register"]
    pub hctsiz6: HCTSIZ6,
    #[doc = "0x1d4 - OTG_HS host channel-6 DMA address register"]
    pub hcdma6: HCDMA6,
    _reserved49: [u8; 8usize],
    #[doc = "0x1e0 - OTG_HS host channel-7 characteristics register"]
    pub hcchar7: HCCHAR7,
    #[doc = "0x1e4 - OTG_HS host channel-7 split control register"]
    pub hcsplt7: HCSPLT7,
    #[doc = "0x1e8 - OTG_HS host channel-7 interrupt register"]
    pub hcint7: HCINT7,
    #[doc = "0x1ec - OTG_HS host channel-7 interrupt mask register"]
    pub hcintmsk7: HCINTMSK7,
    #[doc = "0x1f0 - OTG_HS host channel-7 transfer size register"]
    pub hctsiz7: HCTSIZ7,
    #[doc = "0x1f4 - OTG_HS host channel-7 DMA address register"]
    pub hcdma7: HCDMA7,
    _reserved55: [u8; 8usize],
    #[doc = "0x200 - OTG_HS host channel-8 characteristics register"]
    pub hcchar8: HCCHAR8,
    #[doc = "0x204 - OTG_HS host channel-8 split control register"]
    pub hcsplt8: HCSPLT8,
    #[doc = "0x208 - OTG_HS host channel-8 interrupt register"]
    pub hcint8: HCINT8,
    #[doc = "0x20c - OTG_HS host channel-8 interrupt mask register"]
    pub hcintmsk8: HCINTMSK8,
    #[doc = "0x210 - OTG_HS host channel-8 transfer size register"]
    pub hctsiz8: HCTSIZ8,
    #[doc = "0x214 - OTG_HS host channel-8 DMA address register"]
    pub hcdma8: HCDMA8,
    _reserved61: [u8; 8usize],
    #[doc = "0x220 - OTG_HS host channel-9 characteristics register"]
    pub hcchar9: HCCHAR9,
    #[doc = "0x224 - OTG_HS host channel-9 split control register"]
    pub hcsplt9: HCSPLT9,
    #[doc = "0x228 - OTG_HS host channel-9 interrupt register"]
    pub hcint9: HCINT9,
    #[doc = "0x22c - OTG_HS host channel-9 interrupt mask register"]
    pub hcintmsk9: HCINTMSK9,
    #[doc = "0x230 - OTG_HS host channel-9 transfer size register"]
    pub hctsiz9: HCTSIZ9,
    #[doc = "0x234 - OTG_HS host channel-9 DMA address register"]
    pub hcdma9: HCDMA9,
    _reserved67: [u8; 8usize],
    #[doc = "0x240 - OTG_HS host channel-10 characteristics register"]
    pub hcchar10: HCCHAR10,
    #[doc = "0x244 - OTG_HS host channel-10 split control register"]
    pub hcsplt10: HCSPLT10,
    #[doc = "0x248 - OTG_HS host channel-10 interrupt register"]
    pub hcint10: HCINT10,
    #[doc = "0x24c - OTG_HS host channel-10 interrupt mask register"]
    pub hcintmsk10: HCINTMSK10,
    #[doc = "0x250 - OTG_HS host channel-10 transfer size register"]
    pub hctsiz10: HCTSIZ10,
    #[doc = "0x254 - OTG_HS host channel-10 DMA address register"]
    pub hcdma10: HCDMA10,
    _reserved73: [u8; 8usize],
    #[doc = "0x260 - OTG_HS host channel-11 characteristics register"]
    pub hcchar11: HCCHAR11,
    #[doc = "0x264 - OTG_HS host channel-11 split control register"]
    pub hcsplt11: HCSPLT11,
    #[doc = "0x268 - OTG_HS host channel-11 interrupt register"]
    pub hcint11: HCINT11,
    #[doc = "0x26c - OTG_HS host channel-11 interrupt mask register"]
    pub hcintmsk11: HCINTMSK11,
    #[doc = "0x270 - OTG_HS host channel-11 transfer size register"]
    pub hctsiz11: HCTSIZ11,
    #[doc = "0x274 - OTG_HS host channel-11 DMA address register"]
    pub hcdma11: HCDMA11,
}
#[doc = "OTG_HS host configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfg](hcfg) module"]
pub type HCFG = crate::Reg<u32, _HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFG;
#[doc = "`read()` method returns [hcfg::R](hcfg::R) reader structure"]
impl crate::Readable for HCFG {}
#[doc = "`write(|w| ..)` method takes [hcfg::W](hcfg::W) writer structure"]
impl crate::Writable for HCFG {}
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "OTG_HS Host frame interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfir](hfir) module"]
pub type HFIR = crate::Reg<u32, _HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFIR;
#[doc = "`read()` method returns [hfir::R](hfir::R) reader structure"]
impl crate::Readable for HFIR {}
#[doc = "`write(|w| ..)` method takes [hfir::W](hfir::W) writer structure"]
impl crate::Writable for HFIR {}
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "OTG_HS host frame number/frame time remaining register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfnum](hfnum) module"]
pub type HFNUM = crate::Reg<u32, _HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFNUM;
#[doc = "`read()` method returns [hfnum::R](hfnum::R) reader structure"]
impl crate::Readable for HFNUM {}
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxsts](hptxsts) module"]
pub type HPTXSTS = crate::Reg<u32, _HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXSTS;
#[doc = "`read()` method returns [hptxsts::R](hptxsts::R) reader structure"]
impl crate::Readable for HPTXSTS {}
#[doc = "`write(|w| ..)` method takes [hptxsts::W](hptxsts::W) writer structure"]
impl crate::Writable for HPTXSTS {}
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "OTG_HS Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haint](haint) module"]
pub type HAINT = crate::Reg<u32, _HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINT;
#[doc = "`read()` method returns [haint::R](haint::R) reader structure"]
impl crate::Readable for HAINT {}
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "OTG_HS host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haintmsk](haintmsk) module"]
pub type HAINTMSK = crate::Reg<u32, _HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINTMSK;
#[doc = "`read()` method returns [haintmsk::R](haintmsk::R) reader structure"]
impl crate::Readable for HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [haintmsk::W](haintmsk::W) writer structure"]
impl crate::Writable for HAINTMSK {}
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "OTG_HS host port control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprt](hprt) module"]
pub type HPRT = crate::Reg<u32, _HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPRT;
#[doc = "`read()` method returns [hprt::R](hprt::R) reader structure"]
impl crate::Readable for HPRT {}
#[doc = "`write(|w| ..)` method takes [hprt::W](hprt::W) writer structure"]
impl crate::Writable for HPRT {}
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "OTG_HS host channel-0 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar0](hcchar0) module"]
pub type HCCHAR0 = crate::Reg<u32, _HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR0;
#[doc = "`read()` method returns [hcchar0::R](hcchar0::R) reader structure"]
impl crate::Readable for HCCHAR0 {}
#[doc = "`write(|w| ..)` method takes [hcchar0::W](hcchar0::W) writer structure"]
impl crate::Writable for HCCHAR0 {}
#[doc = "OTG_HS host channel-0 characteristics register"]
pub mod hcchar0;
#[doc = "OTG_HS host channel-1 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar1](hcchar1) module"]
pub type HCCHAR1 = crate::Reg<u32, _HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR1;
#[doc = "`read()` method returns [hcchar1::R](hcchar1::R) reader structure"]
impl crate::Readable for HCCHAR1 {}
#[doc = "`write(|w| ..)` method takes [hcchar1::W](hcchar1::W) writer structure"]
impl crate::Writable for HCCHAR1 {}
#[doc = "OTG_HS host channel-1 characteristics register"]
pub mod hcchar1;
#[doc = "OTG_HS host channel-2 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar2](hcchar2) module"]
pub type HCCHAR2 = crate::Reg<u32, _HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR2;
#[doc = "`read()` method returns [hcchar2::R](hcchar2::R) reader structure"]
impl crate::Readable for HCCHAR2 {}
#[doc = "`write(|w| ..)` method takes [hcchar2::W](hcchar2::W) writer structure"]
impl crate::Writable for HCCHAR2 {}
#[doc = "OTG_HS host channel-2 characteristics register"]
pub mod hcchar2;
#[doc = "OTG_HS host channel-3 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar3](hcchar3) module"]
pub type HCCHAR3 = crate::Reg<u32, _HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR3;
#[doc = "`read()` method returns [hcchar3::R](hcchar3::R) reader structure"]
impl crate::Readable for HCCHAR3 {}
#[doc = "`write(|w| ..)` method takes [hcchar3::W](hcchar3::W) writer structure"]
impl crate::Writable for HCCHAR3 {}
#[doc = "OTG_HS host channel-3 characteristics register"]
pub mod hcchar3;
#[doc = "OTG_HS host channel-4 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar4](hcchar4) module"]
pub type HCCHAR4 = crate::Reg<u32, _HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR4;
#[doc = "`read()` method returns [hcchar4::R](hcchar4::R) reader structure"]
impl crate::Readable for HCCHAR4 {}
#[doc = "`write(|w| ..)` method takes [hcchar4::W](hcchar4::W) writer structure"]
impl crate::Writable for HCCHAR4 {}
#[doc = "OTG_HS host channel-4 characteristics register"]
pub mod hcchar4;
#[doc = "OTG_HS host channel-5 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar5](hcchar5) module"]
pub type HCCHAR5 = crate::Reg<u32, _HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR5;
#[doc = "`read()` method returns [hcchar5::R](hcchar5::R) reader structure"]
impl crate::Readable for HCCHAR5 {}
#[doc = "`write(|w| ..)` method takes [hcchar5::W](hcchar5::W) writer structure"]
impl crate::Writable for HCCHAR5 {}
#[doc = "OTG_HS host channel-5 characteristics register"]
pub mod hcchar5;
#[doc = "OTG_HS host channel-6 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar6](hcchar6) module"]
pub type HCCHAR6 = crate::Reg<u32, _HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR6;
#[doc = "`read()` method returns [hcchar6::R](hcchar6::R) reader structure"]
impl crate::Readable for HCCHAR6 {}
#[doc = "`write(|w| ..)` method takes [hcchar6::W](hcchar6::W) writer structure"]
impl crate::Writable for HCCHAR6 {}
#[doc = "OTG_HS host channel-6 characteristics register"]
pub mod hcchar6;
#[doc = "OTG_HS host channel-7 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar7](hcchar7) module"]
pub type HCCHAR7 = crate::Reg<u32, _HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR7;
#[doc = "`read()` method returns [hcchar7::R](hcchar7::R) reader structure"]
impl crate::Readable for HCCHAR7 {}
#[doc = "`write(|w| ..)` method takes [hcchar7::W](hcchar7::W) writer structure"]
impl crate::Writable for HCCHAR7 {}
#[doc = "OTG_HS host channel-7 characteristics register"]
pub mod hcchar7;
#[doc = "OTG_HS host channel-8 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar8](hcchar8) module"]
pub type HCCHAR8 = crate::Reg<u32, _HCCHAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR8;
#[doc = "`read()` method returns [hcchar8::R](hcchar8::R) reader structure"]
impl crate::Readable for HCCHAR8 {}
#[doc = "`write(|w| ..)` method takes [hcchar8::W](hcchar8::W) writer structure"]
impl crate::Writable for HCCHAR8 {}
#[doc = "OTG_HS host channel-8 characteristics register"]
pub mod hcchar8;
#[doc = "OTG_HS host channel-9 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar9](hcchar9) module"]
pub type HCCHAR9 = crate::Reg<u32, _HCCHAR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR9;
#[doc = "`read()` method returns [hcchar9::R](hcchar9::R) reader structure"]
impl crate::Readable for HCCHAR9 {}
#[doc = "`write(|w| ..)` method takes [hcchar9::W](hcchar9::W) writer structure"]
impl crate::Writable for HCCHAR9 {}
#[doc = "OTG_HS host channel-9 characteristics register"]
pub mod hcchar9;
#[doc = "OTG_HS host channel-10 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar10](hcchar10) module"]
pub type HCCHAR10 = crate::Reg<u32, _HCCHAR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR10;
#[doc = "`read()` method returns [hcchar10::R](hcchar10::R) reader structure"]
impl crate::Readable for HCCHAR10 {}
#[doc = "`write(|w| ..)` method takes [hcchar10::W](hcchar10::W) writer structure"]
impl crate::Writable for HCCHAR10 {}
#[doc = "OTG_HS host channel-10 characteristics register"]
pub mod hcchar10;
#[doc = "OTG_HS host channel-11 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar11](hcchar11) module"]
pub type HCCHAR11 = crate::Reg<u32, _HCCHAR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR11;
#[doc = "`read()` method returns [hcchar11::R](hcchar11::R) reader structure"]
impl crate::Readable for HCCHAR11 {}
#[doc = "`write(|w| ..)` method takes [hcchar11::W](hcchar11::W) writer structure"]
impl crate::Writable for HCCHAR11 {}
#[doc = "OTG_HS host channel-11 characteristics register"]
pub mod hcchar11;
#[doc = "OTG_HS host channel-0 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt0](hcsplt0) module"]
pub type HCSPLT0 = crate::Reg<u32, _HCSPLT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT0;
#[doc = "`read()` method returns [hcsplt0::R](hcsplt0::R) reader structure"]
impl crate::Readable for HCSPLT0 {}
#[doc = "`write(|w| ..)` method takes [hcsplt0::W](hcsplt0::W) writer structure"]
impl crate::Writable for HCSPLT0 {}
#[doc = "OTG_HS host channel-0 split control register"]
pub mod hcsplt0;
#[doc = "OTG_HS host channel-1 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt1](hcsplt1) module"]
pub type HCSPLT1 = crate::Reg<u32, _HCSPLT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT1;
#[doc = "`read()` method returns [hcsplt1::R](hcsplt1::R) reader structure"]
impl crate::Readable for HCSPLT1 {}
#[doc = "`write(|w| ..)` method takes [hcsplt1::W](hcsplt1::W) writer structure"]
impl crate::Writable for HCSPLT1 {}
#[doc = "OTG_HS host channel-1 split control register"]
pub mod hcsplt1;
#[doc = "OTG_HS host channel-2 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt2](hcsplt2) module"]
pub type HCSPLT2 = crate::Reg<u32, _HCSPLT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT2;
#[doc = "`read()` method returns [hcsplt2::R](hcsplt2::R) reader structure"]
impl crate::Readable for HCSPLT2 {}
#[doc = "`write(|w| ..)` method takes [hcsplt2::W](hcsplt2::W) writer structure"]
impl crate::Writable for HCSPLT2 {}
#[doc = "OTG_HS host channel-2 split control register"]
pub mod hcsplt2;
#[doc = "OTG_HS host channel-3 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt3](hcsplt3) module"]
pub type HCSPLT3 = crate::Reg<u32, _HCSPLT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT3;
#[doc = "`read()` method returns [hcsplt3::R](hcsplt3::R) reader structure"]
impl crate::Readable for HCSPLT3 {}
#[doc = "`write(|w| ..)` method takes [hcsplt3::W](hcsplt3::W) writer structure"]
impl crate::Writable for HCSPLT3 {}
#[doc = "OTG_HS host channel-3 split control register"]
pub mod hcsplt3;
#[doc = "OTG_HS host channel-4 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt4](hcsplt4) module"]
pub type HCSPLT4 = crate::Reg<u32, _HCSPLT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT4;
#[doc = "`read()` method returns [hcsplt4::R](hcsplt4::R) reader structure"]
impl crate::Readable for HCSPLT4 {}
#[doc = "`write(|w| ..)` method takes [hcsplt4::W](hcsplt4::W) writer structure"]
impl crate::Writable for HCSPLT4 {}
#[doc = "OTG_HS host channel-4 split control register"]
pub mod hcsplt4;
#[doc = "OTG_HS host channel-5 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt5](hcsplt5) module"]
pub type HCSPLT5 = crate::Reg<u32, _HCSPLT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT5;
#[doc = "`read()` method returns [hcsplt5::R](hcsplt5::R) reader structure"]
impl crate::Readable for HCSPLT5 {}
#[doc = "`write(|w| ..)` method takes [hcsplt5::W](hcsplt5::W) writer structure"]
impl crate::Writable for HCSPLT5 {}
#[doc = "OTG_HS host channel-5 split control register"]
pub mod hcsplt5;
#[doc = "OTG_HS host channel-6 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt6](hcsplt6) module"]
pub type HCSPLT6 = crate::Reg<u32, _HCSPLT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT6;
#[doc = "`read()` method returns [hcsplt6::R](hcsplt6::R) reader structure"]
impl crate::Readable for HCSPLT6 {}
#[doc = "`write(|w| ..)` method takes [hcsplt6::W](hcsplt6::W) writer structure"]
impl crate::Writable for HCSPLT6 {}
#[doc = "OTG_HS host channel-6 split control register"]
pub mod hcsplt6;
#[doc = "OTG_HS host channel-7 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt7](hcsplt7) module"]
pub type HCSPLT7 = crate::Reg<u32, _HCSPLT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT7;
#[doc = "`read()` method returns [hcsplt7::R](hcsplt7::R) reader structure"]
impl crate::Readable for HCSPLT7 {}
#[doc = "`write(|w| ..)` method takes [hcsplt7::W](hcsplt7::W) writer structure"]
impl crate::Writable for HCSPLT7 {}
#[doc = "OTG_HS host channel-7 split control register"]
pub mod hcsplt7;
#[doc = "OTG_HS host channel-8 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt8](hcsplt8) module"]
pub type HCSPLT8 = crate::Reg<u32, _HCSPLT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT8;
#[doc = "`read()` method returns [hcsplt8::R](hcsplt8::R) reader structure"]
impl crate::Readable for HCSPLT8 {}
#[doc = "`write(|w| ..)` method takes [hcsplt8::W](hcsplt8::W) writer structure"]
impl crate::Writable for HCSPLT8 {}
#[doc = "OTG_HS host channel-8 split control register"]
pub mod hcsplt8;
#[doc = "OTG_HS host channel-9 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt9](hcsplt9) module"]
pub type HCSPLT9 = crate::Reg<u32, _HCSPLT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT9;
#[doc = "`read()` method returns [hcsplt9::R](hcsplt9::R) reader structure"]
impl crate::Readable for HCSPLT9 {}
#[doc = "`write(|w| ..)` method takes [hcsplt9::W](hcsplt9::W) writer structure"]
impl crate::Writable for HCSPLT9 {}
#[doc = "OTG_HS host channel-9 split control register"]
pub mod hcsplt9;
#[doc = "OTG_HS host channel-10 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt10](hcsplt10) module"]
pub type HCSPLT10 = crate::Reg<u32, _HCSPLT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT10;
#[doc = "`read()` method returns [hcsplt10::R](hcsplt10::R) reader structure"]
impl crate::Readable for HCSPLT10 {}
#[doc = "`write(|w| ..)` method takes [hcsplt10::W](hcsplt10::W) writer structure"]
impl crate::Writable for HCSPLT10 {}
#[doc = "OTG_HS host channel-10 split control register"]
pub mod hcsplt10;
#[doc = "OTG_HS host channel-11 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsplt11](hcsplt11) module"]
pub type HCSPLT11 = crate::Reg<u32, _HCSPLT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPLT11;
#[doc = "`read()` method returns [hcsplt11::R](hcsplt11::R) reader structure"]
impl crate::Readable for HCSPLT11 {}
#[doc = "`write(|w| ..)` method takes [hcsplt11::W](hcsplt11::W) writer structure"]
impl crate::Writable for HCSPLT11 {}
#[doc = "OTG_HS host channel-11 split control register"]
pub mod hcsplt11;
#[doc = "OTG_HS host channel-11 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint0](hcint0) module"]
pub type HCINT0 = crate::Reg<u32, _HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT0;
#[doc = "`read()` method returns [hcint0::R](hcint0::R) reader structure"]
impl crate::Readable for HCINT0 {}
#[doc = "`write(|w| ..)` method takes [hcint0::W](hcint0::W) writer structure"]
impl crate::Writable for HCINT0 {}
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod hcint0;
#[doc = "OTG_HS host channel-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint1](hcint1) module"]
pub type HCINT1 = crate::Reg<u32, _HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT1;
#[doc = "`read()` method returns [hcint1::R](hcint1::R) reader structure"]
impl crate::Readable for HCINT1 {}
#[doc = "`write(|w| ..)` method takes [hcint1::W](hcint1::W) writer structure"]
impl crate::Writable for HCINT1 {}
#[doc = "OTG_HS host channel-1 interrupt register"]
pub mod hcint1;
#[doc = "OTG_HS host channel-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint2](hcint2) module"]
pub type HCINT2 = crate::Reg<u32, _HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT2;
#[doc = "`read()` method returns [hcint2::R](hcint2::R) reader structure"]
impl crate::Readable for HCINT2 {}
#[doc = "`write(|w| ..)` method takes [hcint2::W](hcint2::W) writer structure"]
impl crate::Writable for HCINT2 {}
#[doc = "OTG_HS host channel-2 interrupt register"]
pub mod hcint2;
#[doc = "OTG_HS host channel-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint3](hcint3) module"]
pub type HCINT3 = crate::Reg<u32, _HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT3;
#[doc = "`read()` method returns [hcint3::R](hcint3::R) reader structure"]
impl crate::Readable for HCINT3 {}
#[doc = "`write(|w| ..)` method takes [hcint3::W](hcint3::W) writer structure"]
impl crate::Writable for HCINT3 {}
#[doc = "OTG_HS host channel-3 interrupt register"]
pub mod hcint3;
#[doc = "OTG_HS host channel-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint4](hcint4) module"]
pub type HCINT4 = crate::Reg<u32, _HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT4;
#[doc = "`read()` method returns [hcint4::R](hcint4::R) reader structure"]
impl crate::Readable for HCINT4 {}
#[doc = "`write(|w| ..)` method takes [hcint4::W](hcint4::W) writer structure"]
impl crate::Writable for HCINT4 {}
#[doc = "OTG_HS host channel-4 interrupt register"]
pub mod hcint4;
#[doc = "OTG_HS host channel-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint5](hcint5) module"]
pub type HCINT5 = crate::Reg<u32, _HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT5;
#[doc = "`read()` method returns [hcint5::R](hcint5::R) reader structure"]
impl crate::Readable for HCINT5 {}
#[doc = "`write(|w| ..)` method takes [hcint5::W](hcint5::W) writer structure"]
impl crate::Writable for HCINT5 {}
#[doc = "OTG_HS host channel-5 interrupt register"]
pub mod hcint5;
#[doc = "OTG_HS host channel-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint6](hcint6) module"]
pub type HCINT6 = crate::Reg<u32, _HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT6;
#[doc = "`read()` method returns [hcint6::R](hcint6::R) reader structure"]
impl crate::Readable for HCINT6 {}
#[doc = "`write(|w| ..)` method takes [hcint6::W](hcint6::W) writer structure"]
impl crate::Writable for HCINT6 {}
#[doc = "OTG_HS host channel-6 interrupt register"]
pub mod hcint6;
#[doc = "OTG_HS host channel-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint7](hcint7) module"]
pub type HCINT7 = crate::Reg<u32, _HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT7;
#[doc = "`read()` method returns [hcint7::R](hcint7::R) reader structure"]
impl crate::Readable for HCINT7 {}
#[doc = "`write(|w| ..)` method takes [hcint7::W](hcint7::W) writer structure"]
impl crate::Writable for HCINT7 {}
#[doc = "OTG_HS host channel-7 interrupt register"]
pub mod hcint7;
#[doc = "OTG_HS host channel-8 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint8](hcint8) module"]
pub type HCINT8 = crate::Reg<u32, _HCINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT8;
#[doc = "`read()` method returns [hcint8::R](hcint8::R) reader structure"]
impl crate::Readable for HCINT8 {}
#[doc = "`write(|w| ..)` method takes [hcint8::W](hcint8::W) writer structure"]
impl crate::Writable for HCINT8 {}
#[doc = "OTG_HS host channel-8 interrupt register"]
pub mod hcint8;
#[doc = "OTG_HS host channel-9 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint9](hcint9) module"]
pub type HCINT9 = crate::Reg<u32, _HCINT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT9;
#[doc = "`read()` method returns [hcint9::R](hcint9::R) reader structure"]
impl crate::Readable for HCINT9 {}
#[doc = "`write(|w| ..)` method takes [hcint9::W](hcint9::W) writer structure"]
impl crate::Writable for HCINT9 {}
#[doc = "OTG_HS host channel-9 interrupt register"]
pub mod hcint9;
#[doc = "OTG_HS host channel-10 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint10](hcint10) module"]
pub type HCINT10 = crate::Reg<u32, _HCINT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT10;
#[doc = "`read()` method returns [hcint10::R](hcint10::R) reader structure"]
impl crate::Readable for HCINT10 {}
#[doc = "`write(|w| ..)` method takes [hcint10::W](hcint10::W) writer structure"]
impl crate::Writable for HCINT10 {}
#[doc = "OTG_HS host channel-10 interrupt register"]
pub mod hcint10;
#[doc = "OTG_HS host channel-11 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint11](hcint11) module"]
pub type HCINT11 = crate::Reg<u32, _HCINT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT11;
#[doc = "`read()` method returns [hcint11::R](hcint11::R) reader structure"]
impl crate::Readable for HCINT11 {}
#[doc = "`write(|w| ..)` method takes [hcint11::W](hcint11::W) writer structure"]
impl crate::Writable for HCINT11 {}
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod hcint11;
#[doc = "OTG_HS host channel-11 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk0](hcintmsk0) module"]
pub type HCINTMSK0 = crate::Reg<u32, _HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK0;
#[doc = "`read()` method returns [hcintmsk0::R](hcintmsk0::R) reader structure"]
impl crate::Readable for HCINTMSK0 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk0::W](hcintmsk0::W) writer structure"]
impl crate::Writable for HCINTMSK0 {}
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod hcintmsk0;
#[doc = "OTG_HS host channel-1 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk1](hcintmsk1) module"]
pub type HCINTMSK1 = crate::Reg<u32, _HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK1;
#[doc = "`read()` method returns [hcintmsk1::R](hcintmsk1::R) reader structure"]
impl crate::Readable for HCINTMSK1 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk1::W](hcintmsk1::W) writer structure"]
impl crate::Writable for HCINTMSK1 {}
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub mod hcintmsk1;
#[doc = "OTG_HS host channel-2 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk2](hcintmsk2) module"]
pub type HCINTMSK2 = crate::Reg<u32, _HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK2;
#[doc = "`read()` method returns [hcintmsk2::R](hcintmsk2::R) reader structure"]
impl crate::Readable for HCINTMSK2 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk2::W](hcintmsk2::W) writer structure"]
impl crate::Writable for HCINTMSK2 {}
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub mod hcintmsk2;
#[doc = "OTG_HS host channel-3 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk3](hcintmsk3) module"]
pub type HCINTMSK3 = crate::Reg<u32, _HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK3;
#[doc = "`read()` method returns [hcintmsk3::R](hcintmsk3::R) reader structure"]
impl crate::Readable for HCINTMSK3 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk3::W](hcintmsk3::W) writer structure"]
impl crate::Writable for HCINTMSK3 {}
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub mod hcintmsk3;
#[doc = "OTG_HS host channel-4 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk4](hcintmsk4) module"]
pub type HCINTMSK4 = crate::Reg<u32, _HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK4;
#[doc = "`read()` method returns [hcintmsk4::R](hcintmsk4::R) reader structure"]
impl crate::Readable for HCINTMSK4 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk4::W](hcintmsk4::W) writer structure"]
impl crate::Writable for HCINTMSK4 {}
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub mod hcintmsk4;
#[doc = "OTG_HS host channel-5 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk5](hcintmsk5) module"]
pub type HCINTMSK5 = crate::Reg<u32, _HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK5;
#[doc = "`read()` method returns [hcintmsk5::R](hcintmsk5::R) reader structure"]
impl crate::Readable for HCINTMSK5 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk5::W](hcintmsk5::W) writer structure"]
impl crate::Writable for HCINTMSK5 {}
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub mod hcintmsk5;
#[doc = "OTG_HS host channel-6 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk6](hcintmsk6) module"]
pub type HCINTMSK6 = crate::Reg<u32, _HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK6;
#[doc = "`read()` method returns [hcintmsk6::R](hcintmsk6::R) reader structure"]
impl crate::Readable for HCINTMSK6 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk6::W](hcintmsk6::W) writer structure"]
impl crate::Writable for HCINTMSK6 {}
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub mod hcintmsk6;
#[doc = "OTG_HS host channel-7 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk7](hcintmsk7) module"]
pub type HCINTMSK7 = crate::Reg<u32, _HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK7;
#[doc = "`read()` method returns [hcintmsk7::R](hcintmsk7::R) reader structure"]
impl crate::Readable for HCINTMSK7 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk7::W](hcintmsk7::W) writer structure"]
impl crate::Writable for HCINTMSK7 {}
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub mod hcintmsk7;
#[doc = "OTG_HS host channel-8 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk8](hcintmsk8) module"]
pub type HCINTMSK8 = crate::Reg<u32, _HCINTMSK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK8;
#[doc = "`read()` method returns [hcintmsk8::R](hcintmsk8::R) reader structure"]
impl crate::Readable for HCINTMSK8 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk8::W](hcintmsk8::W) writer structure"]
impl crate::Writable for HCINTMSK8 {}
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub mod hcintmsk8;
#[doc = "OTG_HS host channel-9 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk9](hcintmsk9) module"]
pub type HCINTMSK9 = crate::Reg<u32, _HCINTMSK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK9;
#[doc = "`read()` method returns [hcintmsk9::R](hcintmsk9::R) reader structure"]
impl crate::Readable for HCINTMSK9 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk9::W](hcintmsk9::W) writer structure"]
impl crate::Writable for HCINTMSK9 {}
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub mod hcintmsk9;
#[doc = "OTG_HS host channel-10 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk10](hcintmsk10) module"]
pub type HCINTMSK10 = crate::Reg<u32, _HCINTMSK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK10;
#[doc = "`read()` method returns [hcintmsk10::R](hcintmsk10::R) reader structure"]
impl crate::Readable for HCINTMSK10 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk10::W](hcintmsk10::W) writer structure"]
impl crate::Writable for HCINTMSK10 {}
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub mod hcintmsk10;
#[doc = "OTG_HS host channel-11 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk11](hcintmsk11) module"]
pub type HCINTMSK11 = crate::Reg<u32, _HCINTMSK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK11;
#[doc = "`read()` method returns [hcintmsk11::R](hcintmsk11::R) reader structure"]
impl crate::Readable for HCINTMSK11 {}
#[doc = "`write(|w| ..)` method takes [hcintmsk11::W](hcintmsk11::W) writer structure"]
impl crate::Writable for HCINTMSK11 {}
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod hcintmsk11;
#[doc = "OTG_HS host channel-11 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz0](hctsiz0) module"]
pub type HCTSIZ0 = crate::Reg<u32, _HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ0;
#[doc = "`read()` method returns [hctsiz0::R](hctsiz0::R) reader structure"]
impl crate::Readable for HCTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [hctsiz0::W](hctsiz0::W) writer structure"]
impl crate::Writable for HCTSIZ0 {}
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod hctsiz0;
#[doc = "OTG_HS host channel-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz1](hctsiz1) module"]
pub type HCTSIZ1 = crate::Reg<u32, _HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ1;
#[doc = "`read()` method returns [hctsiz1::R](hctsiz1::R) reader structure"]
impl crate::Readable for HCTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [hctsiz1::W](hctsiz1::W) writer structure"]
impl crate::Writable for HCTSIZ1 {}
#[doc = "OTG_HS host channel-1 transfer size register"]
pub mod hctsiz1;
#[doc = "OTG_HS host channel-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz2](hctsiz2) module"]
pub type HCTSIZ2 = crate::Reg<u32, _HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ2;
#[doc = "`read()` method returns [hctsiz2::R](hctsiz2::R) reader structure"]
impl crate::Readable for HCTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [hctsiz2::W](hctsiz2::W) writer structure"]
impl crate::Writable for HCTSIZ2 {}
#[doc = "OTG_HS host channel-2 transfer size register"]
pub mod hctsiz2;
#[doc = "OTG_HS host channel-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz3](hctsiz3) module"]
pub type HCTSIZ3 = crate::Reg<u32, _HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ3;
#[doc = "`read()` method returns [hctsiz3::R](hctsiz3::R) reader structure"]
impl crate::Readable for HCTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [hctsiz3::W](hctsiz3::W) writer structure"]
impl crate::Writable for HCTSIZ3 {}
#[doc = "OTG_HS host channel-3 transfer size register"]
pub mod hctsiz3;
#[doc = "OTG_HS host channel-4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz4](hctsiz4) module"]
pub type HCTSIZ4 = crate::Reg<u32, _HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ4;
#[doc = "`read()` method returns [hctsiz4::R](hctsiz4::R) reader structure"]
impl crate::Readable for HCTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [hctsiz4::W](hctsiz4::W) writer structure"]
impl crate::Writable for HCTSIZ4 {}
#[doc = "OTG_HS host channel-4 transfer size register"]
pub mod hctsiz4;
#[doc = "OTG_HS host channel-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz5](hctsiz5) module"]
pub type HCTSIZ5 = crate::Reg<u32, _HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ5;
#[doc = "`read()` method returns [hctsiz5::R](hctsiz5::R) reader structure"]
impl crate::Readable for HCTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [hctsiz5::W](hctsiz5::W) writer structure"]
impl crate::Writable for HCTSIZ5 {}
#[doc = "OTG_HS host channel-5 transfer size register"]
pub mod hctsiz5;
#[doc = "OTG_HS host channel-6 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz6](hctsiz6) module"]
pub type HCTSIZ6 = crate::Reg<u32, _HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ6;
#[doc = "`read()` method returns [hctsiz6::R](hctsiz6::R) reader structure"]
impl crate::Readable for HCTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [hctsiz6::W](hctsiz6::W) writer structure"]
impl crate::Writable for HCTSIZ6 {}
#[doc = "OTG_HS host channel-6 transfer size register"]
pub mod hctsiz6;
#[doc = "OTG_HS host channel-7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz7](hctsiz7) module"]
pub type HCTSIZ7 = crate::Reg<u32, _HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ7;
#[doc = "`read()` method returns [hctsiz7::R](hctsiz7::R) reader structure"]
impl crate::Readable for HCTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [hctsiz7::W](hctsiz7::W) writer structure"]
impl crate::Writable for HCTSIZ7 {}
#[doc = "OTG_HS host channel-7 transfer size register"]
pub mod hctsiz7;
#[doc = "OTG_HS host channel-8 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz8](hctsiz8) module"]
pub type HCTSIZ8 = crate::Reg<u32, _HCTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ8;
#[doc = "`read()` method returns [hctsiz8::R](hctsiz8::R) reader structure"]
impl crate::Readable for HCTSIZ8 {}
#[doc = "`write(|w| ..)` method takes [hctsiz8::W](hctsiz8::W) writer structure"]
impl crate::Writable for HCTSIZ8 {}
#[doc = "OTG_HS host channel-8 transfer size register"]
pub mod hctsiz8;
#[doc = "OTG_HS host channel-9 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz9](hctsiz9) module"]
pub type HCTSIZ9 = crate::Reg<u32, _HCTSIZ9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ9;
#[doc = "`read()` method returns [hctsiz9::R](hctsiz9::R) reader structure"]
impl crate::Readable for HCTSIZ9 {}
#[doc = "`write(|w| ..)` method takes [hctsiz9::W](hctsiz9::W) writer structure"]
impl crate::Writable for HCTSIZ9 {}
#[doc = "OTG_HS host channel-9 transfer size register"]
pub mod hctsiz9;
#[doc = "OTG_HS host channel-10 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz10](hctsiz10) module"]
pub type HCTSIZ10 = crate::Reg<u32, _HCTSIZ10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ10;
#[doc = "`read()` method returns [hctsiz10::R](hctsiz10::R) reader structure"]
impl crate::Readable for HCTSIZ10 {}
#[doc = "`write(|w| ..)` method takes [hctsiz10::W](hctsiz10::W) writer structure"]
impl crate::Writable for HCTSIZ10 {}
#[doc = "OTG_HS host channel-10 transfer size register"]
pub mod hctsiz10;
#[doc = "OTG_HS host channel-11 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz11](hctsiz11) module"]
pub type HCTSIZ11 = crate::Reg<u32, _HCTSIZ11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ11;
#[doc = "`read()` method returns [hctsiz11::R](hctsiz11::R) reader structure"]
impl crate::Readable for HCTSIZ11 {}
#[doc = "`write(|w| ..)` method takes [hctsiz11::W](hctsiz11::W) writer structure"]
impl crate::Writable for HCTSIZ11 {}
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod hctsiz11;
#[doc = "OTG_HS host channel-0 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma0](hcdma0) module"]
pub type HCDMA0 = crate::Reg<u32, _HCDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA0;
#[doc = "`read()` method returns [hcdma0::R](hcdma0::R) reader structure"]
impl crate::Readable for HCDMA0 {}
#[doc = "`write(|w| ..)` method takes [hcdma0::W](hcdma0::W) writer structure"]
impl crate::Writable for HCDMA0 {}
#[doc = "OTG_HS host channel-0 DMA address register"]
pub mod hcdma0;
#[doc = "OTG_HS host channel-1 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma1](hcdma1) module"]
pub type HCDMA1 = crate::Reg<u32, _HCDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA1;
#[doc = "`read()` method returns [hcdma1::R](hcdma1::R) reader structure"]
impl crate::Readable for HCDMA1 {}
#[doc = "`write(|w| ..)` method takes [hcdma1::W](hcdma1::W) writer structure"]
impl crate::Writable for HCDMA1 {}
#[doc = "OTG_HS host channel-1 DMA address register"]
pub mod hcdma1;
#[doc = "OTG_HS host channel-2 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma2](hcdma2) module"]
pub type HCDMA2 = crate::Reg<u32, _HCDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA2;
#[doc = "`read()` method returns [hcdma2::R](hcdma2::R) reader structure"]
impl crate::Readable for HCDMA2 {}
#[doc = "`write(|w| ..)` method takes [hcdma2::W](hcdma2::W) writer structure"]
impl crate::Writable for HCDMA2 {}
#[doc = "OTG_HS host channel-2 DMA address register"]
pub mod hcdma2;
#[doc = "OTG_HS host channel-3 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma3](hcdma3) module"]
pub type HCDMA3 = crate::Reg<u32, _HCDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA3;
#[doc = "`read()` method returns [hcdma3::R](hcdma3::R) reader structure"]
impl crate::Readable for HCDMA3 {}
#[doc = "`write(|w| ..)` method takes [hcdma3::W](hcdma3::W) writer structure"]
impl crate::Writable for HCDMA3 {}
#[doc = "OTG_HS host channel-3 DMA address register"]
pub mod hcdma3;
#[doc = "OTG_HS host channel-4 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma4](hcdma4) module"]
pub type HCDMA4 = crate::Reg<u32, _HCDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA4;
#[doc = "`read()` method returns [hcdma4::R](hcdma4::R) reader structure"]
impl crate::Readable for HCDMA4 {}
#[doc = "`write(|w| ..)` method takes [hcdma4::W](hcdma4::W) writer structure"]
impl crate::Writable for HCDMA4 {}
#[doc = "OTG_HS host channel-4 DMA address register"]
pub mod hcdma4;
#[doc = "OTG_HS host channel-5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma5](hcdma5) module"]
pub type HCDMA5 = crate::Reg<u32, _HCDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA5;
#[doc = "`read()` method returns [hcdma5::R](hcdma5::R) reader structure"]
impl crate::Readable for HCDMA5 {}
#[doc = "`write(|w| ..)` method takes [hcdma5::W](hcdma5::W) writer structure"]
impl crate::Writable for HCDMA5 {}
#[doc = "OTG_HS host channel-5 DMA address register"]
pub mod hcdma5;
#[doc = "OTG_HS host channel-6 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma6](hcdma6) module"]
pub type HCDMA6 = crate::Reg<u32, _HCDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA6;
#[doc = "`read()` method returns [hcdma6::R](hcdma6::R) reader structure"]
impl crate::Readable for HCDMA6 {}
#[doc = "`write(|w| ..)` method takes [hcdma6::W](hcdma6::W) writer structure"]
impl crate::Writable for HCDMA6 {}
#[doc = "OTG_HS host channel-6 DMA address register"]
pub mod hcdma6;
#[doc = "OTG_HS host channel-7 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma7](hcdma7) module"]
pub type HCDMA7 = crate::Reg<u32, _HCDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA7;
#[doc = "`read()` method returns [hcdma7::R](hcdma7::R) reader structure"]
impl crate::Readable for HCDMA7 {}
#[doc = "`write(|w| ..)` method takes [hcdma7::W](hcdma7::W) writer structure"]
impl crate::Writable for HCDMA7 {}
#[doc = "OTG_HS host channel-7 DMA address register"]
pub mod hcdma7;
#[doc = "OTG_HS host channel-8 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma8](hcdma8) module"]
pub type HCDMA8 = crate::Reg<u32, _HCDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA8;
#[doc = "`read()` method returns [hcdma8::R](hcdma8::R) reader structure"]
impl crate::Readable for HCDMA8 {}
#[doc = "`write(|w| ..)` method takes [hcdma8::W](hcdma8::W) writer structure"]
impl crate::Writable for HCDMA8 {}
#[doc = "OTG_HS host channel-8 DMA address register"]
pub mod hcdma8;
#[doc = "OTG_HS host channel-9 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma9](hcdma9) module"]
pub type HCDMA9 = crate::Reg<u32, _HCDMA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA9;
#[doc = "`read()` method returns [hcdma9::R](hcdma9::R) reader structure"]
impl crate::Readable for HCDMA9 {}
#[doc = "`write(|w| ..)` method takes [hcdma9::W](hcdma9::W) writer structure"]
impl crate::Writable for HCDMA9 {}
#[doc = "OTG_HS host channel-9 DMA address register"]
pub mod hcdma9;
#[doc = "OTG_HS host channel-10 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma10](hcdma10) module"]
pub type HCDMA10 = crate::Reg<u32, _HCDMA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA10;
#[doc = "`read()` method returns [hcdma10::R](hcdma10::R) reader structure"]
impl crate::Readable for HCDMA10 {}
#[doc = "`write(|w| ..)` method takes [hcdma10::W](hcdma10::W) writer structure"]
impl crate::Writable for HCDMA10 {}
#[doc = "OTG_HS host channel-10 DMA address register"]
pub mod hcdma10;
#[doc = "OTG_HS host channel-11 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma11](hcdma11) module"]
pub type HCDMA11 = crate::Reg<u32, _HCDMA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA11;
#[doc = "`read()` method returns [hcdma11::R](hcdma11::R) reader structure"]
impl crate::Readable for HCDMA11 {}
#[doc = "`write(|w| ..)` method takes [hcdma11::W](hcdma11::W) writer structure"]
impl crate::Writable for HCDMA11 {}
#[doc = "OTG_HS host channel-11 DMA address register"]
pub mod hcdma11;
