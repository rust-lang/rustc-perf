#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDIO power control register"]
    pub power: POWER,
    #[doc = "0x04 - CLKCR register controls the SDIO_CK output clock."]
    pub clkcr: CLKCR,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub arg: ARG,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub cmd: CMD,
    #[doc = "0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp1: RESP,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp2: RESP,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp3: RESP,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp4: RESP,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub dtimer: DTIMER,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub dlen: DLEN,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub dctrl: DCTRL,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub dcount: DCOUNT,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub sta: STA,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub icr: ICR,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub mask: MASK,
    _reserved16: [u8; 8usize],
    #[doc = "0x48 - The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."]
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 52usize],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub fifo: FIFO,
}
#[doc = "SDIO power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "SDIO power control register"]
pub mod power;
#[doc = "CLKCR register controls the SDIO_CK output clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](clkcr) module"]
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
#[doc = "`read()` method returns [clkcr::R](clkcr::R) reader structure"]
impl crate::Readable for CLKCR {}
#[doc = "`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure"]
impl crate::Writable for CLKCR {}
#[doc = "CLKCR register controls the SDIO_CK output clock."]
pub mod clkcr;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg](arg) module"]
pub type ARG = crate::Reg<u32, _ARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG;
#[doc = "`read()` method returns [arg::R](arg::R) reader structure"]
impl crate::Readable for ARG {}
#[doc = "`write(|w| ..)` method takes [arg::W](arg::W) writer structure"]
impl crate::Writable for ARG {}
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod arg;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod cmd;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](resp) module"]
pub type RESP = crate::Reg<u32, _RESP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP;
#[doc = "`read()` method returns [resp::R](resp::R) reader structure"]
impl crate::Readable for RESP {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtimer](dtimer) module"]
pub type DTIMER = crate::Reg<u32, _DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTIMER;
#[doc = "`read()` method returns [dtimer::R](dtimer::R) reader structure"]
impl crate::Readable for DTIMER {}
#[doc = "`write(|w| ..)` method takes [dtimer::W](dtimer::W) writer structure"]
impl crate::Writable for DTIMER {}
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod dtimer;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlen](dlen) module"]
pub type DLEN = crate::Reg<u32, _DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLEN;
#[doc = "`read()` method returns [dlen::R](dlen::R) reader structure"]
impl crate::Readable for DLEN {}
#[doc = "`write(|w| ..)` method takes [dlen::W](dlen::W) writer structure"]
impl crate::Writable for DLEN {}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod dlen;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](dctrl) module"]
pub type DCTRL = crate::Reg<u32, _DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTRL;
#[doc = "`read()` method returns [dctrl::R](dctrl::R) reader structure"]
impl crate::Readable for DCTRL {}
#[doc = "`write(|w| ..)` method takes [dctrl::W](dctrl::W) writer structure"]
impl crate::Writable for DCTRL {}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod dctrl;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcount](dcount) module"]
pub type DCOUNT = crate::Reg<u32, _DCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOUNT;
#[doc = "`read()` method returns [dcount::R](dcount::R) reader structure"]
impl crate::Readable for DCOUNT {}
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod dcount;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](sta) module"]
pub type STA = crate::Reg<u32, _STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STA;
#[doc = "`read()` method returns [sta::R](sta::R) reader structure"]
impl crate::Readable for STA {}
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sta;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod icr;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod mask;
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmd](respcmd) module"]
pub type RESPCMD = crate::Reg<u32, _RESPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMD;
#[doc = "`read()` method returns [respcmd::R](respcmd::R) reader structure"]
impl crate::Readable for RESPCMD {}
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
pub mod respcmd;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod fifo;
#[doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](fifocnt) module"]
pub type FIFOCNT = crate::Reg<u32, _FIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCNT;
#[doc = "`read()` method returns [fifocnt::R](fifocnt::R) reader structure"]
impl crate::Readable for FIFOCNT {}
#[doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."]
pub mod fifocnt;
