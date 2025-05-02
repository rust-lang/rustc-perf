#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSVREFE_A {
    #[doc = "0: Temperature sensor and V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor and V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<TSVREFE_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TSVREFE_R = crate::BitReader<TSVREFE_A>;
impl TSVREFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSVREFE_A {
        match self.bits {
            false => TSVREFE_A::Disabled,
            true => TSVREFE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE_A::Enabled
    }
}
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TSVREFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, TSVREFE_A, O>;
impl<'a, const O: u8> TSVREFE_W<'a, O> {
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Disabled)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Enabled)
    }
}
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATE_A {
    #[doc = "0: V_BAT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_BAT channel enabled"]
    Enabled = 1,
}
impl From<VBATE_A> for bool {
    #[inline(always)]
    fn from(variant: VBATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATE` reader - VBAT enable"]
pub type VBATE_R = crate::BitReader<VBATE_A>;
impl VBATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATE_A {
        match self.bits {
            false => VBATE_A::Disabled,
            true => VBATE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATE_A::Enabled
    }
}
#[doc = "Field `VBATE` writer - VBAT enable"]
pub type VBATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VBATE_A, O>;
impl<'a, const O: u8> VBATE_W<'a, O> {
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATE_A::Disabled)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATE_A::Enabled)
    }
}
#[doc = "ADC prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCPRE_A {
    #[doc = "0: PCLK2 divided by 2"]
    Div2 = 0,
    #[doc = "1: PCLK2 divided by 4"]
    Div4 = 1,
    #[doc = "2: PCLK2 divided by 6"]
    Div6 = 2,
    #[doc = "3: PCLK2 divided by 8"]
    Div8 = 3,
}
impl From<ADCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCPRE` reader - ADC prescaler"]
pub type ADCPRE_R = crate::FieldReader<u8, ADCPRE_A>;
impl ADCPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPRE_A {
        match self.bits {
            0 => ADCPRE_A::Div2,
            1 => ADCPRE_A::Div4,
            2 => ADCPRE_A::Div6,
            3 => ADCPRE_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE_A::Div8
    }
}
#[doc = "Field `ADCPRE` writer - ADC prescaler"]
pub type ADCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, ADCPRE_A, 2, O>;
impl<'a, const O: u8> ADCPRE_W<'a, O> {
    #[doc = "PCLK2 divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div2)
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div4)
    }
    #[doc = "PCLK2 divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div6)
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div8)
    }
}
#[doc = "Direct memory access mode for multi ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_A {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    Mode1 = 1,
    #[doc = "2: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    Mode2 = 2,
    #[doc = "3: DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    Mode3 = 3,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA` reader - Direct memory access mode for multi ADC mode"]
pub type DMA_R = crate::FieldReader<u8, DMA_A>;
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            0 => DMA_A::Disabled,
            1 => DMA_A::Mode1,
            2 => DMA_A::Mode2,
            3 => DMA_A::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Mode1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == DMA_A::Mode1
    }
    #[doc = "Checks if the value of the field is `Mode2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == DMA_A::Mode2
    }
    #[doc = "Checks if the value of the field is `Mode3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == DMA_A::Mode3
    }
}
#[doc = "Field `DMA` writer - Direct memory access mode for multi ADC mode"]
pub type DMA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, DMA_A, 2, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::Disabled)
    }
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(DMA_A::Mode1)
    }
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(DMA_A::Mode2)
    }
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(DMA_A::Mode3)
    }
}
#[doc = "DMA disable selection for multi-ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDS_A {
    #[doc = "0: No new DMA request is issued after the last transfer"]
    Single = 0,
    #[doc = "1: DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    Continuous = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDS` reader - DMA disable selection for multi-ADC mode"]
pub type DDS_R = crate::BitReader<DDS_A>;
impl DDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::Single,
            true => DDS_A::Continuous,
        }
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS_A::Single
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS_A::Continuous
    }
}
#[doc = "Field `DDS` writer - DMA disable selection for multi-ADC mode"]
pub type DDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DDS_A, O>;
impl<'a, const O: u8> DDS_W<'a, O> {
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDS_A::Single)
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDS_A::Continuous)
    }
}
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 4, O>;
#[doc = "Multi ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULTI_A {
    #[doc = "0: All the ADCs independent: independent mode"]
    Independent = 0,
    #[doc = "1: Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    DualRj = 1,
    #[doc = "2: Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    DualRa = 2,
    #[doc = "5: Dual ADC1 and ADC2, injected simultaneous mode only"]
    DualJ = 5,
    #[doc = "6: Dual ADC1 and ADC2, regular simultaneous mode only"]
    DualR = 6,
    #[doc = "7: Dual ADC1 and ADC2, interleaved mode only"]
    DualI = 7,
    #[doc = "9: Dual ADC1 and ADC2, alternate trigger mode only"]
    DualA = 9,
    #[doc = "17: Triple ADC, regular and injected simultaneous mode"]
    TripleRj = 17,
    #[doc = "18: Triple ADC, regular and alternate trigger mode"]
    TripleRa = 18,
    #[doc = "21: Triple ADC, injected simultaneous mode only"]
    TripleJ = 21,
    #[doc = "22: Triple ADC, regular simultaneous mode only"]
    TripleR = 22,
    #[doc = "23: Triple ADC, interleaved mode only"]
    TripleI = 23,
    #[doc = "24: Triple ADC, alternate trigger mode only"]
    TripleA = 24,
}
impl From<MULTI_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MULTI` reader - Multi ADC mode selection"]
pub type MULTI_R = crate::FieldReader<u8, MULTI_A>;
impl MULTI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULTI_A> {
        match self.bits {
            0 => Some(MULTI_A::Independent),
            1 => Some(MULTI_A::DualRj),
            2 => Some(MULTI_A::DualRa),
            5 => Some(MULTI_A::DualJ),
            6 => Some(MULTI_A::DualR),
            7 => Some(MULTI_A::DualI),
            9 => Some(MULTI_A::DualA),
            17 => Some(MULTI_A::TripleRj),
            18 => Some(MULTI_A::TripleRa),
            21 => Some(MULTI_A::TripleJ),
            22 => Some(MULTI_A::TripleR),
            23 => Some(MULTI_A::TripleI),
            24 => Some(MULTI_A::TripleA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Independent`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == MULTI_A::Independent
    }
    #[doc = "Checks if the value of the field is `DualRj`"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == MULTI_A::DualRj
    }
    #[doc = "Checks if the value of the field is `DualRa`"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == MULTI_A::DualRa
    }
    #[doc = "Checks if the value of the field is `DualJ`"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == MULTI_A::DualJ
    }
    #[doc = "Checks if the value of the field is `DualR`"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == MULTI_A::DualR
    }
    #[doc = "Checks if the value of the field is `DualI`"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == MULTI_A::DualI
    }
    #[doc = "Checks if the value of the field is `DualA`"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == MULTI_A::DualA
    }
    #[doc = "Checks if the value of the field is `TripleRj`"]
    #[inline(always)]
    pub fn is_triple_rj(&self) -> bool {
        *self == MULTI_A::TripleRj
    }
    #[doc = "Checks if the value of the field is `TripleRa`"]
    #[inline(always)]
    pub fn is_triple_ra(&self) -> bool {
        *self == MULTI_A::TripleRa
    }
    #[doc = "Checks if the value of the field is `TripleJ`"]
    #[inline(always)]
    pub fn is_triple_j(&self) -> bool {
        *self == MULTI_A::TripleJ
    }
    #[doc = "Checks if the value of the field is `TripleR`"]
    #[inline(always)]
    pub fn is_triple_r(&self) -> bool {
        *self == MULTI_A::TripleR
    }
    #[doc = "Checks if the value of the field is `TripleI`"]
    #[inline(always)]
    pub fn is_triple_i(&self) -> bool {
        *self == MULTI_A::TripleI
    }
    #[doc = "Checks if the value of the field is `TripleA`"]
    #[inline(always)]
    pub fn is_triple_a(&self) -> bool {
        *self == MULTI_A::TripleA
    }
}
#[doc = "Field `MULTI` writer - Multi ADC mode selection"]
pub type MULTI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, MULTI_A, 5, O>;
impl<'a, const O: u8> MULTI_W<'a, O> {
    #[doc = "All the ADCs independent: independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(MULTI_A::Independent)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(MULTI_A::DualRj)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(MULTI_A::DualRa)
    }
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(MULTI_A::DualJ)
    }
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(MULTI_A::DualR)
    }
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(MULTI_A::DualI)
    }
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(MULTI_A::DualA)
    }
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn triple_rj(self) -> &'a mut W {
        self.variant(MULTI_A::TripleRj)
    }
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    #[inline(always)]
    pub fn triple_ra(self) -> &'a mut W {
        self.variant(MULTI_A::TripleRa)
    }
    #[doc = "Triple ADC, injected simultaneous mode only"]
    #[inline(always)]
    pub fn triple_j(self) -> &'a mut W {
        self.variant(MULTI_A::TripleJ)
    }
    #[doc = "Triple ADC, regular simultaneous mode only"]
    #[inline(always)]
    pub fn triple_r(self) -> &'a mut W {
        self.variant(MULTI_A::TripleR)
    }
    #[doc = "Triple ADC, interleaved mode only"]
    #[inline(always)]
    pub fn triple_i(self) -> &'a mut W {
        self.variant(MULTI_A::TripleI)
    }
    #[doc = "Triple ADC, alternate trigger mode only"]
    #[inline(always)]
    pub fn triple_a(self) -> &'a mut W {
        self.variant(MULTI_A::TripleA)
    }
}
impl R {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - Multi ADC mode selection"]
    #[inline(always)]
    pub fn multi(&self) -> MULTI_R {
        MULTI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<23> {
        TSVREFE_W::new(self)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&mut self) -> VBATE_W<22> {
        VBATE_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<16> {
        ADCPRE_W::new(self)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<14> {
        DMA_W::new(self)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<13> {
        DDS_W::new(self)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    #[doc = "Bits 0:4 - Multi ADC mode selection"]
    #[inline(always)]
    pub fn multi(&mut self) -> MULTI_W<0> {
        MULTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
