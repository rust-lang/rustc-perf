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
    DISABLED = 0,
    #[doc = "1: Address filters pass all incoming frames regardless of their destination or source address"]
    ENABLED = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Promiscuous mode"]
pub struct PM_R(crate::FieldReader<bool, PM_A>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::DISABLED,
            true => PM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PM_A::ENABLED
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Promiscuous mode"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal address filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PM_A::DISABLED)
    }
    #[doc = "Address filters pass all incoming frames regardless of their destination or source address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PM_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Hash unicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HU_A {
    #[doc = "0: MAC performs a perfect destination address filtering for unicast frames"]
    PERFECT = 0,
    #[doc = "1: MAC performs destination address filtering of received unicast frames according to the hash table"]
    HASH = 1,
}
impl From<HU_A> for bool {
    #[inline(always)]
    fn from(variant: HU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HU` reader - Hash unicast"]
pub struct HU_R(crate::FieldReader<bool, HU_A>);
impl HU_R {
    pub(crate) fn new(bits: bool) -> Self {
        HU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HU_A {
        match self.bits {
            false => HU_A::PERFECT,
            true => HU_A::HASH,
        }
    }
    #[doc = "Checks if the value of the field is `PERFECT`"]
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        **self == HU_A::PERFECT
    }
    #[doc = "Checks if the value of the field is `HASH`"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        **self == HU_A::HASH
    }
}
impl core::ops::Deref for HU_R {
    type Target = crate::FieldReader<bool, HU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HU` writer - Hash unicast"]
pub struct HU_W<'a> {
    w: &'a mut W,
}
impl<'a> HU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HU_A::PERFECT)
    }
    #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HU_A::HASH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Hash multicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HM_A {
    #[doc = "0: MAC performs a perfect destination address filtering for multicast frames"]
    PERFECT = 0,
    #[doc = "1: MAC performs destination address filtering of received multicast frames according to the hash table"]
    HASH = 1,
}
impl From<HM_A> for bool {
    #[inline(always)]
    fn from(variant: HM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HM` reader - Hash multicast"]
pub struct HM_R(crate::FieldReader<bool, HM_A>);
impl HM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HM_A {
        match self.bits {
            false => HM_A::PERFECT,
            true => HM_A::HASH,
        }
    }
    #[doc = "Checks if the value of the field is `PERFECT`"]
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        **self == HM_A::PERFECT
    }
    #[doc = "Checks if the value of the field is `HASH`"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        **self == HM_A::HASH
    }
}
impl core::ops::Deref for HM_R {
    type Target = crate::FieldReader<bool, HM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HM` writer - Hash multicast"]
pub struct HM_W<'a> {
    w: &'a mut W,
}
impl<'a> HM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HM_A::PERFECT)
    }
    #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HM_A::HASH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Destination address unique filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAIF_A {
    #[doc = "0: Normal filtering of frames"]
    NORMAL = 0,
    #[doc = "1: Address check block operates in inverse filtering mode for the DA address comparison"]
    INVERT = 1,
}
impl From<DAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAIF` reader - Destination address unique filtering"]
pub struct DAIF_R(crate::FieldReader<bool, DAIF_A>);
impl DAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAIF_A {
        match self.bits {
            false => DAIF_A::NORMAL,
            true => DAIF_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == DAIF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == DAIF_A::INVERT
    }
}
impl core::ops::Deref for DAIF_R {
    type Target = crate::FieldReader<bool, DAIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAIF` writer - Destination address unique filtering"]
pub struct DAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal filtering of frames"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAIF_A::NORMAL)
    }
    #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(DAIF_A::INVERT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Pass all multicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAM_A {
    #[doc = "0: Filtering of multicast frames depends on HM"]
    DISABLED = 0,
    #[doc = "1: All received frames with a multicast destination address are passed"]
    ENABLED = 1,
}
impl From<PAM_A> for bool {
    #[inline(always)]
    fn from(variant: PAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAM` reader - Pass all multicast"]
pub struct PAM_R(crate::FieldReader<bool, PAM_A>);
impl PAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAM_A {
        match self.bits {
            false => PAM_A::DISABLED,
            true => PAM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PAM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PAM_A::ENABLED
    }
}
impl core::ops::Deref for PAM_R {
    type Target = crate::FieldReader<bool, PAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAM` writer - Pass all multicast"]
pub struct PAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Filtering of multicast frames depends on HM"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAM_A::DISABLED)
    }
    #[doc = "All received frames with a multicast destination address are passed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAM_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Broadcast frames disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFD_A {
    #[doc = "0: Address filters pass all received broadcast frames"]
    ENABLED = 0,
    #[doc = "1: Address filters filter all incoming broadcast frames"]
    DISABLED = 1,
}
impl From<BFD_A> for bool {
    #[inline(always)]
    fn from(variant: BFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFD` reader - Broadcast frames disable"]
pub struct BFD_R(crate::FieldReader<bool, BFD_A>);
impl BFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFD_A {
        match self.bits {
            false => BFD_A::ENABLED,
            true => BFD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BFD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BFD_A::DISABLED
    }
}
impl core::ops::Deref for BFD_R {
    type Target = crate::FieldReader<bool, BFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFD` writer - Broadcast frames disable"]
pub struct BFD_W<'a> {
    w: &'a mut W,
}
impl<'a> BFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address filters pass all received broadcast frames"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFD_A::ENABLED)
    }
    #[doc = "Address filters filter all incoming broadcast frames"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFD_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Pass control frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCF_A {
    #[doc = "0: MAC prevents all control frames from reaching the application"]
    PREVENTALL = 0,
    #[doc = "1: MAC forwards all control frames to application except Pause"]
    FORWARDALLEXCEPTPAUSE = 1,
    #[doc = "2: MAC forwards all control frames to application even if they fail the address filter"]
    FORWARDALL = 2,
    #[doc = "3: MAC forwards control frames that pass the address filter"]
    FORWARDALLFILTERED = 3,
}
impl From<PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCF` reader - Pass control frames"]
pub struct PCF_R(crate::FieldReader<u8, PCF_A>);
impl PCF_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCF_A {
        match self.bits {
            0 => PCF_A::PREVENTALL,
            1 => PCF_A::FORWARDALLEXCEPTPAUSE,
            2 => PCF_A::FORWARDALL,
            3 => PCF_A::FORWARDALLFILTERED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PREVENTALL`"]
    #[inline(always)]
    pub fn is_prevent_all(&self) -> bool {
        **self == PCF_A::PREVENTALL
    }
    #[doc = "Checks if the value of the field is `FORWARDALLEXCEPTPAUSE`"]
    #[inline(always)]
    pub fn is_forward_all_except_pause(&self) -> bool {
        **self == PCF_A::FORWARDALLEXCEPTPAUSE
    }
    #[doc = "Checks if the value of the field is `FORWARDALL`"]
    #[inline(always)]
    pub fn is_forward_all(&self) -> bool {
        **self == PCF_A::FORWARDALL
    }
    #[doc = "Checks if the value of the field is `FORWARDALLFILTERED`"]
    #[inline(always)]
    pub fn is_forward_all_filtered(&self) -> bool {
        **self == PCF_A::FORWARDALLFILTERED
    }
}
impl core::ops::Deref for PCF_R {
    type Target = crate::FieldReader<u8, PCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCF` writer - Pass control frames"]
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "MAC prevents all control frames from reaching the application"]
    #[inline(always)]
    pub fn prevent_all(self) -> &'a mut W {
        self.variant(PCF_A::PREVENTALL)
    }
    #[doc = "MAC forwards all control frames to application except Pause"]
    #[inline(always)]
    pub fn forward_all_except_pause(self) -> &'a mut W {
        self.variant(PCF_A::FORWARDALLEXCEPTPAUSE)
    }
    #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
    #[inline(always)]
    pub fn forward_all(self) -> &'a mut W {
        self.variant(PCF_A::FORWARDALL)
    }
    #[doc = "MAC forwards control frames that pass the address filter"]
    #[inline(always)]
    pub fn forward_all_filtered(self) -> &'a mut W {
        self.variant(PCF_A::FORWARDALLFILTERED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Source address inverse filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAIF_A {
    #[doc = "0: Source address filter operates normally"]
    NORMAL = 0,
    #[doc = "1: Source address filter operation inverted"]
    INVERT = 1,
}
impl From<SAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub struct SAIF_R(crate::FieldReader<bool, SAIF_A>);
impl SAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIF_A {
        match self.bits {
            false => SAIF_A::NORMAL,
            true => SAIF_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SAIF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == SAIF_A::INVERT
    }
}
impl core::ops::Deref for SAIF_R {
    type Target = crate::FieldReader<bool, SAIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub struct SAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Source address filter operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SAIF_A::NORMAL)
    }
    #[doc = "Source address filter operation inverted"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(SAIF_A::INVERT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Source address filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAF_A {
    #[doc = "0: Source address ignored"]
    DISABLED = 0,
    #[doc = "1: MAC drops frames that fail the source address filter"]
    ENABLED = 1,
}
impl From<SAF_A> for bool {
    #[inline(always)]
    fn from(variant: SAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAF` reader - Source address filter"]
pub struct SAF_R(crate::FieldReader<bool, SAF_A>);
impl SAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAF_A {
        match self.bits {
            false => SAF_A::DISABLED,
            true => SAF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SAF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SAF_A::ENABLED
    }
}
impl core::ops::Deref for SAF_R {
    type Target = crate::FieldReader<bool, SAF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAF` writer - Source address filter"]
pub struct SAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Source address ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAF_A::DISABLED)
    }
    #[doc = "MAC drops frames that fail the source address filter"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAF_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Hash or perfect filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPF_A {
    #[doc = "0: If HM or HU is set, only frames that match the Hash filter are passed"]
    HASHONLY = 0,
    #[doc = "1: If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    HASHORPERFECT = 1,
}
impl From<HPF_A> for bool {
    #[inline(always)]
    fn from(variant: HPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub struct HPF_R(crate::FieldReader<bool, HPF_A>);
impl HPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPF_A {
        match self.bits {
            false => HPF_A::HASHONLY,
            true => HPF_A::HASHORPERFECT,
        }
    }
    #[doc = "Checks if the value of the field is `HASHONLY`"]
    #[inline(always)]
    pub fn is_hash_only(&self) -> bool {
        **self == HPF_A::HASHONLY
    }
    #[doc = "Checks if the value of the field is `HASHORPERFECT`"]
    #[inline(always)]
    pub fn is_hash_or_perfect(&self) -> bool {
        **self == HPF_A::HASHORPERFECT
    }
}
impl core::ops::Deref for HPF_R {
    type Target = crate::FieldReader<bool, HPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub struct HPF_W<'a> {
    w: &'a mut W,
}
impl<'a> HPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
    #[inline(always)]
    pub fn hash_only(self) -> &'a mut W {
        self.variant(HPF_A::HASHONLY)
    }
    #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    #[inline(always)]
    pub fn hash_or_perfect(self) -> &'a mut W {
        self.variant(HPF_A::HASHORPERFECT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Receive all\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RA_A {
    #[doc = "0: MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    DISABLED = 0,
    #[doc = "1: MAC receiver passes oll received frames on to the application"]
    ENABLED = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RA` reader - Receive all"]
pub struct RA_R(crate::FieldReader<bool, RA_A>);
impl RA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::DISABLED,
            true => RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RA_A::ENABLED
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<bool, RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Receive all"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RA_A::DISABLED)
    }
    #[doc = "MAC receiver passes oll received frames on to the application"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RA_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination address unique filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W {
        HU_W { w: self }
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W {
        HM_W { w: self }
    }
    #[doc = "Bit 3 - Destination address unique filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W { w: self }
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W {
        PAM_W { w: self }
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W {
        BFD_W { w: self }
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    #[doc = "Bit 7 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W {
        SAIF_W { w: self }
    }
    #[doc = "Bit 8 - Source address filter"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W { w: self }
    }
    #[doc = "Bit 9 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W {
        HPF_W { w: self }
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
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
