#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    pub maccr: MACCR,
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    pub macffr: MACFFR,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machthr: MACHTHR,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtlr: MACHTLR,
    #[doc = "0x10 - Ethernet MAC MII address register"]
    pub macmiiar: MACMIIAR,
    #[doc = "0x14 - Ethernet MAC MII data register"]
    pub macmiidr: MACMIIDR,
    #[doc = "0x18 - Ethernet MAC flow control register"]
    pub macfcr: MACFCR,
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    pub macvlantr: MACVLANTR,
    _reserved8: [u8; 12usize],
    #[doc = "0x2c - Ethernet MAC PMT control and status register"]
    pub macpmtcsr: MACPMTCSR,
    _reserved9: [u8; 4usize],
    #[doc = "0x34 - Ethernet MAC debug register"]
    pub macdbgr: MACDBGR,
    #[doc = "0x38 - Ethernet MAC interrupt status register"]
    pub macsr: MACSR,
    #[doc = "0x3c - Ethernet MAC interrupt mask register"]
    pub macimr: MACIMR,
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    pub maca0hr: MACA0HR,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    pub maca1hr: MACA1HR,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    pub maca2hr: MACA2HR,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    pub maca3hr: MACA3HR,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3lr: MACA3LR,
}
#[doc = "Ethernet MAC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maccr](maccr) module"]
pub type MACCR = crate::Reg<u32, _MACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACCR;
#[doc = "`read()` method returns [maccr::R](maccr::R) reader structure"]
impl crate::Readable for MACCR {}
#[doc = "`write(|w| ..)` method takes [maccr::W](maccr::W) writer structure"]
impl crate::Writable for MACCR {}
#[doc = "Ethernet MAC configuration register"]
pub mod maccr;
#[doc = "Ethernet MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macffr](macffr) module"]
pub type MACFFR = crate::Reg<u32, _MACFFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACFFR;
#[doc = "`read()` method returns [macffr::R](macffr::R) reader structure"]
impl crate::Readable for MACFFR {}
#[doc = "`write(|w| ..)` method takes [macffr::W](macffr::W) writer structure"]
impl crate::Writable for MACFFR {}
#[doc = "Ethernet MAC frame filter register"]
pub mod macffr;
#[doc = "Ethernet MAC hash table high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machthr](machthr) module"]
pub type MACHTHR = crate::Reg<u32, _MACHTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHTHR;
#[doc = "`read()` method returns [machthr::R](machthr::R) reader structure"]
impl crate::Readable for MACHTHR {}
#[doc = "`write(|w| ..)` method takes [machthr::W](machthr::W) writer structure"]
impl crate::Writable for MACHTHR {}
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "Ethernet MAC hash table low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machtlr](machtlr) module"]
pub type MACHTLR = crate::Reg<u32, _MACHTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHTLR;
#[doc = "`read()` method returns [machtlr::R](machtlr::R) reader structure"]
impl crate::Readable for MACHTLR {}
#[doc = "`write(|w| ..)` method takes [machtlr::W](machtlr::W) writer structure"]
impl crate::Writable for MACHTLR {}
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "Ethernet MAC MII address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmiiar](macmiiar) module"]
pub type MACMIIAR = crate::Reg<u32, _MACMIIAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACMIIAR;
#[doc = "`read()` method returns [macmiiar::R](macmiiar::R) reader structure"]
impl crate::Readable for MACMIIAR {}
#[doc = "`write(|w| ..)` method takes [macmiiar::W](macmiiar::W) writer structure"]
impl crate::Writable for MACMIIAR {}
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiar;
#[doc = "Ethernet MAC MII data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmiidr](macmiidr) module"]
pub type MACMIIDR = crate::Reg<u32, _MACMIIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACMIIDR;
#[doc = "`read()` method returns [macmiidr::R](macmiidr::R) reader structure"]
impl crate::Readable for MACMIIDR {}
#[doc = "`write(|w| ..)` method takes [macmiidr::W](macmiidr::W) writer structure"]
impl crate::Writable for MACMIIDR {}
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidr;
#[doc = "Ethernet MAC flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macfcr](macfcr) module"]
pub type MACFCR = crate::Reg<u32, _MACFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACFCR;
#[doc = "`read()` method returns [macfcr::R](macfcr::R) reader structure"]
impl crate::Readable for MACFCR {}
#[doc = "`write(|w| ..)` method takes [macfcr::W](macfcr::W) writer structure"]
impl crate::Writable for MACFCR {}
#[doc = "Ethernet MAC flow control register"]
pub mod macfcr;
#[doc = "Ethernet MAC VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvlantr](macvlantr) module"]
pub type MACVLANTR = crate::Reg<u32, _MACVLANTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACVLANTR;
#[doc = "`read()` method returns [macvlantr::R](macvlantr::R) reader structure"]
impl crate::Readable for MACVLANTR {}
#[doc = "`write(|w| ..)` method takes [macvlantr::W](macvlantr::W) writer structure"]
impl crate::Writable for MACVLANTR {}
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlantr;
#[doc = "Ethernet MAC PMT control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpmtcsr](macpmtcsr) module"]
pub type MACPMTCSR = crate::Reg<u32, _MACPMTCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPMTCSR;
#[doc = "`read()` method returns [macpmtcsr::R](macpmtcsr::R) reader structure"]
impl crate::Readable for MACPMTCSR {}
#[doc = "`write(|w| ..)` method takes [macpmtcsr::W](macpmtcsr::W) writer structure"]
impl crate::Writable for MACPMTCSR {}
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtcsr;
#[doc = "Ethernet MAC debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macdbgr](macdbgr) module"]
pub type MACDBGR = crate::Reg<u32, _MACDBGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACDBGR;
#[doc = "`read()` method returns [macdbgr::R](macdbgr::R) reader structure"]
impl crate::Readable for MACDBGR {}
#[doc = "Ethernet MAC debug register"]
pub mod macdbgr;
#[doc = "Ethernet MAC interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macsr](macsr) module"]
pub type MACSR = crate::Reg<u32, _MACSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSR;
#[doc = "`read()` method returns [macsr::R](macsr::R) reader structure"]
impl crate::Readable for MACSR {}
#[doc = "`write(|w| ..)` method takes [macsr::W](macsr::W) writer structure"]
impl crate::Writable for MACSR {}
#[doc = "Ethernet MAC interrupt status register"]
pub mod macsr;
#[doc = "Ethernet MAC interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macimr](macimr) module"]
pub type MACIMR = crate::Reg<u32, _MACIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACIMR;
#[doc = "`read()` method returns [macimr::R](macimr::R) reader structure"]
impl crate::Readable for MACIMR {}
#[doc = "`write(|w| ..)` method takes [macimr::W](macimr::W) writer structure"]
impl crate::Writable for MACIMR {}
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "Ethernet MAC address 0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0hr](maca0hr) module"]
pub type MACA0HR = crate::Reg<u32, _MACA0HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA0HR;
#[doc = "`read()` method returns [maca0hr::R](maca0hr::R) reader structure"]
impl crate::Readable for MACA0HR {}
#[doc = "`write(|w| ..)` method takes [maca0hr::W](maca0hr::W) writer structure"]
impl crate::Writable for MACA0HR {}
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "Ethernet MAC address 0 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0lr](maca0lr) module"]
pub type MACA0LR = crate::Reg<u32, _MACA0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA0LR;
#[doc = "`read()` method returns [maca0lr::R](maca0lr::R) reader structure"]
impl crate::Readable for MACA0LR {}
#[doc = "`write(|w| ..)` method takes [maca0lr::W](maca0lr::W) writer structure"]
impl crate::Writable for MACA0LR {}
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "Ethernet MAC address 1 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca1hr](maca1hr) module"]
pub type MACA1HR = crate::Reg<u32, _MACA1HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA1HR;
#[doc = "`read()` method returns [maca1hr::R](maca1hr::R) reader structure"]
impl crate::Readable for MACA1HR {}
#[doc = "`write(|w| ..)` method takes [maca1hr::W](maca1hr::W) writer structure"]
impl crate::Writable for MACA1HR {}
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "Ethernet MAC address1 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca1lr](maca1lr) module"]
pub type MACA1LR = crate::Reg<u32, _MACA1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA1LR;
#[doc = "`read()` method returns [maca1lr::R](maca1lr::R) reader structure"]
impl crate::Readable for MACA1LR {}
#[doc = "`write(|w| ..)` method takes [maca1lr::W](maca1lr::W) writer structure"]
impl crate::Writable for MACA1LR {}
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "Ethernet MAC address 2 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2hr](maca2hr) module"]
pub type MACA2HR = crate::Reg<u32, _MACA2HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA2HR;
#[doc = "`read()` method returns [maca2hr::R](maca2hr::R) reader structure"]
impl crate::Readable for MACA2HR {}
#[doc = "`write(|w| ..)` method takes [maca2hr::W](maca2hr::W) writer structure"]
impl crate::Writable for MACA2HR {}
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "Ethernet MAC address 2 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2lr](maca2lr) module"]
pub type MACA2LR = crate::Reg<u32, _MACA2LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA2LR;
#[doc = "`read()` method returns [maca2lr::R](maca2lr::R) reader structure"]
impl crate::Readable for MACA2LR {}
#[doc = "`write(|w| ..)` method takes [maca2lr::W](maca2lr::W) writer structure"]
impl crate::Writable for MACA2LR {}
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "Ethernet MAC address 3 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca3hr](maca3hr) module"]
pub type MACA3HR = crate::Reg<u32, _MACA3HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA3HR;
#[doc = "`read()` method returns [maca3hr::R](maca3hr::R) reader structure"]
impl crate::Readable for MACA3HR {}
#[doc = "`write(|w| ..)` method takes [maca3hr::W](maca3hr::W) writer structure"]
impl crate::Writable for MACA3HR {}
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "Ethernet MAC address 3 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca3lr](maca3lr) module"]
pub type MACA3LR = crate::Reg<u32, _MACA3LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA3LR;
#[doc = "`read()` method returns [maca3lr::R](maca3lr::R) reader structure"]
impl crate::Readable for MACA3LR {}
#[doc = "`write(|w| ..)` method takes [maca3lr::W](maca3lr::W) writer structure"]
impl crate::Writable for MACA3LR {}
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
