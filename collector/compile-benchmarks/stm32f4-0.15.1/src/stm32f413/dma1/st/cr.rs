#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL` reader - Channel selection"]
pub type CHSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHSEL` writer - Channel selection"]
pub type CHSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Memory burst transfer configuration"]
pub use PBURST_A as MBURST_A;
#[doc = "Field `MBURST` reader - Memory burst transfer configuration"]
pub use PBURST_R as MBURST_R;
#[doc = "Field `MBURST` writer - Memory burst transfer configuration"]
pub use PBURST_W as MBURST_W;
#[doc = "Peripheral burst transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PBURST_A {
    #[doc = "0: Single transfer"]
    Single = 0,
    #[doc = "1: Incremental burst of 4 beats"]
    Incr4 = 1,
    #[doc = "2: Incremental burst of 8 beats"]
    Incr8 = 2,
    #[doc = "3: Incremental burst of 16 beats"]
    Incr16 = 3,
}
impl From<PBURST_A> for u8 {
    #[inline(always)]
    fn from(variant: PBURST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PBURST` reader - Peripheral burst transfer configuration"]
pub type PBURST_R = crate::FieldReader<u8, PBURST_A>;
impl PBURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBURST_A {
        match self.bits {
            0 => PBURST_A::Single,
            1 => PBURST_A::Incr4,
            2 => PBURST_A::Incr8,
            3 => PBURST_A::Incr16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PBURST_A::Single
    }
    #[doc = "Checks if the value of the field is `Incr4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == PBURST_A::Incr4
    }
    #[doc = "Checks if the value of the field is `Incr8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == PBURST_A::Incr8
    }
    #[doc = "Checks if the value of the field is `Incr16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == PBURST_A::Incr16
    }
}
#[doc = "Field `PBURST` writer - Peripheral burst transfer configuration"]
pub type PBURST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, PBURST_A, 2, O>;
impl<'a, const O: u8> PBURST_W<'a, O> {
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(PBURST_A::Single)
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(PBURST_A::Incr4)
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(PBURST_A::Incr8)
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(PBURST_A::Incr16)
    }
}
#[doc = "Current target (only in double buffer mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT_A {
    #[doc = "0: The current target memory is Memory 0"]
    Memory0 = 0,
    #[doc = "1: The current target memory is Memory 1"]
    Memory1 = 1,
}
impl From<CT_A> for bool {
    #[inline(always)]
    fn from(variant: CT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT` reader - Current target (only in double buffer mode)"]
pub type CT_R = crate::BitReader<CT_A>;
impl CT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT_A {
        match self.bits {
            false => CT_A::Memory0,
            true => CT_A::Memory1,
        }
    }
    #[doc = "Checks if the value of the field is `Memory0`"]
    #[inline(always)]
    pub fn is_memory0(&self) -> bool {
        *self == CT_A::Memory0
    }
    #[doc = "Checks if the value of the field is `Memory1`"]
    #[inline(always)]
    pub fn is_memory1(&self) -> bool {
        *self == CT_A::Memory1
    }
}
#[doc = "Field `CT` writer - Current target (only in double buffer mode)"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CT_A, O>;
impl<'a, const O: u8> CT_W<'a, O> {
    #[doc = "The current target memory is Memory 0"]
    #[inline(always)]
    pub fn memory0(self) -> &'a mut W {
        self.variant(CT_A::Memory0)
    }
    #[doc = "The current target memory is Memory 1"]
    #[inline(always)]
    pub fn memory1(self) -> &'a mut W {
        self.variant(CT_A::Memory1)
    }
}
#[doc = "Double buffer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBM_A {
    #[doc = "0: No buffer switching at the end of transfer"]
    Disabled = 0,
    #[doc = "1: Memory target switched at the end of the DMA transfer"]
    Enabled = 1,
}
impl From<DBM_A> for bool {
    #[inline(always)]
    fn from(variant: DBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBM` reader - Double buffer mode"]
pub type DBM_R = crate::BitReader<DBM_A>;
impl DBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBM_A {
        match self.bits {
            false => DBM_A::Disabled,
            true => DBM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBM_A::Enabled
    }
}
#[doc = "Field `DBM` writer - Double buffer mode"]
pub type DBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBM_A, O>;
impl<'a, const O: u8> DBM_W<'a, O> {
    #[doc = "No buffer switching at the end of transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBM_A::Disabled)
    }
    #[doc = "Memory target switched at the end of the DMA transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBM_A::Enabled)
    }
}
#[doc = "Priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: Medium"]
    Medium = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "3: Very high"]
    VeryHigh = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PL` reader - Priority level"]
pub type PL_R = crate::FieldReader<u8, PL_A>;
impl PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::Low,
            1 => PL_A::Medium,
            2 => PL_A::High,
            3 => PL_A::VeryHigh,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL_A::Low
    }
    #[doc = "Checks if the value of the field is `Medium`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL_A::Medium
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL_A::High
    }
    #[doc = "Checks if the value of the field is `VeryHigh`"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL_A::VeryHigh
    }
}
#[doc = "Field `PL` writer - Priority level"]
pub type PL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, PL_A, 2, O>;
impl<'a, const O: u8> PL_W<'a, O> {
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PL_A::Low)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PL_A::Medium)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PL_A::High)
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PL_A::VeryHigh)
    }
}
#[doc = "Peripheral increment offset size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCOS_A {
    #[doc = "0: The offset size for the peripheral address calculation is linked to the PSIZE"]
    Psize = 0,
    #[doc = "1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    Fixed4 = 1,
}
impl From<PINCOS_A> for bool {
    #[inline(always)]
    fn from(variant: PINCOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub type PINCOS_R = crate::BitReader<PINCOS_A>;
impl PINCOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCOS_A {
        match self.bits {
            false => PINCOS_A::Psize,
            true => PINCOS_A::Fixed4,
        }
    }
    #[doc = "Checks if the value of the field is `Psize`"]
    #[inline(always)]
    pub fn is_psize(&self) -> bool {
        *self == PINCOS_A::Psize
    }
    #[doc = "Checks if the value of the field is `Fixed4`"]
    #[inline(always)]
    pub fn is_fixed4(&self) -> bool {
        *self == PINCOS_A::Fixed4
    }
}
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub type PINCOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PINCOS_A, O>;
impl<'a, const O: u8> PINCOS_W<'a, O> {
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    #[inline(always)]
    pub fn psize(self) -> &'a mut W {
        self.variant(PINCOS_A::Psize)
    }
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    #[inline(always)]
    pub fn fixed4(self) -> &'a mut W {
        self.variant(PINCOS_A::Fixed4)
    }
}
#[doc = "Memory data size"]
pub use PSIZE_A as MSIZE_A;
#[doc = "Field `MSIZE` reader - Memory data size"]
pub use PSIZE_R as MSIZE_R;
#[doc = "Field `MSIZE` writer - Memory data size"]
pub use PSIZE_W as MSIZE_W;
#[doc = "Peripheral data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: Byte (8-bit)"]
    Bits8 = 0,
    #[doc = "1: Half-word (16-bit)"]
    Bits16 = 1,
    #[doc = "2: Word (32-bit)"]
    Bits32 = 2,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSIZE` reader - Peripheral data size"]
pub type PSIZE_R = crate::FieldReader<u8, PSIZE_A>;
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSIZE_A> {
        match self.bits {
            0 => Some(PSIZE_A::Bits8),
            1 => Some(PSIZE_A::Bits16),
            2 => Some(PSIZE_A::Bits32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE_A::Bits32
    }
}
#[doc = "Field `PSIZE` writer - Peripheral data size"]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, PSIZE_A, 2, O>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits8)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits16)
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits32)
    }
}
#[doc = "Memory increment mode"]
pub use PINC_A as MINC_A;
#[doc = "Field `MINC` reader - Memory increment mode"]
pub use PINC_R as MINC_R;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub use PINC_W as MINC_W;
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINC_A {
    #[doc = "0: Address pointer is fixed"]
    Fixed = 0,
    #[doc = "1: Address pointer is incremented after each data transfer"]
    Incremented = 1,
}
impl From<PINC_A> for bool {
    #[inline(always)]
    fn from(variant: PINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PINC_R = crate::BitReader<PINC_A>;
impl PINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINC_A {
        match self.bits {
            false => PINC_A::Fixed,
            true => PINC_A::Incremented,
        }
    }
    #[doc = "Checks if the value of the field is `Fixed`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == PINC_A::Fixed
    }
    #[doc = "Checks if the value of the field is `Incremented`"]
    #[inline(always)]
    pub fn is_incremented(&self) -> bool {
        *self == PINC_A::Incremented
    }
}
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PINC_A, O>;
impl<'a, const O: u8> PINC_W<'a, O> {
    #[doc = "Address pointer is fixed"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(PINC_A::Fixed)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline(always)]
    pub fn incremented(self) -> &'a mut W {
        self.variant(PINC_A::Incremented)
    }
}
#[doc = "Circular mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRC_A {
    #[doc = "0: Circular mode disabled"]
    Disabled = 0,
    #[doc = "1: Circular mode enabled"]
    Enabled = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CIRC_R = crate::BitReader<CIRC_A>;
impl CIRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::Disabled,
            true => CIRC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC_A::Enabled
    }
}
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CIRC_A, O>;
impl<'a, const O: u8> CIRC_W<'a, O> {
    #[doc = "Circular mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRC_A::Disabled)
    }
    #[doc = "Circular mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRC_A::Enabled)
    }
}
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIR_A {
    #[doc = "0: Peripheral-to-memory"]
    PeripheralToMemory = 0,
    #[doc = "1: Memory-to-peripheral"]
    MemoryToPeripheral = 1,
    #[doc = "2: Memory-to-memory"]
    MemoryToMemory = 2,
}
impl From<DIR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DIR_R = crate::FieldReader<u8, DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIR_A> {
        match self.bits {
            0 => Some(DIR_A::PeripheralToMemory),
            1 => Some(DIR_A::MemoryToPeripheral),
            2 => Some(DIR_A::MemoryToMemory),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PeripheralToMemory`"]
    #[inline(always)]
    pub fn is_peripheral_to_memory(&self) -> bool {
        *self == DIR_A::PeripheralToMemory
    }
    #[doc = "Checks if the value of the field is `MemoryToPeripheral`"]
    #[inline(always)]
    pub fn is_memory_to_peripheral(&self) -> bool {
        *self == DIR_A::MemoryToPeripheral
    }
    #[doc = "Checks if the value of the field is `MemoryToMemory`"]
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == DIR_A::MemoryToMemory
    }
}
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, DIR_A, 2, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Peripheral-to-memory"]
    #[inline(always)]
    pub fn peripheral_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::PeripheralToMemory)
    }
    #[doc = "Memory-to-peripheral"]
    #[inline(always)]
    pub fn memory_to_peripheral(self) -> &'a mut W {
        self.variant(DIR_A::MemoryToPeripheral)
    }
    #[doc = "Memory-to-memory"]
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::MemoryToMemory)
    }
}
#[doc = "Peripheral flow controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFCTRL_A {
    #[doc = "0: The DMA is the flow controller"]
    Dma = 0,
    #[doc = "1: The peripheral is the flow controller"]
    Peripheral = 1,
}
impl From<PFCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: PFCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub type PFCTRL_R = crate::BitReader<PFCTRL_A>;
impl PFCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFCTRL_A {
        match self.bits {
            false => PFCTRL_A::Dma,
            true => PFCTRL_A::Peripheral,
        }
    }
    #[doc = "Checks if the value of the field is `Dma`"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == PFCTRL_A::Dma
    }
    #[doc = "Checks if the value of the field is `Peripheral`"]
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == PFCTRL_A::Peripheral
    }
}
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub type PFCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PFCTRL_A, O>;
impl<'a, const O: u8> PFCTRL_W<'a, O> {
    #[doc = "The DMA is the flow controller"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut W {
        self.variant(PFCTRL_A::Dma)
    }
    #[doc = "The peripheral is the flow controller"]
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut W {
        self.variant(PFCTRL_A::Peripheral)
    }
}
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: TC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TC interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::Disabled,
            true => TCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIE_A {
    #[doc = "0: HT interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HT interrupt enabled"]
    Enabled = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIE` reader - Half transfer interrupt enable"]
pub type HTIE_R = crate::BitReader<HTIE_A>;
impl HTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::Disabled,
            true => HTIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE_A::Enabled
    }
}
#[doc = "Field `HTIE` writer - Half transfer interrupt enable"]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HTIE_A, O>;
impl<'a, const O: u8> HTIE_W<'a, O> {
    #[doc = "HT interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIE_A::Disabled)
    }
    #[doc = "HT interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIE_A::Enabled)
    }
}
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIE_A {
    #[doc = "0: TE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TE interrupt enabled"]
    Enabled = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::Disabled,
            true => TEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE_A::Enabled
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::Disabled)
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::Enabled)
    }
}
#[doc = "Direct mode error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIE_A {
    #[doc = "0: DME interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DME interrupt enabled"]
    Enabled = 1,
}
impl From<DMEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIE` reader - Direct mode error interrupt enable"]
pub type DMEIE_R = crate::BitReader<DMEIE_A>;
impl DMEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIE_A {
        match self.bits {
            false => DMEIE_A::Disabled,
            true => DMEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMEIE_A::Enabled
    }
}
#[doc = "Field `DMEIE` writer - Direct mode error interrupt enable"]
pub type DMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMEIE_A, O>;
impl<'a, const O: u8> DMEIE_W<'a, O> {
    #[doc = "DME interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMEIE_A::Disabled)
    }
    #[doc = "DME interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMEIE_A::Enabled)
    }
}
#[doc = "Stream enable / flag stream ready when read low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Stream disabled"]
    Disabled = 0,
    #[doc = "1: Stream enabled"]
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Stream enable / flag stream ready when read low"]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
#[doc = "Field `EN` writer - Stream enable / flag stream ready when read low"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Stream disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "Stream enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 25:28 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 25:28 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W<25> {
        CHSEL_W::new(self)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W<23> {
        MBURST_W::new(self)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W<21> {
        PBURST_W::new(self)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<19> {
        CT_W::new(self)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<18> {
        DBM_W::new(self)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<16> {
        PL_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W<15> {
        PINCOS_W::new(self)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<13> {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<11> {
        PSIZE_W::new(self)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<10> {
        MINC_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<9> {
        PINC_W::new(self)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<8> {
        CIRC_W::new(self)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<6> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W<5> {
        PFCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<4> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<3> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<2> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W<1> {
        DMEIE_W::new(self)
    }
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
