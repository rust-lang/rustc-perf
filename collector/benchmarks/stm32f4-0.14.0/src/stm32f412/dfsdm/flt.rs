#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod icr;
#[doc = "JCHGR register accessor: an alias for `Reg<JCHGR_SPEC>`"]
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod jchgr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "filter control register"]
pub mod fcr;
#[doc = "JDATAR register accessor: an alias for `Reg<JDATAR_SPEC>`"]
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod jdatar;
#[doc = "RDATAR register accessor: an alias for `Reg<RDATAR_SPEC>`"]
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod rdatar;
#[doc = "AWHTR register accessor: an alias for `Reg<AWHTR_SPEC>`"]
pub type AWHTR = crate::Reg<awhtr::AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod awhtr;
#[doc = "AWLTR register accessor: an alias for `Reg<AWLTR_SPEC>`"]
pub type AWLTR = crate::Reg<awltr::AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod awltr;
#[doc = "AWSR register accessor: an alias for `Reg<AWSR_SPEC>`"]
pub type AWSR = crate::Reg<awsr::AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod awsr;
#[doc = "AWCFR register accessor: an alias for `Reg<AWCFR_SPEC>`"]
pub type AWCFR = crate::Reg<awcfr::AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod awcfr;
#[doc = "EXMAX register accessor: an alias for `Reg<EXMAX_SPEC>`"]
pub type EXMAX = crate::Reg<exmax::EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod exmax;
#[doc = "EXMIN register accessor: an alias for `Reg<EXMIN_SPEC>`"]
pub type EXMIN = crate::Reg<exmin::EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod exmin;
#[doc = "CNVTIMR register accessor: an alias for `Reg<CNVTIMR_SPEC>`"]
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod cnvtimr;
