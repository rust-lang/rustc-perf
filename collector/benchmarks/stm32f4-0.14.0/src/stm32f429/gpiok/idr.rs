#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Port input data (y = 0..15)"]
pub type IDR15_A = IDR0_A;
#[doc = "Field `IDR15` reader - Port input data (y = 0..15)"]
pub type IDR15_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR14_A = IDR0_A;
#[doc = "Field `IDR14` reader - Port input data (y = 0..15)"]
pub type IDR14_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR13_A = IDR0_A;
#[doc = "Field `IDR13` reader - Port input data (y = 0..15)"]
pub type IDR13_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR12_A = IDR0_A;
#[doc = "Field `IDR12` reader - Port input data (y = 0..15)"]
pub type IDR12_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR11_A = IDR0_A;
#[doc = "Field `IDR11` reader - Port input data (y = 0..15)"]
pub type IDR11_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR10_A = IDR0_A;
#[doc = "Field `IDR10` reader - Port input data (y = 0..15)"]
pub type IDR10_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR9_A = IDR0_A;
#[doc = "Field `IDR9` reader - Port input data (y = 0..15)"]
pub type IDR9_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR8_A = IDR0_A;
#[doc = "Field `IDR8` reader - Port input data (y = 0..15)"]
pub type IDR8_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR7_A = IDR0_A;
#[doc = "Field `IDR7` reader - Port input data (y = 0..15)"]
pub type IDR7_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR6_A = IDR0_A;
#[doc = "Field `IDR6` reader - Port input data (y = 0..15)"]
pub type IDR6_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR5_A = IDR0_A;
#[doc = "Field `IDR5` reader - Port input data (y = 0..15)"]
pub type IDR5_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR4_A = IDR0_A;
#[doc = "Field `IDR4` reader - Port input data (y = 0..15)"]
pub type IDR4_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR3_A = IDR0_A;
#[doc = "Field `IDR3` reader - Port input data (y = 0..15)"]
pub type IDR3_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR2_A = IDR0_A;
#[doc = "Field `IDR2` reader - Port input data (y = 0..15)"]
pub type IDR2_R = IDR0_R;
#[doc = "Port input data (y = 0..15)"]
pub type IDR1_A = IDR0_A;
#[doc = "Field `IDR1` reader - Port input data (y = 0..15)"]
pub type IDR1_R = IDR0_R;
#[doc = "Port input data (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR0_A {
    #[doc = "1: Input is logic high"]
    HIGH = 1,
    #[doc = "0: Input is logic low"]
    LOW = 0,
}
impl From<IDR0_A> for bool {
    #[inline(always)]
    fn from(variant: IDR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDR0` reader - Port input data (y = 0..15)"]
pub struct IDR0_R(crate::FieldReader<bool, IDR0_A>);
impl IDR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDR0_A {
        match self.bits {
            true => IDR0_A::HIGH,
            false => IDR0_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == IDR0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == IDR0_A::LOW
    }
}
impl core::ops::Deref for IDR0_R {
    type Target = crate::FieldReader<bool, IDR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
