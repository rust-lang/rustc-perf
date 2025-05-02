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
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `OFFU` reader - Output FIFO full"]
pub type OFFU_R = crate::BitReader<bool>;
#[doc = "Field `OFNE` reader - Output FIFO not empty"]
pub type OFNE_R = crate::BitReader<bool>;
#[doc = "Field `IFNF` reader - Input FIFO not full"]
pub type IFNF_R = crate::BitReader<bool>;
#[doc = "Field `IFEM` reader - Input FIFO empty"]
pub type IFEM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO full"]
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Output FIFO not empty"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Input FIFO not full"]
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Input FIFO empty"]
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
