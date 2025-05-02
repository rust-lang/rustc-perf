#[doc = "Register `MACFFR` reader"]
pub struct R(crate::R<MACFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFFR` writer"]
pub struct W(crate::W<MACFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MACFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Promiscuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: Normal address filtering"]
    Disabled = 0,
    #[doc = "1: Address filters pass all incoming frames regardless of their destination or source address"]
    Enabled = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Promiscuous mode"]
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::Disabled,
            true => PM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PM_A::Enabled
    }
}
#[doc = "Field `PM` writer - Promiscuous mode"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, PM_A, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    #[doc = "Normal address filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PM_A::Disabled)
    }
    #[doc = "Address filters pass all incoming frames regardless of their destination or source address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PM_A::Enabled)
    }
}
#[doc = "Hash unicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HU_A {
    #[doc = "0: MAC performs a perfect destination address filtering for unicast frames"]
    Perfect = 0,
    #[doc = "1: MAC performs destination address filtering of received unicast frames according to the hash table"]
    Hash = 1,
}
impl From<HU_A> for bool {
    #[inline(always)]
    fn from(variant: HU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HU` reader - Hash unicast"]
pub type HU_R = crate::BitReader<HU_A>;
impl HU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HU_A {
        match self.bits {
            false => HU_A::Perfect,
            true => HU_A::Hash,
        }
    }
    #[doc = "Checks if the value of the field is `Perfect`"]
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HU_A::Perfect
    }
    #[doc = "Checks if the value of the field is `Hash`"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HU_A::Hash
    }
}
#[doc = "Field `HU` writer - Hash unicast"]
pub type HU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, HU_A, O>;
impl<'a, const O: u8> HU_W<'a, O> {
    #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HU_A::Perfect)
    }
    #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HU_A::Hash)
    }
}
#[doc = "Hash multicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HM_A {
    #[doc = "0: MAC performs a perfect destination address filtering for multicast frames"]
    Perfect = 0,
    #[doc = "1: MAC performs destination address filtering of received multicast frames according to the hash table"]
    Hash = 1,
}
impl From<HM_A> for bool {
    #[inline(always)]
    fn from(variant: HM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HM` reader - Hash multicast"]
pub type HM_R = crate::BitReader<HM_A>;
impl HM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HM_A {
        match self.bits {
            false => HM_A::Perfect,
            true => HM_A::Hash,
        }
    }
    #[doc = "Checks if the value of the field is `Perfect`"]
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HM_A::Perfect
    }
    #[doc = "Checks if the value of the field is `Hash`"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HM_A::Hash
    }
}
#[doc = "Field `HM` writer - Hash multicast"]
pub type HM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, HM_A, O>;
impl<'a, const O: u8> HM_W<'a, O> {
    #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HM_A::Perfect)
    }
    #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HM_A::Hash)
    }
}
#[doc = "Destination address unique filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAIF_A {
    #[doc = "0: Normal filtering of frames"]
    Normal = 0,
    #[doc = "1: Address check block operates in inverse filtering mode for the DA address comparison"]
    Invert = 1,
}
impl From<DAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAIF` reader - Destination address unique filtering"]
pub type DAIF_R = crate::BitReader<DAIF_A>;
impl DAIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAIF_A {
        match self.bits {
            false => DAIF_A::Normal,
            true => DAIF_A::Invert,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAIF_A::Normal
    }
    #[doc = "Checks if the value of the field is `Invert`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == DAIF_A::Invert
    }
}
#[doc = "Field `DAIF` writer - Destination address unique filtering"]
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, DAIF_A, O>;
impl<'a, const O: u8> DAIF_W<'a, O> {
    #[doc = "Normal filtering of frames"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAIF_A::Normal)
    }
    #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(DAIF_A::Invert)
    }
}
#[doc = "Pass all multicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAM_A {
    #[doc = "0: Filtering of multicast frames depends on HM"]
    Disabled = 0,
    #[doc = "1: All received frames with a multicast destination address are passed"]
    Enabled = 1,
}
impl From<PAM_A> for bool {
    #[inline(always)]
    fn from(variant: PAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAM` reader - Pass all multicast"]
pub type PAM_R = crate::BitReader<PAM_A>;
impl PAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAM_A {
        match self.bits {
            false => PAM_A::Disabled,
            true => PAM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAM_A::Enabled
    }
}
#[doc = "Field `PAM` writer - Pass all multicast"]
pub type PAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, PAM_A, O>;
impl<'a, const O: u8> PAM_W<'a, O> {
    #[doc = "Filtering of multicast frames depends on HM"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAM_A::Disabled)
    }
    #[doc = "All received frames with a multicast destination address are passed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAM_A::Enabled)
    }
}
#[doc = "Broadcast frames disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFD_A {
    #[doc = "0: Address filters pass all received broadcast frames"]
    Enabled = 0,
    #[doc = "1: Address filters filter all incoming broadcast frames"]
    Disabled = 1,
}
impl From<BFD_A> for bool {
    #[inline(always)]
    fn from(variant: BFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFD` reader - Broadcast frames disable"]
pub type BFD_R = crate::BitReader<BFD_A>;
impl BFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFD_A {
        match self.bits {
            false => BFD_A::Enabled,
            true => BFD_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFD_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFD_A::Disabled
    }
}
#[doc = "Field `BFD` writer - Broadcast frames disable"]
pub type BFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, BFD_A, O>;
impl<'a, const O: u8> BFD_W<'a, O> {
    #[doc = "Address filters pass all received broadcast frames"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFD_A::Enabled)
    }
    #[doc = "Address filters filter all incoming broadcast frames"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFD_A::Disabled)
    }
}
#[doc = "Pass control frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCF_A {
    #[doc = "0: MAC prevents all control frames from reaching the application"]
    PreventAll = 0,
    #[doc = "1: MAC forwards all control frames to application except Pause"]
    ForwardAllExceptPause = 1,
    #[doc = "2: MAC forwards all control frames to application even if they fail the address filter"]
    ForwardAll = 2,
    #[doc = "3: MAC forwards control frames that pass the address filter"]
    ForwardAllFiltered = 3,
}
impl From<PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PCF_R = crate::FieldReader<u8, PCF_A>;
impl PCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCF_A {
        match self.bits {
            0 => PCF_A::PreventAll,
            1 => PCF_A::ForwardAllExceptPause,
            2 => PCF_A::ForwardAll,
            3 => PCF_A::ForwardAllFiltered,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PreventAll`"]
    #[inline(always)]
    pub fn is_prevent_all(&self) -> bool {
        *self == PCF_A::PreventAll
    }
    #[doc = "Checks if the value of the field is `ForwardAllExceptPause`"]
    #[inline(always)]
    pub fn is_forward_all_except_pause(&self) -> bool {
        *self == PCF_A::ForwardAllExceptPause
    }
    #[doc = "Checks if the value of the field is `ForwardAll`"]
    #[inline(always)]
    pub fn is_forward_all(&self) -> bool {
        *self == PCF_A::ForwardAll
    }
    #[doc = "Checks if the value of the field is `ForwardAllFiltered`"]
    #[inline(always)]
    pub fn is_forward_all_filtered(&self) -> bool {
        *self == PCF_A::ForwardAllFiltered
    }
}
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PCF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACFFR_SPEC, u8, PCF_A, 2, O>;
impl<'a, const O: u8> PCF_W<'a, O> {
    #[doc = "MAC prevents all control frames from reaching the application"]
    #[inline(always)]
    pub fn prevent_all(self) -> &'a mut W {
        self.variant(PCF_A::PreventAll)
    }
    #[doc = "MAC forwards all control frames to application except Pause"]
    #[inline(always)]
    pub fn forward_all_except_pause(self) -> &'a mut W {
        self.variant(PCF_A::ForwardAllExceptPause)
    }
    #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
    #[inline(always)]
    pub fn forward_all(self) -> &'a mut W {
        self.variant(PCF_A::ForwardAll)
    }
    #[doc = "MAC forwards control frames that pass the address filter"]
    #[inline(always)]
    pub fn forward_all_filtered(self) -> &'a mut W {
        self.variant(PCF_A::ForwardAllFiltered)
    }
}
#[doc = "Source address inverse filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAIF_A {
    #[doc = "0: Source address filter operates normally"]
    Normal = 0,
    #[doc = "1: Source address filter operation inverted"]
    Invert = 1,
}
impl From<SAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SAIF_R = crate::BitReader<SAIF_A>;
impl SAIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIF_A {
        match self.bits {
            false => SAIF_A::Normal,
            true => SAIF_A::Invert,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SAIF_A::Normal
    }
    #[doc = "Checks if the value of the field is `Invert`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SAIF_A::Invert
    }
}
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, SAIF_A, O>;
impl<'a, const O: u8> SAIF_W<'a, O> {
    #[doc = "Source address filter operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SAIF_A::Normal)
    }
    #[doc = "Source address filter operation inverted"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(SAIF_A::Invert)
    }
}
#[doc = "Source address filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAF_A {
    #[doc = "0: Source address ignored"]
    Disabled = 0,
    #[doc = "1: MAC drops frames that fail the source address filter"]
    Enabled = 1,
}
impl From<SAF_A> for bool {
    #[inline(always)]
    fn from(variant: SAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader<SAF_A>;
impl SAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAF_A {
        match self.bits {
            false => SAF_A::Disabled,
            true => SAF_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAF_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAF_A::Enabled
    }
}
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, SAF_A, O>;
impl<'a, const O: u8> SAF_W<'a, O> {
    #[doc = "Source address ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAF_A::Disabled)
    }
    #[doc = "MAC drops frames that fail the source address filter"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAF_A::Enabled)
    }
}
#[doc = "Hash or perfect filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPF_A {
    #[doc = "0: If HM or HU is set, only frames that match the Hash filter are passed"]
    HashOnly = 0,
    #[doc = "1: If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    HashOrPerfect = 1,
}
impl From<HPF_A> for bool {
    #[inline(always)]
    fn from(variant: HPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HPF_R = crate::BitReader<HPF_A>;
impl HPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPF_A {
        match self.bits {
            false => HPF_A::HashOnly,
            true => HPF_A::HashOrPerfect,
        }
    }
    #[doc = "Checks if the value of the field is `HashOnly`"]
    #[inline(always)]
    pub fn is_hash_only(&self) -> bool {
        *self == HPF_A::HashOnly
    }
    #[doc = "Checks if the value of the field is `HashOrPerfect`"]
    #[inline(always)]
    pub fn is_hash_or_perfect(&self) -> bool {
        *self == HPF_A::HashOrPerfect
    }
}
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, HPF_A, O>;
impl<'a, const O: u8> HPF_W<'a, O> {
    #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
    #[inline(always)]
    pub fn hash_only(self) -> &'a mut W {
        self.variant(HPF_A::HashOnly)
    }
    #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    #[inline(always)]
    pub fn hash_or_perfect(self) -> &'a mut W {
        self.variant(HPF_A::HashOrPerfect)
    }
}
#[doc = "Receive all\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RA_A {
    #[doc = "0: MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    Disabled = 0,
    #[doc = "1: MAC receiver passes oll received frames on to the application"]
    Enabled = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RA` reader - Receive all"]
pub type RA_R = crate::BitReader<RA_A>;
impl RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::Disabled,
            true => RA_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RA_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RA_A::Enabled
    }
}
#[doc = "Field `RA` writer - Receive all"]
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, RA_A, O>;
impl<'a, const O: u8> RA_W<'a, O> {
    #[doc = "MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RA_A::Disabled)
    }
    #[doc = "MAC receiver passes oll received frames on to the application"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RA_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address unique filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 7 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<0> {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<1> {
        HU_W::new(self)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W<2> {
        HM_W::new(self)
    }
    #[doc = "Bit 3 - Destination address unique filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<4> {
        PAM_W::new(self)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W<5> {
        BFD_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 7 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<7> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 8 - Source address filter"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<8> {
        SAF_W::new(self)
    }
    #[doc = "Bit 9 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<9> {
        HPF_W::new(self)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macffr](index.html) module"]
pub struct MACFFR_SPEC;
impl crate::RegisterSpec for MACFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macffr::R](R) reader structure"]
impl crate::Readable for MACFFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macffr::W](W) writer structure"]
impl crate::Writable for MACFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFFR to value 0"]
impl crate::Resettable for MACFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
