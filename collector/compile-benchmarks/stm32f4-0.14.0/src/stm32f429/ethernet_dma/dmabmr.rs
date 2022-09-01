#[doc = "Register `DMABMR` reader"]
pub struct R(crate::R<DMABMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABMR` writer"]
pub struct W(crate::W<DMABMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMABMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "1: Reset all MAC subsystem internal registers and logic. Cleared automatically"]
    RESET = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Software reset"]
pub struct SR_R(crate::FieldReader<bool, SR_A>);
impl SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SR_A> {
        match self.bits {
            true => Some(SR_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SR_A::RESET
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Software reset"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset all MAC subsystem internal registers and logic. Cleared automatically"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SR_A::RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "DMA arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DA_A {
    #[doc = "0: Round-robin with Rx:Tx priority given by PM"]
    ROUNDROBIN = 0,
    #[doc = "1: Rx has priority over Tx"]
    RXPRIORITY = 1,
}
impl From<DA_A> for bool {
    #[inline(always)]
    fn from(variant: DA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DA` reader - DMA arbitration"]
pub struct DA_R(crate::FieldReader<bool, DA_A>);
impl DA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DA_A {
        match self.bits {
            false => DA_A::ROUNDROBIN,
            true => DA_A::RXPRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `ROUNDROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        **self == DA_A::ROUNDROBIN
    }
    #[doc = "Checks if the value of the field is `RXPRIORITY`"]
    #[inline(always)]
    pub fn is_rx_priority(&self) -> bool {
        **self == DA_A::RXPRIORITY
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<bool, DA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - DMA arbitration"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Round-robin with Rx:Tx priority given by PM"]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(DA_A::ROUNDROBIN)
    }
    #[doc = "Rx has priority over Tx"]
    #[inline(always)]
    pub fn rx_priority(self) -> &'a mut W {
        self.variant(DA_A::RXPRIORITY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub struct DSL_R(crate::FieldReader<u8, u8>);
impl DSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Enhanced descriptor format enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFE_A {
    #[doc = "0: Normal descriptor format"]
    DISABLED = 0,
    #[doc = "1: Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload"]
    ENABLED = 1,
}
impl From<EDFE_A> for bool {
    #[inline(always)]
    fn from(variant: EDFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDFE` reader - Enhanced descriptor format enable"]
pub struct EDFE_R(crate::FieldReader<bool, EDFE_A>);
impl EDFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDFE_A {
        match self.bits {
            false => EDFE_A::DISABLED,
            true => EDFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EDFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EDFE_A::ENABLED
    }
}
impl core::ops::Deref for EDFE_R {
    type Target = crate::FieldReader<bool, EDFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDFE` writer - Enhanced descriptor format enable"]
pub struct EDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal descriptor format"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDFE_A::DISABLED)
    }
    #[doc = "Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDFE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Programmable burst length\n\nValue on reset: 33"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PBL_A {
    #[doc = "1: Maximum of 1 beat per DMA transaction"]
    PBL1 = 1,
    #[doc = "2: Maximum of 2 beats per DMA transaction"]
    PBL2 = 2,
    #[doc = "4: Maximum of 4 beats per DMA transaction"]
    PBL4 = 4,
    #[doc = "8: Maximum of 8 beats per DMA transaction"]
    PBL8 = 8,
    #[doc = "16: Maximum of 16 beats per DMA transaction"]
    PBL16 = 16,
    #[doc = "32: Maximum of 32 beats per DMA transaction"]
    PBL32 = 32,
}
impl From<PBL_A> for u8 {
    #[inline(always)]
    fn from(variant: PBL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PBL` reader - Programmable burst length"]
pub struct PBL_R(crate::FieldReader<u8, PBL_A>);
impl PBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PBL_A> {
        match self.bits {
            1 => Some(PBL_A::PBL1),
            2 => Some(PBL_A::PBL2),
            4 => Some(PBL_A::PBL4),
            8 => Some(PBL_A::PBL8),
            16 => Some(PBL_A::PBL16),
            32 => Some(PBL_A::PBL32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PBL1`"]
    #[inline(always)]
    pub fn is_pbl1(&self) -> bool {
        **self == PBL_A::PBL1
    }
    #[doc = "Checks if the value of the field is `PBL2`"]
    #[inline(always)]
    pub fn is_pbl2(&self) -> bool {
        **self == PBL_A::PBL2
    }
    #[doc = "Checks if the value of the field is `PBL4`"]
    #[inline(always)]
    pub fn is_pbl4(&self) -> bool {
        **self == PBL_A::PBL4
    }
    #[doc = "Checks if the value of the field is `PBL8`"]
    #[inline(always)]
    pub fn is_pbl8(&self) -> bool {
        **self == PBL_A::PBL8
    }
    #[doc = "Checks if the value of the field is `PBL16`"]
    #[inline(always)]
    pub fn is_pbl16(&self) -> bool {
        **self == PBL_A::PBL16
    }
    #[doc = "Checks if the value of the field is `PBL32`"]
    #[inline(always)]
    pub fn is_pbl32(&self) -> bool {
        **self == PBL_A::PBL32
    }
}
impl core::ops::Deref for PBL_R {
    type Target = crate::FieldReader<u8, PBL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBL` writer - Programmable burst length"]
pub struct PBL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Maximum of 1 beat per DMA transaction"]
    #[inline(always)]
    pub fn pbl1(self) -> &'a mut W {
        self.variant(PBL_A::PBL1)
    }
    #[doc = "Maximum of 2 beats per DMA transaction"]
    #[inline(always)]
    pub fn pbl2(self) -> &'a mut W {
        self.variant(PBL_A::PBL2)
    }
    #[doc = "Maximum of 4 beats per DMA transaction"]
    #[inline(always)]
    pub fn pbl4(self) -> &'a mut W {
        self.variant(PBL_A::PBL4)
    }
    #[doc = "Maximum of 8 beats per DMA transaction"]
    #[inline(always)]
    pub fn pbl8(self) -> &'a mut W {
        self.variant(PBL_A::PBL8)
    }
    #[doc = "Maximum of 16 beats per DMA transaction"]
    #[inline(always)]
    pub fn pbl16(self) -> &'a mut W {
        self.variant(PBL_A::PBL16)
    }
    #[doc = "Maximum of 32 beats per DMA transaction"]
    #[inline(always)]
    pub fn pbl32(self) -> &'a mut W {
        self.variant(PBL_A::PBL32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Rx-Tx priority ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: RxDMA priority over TxDMA is 1:1"]
    ONETOONE = 0,
    #[doc = "1: RxDMA priority over TxDMA is 2:1"]
    TWOTOONE = 1,
    #[doc = "2: RxDMA priority over TxDMA is 3:1"]
    THREETOONE = 2,
    #[doc = "3: RxDMA priority over TxDMA is 4:1"]
    FOURTOONE = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PM` reader - Rx-Tx priority ratio"]
pub struct PM_R(crate::FieldReader<u8, PM_A>);
impl PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            0 => PM_A::ONETOONE,
            1 => PM_A::TWOTOONE,
            2 => PM_A::THREETOONE,
            3 => PM_A::FOURTOONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONETOONE`"]
    #[inline(always)]
    pub fn is_one_to_one(&self) -> bool {
        **self == PM_A::ONETOONE
    }
    #[doc = "Checks if the value of the field is `TWOTOONE`"]
    #[inline(always)]
    pub fn is_two_to_one(&self) -> bool {
        **self == PM_A::TWOTOONE
    }
    #[doc = "Checks if the value of the field is `THREETOONE`"]
    #[inline(always)]
    pub fn is_three_to_one(&self) -> bool {
        **self == PM_A::THREETOONE
    }
    #[doc = "Checks if the value of the field is `FOURTOONE`"]
    #[inline(always)]
    pub fn is_four_to_one(&self) -> bool {
        **self == PM_A::FOURTOONE
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<u8, PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Rx-Tx priority ratio"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RxDMA priority over TxDMA is 1:1"]
    #[inline(always)]
    pub fn one_to_one(self) -> &'a mut W {
        self.variant(PM_A::ONETOONE)
    }
    #[doc = "RxDMA priority over TxDMA is 2:1"]
    #[inline(always)]
    pub fn two_to_one(self) -> &'a mut W {
        self.variant(PM_A::TWOTOONE)
    }
    #[doc = "RxDMA priority over TxDMA is 3:1"]
    #[inline(always)]
    pub fn three_to_one(self) -> &'a mut W {
        self.variant(PM_A::THREETOONE)
    }
    #[doc = "RxDMA priority over TxDMA is 4:1"]
    #[inline(always)]
    pub fn four_to_one(self) -> &'a mut W {
        self.variant(PM_A::FOURTOONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Fixed burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB_A {
    #[doc = "0: AHB uses SINGLE and INCR burst transfers"]
    VARIABLE = 0,
    #[doc = "1: AHB uses only fixed burst transfers"]
    FIXED = 1,
}
impl From<FB_A> for bool {
    #[inline(always)]
    fn from(variant: FB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FB` reader - Fixed burst"]
pub struct FB_R(crate::FieldReader<bool, FB_A>);
impl FB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB_A {
        match self.bits {
            false => FB_A::VARIABLE,
            true => FB_A::FIXED,
        }
    }
    #[doc = "Checks if the value of the field is `VARIABLE`"]
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        **self == FB_A::VARIABLE
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == FB_A::FIXED
    }
}
impl core::ops::Deref for FB_R {
    type Target = crate::FieldReader<bool, FB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB` writer - Fixed burst"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "AHB uses SINGLE and INCR burst transfers"]
    #[inline(always)]
    pub fn variable(self) -> &'a mut W {
        self.variant(FB_A::VARIABLE)
    }
    #[doc = "AHB uses only fixed burst transfers"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(FB_A::FIXED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Rx DMA PBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDP_A {
    #[doc = "1: 1 beat per RxDMA transaction"]
    RDP1 = 1,
    #[doc = "2: 2 beats per RxDMA transaction"]
    RDP2 = 2,
    #[doc = "4: 4 beats per RxDMA transaction"]
    RDP4 = 4,
    #[doc = "8: 8 beats per RxDMA transaction"]
    RDP8 = 8,
    #[doc = "16: 16 beats per RxDMA transaction"]
    RDP16 = 16,
    #[doc = "32: 32 beats per RxDMA transaction"]
    RDP32 = 32,
}
impl From<RDP_A> for u8 {
    #[inline(always)]
    fn from(variant: RDP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub struct RDP_R(crate::FieldReader<u8, RDP_A>);
impl RDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDP_A> {
        match self.bits {
            1 => Some(RDP_A::RDP1),
            2 => Some(RDP_A::RDP2),
            4 => Some(RDP_A::RDP4),
            8 => Some(RDP_A::RDP8),
            16 => Some(RDP_A::RDP16),
            32 => Some(RDP_A::RDP32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RDP1`"]
    #[inline(always)]
    pub fn is_rdp1(&self) -> bool {
        **self == RDP_A::RDP1
    }
    #[doc = "Checks if the value of the field is `RDP2`"]
    #[inline(always)]
    pub fn is_rdp2(&self) -> bool {
        **self == RDP_A::RDP2
    }
    #[doc = "Checks if the value of the field is `RDP4`"]
    #[inline(always)]
    pub fn is_rdp4(&self) -> bool {
        **self == RDP_A::RDP4
    }
    #[doc = "Checks if the value of the field is `RDP8`"]
    #[inline(always)]
    pub fn is_rdp8(&self) -> bool {
        **self == RDP_A::RDP8
    }
    #[doc = "Checks if the value of the field is `RDP16`"]
    #[inline(always)]
    pub fn is_rdp16(&self) -> bool {
        **self == RDP_A::RDP16
    }
    #[doc = "Checks if the value of the field is `RDP32`"]
    #[inline(always)]
    pub fn is_rdp32(&self) -> bool {
        **self == RDP_A::RDP32
    }
}
impl core::ops::Deref for RDP_R {
    type Target = crate::FieldReader<u8, RDP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 beat per RxDMA transaction"]
    #[inline(always)]
    pub fn rdp1(self) -> &'a mut W {
        self.variant(RDP_A::RDP1)
    }
    #[doc = "2 beats per RxDMA transaction"]
    #[inline(always)]
    pub fn rdp2(self) -> &'a mut W {
        self.variant(RDP_A::RDP2)
    }
    #[doc = "4 beats per RxDMA transaction"]
    #[inline(always)]
    pub fn rdp4(self) -> &'a mut W {
        self.variant(RDP_A::RDP4)
    }
    #[doc = "8 beats per RxDMA transaction"]
    #[inline(always)]
    pub fn rdp8(self) -> &'a mut W {
        self.variant(RDP_A::RDP8)
    }
    #[doc = "16 beats per RxDMA transaction"]
    #[inline(always)]
    pub fn rdp16(self) -> &'a mut W {
        self.variant(RDP_A::RDP16)
    }
    #[doc = "32 beats per RxDMA transaction"]
    #[inline(always)]
    pub fn rdp32(self) -> &'a mut W {
        self.variant(RDP_A::RDP32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | ((value as u32 & 0x3f) << 17);
        self.w
    }
}
#[doc = "Use separate PBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USP_A {
    #[doc = "0: PBL value used for both Rx and Tx DMA"]
    COMBINED = 0,
    #[doc = "1: RxDMA uses RDP value, TxDMA uses PBL value"]
    SEPARATE = 1,
}
impl From<USP_A> for bool {
    #[inline(always)]
    fn from(variant: USP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USP` reader - Use separate PBL"]
pub struct USP_R(crate::FieldReader<bool, USP_A>);
impl USP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USP_A {
        match self.bits {
            false => USP_A::COMBINED,
            true => USP_A::SEPARATE,
        }
    }
    #[doc = "Checks if the value of the field is `COMBINED`"]
    #[inline(always)]
    pub fn is_combined(&self) -> bool {
        **self == USP_A::COMBINED
    }
    #[doc = "Checks if the value of the field is `SEPARATE`"]
    #[inline(always)]
    pub fn is_separate(&self) -> bool {
        **self == USP_A::SEPARATE
    }
}
impl core::ops::Deref for USP_R {
    type Target = crate::FieldReader<bool, USP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USP` writer - Use separate PBL"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PBL value used for both Rx and Tx DMA"]
    #[inline(always)]
    pub fn combined(self) -> &'a mut W {
        self.variant(USP_A::COMBINED)
    }
    #[doc = "RxDMA uses RDP value, TxDMA uses PBL value"]
    #[inline(always)]
    pub fn separate(self) -> &'a mut W {
        self.variant(USP_A::SEPARATE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "4xPBL mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPM_A {
    #[doc = "0: PBL values used as-is"]
    X1 = 0,
    #[doc = "1: PBL values multiplied by 4"]
    X4 = 1,
}
impl From<FPM_A> for bool {
    #[inline(always)]
    fn from(variant: FPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPM` reader - 4xPBL mode"]
pub struct FPM_R(crate::FieldReader<bool, FPM_A>);
impl FPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPM_A {
        match self.bits {
            false => FPM_A::X1,
            true => FPM_A::X4,
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        **self == FPM_A::X1
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        **self == FPM_A::X4
    }
}
impl core::ops::Deref for FPM_R {
    type Target = crate::FieldReader<bool, FPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPM` writer - 4xPBL mode"]
pub struct FPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PBL values used as-is"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(FPM_A::X1)
    }
    #[doc = "PBL values multiplied by 4"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(FPM_A::X4)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Address-aligned beats\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAB_A {
    #[doc = "0: Bursts are not aligned"]
    UNALIGNED = 0,
    #[doc = "1: Align bursts to start address LS bits. First burst alignment depends on FB bit"]
    ALIGNED = 1,
}
impl From<AAB_A> for bool {
    #[inline(always)]
    fn from(variant: AAB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub struct AAB_R(crate::FieldReader<bool, AAB_A>);
impl AAB_R {
    pub(crate) fn new(bits: bool) -> Self {
        AAB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAB_A {
        match self.bits {
            false => AAB_A::UNALIGNED,
            true => AAB_A::ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGNED`"]
    #[inline(always)]
    pub fn is_unaligned(&self) -> bool {
        **self == AAB_A::UNALIGNED
    }
    #[doc = "Checks if the value of the field is `ALIGNED`"]
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        **self == AAB_A::ALIGNED
    }
}
impl core::ops::Deref for AAB_R {
    type Target = crate::FieldReader<bool, AAB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub struct AAB_W<'a> {
    w: &'a mut W,
}
impl<'a> AAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AAB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bursts are not aligned"]
    #[inline(always)]
    pub fn unaligned(self) -> &'a mut W {
        self.variant(AAB_A::UNALIGNED)
    }
    #[doc = "Align bursts to start address LS bits. First burst alignment depends on FB bit"]
    #[inline(always)]
    pub fn aligned(self) -> &'a mut W {
        self.variant(AAB_A::ALIGNED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Mixed burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_A {
    #[doc = "0: Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
    NORMAL = 0,
    #[doc = "1: If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
    MIXED = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB` reader - Mixed burst"]
pub struct MB_R(crate::FieldReader<bool, MB_A>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB_A {
        match self.bits {
            false => MB_A::NORMAL,
            true => MB_A::MIXED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == MB_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MIXED`"]
    #[inline(always)]
    pub fn is_mixed(&self) -> bool {
        **self == MB_A::MIXED
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, MB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - Mixed burst"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MB_A::NORMAL)
    }
    #[doc = "If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
    #[inline(always)]
    pub fn mixed(self) -> &'a mut W {
        self.variant(MB_A::MIXED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enhanced descriptor format enable"]
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx-Tx priority ratio"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Mixed burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 1 - DMA arbitration"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    #[doc = "Bit 7 - Enhanced descriptor format enable"]
    #[inline(always)]
    pub fn edfe(&mut self) -> EDFE_W {
        EDFE_W { w: self }
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W {
        PBL_W { w: self }
    }
    #[doc = "Bits 14:15 - Rx-Tx priority ratio"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W {
        FPM_W { w: self }
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W {
        AAB_W { w: self }
    }
    #[doc = "Bit 26 - Mixed burst"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabmr](index.html) module"]
pub struct DMABMR_SPEC;
impl crate::RegisterSpec for DMABMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabmr::R](R) reader structure"]
impl crate::Readable for DMABMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabmr::W](W) writer structure"]
impl crate::Writable for DMABMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABMR to value 0x2101"]
impl crate::Resettable for DMABMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101
    }
}
