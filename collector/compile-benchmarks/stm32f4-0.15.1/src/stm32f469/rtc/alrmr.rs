#[doc = "Register `ALRM%sR` reader"]
pub struct R(crate::R<ALRMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRM%sR` writer"]
pub struct W(crate::W<ALRMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMR_SPEC>;
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
impl From<crate::W<ALRMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alarm date mask"]
pub use MSK1_A as MSK4_A;
#[doc = "Field `MSK4` reader - Alarm date mask"]
pub use MSK1_R as MSK4_R;
#[doc = "Field `MSK4` writer - Alarm date mask"]
pub use MSK1_W as MSK4_W;
#[doc = "Week day selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDSEL_A {
    #[doc = "0: DU\\[3:0\\]
represents the date units"]
    DateUnits = 0,
    #[doc = "1: DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don’t care."]
    WeekDay = 1,
}
impl From<WDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader<WDSEL_A>;
impl WDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDSEL_A {
        match self.bits {
            false => WDSEL_A::DateUnits,
            true => WDSEL_A::WeekDay,
        }
    }
    #[doc = "Checks if the value of the field is `DateUnits`"]
    #[inline(always)]
    pub fn is_date_units(&self) -> bool {
        *self == WDSEL_A::DateUnits
    }
    #[doc = "Checks if the value of the field is `WeekDay`"]
    #[inline(always)]
    pub fn is_week_day(&self) -> bool {
        *self == WDSEL_A::WeekDay
    }
}
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMR_SPEC, WDSEL_A, O>;
impl<'a, const O: u8> WDSEL_W<'a, O> {
    #[doc = "DU\\[3:0\\]
represents the date units"]
    #[inline(always)]
    pub fn date_units(self) -> &'a mut W {
        self.variant(WDSEL_A::DateUnits)
    }
    #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don’t care."]
    #[inline(always)]
    pub fn week_day(self) -> &'a mut W {
        self.variant(WDSEL_A::WeekDay)
    }
}
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 4, O>;
#[doc = "Alarm hours mask"]
pub use MSK1_A as MSK3_A;
#[doc = "Field `MSK3` reader - Alarm hours mask"]
pub use MSK1_R as MSK3_R;
#[doc = "Field `MSK3` writer - Alarm hours mask"]
pub use MSK1_W as MSK3_W;
#[doc = "AM/PM notation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: AM or 24-hour format"]
    Am = 0,
    #[doc = "1: PM"]
    Pm = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::Am,
            true => PM_A::Pm,
        }
    }
    #[doc = "Checks if the value of the field is `Am`"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == PM_A::Am
    }
    #[doc = "Checks if the value of the field is `Pm`"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == PM_A::Pm
    }
}
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMR_SPEC, PM_A, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn am(self) -> &'a mut W {
        self.variant(PM_A::Am)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut W {
        self.variant(PM_A::Pm)
    }
}
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 4, O>;
#[doc = "Alarm minutes mask"]
pub use MSK1_A as MSK2_A;
#[doc = "Field `MSK2` reader - Alarm minutes mask"]
pub use MSK1_R as MSK2_R;
#[doc = "Field `MSK2` writer - Alarm minutes mask"]
pub use MSK1_W as MSK2_W;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 4, O>;
#[doc = "Alarm seconds mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSK1_A {
    #[doc = "0: Alarm set if the date/day match"]
    Mask = 0,
    #[doc = "1: Date/day don’t care in Alarm comparison"]
    NotMask = 1,
}
impl From<MSK1_A> for bool {
    #[inline(always)]
    fn from(variant: MSK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK1` reader - Alarm seconds mask"]
pub type MSK1_R = crate::BitReader<MSK1_A>;
impl MSK1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSK1_A {
        match self.bits {
            false => MSK1_A::Mask,
            true => MSK1_A::NotMask,
        }
    }
    #[doc = "Checks if the value of the field is `Mask`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MSK1_A::Mask
    }
    #[doc = "Checks if the value of the field is `NotMask`"]
    #[inline(always)]
    pub fn is_not_mask(&self) -> bool {
        *self == MSK1_A::NotMask
    }
}
#[doc = "Field `MSK1` writer - Alarm seconds mask"]
pub type MSK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMR_SPEC, MSK1_A, O>;
impl<'a, const O: u8> MSK1_W<'a, O> {
    #[doc = "Alarm set if the date/day match"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSK1_A::Mask)
    }
    #[doc = "Date/day don’t care in Alarm comparison"]
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MSK1_A::NotMask)
    }
}
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn msk4(&mut self) -> MSK4_W<31> {
        MSK4_W::new(self)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&mut self) -> WDSEL_W<30> {
        WDSEL_W::new(self)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<28> {
        DT_W::new(self)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<24> {
        DU_W::new(self)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn msk3(&mut self) -> MSK3_W<23> {
        MSK3_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<22> {
        PM_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<20> {
        HT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<16> {
        HU_W::new(self)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn msk2(&mut self) -> MSK2_W<15> {
        MSK2_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<12> {
        MNT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<8> {
        MNU_W::new(self)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msk1(&mut self) -> MSK1_W<7> {
        MSK1_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<4> {
        ST_W::new(self)
    }
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<0> {
        SU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmr](index.html) module"]
pub struct ALRMR_SPEC;
impl crate::RegisterSpec for ALRMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrmr::R](R) reader structure"]
impl crate::Readable for ALRMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrmr::W](W) writer structure"]
impl crate::Writable for ALRMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRM%sR to value 0"]
impl crate::Resettable for ALRMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
