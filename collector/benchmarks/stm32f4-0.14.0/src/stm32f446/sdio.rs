#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDIO power control register"]
    pub power: crate::Reg<power::POWER_SPEC>,
    #[doc = "0x04 - CLKCR register controls the SDIO_CK output clock."]
    pub clkcr: crate::Reg<clkcr::CLKCR_SPEC>,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub arg: crate::Reg<arg::ARG_SPEC>,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
    pub respcmd: crate::Reg<respcmd::RESPCMD_SPEC>,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp1: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp2: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp3: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp4: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub dtimer: crate::Reg<dtimer::DTIMER_SPEC>,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub dlen: crate::Reg<dlen::DLEN_SPEC>,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub dctrl: crate::Reg<dctrl::DCTRL_SPEC>,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub dcount: crate::Reg<dcount::DCOUNT_SPEC>,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub sta: crate::Reg<sta::STA_SPEC>,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub mask: crate::Reg<mask::MASK_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x48 - The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."]
    pub fifocnt: crate::Reg<fifocnt::FIFOCNT_SPEC>,
    _reserved17: [u8; 0x34],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
}
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "SDIO power control register"]
pub mod power;
#[doc = "CLKCR register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "CLKCR register controls the SDIO_CK output clock."]
pub mod clkcr;
#[doc = "ARG register accessor: an alias for `Reg<ARG_SPEC>`"]
pub type ARG = crate::Reg<arg::ARG_SPEC>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod arg;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod cmd;
#[doc = "RESP register accessor: an alias for `Reg<RESP_SPEC>`"]
pub type RESP = crate::Reg<resp::RESP_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp;
#[doc = "DTIMER register accessor: an alias for `Reg<DTIMER_SPEC>`"]
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod dtimer;
#[doc = "DLEN register accessor: an alias for `Reg<DLEN_SPEC>`"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod dlen;
#[doc = "DCTRL register accessor: an alias for `Reg<DCTRL_SPEC>`"]
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod dctrl;
#[doc = "DCOUNT register accessor: an alias for `Reg<DCOUNT_SPEC>`"]
pub type DCOUNT = crate::Reg<dcount::DCOUNT_SPEC>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod dcount;
#[doc = "STA register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sta;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod icr;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod mask;
#[doc = "RESPCMD register accessor: an alias for `Reg<RESPCMD_SPEC>`"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
pub mod respcmd;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod fifo;
#[doc = "FIFOCNT register accessor: an alias for `Reg<FIFOCNT_SPEC>`"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
#[doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."]
pub mod fifocnt;
