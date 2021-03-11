#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSVREFE_A {
    #[doc = "0: Temperature sensor and V_REFINT channel disabled"]
    DISABLED = 0,
    #[doc = "1: Temperature sensor and V_REFINT channel enabled"]
    ENABLED = 1,
}
impl From<TSVREFE_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSVREFE`"]
pub type TSVREFE_R = crate::R<bool, TSVREFE_A>;
impl TSVREFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSVREFE_A {
        match self.bits {
            false => TSVREFE_A::DISABLED,
            true => TSVREFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TSVREFE`"]
pub struct TSVREFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVREFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSVREFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::DISABLED)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATE_A {
    #[doc = "0: V_BAT channel disabled"]
    DISABLED = 0,
    #[doc = "1: V_BAT channel enabled"]
    ENABLED = 1,
}
impl From<VBATE_A> for bool {
    #[inline(always)]
    fn from(variant: VBATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBATE`"]
pub type VBATE_R = crate::R<bool, VBATE_A>;
impl VBATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATE_A {
        match self.bits {
            false => VBATE_A::DISABLED,
            true => VBATE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATE_A::ENABLED
    }
}
#[doc = "Write proxy for field `VBATE`"]
pub struct VBATE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATE_A::DISABLED)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "ADC prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCPRE_A {
    #[doc = "0: PCLK2 divided by 2"]
    DIV2 = 0,
    #[doc = "1: PCLK2 divided by 4"]
    DIV4 = 1,
    #[doc = "2: PCLK2 divided by 6"]
    DIV6 = 2,
    #[doc = "3: PCLK2 divided by 8"]
    DIV8 = 3,
}
impl From<ADCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCPRE`"]
pub type ADCPRE_R = crate::R<u8, ADCPRE_A>;
impl ADCPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPRE_A {
        match self.bits {
            0 => ADCPRE_A::DIV2,
            1 => ADCPRE_A::DIV4,
            2 => ADCPRE_A::DIV6,
            3 => ADCPRE_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE_A::DIV8
    }
}
#[doc = "Write proxy for field `ADCPRE`"]
pub struct ADCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPRE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PCLK2 divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPRE_A::DIV2)
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPRE_A::DIV4)
    }
    #[doc = "PCLK2 divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPRE_A::DIV6)
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPRE_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Direct memory access mode for multi ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_A {
    #[doc = "0: DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    MODE1 = 1,
    #[doc = "2: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    MODE2 = 2,
    #[doc = "3: DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    MODE3 = 3,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<u8, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            0 => DMA_A::DISABLED,
            1 => DMA_A::MODE1,
            2 => DMA_A::MODE2,
            3 => DMA_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == DMA_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == DMA_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == DMA_A::MODE3
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::DISABLED)
    }
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(DMA_A::MODE1)
    }
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(DMA_A::MODE2)
    }
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(DMA_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "DMA disable selection for multi-ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDS_A {
    #[doc = "0: No new DMA request is issued after the last transfer"]
    SINGLE = 0,
    #[doc = "1: DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    CONTINUOUS = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDS`"]
pub type DDS_R = crate::R<bool, DDS_A>;
impl DDS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::SINGLE,
            true => DDS_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `DDS`"]
pub struct DDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDS_A::SINGLE)
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDS_A::CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Multi ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULTI_A {
    #[doc = "0: All the ADCs independent: independent mode"]
    INDEPENDENT = 0,
    #[doc = "1: Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    DUALRJ = 1,
    #[doc = "2: Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    DUALRA = 2,
    #[doc = "5: Dual ADC1 and ADC2, injected simultaneous mode only"]
    DUALJ = 5,
    #[doc = "6: Dual ADC1 and ADC2, regular simultaneous mode only"]
    DUALR = 6,
    #[doc = "7: Dual ADC1 and ADC2, interleaved mode only"]
    DUALI = 7,
    #[doc = "9: Dual ADC1 and ADC2, alternate trigger mode only"]
    DUALA = 9,
    #[doc = "17: Triple ADC, regular and injected simultaneous mode"]
    TRIPLERJ = 17,
    #[doc = "18: Triple ADC, regular and alternate trigger mode"]
    TRIPLERA = 18,
    #[doc = "21: Triple ADC, injected simultaneous mode only"]
    TRIPLEJ = 21,
    #[doc = "22: Triple ADC, regular simultaneous mode only"]
    TRIPLER = 22,
    #[doc = "23: Triple ADC, interleaved mode only"]
    TRIPLEI = 23,
    #[doc = "24: Triple ADC, alternate trigger mode only"]
    TRIPLEA = 24,
}
impl From<MULTI_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MULTI`"]
pub type MULTI_R = crate::R<u8, MULTI_A>;
impl MULTI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULTI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULTI_A::INDEPENDENT),
            1 => Val(MULTI_A::DUALRJ),
            2 => Val(MULTI_A::DUALRA),
            5 => Val(MULTI_A::DUALJ),
            6 => Val(MULTI_A::DUALR),
            7 => Val(MULTI_A::DUALI),
            9 => Val(MULTI_A::DUALA),
            17 => Val(MULTI_A::TRIPLERJ),
            18 => Val(MULTI_A::TRIPLERA),
            21 => Val(MULTI_A::TRIPLEJ),
            22 => Val(MULTI_A::TRIPLER),
            23 => Val(MULTI_A::TRIPLEI),
            24 => Val(MULTI_A::TRIPLEA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == MULTI_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `DUALRJ`"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == MULTI_A::DUALRJ
    }
    #[doc = "Checks if the value of the field is `DUALRA`"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == MULTI_A::DUALRA
    }
    #[doc = "Checks if the value of the field is `DUALJ`"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == MULTI_A::DUALJ
    }
    #[doc = "Checks if the value of the field is `DUALR`"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == MULTI_A::DUALR
    }
    #[doc = "Checks if the value of the field is `DUALI`"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == MULTI_A::DUALI
    }
    #[doc = "Checks if the value of the field is `DUALA`"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == MULTI_A::DUALA
    }
    #[doc = "Checks if the value of the field is `TRIPLERJ`"]
    #[inline(always)]
    pub fn is_triple_rj(&self) -> bool {
        *self == MULTI_A::TRIPLERJ
    }
    #[doc = "Checks if the value of the field is `TRIPLERA`"]
    #[inline(always)]
    pub fn is_triple_ra(&self) -> bool {
        *self == MULTI_A::TRIPLERA
    }
    #[doc = "Checks if the value of the field is `TRIPLEJ`"]
    #[inline(always)]
    pub fn is_triple_j(&self) -> bool {
        *self == MULTI_A::TRIPLEJ
    }
    #[doc = "Checks if the value of the field is `TRIPLER`"]
    #[inline(always)]
    pub fn is_triple_r(&self) -> bool {
        *self == MULTI_A::TRIPLER
    }
    #[doc = "Checks if the value of the field is `TRIPLEI`"]
    #[inline(always)]
    pub fn is_triple_i(&self) -> bool {
        *self == MULTI_A::TRIPLEI
    }
    #[doc = "Checks if the value of the field is `TRIPLEA`"]
    #[inline(always)]
    pub fn is_triple_a(&self) -> bool {
        *self == MULTI_A::TRIPLEA
    }
}
#[doc = "Write proxy for field `MULTI`"]
pub struct MULTI_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULTI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All the ADCs independent: independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(MULTI_A::INDEPENDENT)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(MULTI_A::DUALRJ)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(MULTI_A::DUALRA)
    }
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(MULTI_A::DUALJ)
    }
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(MULTI_A::DUALR)
    }
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(MULTI_A::DUALI)
    }
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(MULTI_A::DUALA)
    }
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn triple_rj(self) -> &'a mut W {
        self.variant(MULTI_A::TRIPLERJ)
    }
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    #[inline(always)]
    pub fn triple_ra(self) -> &'a mut W {
        self.variant(MULTI_A::TRIPLERA)
    }
    #[doc = "Triple ADC, injected simultaneous mode only"]
    #[inline(always)]
    pub fn triple_j(self) -> &'a mut W {
        self.variant(MULTI_A::TRIPLEJ)
    }
    #[doc = "Triple ADC, regular simultaneous mode only"]
    #[inline(always)]
    pub fn triple_r(self) -> &'a mut W {
        self.variant(MULTI_A::TRIPLER)
    }
    #[doc = "Triple ADC, interleaved mode only"]
    #[inline(always)]
    pub fn triple_i(self) -> &'a mut W {
        self.variant(MULTI_A::TRIPLEI)
    }
    #[doc = "Triple ADC, alternate trigger mode only"]
    #[inline(always)]
    pub fn triple_a(self) -> &'a mut W {
        self.variant(MULTI_A::TRIPLEA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 0x01) != 0)
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
    pub fn tsvrefe(&mut self) -> TSVREFE_W {
        TSVREFE_W { w: self }
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&mut self) -> VBATE_W {
        VBATE_W { w: self }
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W {
        ADCPRE_W { w: self }
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bits 0:4 - Multi ADC mode selection"]
    #[inline(always)]
    pub fn multi(&mut self) -> MULTI_W {
        MULTI_W { w: self }
    }
}
