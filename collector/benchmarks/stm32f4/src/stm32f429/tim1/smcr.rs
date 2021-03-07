#[doc = "Reader of register SMCR"]
pub type R = crate::R<u32, super::SMCR>;
#[doc = "Writer for register SMCR"]
pub type W = crate::W<u32, super::SMCR>;
#[doc = "Register SMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETP_A {
    #[doc = "0: ETR is noninverted, active at high level or rising edge"]
    NOTINVERTED = 0,
    #[doc = "1: ETR is inverted, active at low level or falling edge"]
    INVERTED = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETP`"]
pub type ETP_R = crate::R<bool, ETP_A>;
impl ETP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::NOTINVERTED,
            true => ETP_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == ETP_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == ETP_A::INVERTED
    }
}
#[doc = "Write proxy for field `ETP`"]
pub struct ETP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ETR is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(ETP_A::NOTINVERTED)
    }
    #[doc = "ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(ETP_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "External clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECE_A {
    #[doc = "0: External clock mode 2 disabled"]
    DISABLED = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    ENABLED = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECE`"]
pub type ECE_R = crate::R<bool, ECE_A>;
impl ECE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::DISABLED,
            true => ECE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ECE`"]
pub struct ECE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECE_A::DISABLED)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "External trigger prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETPS_A {
    #[doc = "0: Prescaler OFF"]
    DIV1 = 0,
    #[doc = "1: ETRP frequency divided by 2"]
    DIV2 = 1,
    #[doc = "2: ETRP frequency divided by 4"]
    DIV4 = 2,
    #[doc = "3: ETRP frequency divided by 8"]
    DIV8 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETPS`"]
pub type ETPS_R = crate::R<u8, ETPS_A>;
impl ETPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::DIV1,
            1 => ETPS_A::DIV2,
            2 => ETPS_A::DIV4,
            3 => ETPS_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPS_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPS_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPS_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPS_A::DIV8
    }
}
#[doc = "Write proxy for field `ETPS`"]
pub struct ETPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ETPS_A::DIV1)
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ETPS_A::DIV2)
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ETPS_A::DIV4)
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ETPS_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "External trigger filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETF_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NOFILTER = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FCK_INT_N2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FCK_INT_N4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FCK_INT_N8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FDTS_DIV2_N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FDTS_DIV2_N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FDTS_DIV4_N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FDTS_DIV4_N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FDTS_DIV8_N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FDTS_DIV8_N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FDTS_DIV16_N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FDTS_DIV16_N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FDTS_DIV16_N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FDTS_DIV32_N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FDTS_DIV32_N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    FDTS_DIV32_N8 = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETF`"]
pub type ETF_R = crate::R<u8, ETF_A>;
impl ETF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::NOFILTER,
            1 => ETF_A::FCK_INT_N2,
            2 => ETF_A::FCK_INT_N4,
            3 => ETF_A::FCK_INT_N8,
            4 => ETF_A::FDTS_DIV2_N6,
            5 => ETF_A::FDTS_DIV2_N8,
            6 => ETF_A::FDTS_DIV4_N6,
            7 => ETF_A::FDTS_DIV4_N8,
            8 => ETF_A::FDTS_DIV8_N6,
            9 => ETF_A::FDTS_DIV8_N8,
            10 => ETF_A::FDTS_DIV16_N5,
            11 => ETF_A::FDTS_DIV16_N6,
            12 => ETF_A::FDTS_DIV16_N8,
            13 => ETF_A::FDTS_DIV32_N5,
            14 => ETF_A::FDTS_DIV32_N6,
            15 => ETF_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETF_A::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == ETF_A::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == ETF_A::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == ETF_A::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N6`"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N8`"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N6`"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N8`"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N6`"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N8`"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N5`"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETF_A::FDTS_DIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N6`"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N8`"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N5`"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETF_A::FDTS_DIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N6`"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N8`"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV32_N8
    }
}
#[doc = "Write proxy for field `ETF`"]
pub struct ETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(ETF_A::NOFILTER)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    NOSYNC = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    SYNC = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSM`"]
pub type MSM_R = crate::R<bool, MSM_A>;
impl MSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::NOSYNC,
            true => MSM_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM_A::NOSYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM_A::SYNC
    }
}
#[doc = "Write proxy for field `MSM`"]
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut W {
        self.variant(MSM_A::NOSYNC)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(MSM_A::SYNC)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TS_A {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    ITR0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    ITR1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    ITR2 = 2,
    #[doc = "4: TI1 Edge Detector (TI1F_ED)"]
    TI1F_ED = 4,
    #[doc = "5: Filtered Timer Input 1 (TI1FP1)"]
    TI1FP1 = 5,
    #[doc = "6: Filtered Timer Input 2 (TI2FP2)"]
    TI2FP2 = 6,
    #[doc = "7: External Trigger input (ETRF)"]
    ETRF = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<u8, TS_A>;
impl TS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TS_A::ITR0),
            1 => Val(TS_A::ITR1),
            2 => Val(TS_A::ITR2),
            4 => Val(TS_A::TI1F_ED),
            5 => Val(TS_A::TI1FP1),
            6 => Val(TS_A::TI2FP2),
            7 => Val(TS_A::ETRF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ITR0`"]
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == TS_A::ITR0
    }
    #[doc = "Checks if the value of the field is `ITR1`"]
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == TS_A::ITR1
    }
    #[doc = "Checks if the value of the field is `ITR2`"]
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == TS_A::ITR2
    }
    #[doc = "Checks if the value of the field is `TI1F_ED`"]
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TS_A::TI1F_ED
    }
    #[doc = "Checks if the value of the field is `TI1FP1`"]
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TS_A::TI1FP1
    }
    #[doc = "Checks if the value of the field is `TI2FP2`"]
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TS_A::TI2FP2
    }
    #[doc = "Checks if the value of the field is `ETRF`"]
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == TS_A::ETRF
    }
}
#[doc = "Write proxy for field `TS`"]
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn itr0(self) -> &'a mut W {
        self.variant(TS_A::ITR0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn itr1(self) -> &'a mut W {
        self.variant(TS_A::ITR1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn itr2(self) -> &'a mut W {
        self.variant(TS_A::ITR2)
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(TS_A::TI1F_ED)
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(TS_A::TI1FP1)
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(TS_A::TI2FP2)
    }
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn etrf(self) -> &'a mut W {
        self.variant(TS_A::ETRF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Slave mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMS_A {
    #[doc = "0: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
    DISABLED = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
    ENCODER_MODE_1 = 1,
    #[doc = "2: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
    ENCODER_MODE_2 = 2,
    #[doc = "3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    ENCODER_MODE_3 = 3,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    RESET_MODE = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    GATED_MODE = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    TRIGGER_MODE = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    EXT_CLOCK_MODE = 7,
}
impl From<SMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMS`"]
pub type SMS_R = crate::R<u8, SMS_A>;
impl SMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            0 => SMS_A::DISABLED,
            1 => SMS_A::ENCODER_MODE_1,
            2 => SMS_A::ENCODER_MODE_2,
            3 => SMS_A::ENCODER_MODE_3,
            4 => SMS_A::RESET_MODE,
            5 => SMS_A::GATED_MODE,
            6 => SMS_A::TRIGGER_MODE,
            7 => SMS_A::EXT_CLOCK_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENCODER_MODE_1`"]
    #[inline(always)]
    pub fn is_encoder_mode_1(&self) -> bool {
        *self == SMS_A::ENCODER_MODE_1
    }
    #[doc = "Checks if the value of the field is `ENCODER_MODE_2`"]
    #[inline(always)]
    pub fn is_encoder_mode_2(&self) -> bool {
        *self == SMS_A::ENCODER_MODE_2
    }
    #[doc = "Checks if the value of the field is `ENCODER_MODE_3`"]
    #[inline(always)]
    pub fn is_encoder_mode_3(&self) -> bool {
        *self == SMS_A::ENCODER_MODE_3
    }
    #[doc = "Checks if the value of the field is `RESET_MODE`"]
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        *self == SMS_A::RESET_MODE
    }
    #[doc = "Checks if the value of the field is `GATED_MODE`"]
    #[inline(always)]
    pub fn is_gated_mode(&self) -> bool {
        *self == SMS_A::GATED_MODE
    }
    #[doc = "Checks if the value of the field is `TRIGGER_MODE`"]
    #[inline(always)]
    pub fn is_trigger_mode(&self) -> bool {
        *self == SMS_A::TRIGGER_MODE
    }
    #[doc = "Checks if the value of the field is `EXT_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_ext_clock_mode(&self) -> bool {
        *self == SMS_A::EXT_CLOCK_MODE
    }
}
#[doc = "Write proxy for field `SMS`"]
pub struct SMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMS_A::DISABLED)
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
    #[inline(always)]
    pub fn encoder_mode_1(self) -> &'a mut W {
        self.variant(SMS_A::ENCODER_MODE_1)
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
    #[inline(always)]
    pub fn encoder_mode_2(self) -> &'a mut W {
        self.variant(SMS_A::ENCODER_MODE_2)
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn encoder_mode_3(self) -> &'a mut W {
        self.variant(SMS_A::ENCODER_MODE_3)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut W {
        self.variant(SMS_A::RESET_MODE)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn gated_mode(self) -> &'a mut W {
        self.variant(SMS_A::GATED_MODE)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn trigger_mode(self) -> &'a mut W {
        self.variant(SMS_A::TRIGGER_MODE)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn ext_clock_mode(self) -> &'a mut W {
        self.variant(SMS_A::EXT_CLOCK_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W { w: self }
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W {
        ECE_W { w: self }
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W {
        ETPS_W { w: self }
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W { w: self }
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
}
