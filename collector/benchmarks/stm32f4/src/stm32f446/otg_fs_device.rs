#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub dsts: DSTS,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub daint: DAINT,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 8usize],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved9: [u8; 4usize],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved10: [u8; 200usize],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub diepctl0: DIEPCTL0,
    _reserved11: [u8; 4usize],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub diepint0: DIEPINT0,
    _reserved12: [u8; 4usize],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    _reserved13: [u8; 4usize],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS0,
    _reserved14: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: DIEPCTL,
    _reserved15: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diepint1: DIEPINT,
    _reserved16: [u8; 4usize],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub dieptsiz1: DIEPTSIZ,
    _reserved17: [u8; 4usize],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS,
    _reserved18: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-1 control register"]
    pub diepctl2: DIEPCTL,
    _reserved19: [u8; 4usize],
    #[doc = "0x148 - device endpoint-1 interrupt register"]
    pub diepint2: DIEPINT,
    _reserved20: [u8; 4usize],
    #[doc = "0x150 - device endpoint-1 transfer size register"]
    pub dieptsiz2: DIEPTSIZ,
    _reserved21: [u8; 4usize],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS,
    _reserved22: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-1 control register"]
    pub diepctl3: DIEPCTL,
    _reserved23: [u8; 4usize],
    #[doc = "0x168 - device endpoint-1 interrupt register"]
    pub diepint3: DIEPINT,
    _reserved24: [u8; 4usize],
    #[doc = "0x170 - device endpoint-1 transfer size register"]
    pub dieptsiz3: DIEPTSIZ,
    _reserved25: [u8; 4usize],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS,
    _reserved26: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-1 control register"]
    pub diepctl4: DIEPCTL,
    _reserved27: [u8; 4usize],
    #[doc = "0x188 - device endpoint-1 interrupt register"]
    pub diepint4: DIEPINT,
    _reserved28: [u8; 4usize],
    #[doc = "0x190 - device endpoint-1 transfer size register"]
    pub dieptsiz4: DIEPTSIZ,
    _reserved29: [u8; 4usize],
    #[doc = "0x198 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: DTXFSTS,
    _reserved30: [u8; 4usize],
    #[doc = "0x1a0 - OTG device endpoint-1 control register"]
    pub diepctl5: DIEPCTL,
    _reserved31: [u8; 4usize],
    #[doc = "0x1a8 - device endpoint-1 interrupt register"]
    pub diepint5: DIEPINT,
    _reserved32: [u8; 4usize],
    #[doc = "0x1b0 - device endpoint-1 transfer size register"]
    pub dieptsiz5: DIEPTSIZ,
    _reserved33: [u8; 4usize],
    #[doc = "0x1b8 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: DTXFSTS,
    _reserved34: [u8; 324usize],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved35: [u8; 4usize],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub doepint0: DOEPINT0,
    _reserved36: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    _reserved37: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doepctl1: DOEPCTL,
    _reserved38: [u8; 4usize],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub doepint1: DOEPINT,
    _reserved39: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz1: DOEPTSIZ,
    _reserved40: [u8; 12usize],
    #[doc = "0x340 - device endpoint-1 control register"]
    pub doepctl2: DOEPCTL,
    _reserved41: [u8; 4usize],
    #[doc = "0x348 - device endpoint-1 interrupt register"]
    pub doepint2: DOEPINT,
    _reserved42: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz2: DOEPTSIZ,
    _reserved43: [u8; 12usize],
    #[doc = "0x360 - device endpoint-1 control register"]
    pub doepctl3: DOEPCTL,
    _reserved44: [u8; 4usize],
    #[doc = "0x368 - device endpoint-1 interrupt register"]
    pub doepint3: DOEPINT,
    _reserved45: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz3: DOEPTSIZ,
    _reserved46: [u8; 12usize],
    #[doc = "0x380 - device endpoint-1 control register"]
    pub doepctl4: DOEPCTL,
    _reserved47: [u8; 4usize],
    #[doc = "0x388 - device endpoint-1 interrupt register"]
    pub doepint4: DOEPINT,
    _reserved48: [u8; 4usize],
    #[doc = "0x390 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz4: DOEPTSIZ,
    _reserved49: [u8; 12usize],
    #[doc = "0x3a0 - device endpoint-1 control register"]
    pub doepctl5: DOEPCTL,
    _reserved50: [u8; 4usize],
    #[doc = "0x3a8 - device endpoint-1 interrupt register"]
    pub doepint5: DOEPINT,
    _reserved51: [u8; 4usize],
    #[doc = "0x3b0 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz5: DOEPTSIZ,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod dcfg;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod dctl;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](dsts) module"]
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
#[doc = "`read()` method returns [dsts::R](dsts::R) reader structure"]
impl crate::Readable for DSTS {}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod dsts;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepmsk](diepmsk) module"]
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
#[doc = "`read()` method returns [diepmsk::R](diepmsk::R) reader structure"]
impl crate::Readable for DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure"]
impl crate::Writable for DIEPMSK {}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod diepmsk;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](doepmsk) module"]
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
#[doc = "`read()` method returns [doepmsk::R](doepmsk::R) reader structure"]
impl crate::Readable for DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure"]
impl crate::Writable for DOEPMSK {}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod doepmsk;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daint](daint) module"]
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
#[doc = "`read()` method returns [daint::R](daint::R) reader structure"]
impl crate::Readable for DAINT {}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod daint;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daintmsk](daintmsk) module"]
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
#[doc = "`read()` method returns [daintmsk::R](daintmsk::R) reader structure"]
impl crate::Readable for DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure"]
impl crate::Writable for DAINTMSK {}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod daintmsk;
#[doc = "OTG_FS device VBUS discharge time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbusdis](dvbusdis) module"]
pub type DVBUSDIS = crate::Reg<u32, _DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDIS;
#[doc = "`read()` method returns [dvbusdis::R](dvbusdis::R) reader structure"]
impl crate::Readable for DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [dvbusdis::W](dvbusdis::W) writer structure"]
impl crate::Writable for DVBUSDIS {}
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "OTG_FS device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbuspulse](dvbuspulse) module"]
pub type DVBUSPULSE = crate::Reg<u32, _DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPULSE;
#[doc = "`read()` method returns [dvbuspulse::R](dvbuspulse::R) reader structure"]
impl crate::Readable for DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [dvbuspulse::W](dvbuspulse::W) writer structure"]
impl crate::Writable for DVBUSPULSE {}
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepempmsk](diepempmsk) module"]
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
#[doc = "`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure"]
impl crate::Readable for DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure"]
impl crate::Writable for DIEPEMPMSK {}
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl0](diepctl0) module"]
pub type DIEPCTL0 = crate::Reg<u32, _DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL0;
#[doc = "`read()` method returns [diepctl0::R](diepctl0::R) reader structure"]
impl crate::Readable for DIEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [diepctl0::W](diepctl0::W) writer structure"]
impl crate::Writable for DIEPCTL0 {}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
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
#[doc = "device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl0](doepctl0) module"]
pub type DOEPCTL0 = crate::Reg<u32, _DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL0;
#[doc = "`read()` method returns [doepctl0::R](doepctl0::R) reader structure"]
impl crate::Readable for DOEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [doepctl0::W](doepctl0::W) writer structure"]
impl crate::Writable for DOEPCTL0 {}
#[doc = "device endpoint-0 control register"]
pub mod doepctl0;
#[doc = "device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl](doepctl) module"]
pub type DOEPCTL = crate::Reg<u32, _DOEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL;
#[doc = "`read()` method returns [doepctl::R](doepctl::R) reader structure"]
impl crate::Readable for DOEPCTL {}
#[doc = "`write(|w| ..)` method takes [doepctl::W](doepctl::W) writer structure"]
impl crate::Writable for DOEPCTL {}
#[doc = "device endpoint-1 control register"]
pub mod doepctl;
#[doc = "device endpoint-x interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint0](diepint0) module"]
pub type DIEPINT0 = crate::Reg<u32, _DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT0;
#[doc = "`read()` method returns [diepint0::R](diepint0::R) reader structure"]
impl crate::Readable for DIEPINT0 {}
#[doc = "`write(|w| ..)` method takes [diepint0::W](diepint0::W) writer structure"]
impl crate::Writable for DIEPINT0 {}
#[doc = "device endpoint-x interrupt register"]
pub mod diepint0;
#[doc = "device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint](diepint) module"]
pub type DIEPINT = crate::Reg<u32, _DIEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT;
#[doc = "`read()` method returns [diepint::R](diepint::R) reader structure"]
impl crate::Readable for DIEPINT {}
#[doc = "`write(|w| ..)` method takes [diepint::W](diepint::W) writer structure"]
impl crate::Writable for DIEPINT {}
#[doc = "device endpoint-1 interrupt register"]
pub mod diepint;
#[doc = "device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint0](doepint0) module"]
pub type DOEPINT0 = crate::Reg<u32, _DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT0;
#[doc = "`read()` method returns [doepint0::R](doepint0::R) reader structure"]
impl crate::Readable for DOEPINT0 {}
#[doc = "`write(|w| ..)` method takes [doepint0::W](doepint0::W) writer structure"]
impl crate::Writable for DOEPINT0 {}
#[doc = "device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint](doepint) module"]
pub type DOEPINT = crate::Reg<u32, _DOEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT;
#[doc = "`read()` method returns [doepint::R](doepint::R) reader structure"]
impl crate::Readable for DOEPINT {}
#[doc = "`write(|w| ..)` method takes [doepint::W](doepint::W) writer structure"]
impl crate::Writable for DOEPINT {}
#[doc = "device endpoint-1 interrupt register"]
pub mod doepint;
#[doc = "device endpoint-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz0](dieptsiz0) module"]
pub type DIEPTSIZ0 = crate::Reg<u32, _DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ0;
#[doc = "`read()` method returns [dieptsiz0::R](dieptsiz0::R) reader structure"]
impl crate::Readable for DIEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz0::W](dieptsiz0::W) writer structure"]
impl crate::Writable for DIEPTSIZ0 {}
#[doc = "device endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "device OUT endpoint-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz0](doeptsiz0) module"]
pub type DOEPTSIZ0 = crate::Reg<u32, _DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ0;
#[doc = "`read()` method returns [doeptsiz0::R](doeptsiz0::R) reader structure"]
impl crate::Readable for DOEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz0::W](doeptsiz0::W) writer structure"]
impl crate::Writable for DOEPTSIZ0 {}
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "device endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz](dieptsiz) module"]
pub type DIEPTSIZ = crate::Reg<u32, _DIEPTSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ;
#[doc = "`read()` method returns [dieptsiz::R](dieptsiz::R) reader structure"]
impl crate::Readable for DIEPTSIZ {}
#[doc = "`write(|w| ..)` method takes [dieptsiz::W](dieptsiz::W) writer structure"]
impl crate::Writable for DIEPTSIZ {}
#[doc = "device endpoint-1 transfer size register"]
pub mod dieptsiz;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts0](dtxfsts0) module"]
pub type DTXFSTS0 = crate::Reg<u32, _DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS0;
#[doc = "`read()` method returns [dtxfsts0::R](dtxfsts0::R) reader structure"]
impl crate::Readable for DTXFSTS0 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts](dtxfsts) module"]
pub type DTXFSTS = crate::Reg<u32, _DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS;
#[doc = "`read()` method returns [dtxfsts::R](dtxfsts::R) reader structure"]
impl crate::Readable for DTXFSTS {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts;
#[doc = "device OUT endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz](doeptsiz) module"]
pub type DOEPTSIZ = crate::Reg<u32, _DOEPTSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ;
#[doc = "`read()` method returns [doeptsiz::R](doeptsiz::R) reader structure"]
impl crate::Readable for DOEPTSIZ {}
#[doc = "`write(|w| ..)` method takes [doeptsiz::W](doeptsiz::W) writer structure"]
impl crate::Writable for DOEPTSIZ {}
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod doeptsiz;
