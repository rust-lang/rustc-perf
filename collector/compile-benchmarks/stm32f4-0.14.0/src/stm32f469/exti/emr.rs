#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Mask on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<MR0_A> for bool {
    #[inline(always)]
    fn from(variant: MR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0` reader - Event Mask on line 0"]
pub struct MR0_R(crate::FieldReader<bool, MR0_A>);
impl MR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0_A {
        match self.bits {
            false => MR0_A::MASKED,
            true => MR0_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == MR0_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == MR0_A::UNMASKED
    }
}
impl core::ops::Deref for MR0_R {
    type Target = crate::FieldReader<bool, MR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR0` writer - Event Mask on line 0"]
pub struct MR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR0_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR0_A::UNMASKED)
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
#[doc = "Event Mask on line 1"]
pub type MR1_A = MR0_A;
#[doc = "Field `MR1` reader - Event Mask on line 1"]
pub type MR1_R = MR0_R;
#[doc = "Field `MR1` writer - Event Mask on line 1"]
pub struct MR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR1_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR1_A::UNMASKED)
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
#[doc = "Event Mask on line 2"]
pub type MR2_A = MR0_A;
#[doc = "Field `MR2` reader - Event Mask on line 2"]
pub type MR2_R = MR0_R;
#[doc = "Field `MR2` writer - Event Mask on line 2"]
pub struct MR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR2_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR2_A::UNMASKED)
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
#[doc = "Event Mask on line 3"]
pub type MR3_A = MR0_A;
#[doc = "Field `MR3` reader - Event Mask on line 3"]
pub type MR3_R = MR0_R;
#[doc = "Field `MR3` writer - Event Mask on line 3"]
pub struct MR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR3_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR3_A::UNMASKED)
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
#[doc = "Event Mask on line 4"]
pub type MR4_A = MR0_A;
#[doc = "Field `MR4` reader - Event Mask on line 4"]
pub type MR4_R = MR0_R;
#[doc = "Field `MR4` writer - Event Mask on line 4"]
pub struct MR4_W<'a> {
    w: &'a mut W,
}
impl<'a> MR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR4_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR4_A::UNMASKED)
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
#[doc = "Event Mask on line 5"]
pub type MR5_A = MR0_A;
#[doc = "Field `MR5` reader - Event Mask on line 5"]
pub type MR5_R = MR0_R;
#[doc = "Field `MR5` writer - Event Mask on line 5"]
pub struct MR5_W<'a> {
    w: &'a mut W,
}
impl<'a> MR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR5_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR5_A::UNMASKED)
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
#[doc = "Event Mask on line 6"]
pub type MR6_A = MR0_A;
#[doc = "Field `MR6` reader - Event Mask on line 6"]
pub type MR6_R = MR0_R;
#[doc = "Field `MR6` writer - Event Mask on line 6"]
pub struct MR6_W<'a> {
    w: &'a mut W,
}
impl<'a> MR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR6_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR6_A::UNMASKED)
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
#[doc = "Event Mask on line 7"]
pub type MR7_A = MR0_A;
#[doc = "Field `MR7` reader - Event Mask on line 7"]
pub type MR7_R = MR0_R;
#[doc = "Field `MR7` writer - Event Mask on line 7"]
pub struct MR7_W<'a> {
    w: &'a mut W,
}
impl<'a> MR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR7_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR7_A::UNMASKED)
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
#[doc = "Event Mask on line 8"]
pub type MR8_A = MR0_A;
#[doc = "Field `MR8` reader - Event Mask on line 8"]
pub type MR8_R = MR0_R;
#[doc = "Field `MR8` writer - Event Mask on line 8"]
pub struct MR8_W<'a> {
    w: &'a mut W,
}
impl<'a> MR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR8_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR8_A::UNMASKED)
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
#[doc = "Event Mask on line 9"]
pub type MR9_A = MR0_A;
#[doc = "Field `MR9` reader - Event Mask on line 9"]
pub type MR9_R = MR0_R;
#[doc = "Field `MR9` writer - Event Mask on line 9"]
pub struct MR9_W<'a> {
    w: &'a mut W,
}
impl<'a> MR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR9_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR9_A::UNMASKED)
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
#[doc = "Event Mask on line 10"]
pub type MR10_A = MR0_A;
#[doc = "Field `MR10` reader - Event Mask on line 10"]
pub type MR10_R = MR0_R;
#[doc = "Field `MR10` writer - Event Mask on line 10"]
pub struct MR10_W<'a> {
    w: &'a mut W,
}
impl<'a> MR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR10_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR10_A::UNMASKED)
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
#[doc = "Event Mask on line 11"]
pub type MR11_A = MR0_A;
#[doc = "Field `MR11` reader - Event Mask on line 11"]
pub type MR11_R = MR0_R;
#[doc = "Field `MR11` writer - Event Mask on line 11"]
pub struct MR11_W<'a> {
    w: &'a mut W,
}
impl<'a> MR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR11_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR11_A::UNMASKED)
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
#[doc = "Event Mask on line 12"]
pub type MR12_A = MR0_A;
#[doc = "Field `MR12` reader - Event Mask on line 12"]
pub type MR12_R = MR0_R;
#[doc = "Field `MR12` writer - Event Mask on line 12"]
pub struct MR12_W<'a> {
    w: &'a mut W,
}
impl<'a> MR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR12_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR12_A::UNMASKED)
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
#[doc = "Event Mask on line 13"]
pub type MR13_A = MR0_A;
#[doc = "Field `MR13` reader - Event Mask on line 13"]
pub type MR13_R = MR0_R;
#[doc = "Field `MR13` writer - Event Mask on line 13"]
pub struct MR13_W<'a> {
    w: &'a mut W,
}
impl<'a> MR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR13_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR13_A::UNMASKED)
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
#[doc = "Event Mask on line 14"]
pub type MR14_A = MR0_A;
#[doc = "Field `MR14` reader - Event Mask on line 14"]
pub type MR14_R = MR0_R;
#[doc = "Field `MR14` writer - Event Mask on line 14"]
pub struct MR14_W<'a> {
    w: &'a mut W,
}
impl<'a> MR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR14_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR14_A::UNMASKED)
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
#[doc = "Event Mask on line 15"]
pub type MR15_A = MR0_A;
#[doc = "Field `MR15` reader - Event Mask on line 15"]
pub type MR15_R = MR0_R;
#[doc = "Field `MR15` writer - Event Mask on line 15"]
pub struct MR15_W<'a> {
    w: &'a mut W,
}
impl<'a> MR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR15_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR15_A::UNMASKED)
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
#[doc = "Event Mask on line 16"]
pub type MR16_A = MR0_A;
#[doc = "Field `MR16` reader - Event Mask on line 16"]
pub type MR16_R = MR0_R;
#[doc = "Field `MR16` writer - Event Mask on line 16"]
pub struct MR16_W<'a> {
    w: &'a mut W,
}
impl<'a> MR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR16_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR16_A::UNMASKED)
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
#[doc = "Event Mask on line 17"]
pub type MR17_A = MR0_A;
#[doc = "Field `MR17` reader - Event Mask on line 17"]
pub type MR17_R = MR0_R;
#[doc = "Field `MR17` writer - Event Mask on line 17"]
pub struct MR17_W<'a> {
    w: &'a mut W,
}
impl<'a> MR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR17_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR17_A::UNMASKED)
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
#[doc = "Event Mask on line 18"]
pub type MR18_A = MR0_A;
#[doc = "Field `MR18` reader - Event Mask on line 18"]
pub type MR18_R = MR0_R;
#[doc = "Field `MR18` writer - Event Mask on line 18"]
pub struct MR18_W<'a> {
    w: &'a mut W,
}
impl<'a> MR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR18_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR18_A::UNMASKED)
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
#[doc = "Event Mask on line 19"]
pub type MR19_A = MR0_A;
#[doc = "Field `MR19` reader - Event Mask on line 19"]
pub type MR19_R = MR0_R;
#[doc = "Field `MR19` writer - Event Mask on line 19"]
pub struct MR19_W<'a> {
    w: &'a mut W,
}
impl<'a> MR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR19_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR19_A::UNMASKED)
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
#[doc = "Event Mask on line 20"]
pub type MR20_A = MR0_A;
#[doc = "Field `MR20` reader - Event Mask on line 20"]
pub type MR20_R = MR0_R;
#[doc = "Field `MR20` writer - Event Mask on line 20"]
pub struct MR20_W<'a> {
    w: &'a mut W,
}
impl<'a> MR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR20_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR20_A::UNMASKED)
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
#[doc = "Event Mask on line 21"]
pub type MR21_A = MR0_A;
#[doc = "Field `MR21` reader - Event Mask on line 21"]
pub type MR21_R = MR0_R;
#[doc = "Field `MR21` writer - Event Mask on line 21"]
pub struct MR21_W<'a> {
    w: &'a mut W,
}
impl<'a> MR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR21_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR21_A::UNMASKED)
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
#[doc = "Event Mask on line 22"]
pub type MR22_A = MR0_A;
#[doc = "Field `MR22` reader - Event Mask on line 22"]
pub type MR22_R = MR0_R;
#[doc = "Field `MR22` writer - Event Mask on line 22"]
pub struct MR22_W<'a> {
    w: &'a mut W,
}
impl<'a> MR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR22_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR22_A::UNMASKED)
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
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    pub fn mr22(&self) -> MR22_R {
        MR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn mr0(&mut self) -> MR0_W {
        MR0_W { w: self }
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn mr1(&mut self) -> MR1_W {
        MR1_W { w: self }
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn mr2(&mut self) -> MR2_W {
        MR2_W { w: self }
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn mr3(&mut self) -> MR3_W {
        MR3_W { w: self }
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn mr4(&mut self) -> MR4_W {
        MR4_W { w: self }
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn mr5(&mut self) -> MR5_W {
        MR5_W { w: self }
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn mr6(&mut self) -> MR6_W {
        MR6_W { w: self }
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn mr7(&mut self) -> MR7_W {
        MR7_W { w: self }
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn mr8(&mut self) -> MR8_W {
        MR8_W { w: self }
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn mr9(&mut self) -> MR9_W {
        MR9_W { w: self }
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn mr10(&mut self) -> MR10_W {
        MR10_W { w: self }
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn mr11(&mut self) -> MR11_W {
        MR11_W { w: self }
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn mr12(&mut self) -> MR12_W {
        MR12_W { w: self }
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn mr13(&mut self) -> MR13_W {
        MR13_W { w: self }
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn mr14(&mut self) -> MR14_W {
        MR14_W { w: self }
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn mr15(&mut self) -> MR15_W {
        MR15_W { w: self }
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn mr16(&mut self) -> MR16_W {
        MR16_W { w: self }
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn mr17(&mut self) -> MR17_W {
        MR17_W { w: self }
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn mr18(&mut self) -> MR18_W {
        MR18_W { w: self }
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    pub fn mr19(&mut self) -> MR19_W {
        MR19_W { w: self }
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    pub fn mr20(&mut self) -> MR20_W {
        MR20_W { w: self }
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    pub fn mr21(&mut self) -> MR21_W {
        MR21_W { w: self }
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    pub fn mr22(&mut self) -> MR22_W {
        MR22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event mask register (EXTI_EMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
