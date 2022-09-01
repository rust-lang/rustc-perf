#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Overrun / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRUDR_A {
    #[doc = "0: No overrun/underrun error"]
    NOERROR = 0,
    #[doc = "1: Overrun/underrun error detection"]
    OVERRUN = 1,
}
impl From<OVRUDR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRUDR` reader - Overrun / underrun"]
pub struct OVRUDR_R(crate::FieldReader<bool, OVRUDR_A>);
impl OVRUDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRUDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRUDR_A {
        match self.bits {
            false => OVRUDR_A::NOERROR,
            true => OVRUDR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == OVRUDR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        **self == OVRUDR_A::OVERRUN
    }
}
impl core::ops::Deref for OVRUDR_R {
    type Target = crate::FieldReader<bool, OVRUDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mute detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEDET_A {
    #[doc = "0: No MUTE detection on the SD input line"]
    NOMUTE = 0,
    #[doc = "1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame"]
    MUTE = 1,
}
impl From<MUTEDET_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEDET` reader - Mute detection"]
pub struct MUTEDET_R(crate::FieldReader<bool, MUTEDET_A>);
impl MUTEDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTEDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEDET_A {
        match self.bits {
            false => MUTEDET_A::NOMUTE,
            true => MUTEDET_A::MUTE,
        }
    }
    #[doc = "Checks if the value of the field is `NOMUTE`"]
    #[inline(always)]
    pub fn is_no_mute(&self) -> bool {
        **self == MUTEDET_A::NOMUTE
    }
    #[doc = "Checks if the value of the field is `MUTE`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        **self == MUTEDET_A::MUTE
    }
}
impl core::ops::Deref for MUTEDET_R {
    type Target = crate::FieldReader<bool, MUTEDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wrong clock configuration flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCKCFG_A {
    #[doc = "0: Clock configuration is correct"]
    CORRECT = 0,
    #[doc = "1: Clock configuration does not respect the rule concerning the frame length specification"]
    WRONG = 1,
}
impl From<WCKCFG_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag"]
pub struct WCKCFG_R(crate::FieldReader<bool, WCKCFG_A>);
impl WCKCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCKCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCKCFG_A {
        match self.bits {
            false => WCKCFG_A::CORRECT,
            true => WCKCFG_A::WRONG,
        }
    }
    #[doc = "Checks if the value of the field is `CORRECT`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        **self == WCKCFG_A::CORRECT
    }
    #[doc = "Checks if the value of the field is `WRONG`"]
    #[inline(always)]
    pub fn is_wrong(&self) -> bool {
        **self == WCKCFG_A::WRONG
    }
}
impl core::ops::Deref for WCKCFG_R {
    type Target = crate::FieldReader<bool, WCKCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FIFO request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQ_A {
    #[doc = "0: No FIFO request"]
    NOREQUEST = 0,
    #[doc = "1: FIFO request to read or to write the SAI_xDR"]
    REQUEST = 1,
}
impl From<FREQ_A> for bool {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQ` reader - FIFO request"]
pub struct FREQ_R(crate::FieldReader<bool, FREQ_A>);
impl FREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQ_A {
        match self.bits {
            false => FREQ_A::NOREQUEST,
            true => FREQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NOREQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        **self == FREQ_A::NOREQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        **self == FREQ_A::REQUEST
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<bool, FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Codec not ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNRDY_A {
    #[doc = "0: External AC’97 Codec is ready"]
    READY = 0,
    #[doc = "1: External AC’97 Codec is not ready"]
    NOTREADY = 1,
}
impl From<CNRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNRDY` reader - Codec not ready"]
pub struct CNRDY_R(crate::FieldReader<bool, CNRDY_A>);
impl CNRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNRDY_A {
        match self.bits {
            false => CNRDY_A::READY,
            true => CNRDY_A::NOTREADY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == CNRDY_A::READY
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == CNRDY_A::NOTREADY
    }
}
impl core::ops::Deref for CNRDY_R {
    type Target = crate::FieldReader<bool, CNRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Anticipated frame synchronization detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFSDET_A {
    #[doc = "0: No error"]
    NOERROR = 0,
    #[doc = "1: Frame synchronization signal is detected earlier than expected"]
    EARLYSYNC = 1,
}
impl From<AFSDET_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection"]
pub struct AFSDET_R(crate::FieldReader<bool, AFSDET_A>);
impl AFSDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFSDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFSDET_A {
        match self.bits {
            false => AFSDET_A::NOERROR,
            true => AFSDET_A::EARLYSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == AFSDET_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `EARLYSYNC`"]
    #[inline(always)]
    pub fn is_early_sync(&self) -> bool {
        **self == AFSDET_A::EARLYSYNC
    }
}
impl core::ops::Deref for AFSDET_R {
    type Target = crate::FieldReader<bool, AFSDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Late frame synchronization detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSDET_A {
    #[doc = "0: No error"]
    NOERROR = 0,
    #[doc = "1: Frame synchronization signal is not present at the right time"]
    NOSYNC = 1,
}
impl From<LFSDET_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFSDET` reader - Late frame synchronization detection"]
pub struct LFSDET_R(crate::FieldReader<bool, LFSDET_A>);
impl LFSDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFSDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFSDET_A {
        match self.bits {
            false => LFSDET_A::NOERROR,
            true => LFSDET_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == LFSDET_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        **self == LFSDET_A::NOSYNC
    }
}
impl core::ops::Deref for LFSDET_R {
    type Target = crate::FieldReader<bool, LFSDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FIFO level threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLVL_A {
    #[doc = "0: FIFO empty"]
    EMPTY = 0,
    #[doc = "1: FIFO <= 1⁄4 but not empty"]
    QUARTER1 = 1,
    #[doc = "2: 1⁄4 < FIFO <= 1⁄2"]
    QUARTER2 = 2,
    #[doc = "3: 1⁄2 < FIFO <= 3⁄4"]
    QUARTER3 = 3,
    #[doc = "4: 3⁄4 < FIFO but not full"]
    QUARTER4 = 4,
    #[doc = "5: FIFO full"]
    FULL = 5,
}
impl From<FLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLVL` reader - FIFO level threshold"]
pub struct FLVL_R(crate::FieldReader<u8, FLVL_A>);
impl FLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLVL_A> {
        match self.bits {
            0 => Some(FLVL_A::EMPTY),
            1 => Some(FLVL_A::QUARTER1),
            2 => Some(FLVL_A::QUARTER2),
            3 => Some(FLVL_A::QUARTER3),
            4 => Some(FLVL_A::QUARTER4),
            5 => Some(FLVL_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == FLVL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `QUARTER1`"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        **self == FLVL_A::QUARTER1
    }
    #[doc = "Checks if the value of the field is `QUARTER2`"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        **self == FLVL_A::QUARTER2
    }
    #[doc = "Checks if the value of the field is `QUARTER3`"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        **self == FLVL_A::QUARTER3
    }
    #[doc = "Checks if the value of the field is `QUARTER4`"]
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        **self == FLVL_A::QUARTER4
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == FLVL_A::FULL
    }
}
impl core::ops::Deref for FLVL_R {
    type Target = crate::FieldReader<u8, FLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "SAI AStatus register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x08"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
