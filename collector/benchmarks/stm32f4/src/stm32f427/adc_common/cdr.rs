#[doc = "Reader of register CDR"]
pub type R = crate::R<u32, super::CDR>;
#[doc = "Reader of field `DATA2`"]
pub type DATA2_R = crate::R<u16, u16>;
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - 2nd data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 1st data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xffff) as u16)
    }
}
