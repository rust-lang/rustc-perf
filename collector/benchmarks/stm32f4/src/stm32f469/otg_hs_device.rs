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
    pub diepint0: DIEPINT0,
    _reserved17: [u8; 4usize],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub diepdma1: DIEPDMA1,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS0,
    _reserved20: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: DIEPCTL1,
    _reserved21: [u8; 4usize],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub diepint1: DIEPINT1,
    _reserved22: [u8; 4usize],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz1: DIEPTSIZ1,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub diepdma2: DIEPDMA2,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS1,
    _reserved25: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub diepctl2: DIEPCTL2,
    _reserved26: [u8; 4usize],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub diepint2: DIEPINT2,
    _reserved27: [u8; 4usize],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz2: DIEPTSIZ2,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub diepdma3: DIEPDMA3,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS2,
    _reserved30: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub diepctl3: DIEPCTL3,
    _reserved31: [u8; 4usize],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub diepint3: DIEPINT3,
    _reserved32: [u8; 4usize],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz3: DIEPTSIZ3,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub diepdma4: DIEPDMA4,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS3,
    _reserved35: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub diepctl4: DIEPCTL4,
    _reserved36: [u8; 4usize],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub diepint4: DIEPINT4,
    _reserved37: [u8; 4usize],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz4: DIEPTSIZ4,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub diepdma5: DIEPDMA5,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: DTXFSTS4,
    _reserved40: [u8; 4usize],
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub diepctl5: DIEPCTL5,
    _reserved41: [u8; 4usize],
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    pub diepint5: DIEPINT5,
    _reserved42: [u8; 4usize],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz5: DIEPTSIZ5,
    _reserved43: [u8; 4usize],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: DTXFSTS5,
    _reserved44: [u8; 4usize],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub diepctl6: DIEPCTL6,
    _reserved45: [u8; 4usize],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub diepint6: DIEPINT6,
    _reserved46: [u8; 20usize],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub diepctl7: DIEPCTL7,
    _reserved47: [u8; 4usize],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub diepint7: DIEPINT7,
    _reserved48: [u8; 276usize],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved49: [u8; 4usize],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint0: DOEPINT0,
    _reserved50: [u8; 4usize],
    #[doc = "0x310 - OTG_HS device endpoint-1 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    _reserved51: [u8; 12usize],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub doepctl1: DOEPCTL1,
    _reserved52: [u8; 4usize],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub doepint1: DOEPINT1,
    _reserved53: [u8; 4usize],
    #[doc = "0x330 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz1: DOEPTSIZ1,
    _reserved54: [u8; 12usize],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub doepctl2: DOEPCTL2,
    _reserved55: [u8; 4usize],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub doepint2: DOEPINT2,
    _reserved56: [u8; 4usize],
    #[doc = "0x350 - OTG_HS device endpoint-3 transfer size register"]
    pub doeptsiz2: DOEPTSIZ2,
    _reserved57: [u8; 12usize],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub doepctl3: DOEPCTL3,
    _reserved58: [u8; 4usize],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub doepint3: DOEPINT3,
    _reserved59: [u8; 4usize],
    #[doc = "0x370 - OTG_HS device endpoint-4 transfer size register"]
    pub doeptsiz3: DOEPTSIZ3,
    _reserved60: [u8; 20usize],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub doepint4: DOEPINT4,
    _reserved61: [u8; 4usize],
    #[doc = "0x390 - OTG_HS device endpoint-5 transfer size register"]
    pub doeptsiz4: DOEPTSIZ4,
    _reserved62: [u8; 20usize],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub doepint5: DOEPINT5,
    _reserved63: [u8; 28usize],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub doepint6: DOEPINT6,
    _reserved64: [u8; 28usize],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub doepint7: DOEPINT7,
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
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl1](diepctl1) module"]
pub type DIEPCTL1 = crate::Reg<u32, _DIEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL1;
#[doc = "`read()` method returns [diepctl1::R](diepctl1::R) reader structure"]
impl crate::Readable for DIEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [diepctl1::W](diepctl1::W) writer structure"]
impl crate::Writable for DIEPCTL1 {}
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl1;
#[doc = "OTG device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl2](diepctl2) module"]
pub type DIEPCTL2 = crate::Reg<u32, _DIEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL2;
#[doc = "`read()` method returns [diepctl2::R](diepctl2::R) reader structure"]
impl crate::Readable for DIEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [diepctl2::W](diepctl2::W) writer structure"]
impl crate::Writable for DIEPCTL2 {}
#[doc = "OTG device endpoint-2 control register"]
pub mod diepctl2;
#[doc = "OTG device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl3](diepctl3) module"]
pub type DIEPCTL3 = crate::Reg<u32, _DIEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL3;
#[doc = "`read()` method returns [diepctl3::R](diepctl3::R) reader structure"]
impl crate::Readable for DIEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [diepctl3::W](diepctl3::W) writer structure"]
impl crate::Writable for DIEPCTL3 {}
#[doc = "OTG device endpoint-3 control register"]
pub mod diepctl3;
#[doc = "OTG device endpoint-4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl4](diepctl4) module"]
pub type DIEPCTL4 = crate::Reg<u32, _DIEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL4;
#[doc = "`read()` method returns [diepctl4::R](diepctl4::R) reader structure"]
impl crate::Readable for DIEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [diepctl4::W](diepctl4::W) writer structure"]
impl crate::Writable for DIEPCTL4 {}
#[doc = "OTG device endpoint-4 control register"]
pub mod diepctl4;
#[doc = "OTG device endpoint-5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl5](diepctl5) module"]
pub type DIEPCTL5 = crate::Reg<u32, _DIEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL5;
#[doc = "`read()` method returns [diepctl5::R](diepctl5::R) reader structure"]
impl crate::Readable for DIEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [diepctl5::W](diepctl5::W) writer structure"]
impl crate::Writable for DIEPCTL5 {}
#[doc = "OTG device endpoint-5 control register"]
pub mod diepctl5;
#[doc = "OTG device endpoint-6 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl6](diepctl6) module"]
pub type DIEPCTL6 = crate::Reg<u32, _DIEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL6;
#[doc = "`read()` method returns [diepctl6::R](diepctl6::R) reader structure"]
impl crate::Readable for DIEPCTL6 {}
#[doc = "`write(|w| ..)` method takes [diepctl6::W](diepctl6::W) writer structure"]
impl crate::Writable for DIEPCTL6 {}
#[doc = "OTG device endpoint-6 control register"]
pub mod diepctl6;
#[doc = "OTG device endpoint-7 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl7](diepctl7) module"]
pub type DIEPCTL7 = crate::Reg<u32, _DIEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL7;
#[doc = "`read()` method returns [diepctl7::R](diepctl7::R) reader structure"]
impl crate::Readable for DIEPCTL7 {}
#[doc = "`write(|w| ..)` method takes [diepctl7::W](diepctl7::W) writer structure"]
impl crate::Writable for DIEPCTL7 {}
#[doc = "OTG device endpoint-7 control register"]
pub mod diepctl7;
#[doc = "OTG device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint0](diepint0) module"]
pub type DIEPINT0 = crate::Reg<u32, _DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT0;
#[doc = "`read()` method returns [diepint0::R](diepint0::R) reader structure"]
impl crate::Readable for DIEPINT0 {}
#[doc = "`write(|w| ..)` method takes [diepint0::W](diepint0::W) writer structure"]
impl crate::Writable for DIEPINT0 {}
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod diepint0;
#[doc = "OTG device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint1](diepint1) module"]
pub type DIEPINT1 = crate::Reg<u32, _DIEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT1;
#[doc = "`read()` method returns [diepint1::R](diepint1::R) reader structure"]
impl crate::Readable for DIEPINT1 {}
#[doc = "`write(|w| ..)` method takes [diepint1::W](diepint1::W) writer structure"]
impl crate::Writable for DIEPINT1 {}
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "OTG device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint2](diepint2) module"]
pub type DIEPINT2 = crate::Reg<u32, _DIEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT2;
#[doc = "`read()` method returns [diepint2::R](diepint2::R) reader structure"]
impl crate::Readable for DIEPINT2 {}
#[doc = "`write(|w| ..)` method takes [diepint2::W](diepint2::W) writer structure"]
impl crate::Writable for DIEPINT2 {}
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "OTG device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint3](diepint3) module"]
pub type DIEPINT3 = crate::Reg<u32, _DIEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT3;
#[doc = "`read()` method returns [diepint3::R](diepint3::R) reader structure"]
impl crate::Readable for DIEPINT3 {}
#[doc = "`write(|w| ..)` method takes [diepint3::W](diepint3::W) writer structure"]
impl crate::Writable for DIEPINT3 {}
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "OTG device endpoint-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint4](diepint4) module"]
pub type DIEPINT4 = crate::Reg<u32, _DIEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT4;
#[doc = "`read()` method returns [diepint4::R](diepint4::R) reader structure"]
impl crate::Readable for DIEPINT4 {}
#[doc = "`write(|w| ..)` method takes [diepint4::W](diepint4::W) writer structure"]
impl crate::Writable for DIEPINT4 {}
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod diepint4;
#[doc = "OTG device endpoint-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint5](diepint5) module"]
pub type DIEPINT5 = crate::Reg<u32, _DIEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT5;
#[doc = "`read()` method returns [diepint5::R](diepint5::R) reader structure"]
impl crate::Readable for DIEPINT5 {}
#[doc = "`write(|w| ..)` method takes [diepint5::W](diepint5::W) writer structure"]
impl crate::Writable for DIEPINT5 {}
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod diepint5;
#[doc = "OTG device endpoint-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint6](diepint6) module"]
pub type DIEPINT6 = crate::Reg<u32, _DIEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT6;
#[doc = "`read()` method returns [diepint6::R](diepint6::R) reader structure"]
impl crate::Readable for DIEPINT6 {}
#[doc = "`write(|w| ..)` method takes [diepint6::W](diepint6::W) writer structure"]
impl crate::Writable for DIEPINT6 {}
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod diepint6;
#[doc = "OTG device endpoint-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint7](diepint7) module"]
pub type DIEPINT7 = crate::Reg<u32, _DIEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT7;
#[doc = "`read()` method returns [diepint7::R](diepint7::R) reader structure"]
impl crate::Readable for DIEPINT7 {}
#[doc = "`write(|w| ..)` method takes [diepint7::W](diepint7::W) writer structure"]
impl crate::Writable for DIEPINT7 {}
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod diepint7;
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
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts0](dtxfsts0) module"]
pub type DTXFSTS0 = crate::Reg<u32, _DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS0;
#[doc = "`read()` method returns [dtxfsts0::R](dtxfsts0::R) reader structure"]
impl crate::Readable for DTXFSTS0 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts1](dtxfsts1) module"]
pub type DTXFSTS1 = crate::Reg<u32, _DTXFSTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS1;
#[doc = "`read()` method returns [dtxfsts1::R](dtxfsts1::R) reader structure"]
impl crate::Readable for DTXFSTS1 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts2](dtxfsts2) module"]
pub type DTXFSTS2 = crate::Reg<u32, _DTXFSTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS2;
#[doc = "`read()` method returns [dtxfsts2::R](dtxfsts2::R) reader structure"]
impl crate::Readable for DTXFSTS2 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts3](dtxfsts3) module"]
pub type DTXFSTS3 = crate::Reg<u32, _DTXFSTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS3;
#[doc = "`read()` method returns [dtxfsts3::R](dtxfsts3::R) reader structure"]
impl crate::Readable for DTXFSTS3 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts4](dtxfsts4) module"]
pub type DTXFSTS4 = crate::Reg<u32, _DTXFSTS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS4;
#[doc = "`read()` method returns [dtxfsts4::R](dtxfsts4::R) reader structure"]
impl crate::Readable for DTXFSTS4 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts4;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts5](dtxfsts5) module"]
pub type DTXFSTS5 = crate::Reg<u32, _DTXFSTS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS5;
#[doc = "`read()` method returns [dtxfsts5::R](dtxfsts5::R) reader structure"]
impl crate::Readable for DTXFSTS5 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts5;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz1](dieptsiz1) module"]
pub type DIEPTSIZ1 = crate::Reg<u32, _DIEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ1;
#[doc = "`read()` method returns [dieptsiz1::R](dieptsiz1::R) reader structure"]
impl crate::Readable for DIEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz1::W](dieptsiz1::W) writer structure"]
impl crate::Writable for DIEPTSIZ1 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz1;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz2](dieptsiz2) module"]
pub type DIEPTSIZ2 = crate::Reg<u32, _DIEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ2;
#[doc = "`read()` method returns [dieptsiz2::R](dieptsiz2::R) reader structure"]
impl crate::Readable for DIEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz2::W](dieptsiz2::W) writer structure"]
impl crate::Writable for DIEPTSIZ2 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz2;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz3](dieptsiz3) module"]
pub type DIEPTSIZ3 = crate::Reg<u32, _DIEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ3;
#[doc = "`read()` method returns [dieptsiz3::R](dieptsiz3::R) reader structure"]
impl crate::Readable for DIEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz3::W](dieptsiz3::W) writer structure"]
impl crate::Writable for DIEPTSIZ3 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz3;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz4](dieptsiz4) module"]
pub type DIEPTSIZ4 = crate::Reg<u32, _DIEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ4;
#[doc = "`read()` method returns [dieptsiz4::R](dieptsiz4::R) reader structure"]
impl crate::Readable for DIEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz4::W](dieptsiz4::W) writer structure"]
impl crate::Writable for DIEPTSIZ4 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz4;
#[doc = "OTG_HS device endpoint transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz5](dieptsiz5) module"]
pub type DIEPTSIZ5 = crate::Reg<u32, _DIEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ5;
#[doc = "`read()` method returns [dieptsiz5::R](dieptsiz5::R) reader structure"]
impl crate::Readable for DIEPTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz5::W](dieptsiz5::W) writer structure"]
impl crate::Writable for DIEPTSIZ5 {}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz5;
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
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl1](doepctl1) module"]
pub type DOEPCTL1 = crate::Reg<u32, _DOEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL1;
#[doc = "`read()` method returns [doepctl1::R](doepctl1::R) reader structure"]
impl crate::Readable for DOEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [doepctl1::W](doepctl1::W) writer structure"]
impl crate::Writable for DOEPCTL1 {}
#[doc = "OTG device endpoint-1 control register"]
pub mod doepctl1;
#[doc = "OTG device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl2](doepctl2) module"]
pub type DOEPCTL2 = crate::Reg<u32, _DOEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL2;
#[doc = "`read()` method returns [doepctl2::R](doepctl2::R) reader structure"]
impl crate::Readable for DOEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [doepctl2::W](doepctl2::W) writer structure"]
impl crate::Writable for DOEPCTL2 {}
#[doc = "OTG device endpoint-2 control register"]
pub mod doepctl2;
#[doc = "OTG device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl3](doepctl3) module"]
pub type DOEPCTL3 = crate::Reg<u32, _DOEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL3;
#[doc = "`read()` method returns [doepctl3::R](doepctl3::R) reader structure"]
impl crate::Readable for DOEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [doepctl3::W](doepctl3::W) writer structure"]
impl crate::Writable for DOEPCTL3 {}
#[doc = "OTG device endpoint-3 control register"]
pub mod doepctl3;
#[doc = "OTG_HS device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint0](doepint0) module"]
pub type DOEPINT0 = crate::Reg<u32, _DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT0;
#[doc = "`read()` method returns [doepint0::R](doepint0::R) reader structure"]
impl crate::Readable for DOEPINT0 {}
#[doc = "`write(|w| ..)` method takes [doepint0::W](doepint0::W) writer structure"]
impl crate::Writable for DOEPINT0 {}
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "OTG_HS device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint1](doepint1) module"]
pub type DOEPINT1 = crate::Reg<u32, _DOEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT1;
#[doc = "`read()` method returns [doepint1::R](doepint1::R) reader structure"]
impl crate::Readable for DOEPINT1 {}
#[doc = "`write(|w| ..)` method takes [doepint1::W](doepint1::W) writer structure"]
impl crate::Writable for DOEPINT1 {}
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "OTG_HS device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint2](doepint2) module"]
pub type DOEPINT2 = crate::Reg<u32, _DOEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT2;
#[doc = "`read()` method returns [doepint2::R](doepint2::R) reader structure"]
impl crate::Readable for DOEPINT2 {}
#[doc = "`write(|w| ..)` method takes [doepint2::W](doepint2::W) writer structure"]
impl crate::Writable for DOEPINT2 {}
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "OTG_HS device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint3](doepint3) module"]
pub type DOEPINT3 = crate::Reg<u32, _DOEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT3;
#[doc = "`read()` method returns [doepint3::R](doepint3::R) reader structure"]
impl crate::Readable for DOEPINT3 {}
#[doc = "`write(|w| ..)` method takes [doepint3::W](doepint3::W) writer structure"]
impl crate::Writable for DOEPINT3 {}
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "OTG_HS device endpoint-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint4](doepint4) module"]
pub type DOEPINT4 = crate::Reg<u32, _DOEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT4;
#[doc = "`read()` method returns [doepint4::R](doepint4::R) reader structure"]
impl crate::Readable for DOEPINT4 {}
#[doc = "`write(|w| ..)` method takes [doepint4::W](doepint4::W) writer structure"]
impl crate::Writable for DOEPINT4 {}
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod doepint4;
#[doc = "OTG_HS device endpoint-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint5](doepint5) module"]
pub type DOEPINT5 = crate::Reg<u32, _DOEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT5;
#[doc = "`read()` method returns [doepint5::R](doepint5::R) reader structure"]
impl crate::Readable for DOEPINT5 {}
#[doc = "`write(|w| ..)` method takes [doepint5::W](doepint5::W) writer structure"]
impl crate::Writable for DOEPINT5 {}
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod doepint5;
#[doc = "OTG_HS device endpoint-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint6](doepint6) module"]
pub type DOEPINT6 = crate::Reg<u32, _DOEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT6;
#[doc = "`read()` method returns [doepint6::R](doepint6::R) reader structure"]
impl crate::Readable for DOEPINT6 {}
#[doc = "`write(|w| ..)` method takes [doepint6::W](doepint6::W) writer structure"]
impl crate::Writable for DOEPINT6 {}
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod doepint6;
#[doc = "OTG_HS device endpoint-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint7](doepint7) module"]
pub type DOEPINT7 = crate::Reg<u32, _DOEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT7;
#[doc = "`read()` method returns [doepint7::R](doepint7::R) reader structure"]
impl crate::Readable for DOEPINT7 {}
#[doc = "`write(|w| ..)` method takes [doepint7::W](doepint7::W) writer structure"]
impl crate::Writable for DOEPINT7 {}
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod doepint7;
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
#[doc = "OTG_HS device endpoint-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz1](doeptsiz1) module"]
pub type DOEPTSIZ1 = crate::Reg<u32, _DOEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ1;
#[doc = "`read()` method returns [doeptsiz1::R](doeptsiz1::R) reader structure"]
impl crate::Readable for DOEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz1::W](doeptsiz1::W) writer structure"]
impl crate::Writable for DOEPTSIZ1 {}
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod doeptsiz1;
#[doc = "OTG_HS device endpoint-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz2](doeptsiz2) module"]
pub type DOEPTSIZ2 = crate::Reg<u32, _DOEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ2;
#[doc = "`read()` method returns [doeptsiz2::R](doeptsiz2::R) reader structure"]
impl crate::Readable for DOEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz2::W](doeptsiz2::W) writer structure"]
impl crate::Writable for DOEPTSIZ2 {}
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod doeptsiz2;
#[doc = "OTG_HS device endpoint-4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz3](doeptsiz3) module"]
pub type DOEPTSIZ3 = crate::Reg<u32, _DOEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ3;
#[doc = "`read()` method returns [doeptsiz3::R](doeptsiz3::R) reader structure"]
impl crate::Readable for DOEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz3::W](doeptsiz3::W) writer structure"]
impl crate::Writable for DOEPTSIZ3 {}
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod doeptsiz3;
#[doc = "OTG_HS device endpoint-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz4](doeptsiz4) module"]
pub type DOEPTSIZ4 = crate::Reg<u32, _DOEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ4;
#[doc = "`read()` method returns [doeptsiz4::R](doeptsiz4::R) reader structure"]
impl crate::Readable for DOEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz4::W](doeptsiz4::W) writer structure"]
impl crate::Writable for DOEPTSIZ4 {}
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod doeptsiz4;
