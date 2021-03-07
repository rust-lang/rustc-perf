#[doc = "Reader of register ECCR3"]
pub type R = crate::R<u32, super::ECCR3>;
#[doc = "Reader of field `ECC`"]
pub type ECC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
