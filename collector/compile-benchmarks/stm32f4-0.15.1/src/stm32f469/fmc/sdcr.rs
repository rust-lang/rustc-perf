#[doc = "Register `SDCR%s` reader"]
pub struct R(crate::R<SDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCR%s` writer"]
pub struct W(crate::W<SDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCR_SPEC>;
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
impl From<crate::W<SDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Number of column address bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NC_A {
    #[doc = "0: 8 bits"]
    Bits8 = 0,
    #[doc = "1: 9 bits"]
    Bits9 = 1,
    #[doc = "2: 10 bits"]
    Bits10 = 2,
    #[doc = "3: 11 bits"]
    Bits11 = 3,
}
impl From<NC_A> for u8 {
    #[inline(always)]
    fn from(variant: NC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NC` reader - Number of column address bits"]
pub type NC_R = crate::FieldReader<u8, NC_A>;
impl NC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NC_A {
        match self.bits {
            0 => NC_A::Bits8,
            1 => NC_A::Bits9,
            2 => NC_A::Bits10,
            3 => NC_A::Bits11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == NC_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits9`"]
    #[inline(always)]
    pub fn is_bits9(&self) -> bool {
        *self == NC_A::Bits9
    }
    #[doc = "Checks if the value of the field is `Bits10`"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == NC_A::Bits10
    }
    #[doc = "Checks if the value of the field is `Bits11`"]
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        *self == NC_A::Bits11
    }
}
#[doc = "Field `NC` writer - Number of column address bits"]
pub type NC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDCR_SPEC, u8, NC_A, 2, O>;
impl<'a, const O: u8> NC_W<'a, O> {
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(NC_A::Bits8)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn bits9(self) -> &'a mut W {
        self.variant(NC_A::Bits9)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(NC_A::Bits10)
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn bits11(self) -> &'a mut W {
        self.variant(NC_A::Bits11)
    }
}
#[doc = "Number of row address bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NR_A {
    #[doc = "0: 11 bits"]
    Bits11 = 0,
    #[doc = "1: 12 bits"]
    Bits12 = 1,
    #[doc = "2: 13 bits"]
    Bits13 = 2,
}
impl From<NR_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NR` reader - Number of row address bits"]
pub type NR_R = crate::FieldReader<u8, NR_A>;
impl NR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NR_A> {
        match self.bits {
            0 => Some(NR_A::Bits11),
            1 => Some(NR_A::Bits12),
            2 => Some(NR_A::Bits13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits11`"]
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        *self == NR_A::Bits11
    }
    #[doc = "Checks if the value of the field is `Bits12`"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == NR_A::Bits12
    }
    #[doc = "Checks if the value of the field is `Bits13`"]
    #[inline(always)]
    pub fn is_bits13(&self) -> bool {
        *self == NR_A::Bits13
    }
}
#[doc = "Field `NR` writer - Number of row address bits"]
pub type NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, NR_A, 2, O>;
impl<'a, const O: u8> NR_W<'a, O> {
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn bits11(self) -> &'a mut W {
        self.variant(NR_A::Bits11)
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(NR_A::Bits12)
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn bits13(self) -> &'a mut W {
        self.variant(NR_A::Bits13)
    }
}
#[doc = "Memory data bus width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWID_A {
    #[doc = "0: Memory data bus width 8 bits"]
    Bits8 = 0,
    #[doc = "1: Memory data bus width 16 bits"]
    Bits16 = 1,
    #[doc = "2: Memory data bus width 32 bits"]
    Bits32 = 2,
}
impl From<MWID_A> for u8 {
    #[inline(always)]
    fn from(variant: MWID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MWID` reader - Memory data bus width"]
pub type MWID_R = crate::FieldReader<u8, MWID_A>;
impl MWID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MWID_A> {
        match self.bits {
            0 => Some(MWID_A::Bits8),
            1 => Some(MWID_A::Bits16),
            2 => Some(MWID_A::Bits32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID_A::Bits32
    }
}
#[doc = "Field `MWID` writer - Memory data bus width"]
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, MWID_A, 2, O>;
impl<'a, const O: u8> MWID_W<'a, O> {
    #[doc = "Memory data bus width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MWID_A::Bits8)
    }
    #[doc = "Memory data bus width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MWID_A::Bits16)
    }
    #[doc = "Memory data bus width 32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MWID_A::Bits32)
    }
}
#[doc = "Number of internal banks\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NB_A {
    #[doc = "0: Two internal Banks"]
    Nb2 = 0,
    #[doc = "1: Four internal Banks"]
    Nb4 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB` reader - Number of internal banks"]
pub type NB_R = crate::BitReader<NB_A>;
impl NB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::Nb2,
            true => NB_A::Nb4,
        }
    }
    #[doc = "Checks if the value of the field is `Nb2`"]
    #[inline(always)]
    pub fn is_nb2(&self) -> bool {
        *self == NB_A::Nb2
    }
    #[doc = "Checks if the value of the field is `Nb4`"]
    #[inline(always)]
    pub fn is_nb4(&self) -> bool {
        *self == NB_A::Nb4
    }
}
#[doc = "Field `NB` writer - Number of internal banks"]
pub type NB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDCR_SPEC, NB_A, O>;
impl<'a, const O: u8> NB_W<'a, O> {
    #[doc = "Two internal Banks"]
    #[inline(always)]
    pub fn nb2(self) -> &'a mut W {
        self.variant(NB_A::Nb2)
    }
    #[doc = "Four internal Banks"]
    #[inline(always)]
    pub fn nb4(self) -> &'a mut W {
        self.variant(NB_A::Nb4)
    }
}
#[doc = "CAS latency\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAS_A {
    #[doc = "1: 1 cycle"]
    Clocks1 = 1,
    #[doc = "2: 2 cycles"]
    Clocks2 = 2,
    #[doc = "3: 3 cycles"]
    Clocks3 = 3,
}
impl From<CAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAS` reader - CAS latency"]
pub type CAS_R = crate::FieldReader<u8, CAS_A>;
impl CAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAS_A> {
        match self.bits {
            1 => Some(CAS_A::Clocks1),
            2 => Some(CAS_A::Clocks2),
            3 => Some(CAS_A::Clocks3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clocks1`"]
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == CAS_A::Clocks1
    }
    #[doc = "Checks if the value of the field is `Clocks2`"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CAS_A::Clocks2
    }
    #[doc = "Checks if the value of the field is `Clocks3`"]
    #[inline(always)]
    pub fn is_clocks3(&self) -> bool {
        *self == CAS_A::Clocks3
    }
}
#[doc = "Field `CAS` writer - CAS latency"]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, CAS_A, 2, O>;
impl<'a, const O: u8> CAS_W<'a, O> {
    #[doc = "1 cycle"]
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut W {
        self.variant(CAS_A::Clocks1)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(CAS_A::Clocks2)
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn clocks3(self) -> &'a mut W {
        self.variant(CAS_A::Clocks3)
    }
}
#[doc = "Write protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_A {
    #[doc = "0: Write accesses allowed"]
    Disabled = 0,
    #[doc = "1: Write accesses ignored"]
    Enabled = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP` reader - Write protection"]
pub type WP_R = crate::BitReader<WP_A>;
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::Disabled,
            true => WP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WP_A::Enabled
    }
}
#[doc = "Field `WP` writer - Write protection"]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDCR_SPEC, WP_A, O>;
impl<'a, const O: u8> WP_W<'a, O> {
    #[doc = "Write accesses allowed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WP_A::Disabled)
    }
    #[doc = "Write accesses ignored"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WP_A::Enabled)
    }
}
#[doc = "SDRAM clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDCLK_A {
    #[doc = "0: SDCLK clock disabled"]
    Disabled = 0,
    #[doc = "2: SDCLK period = 2 x HCLK period"]
    Div2 = 2,
    #[doc = "3: SDCLK period = 3 x HCLK period"]
    Div3 = 3,
}
impl From<SDCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDCLK` reader - SDRAM clock configuration"]
pub type SDCLK_R = crate::FieldReader<u8, SDCLK_A>;
impl SDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCLK_A> {
        match self.bits {
            0 => Some(SDCLK_A::Disabled),
            2 => Some(SDCLK_A::Div2),
            3 => Some(SDCLK_A::Div3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDCLK_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SDCLK_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SDCLK_A::Div3
    }
}
#[doc = "Field `SDCLK` writer - SDRAM clock configuration"]
pub type SDCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, SDCLK_A, 2, O>;
impl<'a, const O: u8> SDCLK_W<'a, O> {
    #[doc = "SDCLK clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDCLK_A::Disabled)
    }
    #[doc = "SDCLK period = 2 x HCLK period"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SDCLK_A::Div2)
    }
    #[doc = "SDCLK period = 3 x HCLK period"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(SDCLK_A::Div3)
    }
}
#[doc = "Burst read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBURST_A {
    #[doc = "0: Single read requests are not managed as bursts"]
    Disabled = 0,
    #[doc = "1: Single read requests are always managed as bursts"]
    Enabled = 1,
}
impl From<RBURST_A> for bool {
    #[inline(always)]
    fn from(variant: RBURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBURST` reader - Burst read"]
pub type RBURST_R = crate::BitReader<RBURST_A>;
impl RBURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBURST_A {
        match self.bits {
            false => RBURST_A::Disabled,
            true => RBURST_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBURST_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBURST_A::Enabled
    }
}
#[doc = "Field `RBURST` writer - Burst read"]
pub type RBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDCR_SPEC, RBURST_A, O>;
impl<'a, const O: u8> RBURST_W<'a, O> {
    #[doc = "Single read requests are not managed as bursts"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RBURST_A::Disabled)
    }
    #[doc = "Single read requests are always managed as bursts"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RBURST_A::Enabled)
    }
}
#[doc = "Read pipe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RPIPE_A {
    #[doc = "0: No clock cycle delay"]
    NoDelay = 0,
    #[doc = "1: One clock cycle delay"]
    Clocks1 = 1,
    #[doc = "2: Two clock cycles delay"]
    Clocks2 = 2,
}
impl From<RPIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: RPIPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RPIPE` reader - Read pipe"]
pub type RPIPE_R = crate::FieldReader<u8, RPIPE_A>;
impl RPIPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RPIPE_A> {
        match self.bits {
            0 => Some(RPIPE_A::NoDelay),
            1 => Some(RPIPE_A::Clocks1),
            2 => Some(RPIPE_A::Clocks2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoDelay`"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == RPIPE_A::NoDelay
    }
    #[doc = "Checks if the value of the field is `Clocks1`"]
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == RPIPE_A::Clocks1
    }
    #[doc = "Checks if the value of the field is `Clocks2`"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == RPIPE_A::Clocks2
    }
}
#[doc = "Field `RPIPE` writer - Read pipe"]
pub type RPIPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, RPIPE_A, 2, O>;
impl<'a, const O: u8> RPIPE_W<'a, O> {
    #[doc = "No clock cycle delay"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut W {
        self.variant(RPIPE_A::NoDelay)
    }
    #[doc = "One clock cycle delay"]
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut W {
        self.variant(RPIPE_A::Clocks1)
    }
    #[doc = "Two clock cycles delay"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(RPIPE_A::Clocks2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration"]
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Read pipe"]
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W<0> {
        NC_W::new(self)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W<2> {
        NR_W::new(self)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<6> {
        NB_W::new(self)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<7> {
        CAS_W::new(self)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<9> {
        WP_W::new(self)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration"]
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W<10> {
        SDCLK_W::new(self)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W<12> {
        RBURST_W::new(self)
    }
    #[doc = "Bits 13:14 - Read pipe"]
    #[inline(always)]
    pub fn rpipe(&mut self) -> RPIPE_W<13> {
        RPIPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcr](index.html) module"]
pub struct SDCR_SPEC;
impl crate::RegisterSpec for SDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcr::R](R) reader structure"]
impl crate::Readable for SDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcr::W](W) writer structure"]
impl crate::Writable for SDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCR%s to value 0x02d0"]
impl crate::Resettable for SDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02d0
    }
}
