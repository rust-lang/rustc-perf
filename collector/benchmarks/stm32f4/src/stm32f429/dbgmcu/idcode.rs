#[doc = "Reader of register IDCODE"]
pub type R = crate::R<u32, super::IDCODE>;
#[doc = "Reader of field `DEV_ID`"]
pub type DEV_ID_R = crate::R<u16, u16>;
#[doc = "Reader of field `REV_ID`"]
pub type REV_ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DEV_ID"]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - REV_ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
