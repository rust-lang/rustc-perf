#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port output data (y = 0..15)"]
pub type ODR15_A = ODR0_A;
#[doc = "Field `ODR15` reader - Port output data (y = 0..15)"]
pub type ODR15_R = ODR0_R;
#[doc = "Field `ODR15` writer - Port output data (y = 0..15)"]
pub struct ODR15_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR15_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR14_A = ODR0_A;
#[doc = "Field `ODR14` reader - Port output data (y = 0..15)"]
pub type ODR14_R = ODR0_R;
#[doc = "Field `ODR14` writer - Port output data (y = 0..15)"]
pub struct ODR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR14_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR14_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR13_A = ODR0_A;
#[doc = "Field `ODR13` reader - Port output data (y = 0..15)"]
pub type ODR13_R = ODR0_R;
#[doc = "Field `ODR13` writer - Port output data (y = 0..15)"]
pub struct ODR13_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR13_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR13_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR12_A = ODR0_A;
#[doc = "Field `ODR12` reader - Port output data (y = 0..15)"]
pub type ODR12_R = ODR0_R;
#[doc = "Field `ODR12` writer - Port output data (y = 0..15)"]
pub struct ODR12_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR12_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR12_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR11_A = ODR0_A;
#[doc = "Field `ODR11` reader - Port output data (y = 0..15)"]
pub type ODR11_R = ODR0_R;
#[doc = "Field `ODR11` writer - Port output data (y = 0..15)"]
pub struct ODR11_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR11_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR11_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR10_A = ODR0_A;
#[doc = "Field `ODR10` reader - Port output data (y = 0..15)"]
pub type ODR10_R = ODR0_R;
#[doc = "Field `ODR10` writer - Port output data (y = 0..15)"]
pub struct ODR10_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR10_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR10_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR9_A = ODR0_A;
#[doc = "Field `ODR9` reader - Port output data (y = 0..15)"]
pub type ODR9_R = ODR0_R;
#[doc = "Field `ODR9` writer - Port output data (y = 0..15)"]
pub struct ODR9_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR9_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR9_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR8_A = ODR0_A;
#[doc = "Field `ODR8` reader - Port output data (y = 0..15)"]
pub type ODR8_R = ODR0_R;
#[doc = "Field `ODR8` writer - Port output data (y = 0..15)"]
pub struct ODR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR8_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR8_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR7_A = ODR0_A;
#[doc = "Field `ODR7` reader - Port output data (y = 0..15)"]
pub type ODR7_R = ODR0_R;
#[doc = "Field `ODR7` writer - Port output data (y = 0..15)"]
pub struct ODR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR7_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR7_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR6_A = ODR0_A;
#[doc = "Field `ODR6` reader - Port output data (y = 0..15)"]
pub type ODR6_R = ODR0_R;
#[doc = "Field `ODR6` writer - Port output data (y = 0..15)"]
pub struct ODR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR6_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR6_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR5_A = ODR0_A;
#[doc = "Field `ODR5` reader - Port output data (y = 0..15)"]
pub type ODR5_R = ODR0_R;
#[doc = "Field `ODR5` writer - Port output data (y = 0..15)"]
pub struct ODR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR5_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR5_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR4_A = ODR0_A;
#[doc = "Field `ODR4` reader - Port output data (y = 0..15)"]
pub type ODR4_R = ODR0_R;
#[doc = "Field `ODR4` writer - Port output data (y = 0..15)"]
pub struct ODR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR4_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR4_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR3_A = ODR0_A;
#[doc = "Field `ODR3` reader - Port output data (y = 0..15)"]
pub type ODR3_R = ODR0_R;
#[doc = "Field `ODR3` writer - Port output data (y = 0..15)"]
pub struct ODR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR3_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR3_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR2_A = ODR0_A;
#[doc = "Field `ODR2` reader - Port output data (y = 0..15)"]
pub type ODR2_R = ODR0_R;
#[doc = "Field `ODR2` writer - Port output data (y = 0..15)"]
pub struct ODR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR2_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR2_A::LOW)
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
#[doc = "Port output data (y = 0..15)"]
pub type ODR1_A = ODR0_A;
#[doc = "Field `ODR1` reader - Port output data (y = 0..15)"]
pub type ODR1_R = ODR0_R;
#[doc = "Field `ODR1` writer - Port output data (y = 0..15)"]
pub struct ODR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR1_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR1_A::LOW)
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
#[doc = "Port output data (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR0_A {
    #[doc = "1: Set output to logic high"]
    HIGH = 1,
    #[doc = "0: Set output to logic low"]
    LOW = 0,
}
impl From<ODR0_A> for bool {
    #[inline(always)]
    fn from(variant: ODR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODR0` reader - Port output data (y = 0..15)"]
pub struct ODR0_R(crate::FieldReader<bool, ODR0_A>);
impl ODR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODR0_A {
        match self.bits {
            true => ODR0_A::HIGH,
            false => ODR0_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == ODR0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == ODR0_A::LOW
    }
}
impl core::ops::Deref for ODR0_R {
    type Target = crate::FieldReader<bool, ODR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODR0` writer - Port output data (y = 0..15)"]
pub struct ODR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ODR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR0_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR0_A::LOW)
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
impl R {
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr12(&self) -> ODR12_R {
        ODR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr11(&self) -> ODR11_R {
        ODR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr10(&self) -> ODR10_R {
        ODR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr9(&self) -> ODR9_R {
        ODR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr8(&self) -> ODR8_R {
        ODR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr15(&mut self) -> ODR15_W {
        ODR15_W { w: self }
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr14(&mut self) -> ODR14_W {
        ODR14_W { w: self }
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr13(&mut self) -> ODR13_W {
        ODR13_W { w: self }
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr12(&mut self) -> ODR12_W {
        ODR12_W { w: self }
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr11(&mut self) -> ODR11_W {
        ODR11_W { w: self }
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr10(&mut self) -> ODR10_W {
        ODR10_W { w: self }
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr9(&mut self) -> ODR9_W {
        ODR9_W { w: self }
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr8(&mut self) -> ODR8_W {
        ODR8_W { w: self }
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr7(&mut self) -> ODR7_W {
        ODR7_W { w: self }
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr6(&mut self) -> ODR6_W {
        ODR6_W { w: self }
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr5(&mut self) -> ODR5_W {
        ODR5_W { w: self }
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR4_W {
        ODR4_W { w: self }
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W {
        ODR3_W { w: self }
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR2_W {
        ODR2_W { w: self }
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR1_W {
        ODR1_W { w: self }
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&mut self) -> ODR0_W {
        ODR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
