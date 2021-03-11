#[doc = "Reader of register DOUTR"]
pub type R = crate::R<u32, super::DOUTR>;
#[doc = "Reader of field `AES_DOUTR`"]
pub type AES_DOUTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data output register"]
    #[inline(always)]
    pub fn aes_doutr(&self) -> AES_DOUTR_R {
        AES_DOUTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
