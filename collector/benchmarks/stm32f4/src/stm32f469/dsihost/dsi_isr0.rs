#[doc = "Reader of register DSI_ISR0"]
pub type R = crate::R<u32, super::DSI_ISR0>;
#[doc = "Reader of field `PE4`"]
pub type PE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `PE3`"]
pub type PE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PE2`"]
pub type PE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PE1`"]
pub type PE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PE0`"]
pub type PE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE15`"]
pub type AE15_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE14`"]
pub type AE14_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE13`"]
pub type AE13_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE12`"]
pub type AE12_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE11`"]
pub type AE11_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE10`"]
pub type AE10_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE9`"]
pub type AE9_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE8`"]
pub type AE8_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE7`"]
pub type AE7_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE6`"]
pub type AE6_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE5`"]
pub type AE5_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE4`"]
pub type AE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE3`"]
pub type AE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE2`"]
pub type AE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE1`"]
pub type AE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE0`"]
pub type AE0_R = crate::R<bool, bool>;
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
