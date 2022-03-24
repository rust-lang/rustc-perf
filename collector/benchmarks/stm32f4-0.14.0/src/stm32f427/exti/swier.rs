#[doc = "Register `SWIER` reader"]
pub struct R(crate::R<SWIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER` writer"]
pub struct W(crate::W<SWIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER_SPEC>;
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
impl From<crate::W<SWIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Interrupt on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER0_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWIER0_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER0` reader - Software Interrupt on line 0"]
pub struct SWIER0_R(crate::FieldReader<bool, SWIER0_A>);
impl SWIER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWIER0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER0_A> {
        match self.bits {
            true => Some(SWIER0_A::PEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PEND`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        **self == SWIER0_A::PEND
    }
}
impl core::ops::Deref for SWIER0_R {
    type Target = crate::FieldReader<bool, SWIER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWIER0` writer - Software Interrupt on line 0"]
pub struct SWIER0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER0_A::PEND)
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
#[doc = "Software Interrupt on line 1"]
pub type SWIER1_A = SWIER0_A;
#[doc = "Field `SWIER1` reader - Software Interrupt on line 1"]
pub type SWIER1_R = SWIER0_R;
#[doc = "Field `SWIER1` writer - Software Interrupt on line 1"]
pub struct SWIER1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER1_A::PEND)
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
#[doc = "Software Interrupt on line 2"]
pub type SWIER2_A = SWIER0_A;
#[doc = "Field `SWIER2` reader - Software Interrupt on line 2"]
pub type SWIER2_R = SWIER0_R;
#[doc = "Field `SWIER2` writer - Software Interrupt on line 2"]
pub struct SWIER2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER2_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Software Interrupt on line 3"]
pub type SWIER3_A = SWIER0_A;
#[doc = "Field `SWIER3` reader - Software Interrupt on line 3"]
pub type SWIER3_R = SWIER0_R;
#[doc = "Field `SWIER3` writer - Software Interrupt on line 3"]
pub struct SWIER3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER3_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Software Interrupt on line 4"]
pub type SWIER4_A = SWIER0_A;
#[doc = "Field `SWIER4` reader - Software Interrupt on line 4"]
pub type SWIER4_R = SWIER0_R;
#[doc = "Field `SWIER4` writer - Software Interrupt on line 4"]
pub struct SWIER4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER4_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Software Interrupt on line 5"]
pub type SWIER5_A = SWIER0_A;
#[doc = "Field `SWIER5` reader - Software Interrupt on line 5"]
pub type SWIER5_R = SWIER0_R;
#[doc = "Field `SWIER5` writer - Software Interrupt on line 5"]
pub struct SWIER5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER5_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Software Interrupt on line 6"]
pub type SWIER6_A = SWIER0_A;
#[doc = "Field `SWIER6` reader - Software Interrupt on line 6"]
pub type SWIER6_R = SWIER0_R;
#[doc = "Field `SWIER6` writer - Software Interrupt on line 6"]
pub struct SWIER6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER6_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Software Interrupt on line 7"]
pub type SWIER7_A = SWIER0_A;
#[doc = "Field `SWIER7` reader - Software Interrupt on line 7"]
pub type SWIER7_R = SWIER0_R;
#[doc = "Field `SWIER7` writer - Software Interrupt on line 7"]
pub struct SWIER7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER7_A::PEND)
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
#[doc = "Software Interrupt on line 8"]
pub type SWIER8_A = SWIER0_A;
#[doc = "Field `SWIER8` reader - Software Interrupt on line 8"]
pub type SWIER8_R = SWIER0_R;
#[doc = "Field `SWIER8` writer - Software Interrupt on line 8"]
pub struct SWIER8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER8_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Software Interrupt on line 9"]
pub type SWIER9_A = SWIER0_A;
#[doc = "Field `SWIER9` reader - Software Interrupt on line 9"]
pub type SWIER9_R = SWIER0_R;
#[doc = "Field `SWIER9` writer - Software Interrupt on line 9"]
pub struct SWIER9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER9_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Software Interrupt on line 10"]
pub type SWIER10_A = SWIER0_A;
#[doc = "Field `SWIER10` reader - Software Interrupt on line 10"]
pub type SWIER10_R = SWIER0_R;
#[doc = "Field `SWIER10` writer - Software Interrupt on line 10"]
pub struct SWIER10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER10_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Software Interrupt on line 11"]
pub type SWIER11_A = SWIER0_A;
#[doc = "Field `SWIER11` reader - Software Interrupt on line 11"]
pub type SWIER11_R = SWIER0_R;
#[doc = "Field `SWIER11` writer - Software Interrupt on line 11"]
pub struct SWIER11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER11_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Software Interrupt on line 12"]
pub type SWIER12_A = SWIER0_A;
#[doc = "Field `SWIER12` reader - Software Interrupt on line 12"]
pub type SWIER12_R = SWIER0_R;
#[doc = "Field `SWIER12` writer - Software Interrupt on line 12"]
pub struct SWIER12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER12_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Software Interrupt on line 13"]
pub type SWIER13_A = SWIER0_A;
#[doc = "Field `SWIER13` reader - Software Interrupt on line 13"]
pub type SWIER13_R = SWIER0_R;
#[doc = "Field `SWIER13` writer - Software Interrupt on line 13"]
pub struct SWIER13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER13_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Software Interrupt on line 14"]
pub type SWIER14_A = SWIER0_A;
#[doc = "Field `SWIER14` reader - Software Interrupt on line 14"]
pub type SWIER14_R = SWIER0_R;
#[doc = "Field `SWIER14` writer - Software Interrupt on line 14"]
pub struct SWIER14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER14_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Software Interrupt on line 15"]
pub type SWIER15_A = SWIER0_A;
#[doc = "Field `SWIER15` reader - Software Interrupt on line 15"]
pub type SWIER15_R = SWIER0_R;
#[doc = "Field `SWIER15` writer - Software Interrupt on line 15"]
pub struct SWIER15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER15_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Software Interrupt on line 16"]
pub type SWIER16_A = SWIER0_A;
#[doc = "Field `SWIER16` reader - Software Interrupt on line 16"]
pub type SWIER16_R = SWIER0_R;
#[doc = "Field `SWIER16` writer - Software Interrupt on line 16"]
pub struct SWIER16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER16_A::PEND)
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
#[doc = "Software Interrupt on line 17"]
pub type SWIER17_A = SWIER0_A;
#[doc = "Field `SWIER17` reader - Software Interrupt on line 17"]
pub type SWIER17_R = SWIER0_R;
#[doc = "Field `SWIER17` writer - Software Interrupt on line 17"]
pub struct SWIER17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER17_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Software Interrupt on line 18"]
pub type SWIER18_A = SWIER0_A;
#[doc = "Field `SWIER18` reader - Software Interrupt on line 18"]
pub type SWIER18_R = SWIER0_R;
#[doc = "Field `SWIER18` writer - Software Interrupt on line 18"]
pub struct SWIER18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER18_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Software Interrupt on line 19"]
pub type SWIER19_A = SWIER0_A;
#[doc = "Field `SWIER19` reader - Software Interrupt on line 19"]
pub type SWIER19_R = SWIER0_R;
#[doc = "Field `SWIER19` writer - Software Interrupt on line 19"]
pub struct SWIER19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER19_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Software Interrupt on line 20"]
pub type SWIER20_A = SWIER0_A;
#[doc = "Field `SWIER20` reader - Software Interrupt on line 20"]
pub type SWIER20_R = SWIER0_R;
#[doc = "Field `SWIER20` writer - Software Interrupt on line 20"]
pub struct SWIER20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER20_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Software Interrupt on line 21"]
pub type SWIER21_A = SWIER0_A;
#[doc = "Field `SWIER21` reader - Software Interrupt on line 21"]
pub type SWIER21_R = SWIER0_R;
#[doc = "Field `SWIER21` writer - Software Interrupt on line 21"]
pub struct SWIER21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER21_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Software Interrupt on line 22"]
pub type SWIER22_A = SWIER0_A;
#[doc = "Field `SWIER22` reader - Software Interrupt on line 22"]
pub type SWIER22_R = SWIER0_R;
#[doc = "Field `SWIER22` writer - Software Interrupt on line 22"]
pub struct SWIER22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER22_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swier19(&self) -> SWIER19_R {
        SWIER19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swier20(&self) -> SWIER20_R {
        SWIER20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swier21(&self) -> SWIER21_R {
        SWIER21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swier22(&self) -> SWIER22_R {
        SWIER22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER0_W {
        SWIER0_W { w: self }
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER1_W {
        SWIER1_W { w: self }
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER2_W {
        SWIER2_W { w: self }
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER3_W {
        SWIER3_W { w: self }
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER4_W {
        SWIER4_W { w: self }
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER5_W {
        SWIER5_W { w: self }
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER6_W {
        SWIER6_W { w: self }
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER7_W {
        SWIER7_W { w: self }
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER8_W {
        SWIER8_W { w: self }
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER9_W {
        SWIER9_W { w: self }
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER10_W {
        SWIER10_W { w: self }
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER11_W {
        SWIER11_W { w: self }
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER12_W {
        SWIER12_W { w: self }
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER13_W {
        SWIER13_W { w: self }
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER14_W {
        SWIER14_W { w: self }
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER15_W {
        SWIER15_W { w: self }
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER16_W {
        SWIER16_W { w: self }
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&mut self) -> SWIER17_W {
        SWIER17_W { w: self }
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swier18(&mut self) -> SWIER18_W {
        SWIER18_W { w: self }
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swier19(&mut self) -> SWIER19_W {
        SWIER19_W { w: self }
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swier20(&mut self) -> SWIER20_W {
        SWIER20_W { w: self }
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swier21(&mut self) -> SWIER21_W {
        SWIER21_W { w: self }
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swier22(&mut self) -> SWIER22_W {
        SWIER22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register (EXTI_SWIER)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier](index.html) module"]
pub struct SWIER_SPEC;
impl crate::RegisterSpec for SWIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier::R](R) reader structure"]
impl crate::Readable for SWIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier::W](W) writer structure"]
impl crate::Writable for SWIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
