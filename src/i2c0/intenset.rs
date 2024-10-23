#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Master Pending interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpendingen {
    #[doc = "0: Disabled. The MstPending interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The MstPending interrupt is enabled."]
    Enabled = 1,
}
impl From<Mstpendingen> for bool {
    #[inline(always)]
    fn from(variant: Mstpendingen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPENDINGEN` reader - Master Pending interrupt Enable."]
pub type MstpendingenR = crate::BitReader<Mstpendingen>;
impl MstpendingenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpendingen {
        match self.bits {
            false => Mstpendingen::Disabled,
            true => Mstpendingen::Enabled,
        }
    }
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mstpendingen::Disabled
    }
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mstpendingen::Enabled
    }
}
#[doc = "Field `MSTPENDINGEN` writer - Master Pending interrupt Enable."]
pub type MstpendingenW<'a, REG> = crate::BitWriter<'a, REG, Mstpendingen>;
impl<'a, REG> MstpendingenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpendingen::Disabled)
    }
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpendingen::Enabled)
    }
}
#[doc = "Master Arbitration Loss interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstarblossen {
    #[doc = "0: Disabled. The MstArbLoss interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The MstArbLoss interrupt is enabled."]
    Enabled = 1,
}
impl From<Mstarblossen> for bool {
    #[inline(always)]
    fn from(variant: Mstarblossen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTARBLOSSEN` reader - Master Arbitration Loss interrupt Enable."]
pub type MstarblossenR = crate::BitReader<Mstarblossen>;
impl MstarblossenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstarblossen {
        match self.bits {
            false => Mstarblossen::Disabled,
            true => Mstarblossen::Enabled,
        }
    }
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mstarblossen::Disabled
    }
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mstarblossen::Enabled
    }
}
#[doc = "Field `MSTARBLOSSEN` writer - Master Arbitration Loss interrupt Enable."]
pub type MstarblossenW<'a, REG> = crate::BitWriter<'a, REG, Mstarblossen>;
impl<'a, REG> MstarblossenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstarblossen::Disabled)
    }
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstarblossen::Enabled)
    }
}
#[doc = "Master Start/Stop Error interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstststperren {
    #[doc = "0: Disabled. The MstStStpErr interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The MstStStpErr interrupt is enabled."]
    Enabled = 1,
}
impl From<Mstststperren> for bool {
    #[inline(always)]
    fn from(variant: Mstststperren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSTSTPERREN` reader - Master Start/Stop Error interrupt Enable."]
pub type MstststperrenR = crate::BitReader<Mstststperren>;
impl MstststperrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstststperren {
        match self.bits {
            false => Mstststperren::Disabled,
            true => Mstststperren::Enabled,
        }
    }
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mstststperren::Disabled
    }
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mstststperren::Enabled
    }
}
#[doc = "Field `MSTSTSTPERREN` writer - Master Start/Stop Error interrupt Enable."]
pub type MstststperrenW<'a, REG> = crate::BitWriter<'a, REG, Mstststperren>;
impl<'a, REG> MstststperrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstststperren::Disabled)
    }
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstststperren::Enabled)
    }
}
#[doc = "Slave Pending interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvpendingen {
    #[doc = "0: Disabled. The SlvPending interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The SlvPending interrupt is enabled."]
    Enabled = 1,
}
impl From<Slvpendingen> for bool {
    #[inline(always)]
    fn from(variant: Slvpendingen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVPENDINGEN` reader - Slave Pending interrupt Enable."]
pub type SlvpendingenR = crate::BitReader<Slvpendingen>;
impl SlvpendingenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvpendingen {
        match self.bits {
            false => Slvpendingen::Disabled,
            true => Slvpendingen::Enabled,
        }
    }
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slvpendingen::Disabled
    }
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slvpendingen::Enabled
    }
}
#[doc = "Field `SLVPENDINGEN` writer - Slave Pending interrupt Enable."]
pub type SlvpendingenW<'a, REG> = crate::BitWriter<'a, REG, Slvpendingen>;
impl<'a, REG> SlvpendingenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvpendingen::Disabled)
    }
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvpendingen::Enabled)
    }
}
#[doc = "Slave Not Stretching interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvnotstren {
    #[doc = "0: Disabled. The SlvNotStr interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The SlvNotStr interrupt is enabled."]
    Enabled = 1,
}
impl From<Slvnotstren> for bool {
    #[inline(always)]
    fn from(variant: Slvnotstren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVNOTSTREN` reader - Slave Not Stretching interrupt Enable."]
pub type SlvnotstrenR = crate::BitReader<Slvnotstren>;
impl SlvnotstrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvnotstren {
        match self.bits {
            false => Slvnotstren::Disabled,
            true => Slvnotstren::Enabled,
        }
    }
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slvnotstren::Disabled
    }
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slvnotstren::Enabled
    }
}
#[doc = "Field `SLVNOTSTREN` writer - Slave Not Stretching interrupt Enable."]
pub type SlvnotstrenW<'a, REG> = crate::BitWriter<'a, REG, Slvnotstren>;
impl<'a, REG> SlvnotstrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnotstren::Disabled)
    }
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnotstren::Enabled)
    }
}
#[doc = "Slave Deselect interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvdeselen {
    #[doc = "0: Disabled. The SlvDeSel interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The SlvDeSel interrupt is enabled."]
    Enabled = 1,
}
impl From<Slvdeselen> for bool {
    #[inline(always)]
    fn from(variant: Slvdeselen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVDESELEN` reader - Slave Deselect interrupt Enable."]
pub type SlvdeselenR = crate::BitReader<Slvdeselen>;
impl SlvdeselenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvdeselen {
        match self.bits {
            false => Slvdeselen::Disabled,
            true => Slvdeselen::Enabled,
        }
    }
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slvdeselen::Disabled
    }
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slvdeselen::Enabled
    }
}
#[doc = "Field `SLVDESELEN` writer - Slave Deselect interrupt Enable."]
pub type SlvdeselenW<'a, REG> = crate::BitWriter<'a, REG, Slvdeselen>;
impl<'a, REG> SlvdeselenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvdeselen::Disabled)
    }
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvdeselen::Enabled)
    }
}
#[doc = "Monitor data Ready interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monrdyen {
    #[doc = "0: Disabled. The MonRdy interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The MonRdy interrupt is enabled."]
    Enabled = 1,
}
impl From<Monrdyen> for bool {
    #[inline(always)]
    fn from(variant: Monrdyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONRDYEN` reader - Monitor data Ready interrupt Enable."]
pub type MonrdyenR = crate::BitReader<Monrdyen>;
impl MonrdyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monrdyen {
        match self.bits {
            false => Monrdyen::Disabled,
            true => Monrdyen::Enabled,
        }
    }
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Monrdyen::Disabled
    }
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Monrdyen::Enabled
    }
}
#[doc = "Field `MONRDYEN` writer - Monitor data Ready interrupt Enable."]
pub type MonrdyenW<'a, REG> = crate::BitWriter<'a, REG, Monrdyen>;
impl<'a, REG> MonrdyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monrdyen::Disabled)
    }
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monrdyen::Enabled)
    }
}
#[doc = "Monitor Overrun interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monoven {
    #[doc = "0: Disabled. The MonOv interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The MonOv interrupt is enabled."]
    Enabled = 1,
}
impl From<Monoven> for bool {
    #[inline(always)]
    fn from(variant: Monoven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONOVEN` reader - Monitor Overrun interrupt Enable."]
pub type MonovenR = crate::BitReader<Monoven>;
impl MonovenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monoven {
        match self.bits {
            false => Monoven::Disabled,
            true => Monoven::Enabled,
        }
    }
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Monoven::Disabled
    }
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Monoven::Enabled
    }
}
#[doc = "Field `MONOVEN` writer - Monitor Overrun interrupt Enable."]
pub type MonovenW<'a, REG> = crate::BitWriter<'a, REG, Monoven>;
impl<'a, REG> MonovenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monoven::Disabled)
    }
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monoven::Enabled)
    }
}
#[doc = "Monitor Idle interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monidleen {
    #[doc = "0: Disabled. The MonIdle interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The MonIdle interrupt is enabled."]
    Enabled = 1,
}
impl From<Monidleen> for bool {
    #[inline(always)]
    fn from(variant: Monidleen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONIDLEEN` reader - Monitor Idle interrupt Enable."]
pub type MonidleenR = crate::BitReader<Monidleen>;
impl MonidleenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monidleen {
        match self.bits {
            false => Monidleen::Disabled,
            true => Monidleen::Enabled,
        }
    }
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Monidleen::Disabled
    }
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Monidleen::Enabled
    }
}
#[doc = "Field `MONIDLEEN` writer - Monitor Idle interrupt Enable."]
pub type MonidleenW<'a, REG> = crate::BitWriter<'a, REG, Monidleen>;
impl<'a, REG> MonidleenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monidleen::Disabled)
    }
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monidleen::Enabled)
    }
}
#[doc = "Event time-out interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eventtimeouten {
    #[doc = "0: Disabled. The Event time-out interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The Event time-out interrupt is enabled."]
    Enabled = 1,
}
impl From<Eventtimeouten> for bool {
    #[inline(always)]
    fn from(variant: Eventtimeouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTTIMEOUTEN` reader - Event time-out interrupt Enable."]
pub type EventtimeoutenR = crate::BitReader<Eventtimeouten>;
impl EventtimeoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eventtimeouten {
        match self.bits {
            false => Eventtimeouten::Disabled,
            true => Eventtimeouten::Enabled,
        }
    }
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eventtimeouten::Disabled
    }
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eventtimeouten::Enabled
    }
}
#[doc = "Field `EVENTTIMEOUTEN` writer - Event time-out interrupt Enable."]
pub type EventtimeoutenW<'a, REG> = crate::BitWriter<'a, REG, Eventtimeouten>;
impl<'a, REG> EventtimeoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eventtimeouten::Disabled)
    }
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eventtimeouten::Enabled)
    }
}
#[doc = "SCL time-out interrupt Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scltimeouten {
    #[doc = "0: Disabled. The SCL time-out interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The SCL time-out interrupt is enabled."]
    Enabled = 1,
}
impl From<Scltimeouten> for bool {
    #[inline(always)]
    fn from(variant: Scltimeouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLTIMEOUTEN` reader - SCL time-out interrupt Enable."]
pub type ScltimeoutenR = crate::BitReader<Scltimeouten>;
impl ScltimeoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scltimeouten {
        match self.bits {
            false => Scltimeouten::Disabled,
            true => Scltimeouten::Enabled,
        }
    }
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scltimeouten::Disabled
    }
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scltimeouten::Enabled
    }
}
#[doc = "Field `SCLTIMEOUTEN` writer - SCL time-out interrupt Enable."]
pub type ScltimeoutenW<'a, REG> = crate::BitWriter<'a, REG, Scltimeouten>;
impl<'a, REG> ScltimeoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scltimeouten::Disabled)
    }
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scltimeouten::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn mstpendingen(&self) -> MstpendingenR {
        MstpendingenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn mstarblossen(&self) -> MstarblossenR {
        MstarblossenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn mstststperren(&self) -> MstststperrenR {
        MstststperrenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn slvpendingen(&self) -> SlvpendingenR {
        SlvpendingenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn slvnotstren(&self) -> SlvnotstrenR {
        SlvnotstrenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn slvdeselen(&self) -> SlvdeselenR {
        SlvdeselenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn monrdyen(&self) -> MonrdyenR {
        MonrdyenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn monoven(&self) -> MonovenR {
        MonovenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn monidleen(&self) -> MonidleenR {
        MonidleenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn eventtimeouten(&self) -> EventtimeoutenR {
        EventtimeoutenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn scltimeouten(&self) -> ScltimeoutenR {
        ScltimeoutenR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("mstpendingen", &self.mstpendingen())
            .field("mstarblossen", &self.mstarblossen())
            .field("mstststperren", &self.mstststperren())
            .field("slvpendingen", &self.slvpendingen())
            .field("slvnotstren", &self.slvnotstren())
            .field("slvdeselen", &self.slvdeselen())
            .field("monrdyen", &self.monrdyen())
            .field("monoven", &self.monoven())
            .field("monidleen", &self.monidleen())
            .field("eventtimeouten", &self.eventtimeouten())
            .field("scltimeouten", &self.scltimeouten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstpendingen(&mut self) -> MstpendingenW<IntensetSpec> {
        MstpendingenW::new(self, 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstarblossen(&mut self) -> MstarblossenW<IntensetSpec> {
        MstarblossenW::new(self, 4)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstststperren(&mut self) -> MstststperrenW<IntensetSpec> {
        MstststperrenW::new(self, 6)
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvpendingen(&mut self) -> SlvpendingenW<IntensetSpec> {
        SlvpendingenW::new(self, 8)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvnotstren(&mut self) -> SlvnotstrenW<IntensetSpec> {
        SlvnotstrenW::new(self, 11)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvdeselen(&mut self) -> SlvdeselenW<IntensetSpec> {
        SlvdeselenW::new(self, 15)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn monrdyen(&mut self) -> MonrdyenW<IntensetSpec> {
        MonrdyenW::new(self, 16)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn monoven(&mut self) -> MonovenW<IntensetSpec> {
        MonovenW::new(self, 17)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn monidleen(&mut self) -> MonidleenW<IntensetSpec> {
        MonidleenW::new(self, 19)
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn eventtimeouten(&mut self) -> EventtimeoutenW<IntensetSpec> {
        EventtimeoutenW::new(self, 24)
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn scltimeouten(&mut self) -> ScltimeoutenW<IntensetSpec> {
        ScltimeoutenW::new(self, 25)
    }
}
#[doc = "Interrupt Enable Set and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
