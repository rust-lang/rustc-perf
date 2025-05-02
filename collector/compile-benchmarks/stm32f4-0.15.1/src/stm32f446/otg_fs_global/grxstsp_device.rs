#[doc = "Register `GRXSTSP_Device` reader"]
pub struct R(crate::R<GRXSTSP_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSP_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSP_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSP_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STSPHST` reader - Status phase start"]
pub type STSPHST_R = crate::BitReader<bool>;
#[doc = "Field `FRMNUM` reader - Frame number"]
pub type FRMNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PKTSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 27 - Status phase start"]
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 21:24 - Frame number"]
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "OTG status read and pop (device mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp_device](index.html) module"]
pub struct GRXSTSP_DEVICE_SPEC;
impl crate::RegisterSpec for GRXSTSP_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxstsp_device::R](R) reader structure"]
impl crate::Readable for GRXSTSP_DEVICE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRXSTSP_Device to value 0"]
impl crate::Resettable for GRXSTSP_DEVICE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
