#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub dsts: DSTS,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub daint: DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 8usize],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub deachint: DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub deachintmsk: DEACHINTMSK,
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    pub diepeachmsk1: DIEPEACHMSK1,
    _reserved14: [u8; 60usize],
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    pub doepeachmsk1: DOEPEACHMSK1,
    _reserved15: [u8; 124usize],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub diepctl0: DIEPCTL0,
    _reserved16: [u8; 4usize],
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub diepint0: DIEPINT,
    _reserved17: [u8; 4usize],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub diepdma1: DIEPDMA1,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS,
    _reserved20: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: DIEPCTL,
    _reserved21: [u8; 4usize],
    #[doc = "0x128 - OTG device endpoint-0 interrupt register"]
    pub diepint1: DIEPINT,
    _reserved22: [u8; 4usize],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz1: DIEPTSIZ,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub diepdma2: DIEPDMA2,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS,
    _reserved25: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-1 control register"]
    pub diepctl2: DIEPCTL,
    _reserved26: [u8; 4usize],
    #[doc = "0x148 - OTG device endpoint-0 interrupt register"]
    pub diepint2: DIEPINT,
    _reserved27: [u8; 4usize],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz2: DIEPTSIZ,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub diepdma3: DIEPDMA3,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS,
    _reserved30: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-1 control register"]
    pub diepctl3: DIEPCTL,
    _reserved31: [u8; 4usize],
    #[doc = "0x168 - OTG device endpoint-0 interrupt register"]
    pub diepint3: DIEPINT,
    _reserved32: [u8; 4usize],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz3: DIEPTSIZ,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub diepdma4: DIEPDMA4,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS,
    _reserved35: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-1 control register"]
    pub diepctl4: DIEPCTL,
    _reserved36: [u8; 4usize],
    #[doc = "0x188 - OTG device endpoint-0 interrupt register"]
    pub diepint4: DIEPINT,
    _reserved37: [u8; 4usize],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz4: DIEPTSIZ,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub diepdma5: DIEPDMA5,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: DTXFSTS,
    _reserved40: [u8; 4usize],
    #[doc = "0x1a0 - OTG device endpoint-1 control register"]
    pub diepctl5: DIEPCTL,
    _reserved41: [u8; 4usize],
    #[doc = "0x1a8 - OTG device endpoint-0 interrupt register"]
    pub diepint5: DIEPINT,
    _reserved42: [u8; 4usize],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz5: DIEPTSIZ,
    _reserved43: [u8; 4usize],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: DTXFSTS,
    _reserved44: [u8; 324usize],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved45: [u8; 4usize],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint0: DOEPINT,
    _reserved46: [u8; 4usize],
    #[doc = "0x310 - OTG_HS device endpoint-1 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    _reserved47: [u8; 12usize],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub doepctl1: DOEPCTL,
    _reserved48: [u8; 4usize],
    #[doc = "0x328 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint1: DOEPINT,
    _reserved49: [u8; 4usize],
    #[doc = "0x330 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz1: DOEPTSIZ,
    _reserved50: [u8; 12usize],
    #[doc = "0x340 - OTG device endpoint-1 control register"]
    pub doepctl2: DOEPCTL,
    _reserved51: [u8; 4usize],
    #[doc = "0x348 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint2: DOEPINT,
    _reserved52: [u8; 4usize],
    #[doc = "0x350 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz2: DOEPTSIZ,
    _reserved53: [u8; 12usize],
    #[doc = "0x360 - OTG device endpoint-1 control register"]
    pub doepctl3: DOEPCTL,
    _reserved54: [u8; 4usize],
    #[doc = "0x368 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint3: DOEPINT,
    _reserved55: [u8; 4usize],
    #[doc = "0x370 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz3: DOEPTSIZ,
    _reserved56: [u8; 12usize],
    #[doc = "0x380 - OTG device endpoint-1 control register"]
    pub doepctl4: DOEPCTL,
    _reserved57: [u8; 4usize],
    #[doc = "0x388 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint4: DOEPINT,
    _reserved58: [u8; 4usize],
    #[doc = "0x390 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz4: DOEPTSIZ,
    _reserved59: [u8; 12usize],
    #[doc = "0x3a0 - OTG device endpoint-1 control register"]
    pub doepctl5: DOEPCTL,
    _reserved60: [u8; 4usize],
    #[doc = "0x3a8 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint5: DOEPINT,
    _reserved61: [u8; 4usize],
    #[doc = "0x3b0 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz5: DOEPTSIZ,
}
#[doc = "OTG_HS device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "OTG_HS device control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "OTG_HS device status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](dsts) module"]
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
#[doc = "`read()` method returns [dsts::R](dsts::R) reader structure"]
impl crate::Readable for DSTS {}
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "OTG_HS device IN endpoint common interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepmsk](diepmsk) module"]
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
#[doc = "`read()` method returns [diepmsk::R](diepmsk::R) reader structure"]
impl crate::Readable for DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure"]
impl crate::Writable for DIEPMSK {}
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](doepmsk) module"]
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
#[doc = "`read()` method returns [doepmsk::R](doepmsk::R) reader structure"]
impl crate::Readable for DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure"]
impl crate::Writable for DOEPMSK {}
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "OTG_HS device all endpoints interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daint](daint) module"]
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
#[doc = "`read()` method returns [daint::R](daint::R) reader structure"]
impl crate::Readable for DAINT {}
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "OTG_HS all endpoints interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daintmsk](daintmsk) module"]
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
#[doc = "`read()` method returns [daintmsk::R](daintmsk::R) reader structure"]
impl crate::Readable for DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure"]
impl crate::Writable for DAINTMSK {}
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "OTG_HS device VBUS discharge time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbusdis](dvbusdis) module"]
pub type DVBUSDIS = crate::Reg<u32, _DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDIS;
#[doc = "`read()` method returns [dvbusdis::R](dvbusdis::R) reader structure"]
impl crate::Readable for DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [dvbusdis::W](dvbusdis::W) writer structure"]
impl crate::Writable for DVBUSDIS {}
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "OTG_HS device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbuspulse](dvbuspulse) module"]
pub type DVBUSPULSE = crate::Reg<u32, _DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPULSE;
#[doc = "`read()` method returns [dvbuspulse::R](dvbuspulse::R) reader structure"]
impl crate::Readable for DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [dvbuspulse::W](dvbuspulse::W) writer structure"]
impl crate::Writable for DVBUSPULSE {}
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "OTG_HS Device threshold control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dthrctl](dthrctl) module"]
pub type DTHRCTL = crate::Reg<u32, _DTHRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTHRCTL;
#[doc = "`read()` method returns [dthrctl::R](dthrctl::R) reader structure"]
impl crate::Readable for DTHRCTL {}
#[doc = "`write(|w| ..)` method takes [dthrctl::W](dthrctl::W) writer structure"]
impl crate::Writable for DTHRCTL {}
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepempmsk](diepempmsk) module"]
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
#[doc = "`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure"]
impl crate::Readable for DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure"]
impl crate::Writable for DIEPEMPMSK {}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "OTG_HS device each endpoint interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deachint](deachint) module"]
pub type DEACHINT = crate::Reg<u32, _DEACHINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEACHINT;
#[doc = "`read()` method returns [deachint::R](deachint::R) reader structure"]
impl crate::Readable for DEACHINT {}
#[doc = "`write(|w| ..)` method takes [deachint::W](deachint::W) writer structure"]
impl crate::Writable for DEACHINT {}
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deachintmsk](deachintmsk) module"]
pub type DEACHINTMSK = crate::Reg<u32, _DEACHINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEACHINTMSK;
#[doc = "`read()` method returns [deachintmsk::R](deachintmsk::R) reader structure"]
impl crate::Readable for DEACHINTMSK {}
#[doc = "`write(|w| ..)` method takes [deachintmsk::W](deachintmsk::W) writer structure"]
impl crate::Writable for DEACHINTMSK {}
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "OTG_HS device each in endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepeachmsk1](diepeachmsk1) module"]
pub type DIEPEACHMSK1 = crate::Reg<u32, _DIEPEACHMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEACHMSK1;
#[doc = "`read()` method returns [diepeachmsk1::R](diepeachmsk1::R) reader structure"]
impl crate::Readable for DIEPEACHMSK1 {}
#[doc = "`write(|w| ..)` method takes [diepeachmsk1::W](diepeachmsk1::W) writer structure"]
impl crate::Writable for DIEPEACHMSK1 {}
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod diepeachmsk1;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepeachmsk1](doepeachmsk1) module"]
pub type DOEPEACHMSK1 = crate::Reg<u32, _DOEPEACHMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPEACHMSK1;
#[doc = "`read()` method returns [doepeachmsk1::R](doepeachmsk1::R) reader structure"]
impl crate::Readable for DOEPEACHMSK1 {}
#[doc = "`write(|w| ..)` method takes [doepeachmsk1::W](doepeachmsk1::W) writer structure"]
impl crate::Writable for DOEPEACHMSK1 {}
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod doepeachmsk1;
#[doc = "OTG device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl0](diepctl0) module"]
pub type DIEPCTL0 = crate::Reg<u32, _DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL0;
#[doc = "`read()` method returns [diepctl0::R](diepctl0::R) reader structure"]
impl crate::Readable for DIEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [diepctl0::W](diepctl0::W) writer structure"]
impl crate::Writable for DIEPCTL0 {}
#[doc = "OTG device endpoint-0 control register"]
pub mod diepctl0;
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl](diepctl) module"]
pub type DIEPCTL = crate::Reg<u32, _DIEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL;
#[doc = "`read()` method returns [diepctl::R](diepctl::R) reader structure"]
impl crate::Readable for DIEPCTL {}
#[doc = "`write(|w| ..)` method takes [diepctl::W](diepctl::W) writer structure"]
impl crate::Writable for DIEPCTL {}
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl;
#[doc = "OTG device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint](diepint) module"]
pub type DIEPINT = crate::Reg<u32, _DIEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT;
#[doc = "`read()` method returns [diepint::R](diepint::R) reader structure"]
impl crate::Readable for DIEPINT {}
#[doc = "`write(|w| ..)` method takes [diepint::W](diepint::W) writer structure"]
impl crate::Writable for DIEPINT {}
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod diepint;
#[doc = "OTG_HS device IN endpoint 0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz0](dieptsiz0) module"]
pub type DIEPTSIZ0 = crate::Reg<u32, _DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ0;
#[doc = "`read()` method returns [dieptsiz0::R](dieptsiz0::R) reader structure"]
impl crate::Readable for DIEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz0::W](dieptsiz0::W) writer structure"]
impl crate::Writable for DIEPTSIZ0 {}
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod dieptsiz0;
#[doc = "OTG_HS device endpoint-1 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma1](diepdma1) module"]
pub type DIEPDMA1 = crate::Reg<u32, _DIEPDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA1;
#[doc = "`read()` method returns [diepdma1::R](diepdma1::R) reader structure"]
impl crate::Readable for DIEPDMA1 {}
#[doc = "`write(|w| ..)` method takes [diepdma1::W](diepdma1::W) writer structure"]
impl crate::Writable for DIEPDMA1 {}
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod diepdma1;
#[doc = "OTG_HS device endpoint-2 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma2](diepdma2) module"]
pub type DIEPDMA2 = crate::Reg<u32, _DIEPDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA2;
#[doc = "`read()` method returns [diepdma2::R](diepdma2::R) reader structure"]
impl crate::Readable for DIEPDMA2 {}
#[doc = "`write(|w| ..)` method takes [diepdma2::W](diepdma2::W) writer structure"]
impl crate::Writable for DIEPDMA2 {}
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod diepdma2;
#[doc = "OTG_HS device endpoint-3 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma3](diepdma3) module"]
pub type DIEPDMA3 = crate::Reg<u32, _DIEPDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA3;
#[doc = "`read()` method returns [diepdma3::R](diepdma3::R) reader structure"]
impl crate::Readable for DIEPDMA3 {}
#[doc = "`write(|w| ..)` method takes [diepdma3::W](diepdma3::W) writer structure"]
impl crate::Writable for DIEPDMA3 {}
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod diepdma3;
#[doc = "OTG_HS device endpoint-4 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma4](diepdma4) module"]
pub type DIEPDMA4 = crate::Reg<u32, _DIEPDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA4;
#[doc = "`read()` method returns [diepdma4::R](diepdma4::R) reader structure"]
impl crate::Readable for DIEPDMA4 {}
#[doc = "`write(|w| ..)` method takes [diepdma4::W](diepdma4::W) writer structure"]
impl crate::Writable for DIEPDMA4 {}
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod diepdma4;
#[doc = "OTG_HS device endpoint-5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma5](diepdma5) module"]
pub type DIEPDMA5 = crate::Reg<u32, _DIEPDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA5;
#[doc = "`read()` method returns [diepdma5::R](diepdma5::R) reader structure"]
impl crate::Readable for DIEPDMA5 {}
#[doc = "`write(|w| ..)` method takes [diepdma5::W](diepdma5::W) writer structure"]
impl crate::Writable for DIEPDMA5 {}
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod diepdma5;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts](dtxfsts) module"]
pub type DTXFSTS = crate::Reg<u32, _DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS;
#[doc = "`read()` method returns [dtxfsts::R](dtxfsts::R) reader structure"]
impl crate::Readable for DTXFSTS {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz](dieptsiz) module"]
pub type DIEPTSIZ = crate::Reg<u32, _DIEPTSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ;
#[doc = "`read()` method returns [dieptsiz::R](dieptsiz::R) reader structure"]
impl crate::Readable for DIEPTSIZ {}
#[doc = "`write(|w| ..)` method takes [dieptsiz::W](dieptsiz::W) writer structure"]
impl crate::Writable for DIEPTSIZ {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz;
#[doc = "OTG_HS device control OUT endpoint 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl0](doepctl0) module"]
pub type DOEPCTL0 = crate::Reg<u32, _DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL0;
#[doc = "`read()` method returns [doepctl0::R](doepctl0::R) reader structure"]
impl crate::Readable for DOEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [doepctl0::W](doepctl0::W) writer structure"]
impl crate::Writable for DOEPCTL0 {}
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod doepctl0;
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl](doepctl) module"]
pub type DOEPCTL = crate::Reg<u32, _DOEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL;
#[doc = "`read()` method returns [doepctl::R](doepctl::R) reader structure"]
impl crate::Readable for DOEPCTL {}
#[doc = "`write(|w| ..)` method takes [doepctl::W](doepctl::W) writer structure"]
impl crate::Writable for DOEPCTL {}
#[doc = "OTG device endpoint-1 control register"]
pub mod doepctl;
#[doc = "OTG_HS device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint](doepint) module"]
pub type DOEPINT = crate::Reg<u32, _DOEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT;
#[doc = "`read()` method returns [doepint::R](doepint::R) reader structure"]
impl crate::Readable for DOEPINT {}
#[doc = "`write(|w| ..)` method takes [doepint::W](doepint::W) writer structure"]
impl crate::Writable for DOEPINT {}
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod doepint;
#[doc = "OTG_HS device endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz0](doeptsiz0) module"]
pub type DOEPTSIZ0 = crate::Reg<u32, _DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ0;
#[doc = "`read()` method returns [doeptsiz0::R](doeptsiz0::R) reader structure"]
impl crate::Readable for DOEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz0::W](doeptsiz0::W) writer structure"]
impl crate::Writable for DOEPTSIZ0 {}
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod doeptsiz0;
#[doc = "OTG_HS device endpoint-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz](doeptsiz) module"]
pub type DOEPTSIZ = crate::Reg<u32, _DOEPTSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ;
#[doc = "`read()` method returns [doeptsiz::R](doeptsiz::R) reader structure"]
impl crate::Readable for DOEPTSIZ {}
#[doc = "`write(|w| ..)` method takes [doeptsiz::W](doeptsiz::W) writer structure"]
impl crate::Writable for DOEPTSIZ {}
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod doeptsiz;
