#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - argument register"]
    pub arg: ARG,
    #[doc = "0x0c - command register"]
    pub cmd: CMD,
    #[doc = "0x10 - command response register"]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - response 1..4 register"]
    pub resp1: RESP1,
    #[doc = "0x18 - response 1..4 register"]
    pub resp2: RESP2,
    #[doc = "0x1c - response 1..4 register"]
    pub resp3: RESP3,
    #[doc = "0x20 - response 1..4 register"]
    pub resp4: RESP4,
    #[doc = "0x24 - data timer register"]
    pub dtimer: DTIMER,
    #[doc = "0x28 - data length register"]
    pub dlen: DLEN,
    #[doc = "0x2c - data control register"]
    pub dctrl: DCTRL,
    #[doc = "0x30 - data counter register"]
    pub dcount: DCOUNT,
    #[doc = "0x34 - status register"]
    pub sta: STA,
    #[doc = "0x38 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x3c - mask register"]
    pub mask: MASK,
    _reserved16: [u8; 8usize],
    #[doc = "0x48 - FIFO counter register"]
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 52usize],
    #[doc = "0x80 - data FIFO register"]
    pub fifo: FIFO,
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "power control register"]
pub mod power;
#[doc = "SDI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](clkcr) module"]
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
#[doc = "`read()` method returns [clkcr::R](clkcr::R) reader structure"]
impl crate::Readable for CLKCR {}
#[doc = "`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure"]
impl crate::Writable for CLKCR {}
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg](arg) module"]
pub type ARG = crate::Reg<u32, _ARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG;
#[doc = "`read()` method returns [arg::R](arg::R) reader structure"]
impl crate::Readable for ARG {}
#[doc = "`write(|w| ..)` method takes [arg::W](arg::W) writer structure"]
impl crate::Writable for ARG {}
#[doc = "argument register"]
pub mod arg;
#[doc = "command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "command register"]
pub mod cmd;
#[doc = "command response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmd](respcmd) module"]
pub type RESPCMD = crate::Reg<u32, _RESPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMD;
#[doc = "`read()` method returns [respcmd::R](respcmd::R) reader structure"]
impl crate::Readable for RESPCMD {}
#[doc = "command response register"]
pub mod respcmd;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp1](resp1) module"]
pub type RESP1 = crate::Reg<u32, _RESP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP1;
#[doc = "`read()` method returns [resp1::R](resp1::R) reader structure"]
impl crate::Readable for RESP1 {}
#[doc = "response 1..4 register"]
pub mod resp1;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2](resp2) module"]
pub type RESP2 = crate::Reg<u32, _RESP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2;
#[doc = "`read()` method returns [resp2::R](resp2::R) reader structure"]
impl crate::Readable for RESP2 {}
#[doc = "response 1..4 register"]
pub mod resp2;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3](resp3) module"]
pub type RESP3 = crate::Reg<u32, _RESP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP3;
#[doc = "`read()` method returns [resp3::R](resp3::R) reader structure"]
impl crate::Readable for RESP3 {}
#[doc = "response 1..4 register"]
pub mod resp3;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4](resp4) module"]
pub type RESP4 = crate::Reg<u32, _RESP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP4;
#[doc = "`read()` method returns [resp4::R](resp4::R) reader structure"]
impl crate::Readable for RESP4 {}
#[doc = "response 1..4 register"]
pub mod resp4;
#[doc = "data timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtimer](dtimer) module"]
pub type DTIMER = crate::Reg<u32, _DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTIMER;
#[doc = "`read()` method returns [dtimer::R](dtimer::R) reader structure"]
impl crate::Readable for DTIMER {}
#[doc = "`write(|w| ..)` method takes [dtimer::W](dtimer::W) writer structure"]
impl crate::Writable for DTIMER {}
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlen](dlen) module"]
pub type DLEN = crate::Reg<u32, _DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLEN;
#[doc = "`read()` method returns [dlen::R](dlen::R) reader structure"]
impl crate::Readable for DLEN {}
#[doc = "`write(|w| ..)` method takes [dlen::W](dlen::W) writer structure"]
impl crate::Writable for DLEN {}
#[doc = "data length register"]
pub mod dlen;
#[doc = "data control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](dctrl) module"]
pub type DCTRL = crate::Reg<u32, _DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTRL;
#[doc = "`read()` method returns [dctrl::R](dctrl::R) reader structure"]
impl crate::Readable for DCTRL {}
#[doc = "`write(|w| ..)` method takes [dctrl::W](dctrl::W) writer structure"]
impl crate::Writable for DCTRL {}
#[doc = "data control register"]
pub mod dctrl;
#[doc = "data counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcount](dcount) module"]
pub type DCOUNT = crate::Reg<u32, _DCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOUNT;
#[doc = "`read()` method returns [dcount::R](dcount::R) reader structure"]
impl crate::Readable for DCOUNT {}
#[doc = "data counter register"]
pub mod dcount;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](sta) module"]
pub type STA = crate::Reg<u32, _STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STA;
#[doc = "`read()` method returns [sta::R](sta::R) reader structure"]
impl crate::Readable for STA {}
#[doc = "status register"]
pub mod sta;
#[doc = "interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "mask register"]
pub mod mask;
#[doc = "FIFO counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](fifocnt) module"]
pub type FIFOCNT = crate::Reg<u32, _FIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCNT;
#[doc = "`read()` method returns [fifocnt::R](fifocnt::R) reader structure"]
impl crate::Readable for FIFOCNT {}
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "data FIFO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "data FIFO register"]
pub mod fifo;
