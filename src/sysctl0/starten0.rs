#[doc = "Register `STARTEN0` reader"]
pub type R = crate::R<Starten0Spec>;
#[doc = "Register `STARTEN0` writer"]
pub type W = crate::W<Starten0Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Wdt0> for bool {
    #[inline(always)]
    fn from(variant: Wdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT0` reader - no description available"]
pub type Wdt0R = crate::BitReader<Wdt0>;
impl Wdt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt0 {
        match self.bits {
            false => Wdt0::Disabled,
            true => Wdt0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdt0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wdt0::Enabled
    }
}
#[doc = "Field `WDT0` writer - no description available"]
pub type Wdt0W<'a, REG> = crate::BitWriter<'a, REG, Wdt0>;
impl<'a, REG> Wdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0> for bool {
    #[inline(always)]
    fn from(variant: Dmac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0` reader - no description available"]
pub type Dmac0R = crate::BitReader<Dmac0>;
impl Dmac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0 {
        match self.bits {
            false => Dmac0::Disabled,
            true => Dmac0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0::Enabled
    }
}
#[doc = "Field `DMAC0` writer - no description available"]
pub type Dmac0W<'a, REG> = crate::BitWriter<'a, REG, Dmac0>;
impl<'a, REG> Dmac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NshsgpioInt0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<NshsgpioInt0> for bool {
    #[inline(always)]
    fn from(variant: NshsgpioInt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSHSGPIO_INT0` reader - no description available"]
pub type NshsgpioInt0R = crate::BitReader<NshsgpioInt0>;
impl NshsgpioInt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NshsgpioInt0 {
        match self.bits {
            false => NshsgpioInt0::Disabled,
            true => NshsgpioInt0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NshsgpioInt0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NshsgpioInt0::Enabled
    }
}
#[doc = "Field `NSHSGPIO_INT0` writer - no description available"]
pub type NshsgpioInt0W<'a, REG> = crate::BitWriter<'a, REG, NshsgpioInt0>;
impl<'a, REG> NshsgpioInt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NshsgpioInt1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<NshsgpioInt1> for bool {
    #[inline(always)]
    fn from(variant: NshsgpioInt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSHSGPIO_INT1` reader - no description available"]
pub type NshsgpioInt1R = crate::BitReader<NshsgpioInt1>;
impl NshsgpioInt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NshsgpioInt1 {
        match self.bits {
            false => NshsgpioInt1::Disabled,
            true => NshsgpioInt1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NshsgpioInt1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NshsgpioInt1::Enabled
    }
}
#[doc = "Field `NSHSGPIO_INT1` writer - no description available"]
pub type NshsgpioInt1W<'a, REG> = crate::BitWriter<'a, REG, NshsgpioInt1>;
impl<'a, REG> NshsgpioInt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq0> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` reader - no description available"]
pub type GpioInt0Irq0R = crate::BitReader<GpioInt0Irq0>;
impl GpioInt0Irq0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq0 {
        match self.bits {
            false => GpioInt0Irq0::Disabled,
            true => GpioInt0Irq0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq0::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - no description available"]
pub type GpioInt0Irq0W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq0>;
impl<'a, REG> GpioInt0Irq0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq1> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` reader - no description available"]
pub type GpioInt0Irq1R = crate::BitReader<GpioInt0Irq1>;
impl GpioInt0Irq1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq1 {
        match self.bits {
            false => GpioInt0Irq1::Disabled,
            true => GpioInt0Irq1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq1::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - no description available"]
pub type GpioInt0Irq1W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq1>;
impl<'a, REG> GpioInt0Irq1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq2 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq2> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` reader - no description available"]
pub type GpioInt0Irq2R = crate::BitReader<GpioInt0Irq2>;
impl GpioInt0Irq2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq2 {
        match self.bits {
            false => GpioInt0Irq2::Disabled,
            true => GpioInt0Irq2::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq2::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq2::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - no description available"]
pub type GpioInt0Irq2W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq2>;
impl<'a, REG> GpioInt0Irq2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq2::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq2::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq3 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq3> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` reader - no description available"]
pub type GpioInt0Irq3R = crate::BitReader<GpioInt0Irq3>;
impl GpioInt0Irq3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq3 {
        match self.bits {
            false => GpioInt0Irq3::Disabled,
            true => GpioInt0Irq3::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq3::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq3::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - no description available"]
pub type GpioInt0Irq3W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq3>;
impl<'a, REG> GpioInt0Irq3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq3::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq3::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Utick0> for bool {
    #[inline(always)]
    fn from(variant: Utick0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0` reader - no description available"]
pub type Utick0R = crate::BitReader<Utick0>;
impl Utick0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Utick0 {
        match self.bits {
            false => Utick0::Disabled,
            true => Utick0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Utick0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Utick0::Enabled
    }
}
#[doc = "Field `UTICK0` writer - no description available"]
pub type Utick0W<'a, REG> = crate::BitWriter<'a, REG, Utick0>;
impl<'a, REG> Utick0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Mrt0> for bool {
    #[inline(always)]
    fn from(variant: Mrt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0` reader - no description available"]
pub type Mrt0R = crate::BitReader<Mrt0>;
impl Mrt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrt0 {
        match self.bits {
            false => Mrt0::Disabled,
            true => Mrt0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mrt0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mrt0::Enabled
    }
}
#[doc = "Field `MRT0` writer - no description available"]
pub type Mrt0W<'a, REG> = crate::BitWriter<'a, REG, Mrt0>;
impl<'a, REG> Mrt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Ct32bit0> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0` reader - no description available"]
pub type Ct32bit0R = crate::BitReader<Ct32bit0>;
impl Ct32bit0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit0 {
        match self.bits {
            false => Ct32bit0::Disabled,
            true => Ct32bit0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ct32bit0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ct32bit0::Enabled
    }
}
#[doc = "Field `CT32BIT0` writer - no description available"]
pub type Ct32bit0W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0>;
impl<'a, REG> Ct32bit0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Ct32bit1> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1` reader - no description available"]
pub type Ct32bit1R = crate::BitReader<Ct32bit1>;
impl Ct32bit1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit1 {
        match self.bits {
            false => Ct32bit1::Disabled,
            true => Ct32bit1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ct32bit1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ct32bit1::Enabled
    }
}
#[doc = "Field `CT32BIT1` writer - no description available"]
pub type Ct32bit1W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1>;
impl<'a, REG> Ct32bit1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sct0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Sct0> for bool {
    #[inline(always)]
    fn from(variant: Sct0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT0` reader - no description available"]
pub type Sct0R = crate::BitReader<Sct0>;
impl Sct0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sct0 {
        match self.bits {
            false => Sct0::Disabled,
            true => Sct0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sct0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sct0::Enabled
    }
}
#[doc = "Field `SCT0` writer - no description available"]
pub type Sct0W<'a, REG> = crate::BitWriter<'a, REG, Sct0>;
impl<'a, REG> Sct0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sct0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sct0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Ct32bit3> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3` reader - no description available"]
pub type Ct32bit3R = crate::BitReader<Ct32bit3>;
impl Ct32bit3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit3 {
        match self.bits {
            false => Ct32bit3::Disabled,
            true => Ct32bit3::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ct32bit3::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ct32bit3::Enabled
    }
}
#[doc = "Field `CT32BIT3` writer - no description available"]
pub type Ct32bit3W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3>;
impl<'a, REG> Ct32bit3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm0> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0` reader - no description available"]
pub type Flexcomm0R = crate::BitReader<Flexcomm0>;
impl Flexcomm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm0 {
        match self.bits {
            false => Flexcomm0::Disabled,
            true => Flexcomm0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm0::Enabled
    }
}
#[doc = "Field `FLEXCOMM0` writer - no description available"]
pub type Flexcomm0W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0>;
impl<'a, REG> Flexcomm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm1> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1` reader - no description available"]
pub type Flexcomm1R = crate::BitReader<Flexcomm1>;
impl Flexcomm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm1 {
        match self.bits {
            false => Flexcomm1::Disabled,
            true => Flexcomm1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm1::Enabled
    }
}
#[doc = "Field `FLEXCOMM1` writer - no description available"]
pub type Flexcomm1W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1>;
impl<'a, REG> Flexcomm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm2> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2` reader - no description available"]
pub type Flexcomm2R = crate::BitReader<Flexcomm2>;
impl Flexcomm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm2 {
        match self.bits {
            false => Flexcomm2::Disabled,
            true => Flexcomm2::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm2::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm2::Enabled
    }
}
#[doc = "Field `FLEXCOMM2` writer - no description available"]
pub type Flexcomm2W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2>;
impl<'a, REG> Flexcomm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm3> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3` reader - no description available"]
pub type Flexcomm3R = crate::BitReader<Flexcomm3>;
impl Flexcomm3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm3 {
        match self.bits {
            false => Flexcomm3::Disabled,
            true => Flexcomm3::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm3::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm3::Enabled
    }
}
#[doc = "Field `FLEXCOMM3` writer - no description available"]
pub type Flexcomm3W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3>;
impl<'a, REG> Flexcomm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm4> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4` reader - no description available"]
pub type Flexcomm4R = crate::BitReader<Flexcomm4>;
impl Flexcomm4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm4 {
        match self.bits {
            false => Flexcomm4::Disabled,
            true => Flexcomm4::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm4::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm4::Enabled
    }
}
#[doc = "Field `FLEXCOMM4` writer - no description available"]
pub type Flexcomm4W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4>;
impl<'a, REG> Flexcomm4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm5> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5` reader - no description available"]
pub type Flexcomm5R = crate::BitReader<Flexcomm5>;
impl Flexcomm5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm5 {
        match self.bits {
            false => Flexcomm5::Disabled,
            true => Flexcomm5::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm5::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm5::Enabled
    }
}
#[doc = "Field `FLEXCOMM5` writer - no description available"]
pub type Flexcomm5W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5>;
impl<'a, REG> Flexcomm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm14> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14` reader - no description available"]
pub type Flexcomm14R = crate::BitReader<Flexcomm14>;
impl Flexcomm14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm14 {
        match self.bits {
            false => Flexcomm14::Disabled,
            true => Flexcomm14::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm14::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm14::Enabled
    }
}
#[doc = "Field `FLEXCOMM14` writer - no description available"]
pub type Flexcomm14W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14>;
impl<'a, REG> Flexcomm14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm15 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm15> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM15` reader - no description available"]
pub type Flexcomm15R = crate::BitReader<Flexcomm15>;
impl Flexcomm15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm15 {
        match self.bits {
            false => Flexcomm15::Disabled,
            true => Flexcomm15::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm15::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm15::Enabled
    }
}
#[doc = "Field `FLEXCOMM15` writer - no description available"]
pub type Flexcomm15W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm15>;
impl<'a, REG> Flexcomm15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - no description available"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            false => Adc0::Disabled,
            true => Adc0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adc0::Enabled
    }
}
#[doc = "Field `ADC0` writer - no description available"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Acmp> for bool {
    #[inline(always)]
    fn from(variant: Acmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` reader - no description available"]
pub type AcmpR = crate::BitReader<Acmp>;
impl AcmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmp {
        match self.bits {
            false => Acmp::Disabled,
            true => Acmp::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Acmp::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Acmp::Enabled
    }
}
#[doc = "Field `ACMP` writer - no description available"]
pub type AcmpW<'a, REG> = crate::BitWriter<'a, REG, Acmp>;
impl<'a, REG> AcmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0> for bool {
    #[inline(always)]
    fn from(variant: Dmic0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0` reader - no description available"]
pub type Dmic0R = crate::BitReader<Dmic0>;
impl Dmic0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0 {
        match self.bits {
            false => Dmic0::Disabled,
            true => Dmic0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0::Enabled
    }
}
#[doc = "Field `DMIC0` writer - no description available"]
pub type Dmic0W<'a, REG> = crate::BitWriter<'a, REG, Dmic0>;
impl<'a, REG> Dmic0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hypervisor {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Hypervisor> for bool {
    #[inline(always)]
    fn from(variant: Hypervisor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYPERVISOR` reader - no description available"]
pub type HypervisorR = crate::BitReader<Hypervisor>;
impl HypervisorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hypervisor {
        match self.bits {
            false => Hypervisor::Disabled,
            true => Hypervisor::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hypervisor::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hypervisor::Enabled
    }
}
#[doc = "Field `HYPERVISOR` writer - no description available"]
pub type HypervisorW<'a, REG> = crate::BitWriter<'a, REG, Hypervisor>;
impl<'a, REG> HypervisorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hypervisor::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hypervisor::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secureviolation {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Secureviolation> for bool {
    #[inline(always)]
    fn from(variant: Secureviolation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECUREVIOLATION` reader - no description available"]
pub type SecureviolationR = crate::BitReader<Secureviolation>;
impl SecureviolationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secureviolation {
        match self.bits {
            false => Secureviolation::Disabled,
            true => Secureviolation::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Secureviolation::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Secureviolation::Enabled
    }
}
#[doc = "Field `SECUREVIOLATION` writer - no description available"]
pub type SecureviolationW<'a, REG> = crate::BitWriter<'a, REG, Secureviolation>;
impl<'a, REG> SecureviolationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secureviolation::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secureviolation::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwvad0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Hwvad0> for bool {
    #[inline(always)]
    fn from(variant: Hwvad0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWVAD0` reader - no description available"]
pub type Hwvad0R = crate::BitReader<Hwvad0>;
impl Hwvad0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwvad0 {
        match self.bits {
            false => Hwvad0::Disabled,
            true => Hwvad0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hwvad0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hwvad0::Enabled
    }
}
#[doc = "Field `HWVAD0` writer - no description available"]
pub type Hwvad0W<'a, REG> = crate::BitWriter<'a, REG, Hwvad0>;
impl<'a, REG> Hwvad0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwvad0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwvad0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - no description available"]
pub type RngR = crate::BitReader<Rng>;
impl RngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rng {
        match self.bits {
            false => Rng::Disabled,
            true => Rng::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rng::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rng::Enabled
    }
}
#[doc = "Field `RNG` writer - no description available"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn wdt0(&self) -> Wdt0R {
        Wdt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn dmac0(&self) -> Dmac0R {
        Dmac0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nshsgpio_int0(&self) -> NshsgpioInt0R {
        NshsgpioInt0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn nshsgpio_int1(&self) -> NshsgpioInt1R {
        NshsgpioInt1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq0(&self) -> GpioInt0Irq0R {
        GpioInt0Irq0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq1(&self) -> GpioInt0Irq1R {
        GpioInt0Irq1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq2(&self) -> GpioInt0Irq2R {
        GpioInt0Irq2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq3(&self) -> GpioInt0Irq3R {
        GpioInt0Irq3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn utick0(&self) -> Utick0R {
        Utick0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn mrt0(&self) -> Mrt0R {
        Mrt0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn ct32bit0(&self) -> Ct32bit0R {
        Ct32bit0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn ct32bit1(&self) -> Ct32bit1R {
        Ct32bit1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn sct0(&self) -> Sct0R {
        Sct0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn ct32bit3(&self) -> Ct32bit3R {
        Ct32bit3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn flexcomm0(&self) -> Flexcomm0R {
        Flexcomm0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn flexcomm1(&self) -> Flexcomm1R {
        Flexcomm1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn flexcomm2(&self) -> Flexcomm2R {
        Flexcomm2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn flexcomm3(&self) -> Flexcomm3R {
        Flexcomm3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn flexcomm4(&self) -> Flexcomm4R {
        Flexcomm4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn flexcomm5(&self) -> Flexcomm5R {
        Flexcomm5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    pub fn flexcomm14(&self) -> Flexcomm14R {
        Flexcomm14R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn flexcomm15(&self) -> Flexcomm15R {
        Flexcomm15R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn acmp(&self) -> AcmpR {
        AcmpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn dmic0(&self) -> Dmic0R {
        Dmic0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn hypervisor(&self) -> HypervisorR {
        HypervisorR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn secureviolation(&self) -> SecureviolationR {
        SecureviolationR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    pub fn hwvad0(&self) -> Hwvad0R {
        Hwvad0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STARTEN0")
            .field("wdt0", &self.wdt0())
            .field("dmac0", &self.dmac0())
            .field("nshsgpio_int0", &self.nshsgpio_int0())
            .field("nshsgpio_int1", &self.nshsgpio_int1())
            .field("gpio_int0_irq0", &self.gpio_int0_irq0())
            .field("gpio_int0_irq1", &self.gpio_int0_irq1())
            .field("gpio_int0_irq2", &self.gpio_int0_irq2())
            .field("gpio_int0_irq3", &self.gpio_int0_irq3())
            .field("utick0", &self.utick0())
            .field("mrt0", &self.mrt0())
            .field("ct32bit0", &self.ct32bit0())
            .field("ct32bit1", &self.ct32bit1())
            .field("sct0", &self.sct0())
            .field("ct32bit3", &self.ct32bit3())
            .field("flexcomm0", &self.flexcomm0())
            .field("flexcomm1", &self.flexcomm1())
            .field("flexcomm2", &self.flexcomm2())
            .field("flexcomm3", &self.flexcomm3())
            .field("flexcomm4", &self.flexcomm4())
            .field("flexcomm5", &self.flexcomm5())
            .field("flexcomm14", &self.flexcomm14())
            .field("flexcomm15", &self.flexcomm15())
            .field("adc0", &self.adc0())
            .field("acmp", &self.acmp())
            .field("dmic0", &self.dmic0())
            .field("hypervisor", &self.hypervisor())
            .field("secureviolation", &self.secureviolation())
            .field("hwvad0", &self.hwvad0())
            .field("rng", &self.rng())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn wdt0(&mut self) -> Wdt0W<Starten0Spec> {
        Wdt0W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn dmac0(&mut self) -> Dmac0W<Starten0Spec> {
        Dmac0W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nshsgpio_int0(&mut self) -> NshsgpioInt0W<Starten0Spec> {
        NshsgpioInt0W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn nshsgpio_int1(&mut self) -> NshsgpioInt1W<Starten0Spec> {
        NshsgpioInt1W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq0(&mut self) -> GpioInt0Irq0W<Starten0Spec> {
        GpioInt0Irq0W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq1(&mut self) -> GpioInt0Irq1W<Starten0Spec> {
        GpioInt0Irq1W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq2(&mut self) -> GpioInt0Irq2W<Starten0Spec> {
        GpioInt0Irq2W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq3(&mut self) -> GpioInt0Irq3W<Starten0Spec> {
        GpioInt0Irq3W::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn utick0(&mut self) -> Utick0W<Starten0Spec> {
        Utick0W::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn mrt0(&mut self) -> Mrt0W<Starten0Spec> {
        Mrt0W::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn ct32bit0(&mut self) -> Ct32bit0W<Starten0Spec> {
        Ct32bit0W::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn ct32bit1(&mut self) -> Ct32bit1W<Starten0Spec> {
        Ct32bit1W::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn sct0(&mut self) -> Sct0W<Starten0Spec> {
        Sct0W::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn ct32bit3(&mut self) -> Ct32bit3W<Starten0Spec> {
        Ct32bit3W::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> Flexcomm0W<Starten0Spec> {
        Flexcomm0W::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> Flexcomm1W<Starten0Spec> {
        Flexcomm1W::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> Flexcomm2W<Starten0Spec> {
        Flexcomm2W::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> Flexcomm3W<Starten0Spec> {
        Flexcomm3W::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> Flexcomm4W<Starten0Spec> {
        Flexcomm4W::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> Flexcomm5W<Starten0Spec> {
        Flexcomm5W::new(self, 19)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    pub fn flexcomm14(&mut self) -> Flexcomm14W<Starten0Spec> {
        Flexcomm14W::new(self, 20)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn flexcomm15(&mut self) -> Flexcomm15W<Starten0Spec> {
        Flexcomm15W::new(self, 21)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<Starten0Spec> {
        Adc0W::new(self, 22)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn acmp(&mut self) -> AcmpW<Starten0Spec> {
        AcmpW::new(self, 24)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn dmic0(&mut self) -> Dmic0W<Starten0Spec> {
        Dmic0W::new(self, 25)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn hypervisor(&mut self) -> HypervisorW<Starten0Spec> {
        HypervisorW::new(self, 27)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn secureviolation(&mut self) -> SecureviolationW<Starten0Spec> {
        SecureviolationW::new(self, 28)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    pub fn hwvad0(&mut self) -> Hwvad0W<Starten0Spec> {
        Hwvad0W::new(self, 29)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn rng(&mut self) -> RngW<Starten0Spec> {
        RngW::new(self, 31)
    }
}
#[doc = "Start enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`starten0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Starten0Spec;
impl crate::RegisterSpec for Starten0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`starten0::R`](R) reader structure"]
impl crate::Readable for Starten0Spec {}
#[doc = "`write(|w| ..)` method takes [`starten0::W`](W) writer structure"]
impl crate::Writable for Starten0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTEN0 to value 0"]
impl crate::Resettable for Starten0Spec {
    const RESET_VALUE: u32 = 0;
}
