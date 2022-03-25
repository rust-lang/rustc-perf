#[doc = "Register `ISR0` reader"]
pub struct R(crate::R<ISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PE4` reader - PHY Error 4"]
pub struct PE4_R(crate::FieldReader<bool, bool>);
impl PE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE3` reader - PHY Error 3"]
pub struct PE3_R(crate::FieldReader<bool, bool>);
impl PE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE2` reader - PHY Error 2"]
pub struct PE2_R(crate::FieldReader<bool, bool>);
impl PE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE1` reader - PHY Error 1"]
pub struct PE1_R(crate::FieldReader<bool, bool>);
impl PE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE0` reader - PHY Error 0"]
pub struct PE0_R(crate::FieldReader<bool, bool>);
impl PE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE15` reader - Acknowledge Error 15"]
pub struct AE15_R(crate::FieldReader<bool, bool>);
impl AE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE14` reader - Acknowledge Error 14"]
pub struct AE14_R(crate::FieldReader<bool, bool>);
impl AE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE13` reader - Acknowledge Error 13"]
pub struct AE13_R(crate::FieldReader<bool, bool>);
impl AE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE12` reader - Acknowledge Error 12"]
pub struct AE12_R(crate::FieldReader<bool, bool>);
impl AE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE11` reader - Acknowledge Error 11"]
pub struct AE11_R(crate::FieldReader<bool, bool>);
impl AE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE10` reader - Acknowledge Error 10"]
pub struct AE10_R(crate::FieldReader<bool, bool>);
impl AE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE9` reader - Acknowledge Error 9"]
pub struct AE9_R(crate::FieldReader<bool, bool>);
impl AE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE8` reader - Acknowledge Error 8"]
pub struct AE8_R(crate::FieldReader<bool, bool>);
impl AE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE7` reader - Acknowledge Error 7"]
pub struct AE7_R(crate::FieldReader<bool, bool>);
impl AE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE6` reader - Acknowledge Error 6"]
pub struct AE6_R(crate::FieldReader<bool, bool>);
impl AE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE5` reader - Acknowledge Error 5"]
pub struct AE5_R(crate::FieldReader<bool, bool>);
impl AE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE4` reader - Acknowledge Error 4"]
pub struct AE4_R(crate::FieldReader<bool, bool>);
impl AE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE3` reader - Acknowledge Error 3"]
pub struct AE3_R(crate::FieldReader<bool, bool>);
impl AE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE2` reader - Acknowledge Error 2"]
pub struct AE2_R(crate::FieldReader<bool, bool>);
impl AE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE1` reader - Acknowledge Error 1"]
pub struct AE1_R(crate::FieldReader<bool, bool>);
impl AE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE0` reader - Acknowledge Error 0"]
pub struct AE0_R(crate::FieldReader<bool, bool>);
impl AE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 20 - PHY Error 4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PHY Error 3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PHY Error 2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY Error 1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PHY Error 0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Acknowledge Error 15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Acknowledge Error 14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error 13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Acknowledge Error 12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Acknowledge Error 11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge Error 10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Error 9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Acknowledge Error 8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Acknowledge Error 7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Error 6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Acknowledge Error 5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Acknowledge Error 4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Error 3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Error 2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Error 1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Acknowledge Error 0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DSI Host Interrupt & Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr0](index.html) module"]
pub struct ISR0_SPEC;
impl crate::RegisterSpec for ISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr0::R](R) reader structure"]
impl crate::Readable for ISR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR0 to value 0"]
impl crate::Resettable for ISR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
