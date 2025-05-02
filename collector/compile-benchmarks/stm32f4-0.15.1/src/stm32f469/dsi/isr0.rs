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
pub type PE4_R = crate::BitReader<bool>;
#[doc = "Field `PE3` reader - PHY Error 3"]
pub type PE3_R = crate::BitReader<bool>;
#[doc = "Field `PE2` reader - PHY Error 2"]
pub type PE2_R = crate::BitReader<bool>;
#[doc = "Field `PE1` reader - PHY Error 1"]
pub type PE1_R = crate::BitReader<bool>;
#[doc = "Field `PE0` reader - PHY Error 0"]
pub type PE0_R = crate::BitReader<bool>;
#[doc = "Field `AE15` reader - Acknowledge Error 15"]
pub type AE15_R = crate::BitReader<bool>;
#[doc = "Field `AE14` reader - Acknowledge Error 14"]
pub type AE14_R = crate::BitReader<bool>;
#[doc = "Field `AE13` reader - Acknowledge Error 13"]
pub type AE13_R = crate::BitReader<bool>;
#[doc = "Field `AE12` reader - Acknowledge Error 12"]
pub type AE12_R = crate::BitReader<bool>;
#[doc = "Field `AE11` reader - Acknowledge Error 11"]
pub type AE11_R = crate::BitReader<bool>;
#[doc = "Field `AE10` reader - Acknowledge Error 10"]
pub type AE10_R = crate::BitReader<bool>;
#[doc = "Field `AE9` reader - Acknowledge Error 9"]
pub type AE9_R = crate::BitReader<bool>;
#[doc = "Field `AE8` reader - Acknowledge Error 8"]
pub type AE8_R = crate::BitReader<bool>;
#[doc = "Field `AE7` reader - Acknowledge Error 7"]
pub type AE7_R = crate::BitReader<bool>;
#[doc = "Field `AE6` reader - Acknowledge Error 6"]
pub type AE6_R = crate::BitReader<bool>;
#[doc = "Field `AE5` reader - Acknowledge Error 5"]
pub type AE5_R = crate::BitReader<bool>;
#[doc = "Field `AE4` reader - Acknowledge Error 4"]
pub type AE4_R = crate::BitReader<bool>;
#[doc = "Field `AE3` reader - Acknowledge Error 3"]
pub type AE3_R = crate::BitReader<bool>;
#[doc = "Field `AE2` reader - Acknowledge Error 2"]
pub type AE2_R = crate::BitReader<bool>;
#[doc = "Field `AE1` reader - Acknowledge Error 1"]
pub type AE1_R = crate::BitReader<bool>;
#[doc = "Field `AE0` reader - Acknowledge Error 0"]
pub type AE0_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 20 - PHY Error 4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - PHY Error 3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY Error 2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Error 1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - PHY Error 0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Acknowledge Error 15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Acknowledge Error 14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error 13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Acknowledge Error 12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge Error 11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge Error 10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Error 9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Acknowledge Error 8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledge Error 7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Error 6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Acknowledge Error 5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge Error 4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Error 3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Error 2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Error 1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Acknowledge Error 0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 1) != 0)
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
