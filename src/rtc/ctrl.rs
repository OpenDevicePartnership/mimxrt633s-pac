#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Software reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swreset {
    #[doc = "0: Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NotInReset = 0,
    #[doc = "1: In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the RTC_OSC_PD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    InReset = 1,
}
impl From<Swreset> for bool {
    #[inline(always)]
    fn from(variant: Swreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRESET` reader - Software reset control"]
pub type SwresetR = crate::BitReader<Swreset>;
impl SwresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swreset {
        match self.bits {
            false => Swreset::NotInReset,
            true => Swreset::InReset,
        }
    }
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    #[inline(always)]
    pub fn is_not_in_reset(&self) -> bool {
        *self == Swreset::NotInReset
    }
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the RTC_OSC_PD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    #[inline(always)]
    pub fn is_in_reset(&self) -> bool {
        *self == Swreset::InReset
    }
}
#[doc = "Field `SWRESET` writer - Software reset control"]
pub type SwresetW<'a, REG> = crate::BitWriter<'a, REG, Swreset>;
impl<'a, REG> SwresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    #[inline(always)]
    pub fn not_in_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swreset::NotInReset)
    }
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the RTC_OSC_PD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    #[inline(always)]
    pub fn in_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swreset::InReset)
    }
}
#[doc = "RTC 1 Hz timer alarm flag status.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alarm1hz {
    #[doc = "0: No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NoMatch = 0,
    #[doc = "1: Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    Match = 1,
}
impl From<Alarm1hz> for bool {
    #[inline(always)]
    fn from(variant: Alarm1hz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM1HZ` reader - RTC 1 Hz timer alarm flag status."]
pub type Alarm1hzR = crate::BitReader<Alarm1hz>;
impl Alarm1hzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alarm1hz {
        match self.bits {
            false => Alarm1hz::NoMatch,
            true => Alarm1hz::Match,
        }
    }
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == Alarm1hz::NoMatch
    }
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Alarm1hz::Match
    }
}
#[doc = "Field `ALARM1HZ` writer - RTC 1 Hz timer alarm flag status."]
pub type Alarm1hzW<'a, REG> = crate::BitWriter<'a, REG, Alarm1hz>;
impl<'a, REG> Alarm1hzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut crate::W<REG> {
        self.variant(Alarm1hz::NoMatch)
    }
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Alarm1hz::Match)
    }
}
#[doc = "RTC 1 kHz timer wake-up flag status.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wake1khz {
    #[doc = "0: Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    Run = 0,
    #[doc = "1: Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TimeOut = 1,
}
impl From<Wake1khz> for bool {
    #[inline(always)]
    fn from(variant: Wake1khz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE1KHZ` reader - RTC 1 kHz timer wake-up flag status."]
pub type Wake1khzR = crate::BitReader<Wake1khz>;
impl Wake1khzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wake1khz {
        match self.bits {
            false => Wake1khz::Run,
            true => Wake1khz::TimeOut,
        }
    }
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Wake1khz::Run
    }
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn is_time_out(&self) -> bool {
        *self == Wake1khz::TimeOut
    }
}
#[doc = "Field `WAKE1KHZ` writer - RTC 1 kHz timer wake-up flag status."]
pub type Wake1khzW<'a, REG> = crate::BitWriter<'a, REG, Wake1khz>;
impl<'a, REG> Wake1khzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Wake1khz::Run)
    }
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn time_out(self) -> &'a mut crate::W<REG> {
        self.variant(Wake1khz::TimeOut)
    }
}
#[doc = "RTC 1 Hz timer alarm enable for Deep power-down.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlarmdpdEn {
    #[doc = "0: Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    Disable = 0,
    #[doc = "1: Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    Enable = 1,
}
impl From<AlarmdpdEn> for bool {
    #[inline(always)]
    fn from(variant: AlarmdpdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARMDPD_EN` reader - RTC 1 Hz timer alarm enable for Deep power-down."]
pub type AlarmdpdEnR = crate::BitReader<AlarmdpdEn>;
impl AlarmdpdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlarmdpdEn {
        match self.bits {
            false => AlarmdpdEn::Disable,
            true => AlarmdpdEn::Enable,
        }
    }
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AlarmdpdEn::Disable
    }
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AlarmdpdEn::Enable
    }
}
#[doc = "Field `ALARMDPD_EN` writer - RTC 1 Hz timer alarm enable for Deep power-down."]
pub type AlarmdpdEnW<'a, REG> = crate::BitWriter<'a, REG, AlarmdpdEn>;
impl<'a, REG> AlarmdpdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AlarmdpdEn::Disable)
    }
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AlarmdpdEn::Enable)
    }
}
#[doc = "RTC 1 kHz timer wake-up enable for Deep power-down.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakedpdEn {
    #[doc = "0: Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    Disable = 0,
    #[doc = "1: Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    Enable = 1,
}
impl From<WakedpdEn> for bool {
    #[inline(always)]
    fn from(variant: WakedpdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEDPD_EN` reader - RTC 1 kHz timer wake-up enable for Deep power-down."]
pub type WakedpdEnR = crate::BitReader<WakedpdEn>;
impl WakedpdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakedpdEn {
        match self.bits {
            false => WakedpdEn::Disable,
            true => WakedpdEn::Enable,
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WakedpdEn::Disable
    }
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WakedpdEn::Enable
    }
}
#[doc = "Field `WAKEDPD_EN` writer - RTC 1 kHz timer wake-up enable for Deep power-down."]
pub type WakedpdEnW<'a, REG> = crate::BitWriter<'a, REG, WakedpdEn>;
impl<'a, REG> WakedpdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WakedpdEn::Disable)
    }
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WakedpdEn::Enable)
    }
}
#[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc1khzEn {
    #[doc = "0: Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    Disable = 0,
    #[doc = "1: Enable. The 1 kHz RTC timer is enabled."]
    Enable = 1,
}
impl From<Rtc1khzEn> for bool {
    #[inline(always)]
    fn from(variant: Rtc1khzEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC1KHZ_EN` reader - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
pub type Rtc1khzEnR = crate::BitReader<Rtc1khzEn>;
impl Rtc1khzEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc1khzEn {
        match self.bits {
            false => Rtc1khzEn::Disable,
            true => Rtc1khzEn::Enable,
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtc1khzEn::Disable
    }
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtc1khzEn::Enable
    }
}
#[doc = "Field `RTC1KHZ_EN` writer - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
pub type Rtc1khzEnW<'a, REG> = crate::BitWriter<'a, REG, Rtc1khzEn>;
impl<'a, REG> Rtc1khzEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc1khzEn::Disable)
    }
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc1khzEn::Enable)
    }
}
#[doc = "RTC enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcEn {
    #[doc = "0: Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    Disable = 0,
    #[doc = "1: Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    Enable = 1,
}
impl From<RtcEn> for bool {
    #[inline(always)]
    fn from(variant: RtcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_EN` reader - RTC enable."]
pub type RtcEnR = crate::BitReader<RtcEn>;
impl RtcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcEn {
        match self.bits {
            false => RtcEn::Disable,
            true => RtcEn::Enable,
        }
    }
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RtcEn::Disable
    }
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RtcEn::Enable
    }
}
#[doc = "Field `RTC_EN` writer - RTC enable."]
pub type RtcEnW<'a, REG> = crate::BitWriter<'a, REG, RtcEn>;
impl<'a, REG> RtcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RtcEn::Disable)
    }
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RtcEn::Enable)
    }
}
#[doc = "The RTC oscillator enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcOscPd {
    #[doc = "0: The RTC oscillator is enabled. This bit must be cleared in order for the RTC module to function"]
    Enable = 0,
    #[doc = "1: The RTC oscillator is shut-off to reserve power consumption. RTC operation is disabled."]
    ShutOff = 1,
}
impl From<RtcOscPd> for bool {
    #[inline(always)]
    fn from(variant: RtcOscPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_OSC_PD` reader - The RTC oscillator enable"]
pub type RtcOscPdR = crate::BitReader<RtcOscPd>;
impl RtcOscPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcOscPd {
        match self.bits {
            false => RtcOscPd::Enable,
            true => RtcOscPd::ShutOff,
        }
    }
    #[doc = "The RTC oscillator is enabled. This bit must be cleared in order for the RTC module to function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RtcOscPd::Enable
    }
    #[doc = "The RTC oscillator is shut-off to reserve power consumption. RTC operation is disabled."]
    #[inline(always)]
    pub fn is_shut_off(&self) -> bool {
        *self == RtcOscPd::ShutOff
    }
}
#[doc = "Field `RTC_OSC_PD` writer - The RTC oscillator enable"]
pub type RtcOscPdW<'a, REG> = crate::BitWriter<'a, REG, RtcOscPd>;
impl<'a, REG> RtcOscPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTC oscillator is enabled. This bit must be cleared in order for the RTC module to function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RtcOscPd::Enable)
    }
    #[doc = "The RTC oscillator is shut-off to reserve power consumption. RTC operation is disabled."]
    #[inline(always)]
    pub fn shut_off(self) -> &'a mut crate::W<REG> {
        self.variant(RtcOscPd::ShutOff)
    }
}
#[doc = "The 32 KHz sub-second counter enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcSubsecEna {
    #[doc = "0: The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'"]
    Disable = 0,
    #[doc = "1: The 32 KHz sub-second counter is enabled (if implemented). Counting will commence on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) has been set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep_powerdown mode."]
    Enable = 1,
}
impl From<RtcSubsecEna> for bool {
    #[inline(always)]
    fn from(variant: RtcSubsecEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_SUBSEC_ENA` reader - The 32 KHz sub-second counter enable"]
pub type RtcSubsecEnaR = crate::BitReader<RtcSubsecEna>;
impl RtcSubsecEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcSubsecEna {
        match self.bits {
            false => RtcSubsecEna::Disable,
            true => RtcSubsecEna::Enable,
        }
    }
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RtcSubsecEna::Disable
    }
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting will commence on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) has been set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep_powerdown mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RtcSubsecEna::Enable
    }
}
#[doc = "Field `RTC_SUBSEC_ENA` writer - The 32 KHz sub-second counter enable"]
pub type RtcSubsecEnaW<'a, REG> = crate::BitWriter<'a, REG, RtcSubsecEna>;
impl<'a, REG> RtcSubsecEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RtcSubsecEna::Disable)
    }
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting will commence on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) has been set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep_powerdown mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RtcSubsecEna::Enable)
    }
}
#[doc = "Field `RTC_OSC_loadcap` reader - capacitive load selection"]
pub type RtcOscLoadcapR = crate::FieldReader;
#[doc = "Field `RTC_OSC_loadcap` writer - capacitive load selection"]
pub type RtcOscLoadcapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Software reset control"]
    #[inline(always)]
    pub fn swreset(&self) -> SwresetR {
        SwresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub fn alarm1hz(&self) -> Alarm1hzR {
        Alarm1hzR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub fn wake1khz(&self) -> Wake1khzR {
        Wake1khzR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub fn alarmdpd_en(&self) -> AlarmdpdEnR {
        AlarmdpdEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub fn wakedpd_en(&self) -> WakedpdEnR {
        WakedpdEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub fn rtc1khz_en(&self) -> Rtc1khzEnR {
        Rtc1khzEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline(always)]
    pub fn rtc_en(&self) -> RtcEnR {
        RtcEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The RTC oscillator enable"]
    #[inline(always)]
    pub fn rtc_osc_pd(&self) -> RtcOscPdR {
        RtcOscPdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - The 32 KHz sub-second counter enable"]
    #[inline(always)]
    pub fn rtc_subsec_ena(&self) -> RtcSubsecEnaR {
        RtcSubsecEnaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 28:31 - capacitive load selection"]
    #[inline(always)]
    pub fn rtc_osc_loadcap(&self) -> RtcOscLoadcapR {
        RtcOscLoadcapR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("swreset", &self.swreset())
            .field("alarm1hz", &self.alarm1hz())
            .field("wake1khz", &self.wake1khz())
            .field("alarmdpd_en", &self.alarmdpd_en())
            .field("wakedpd_en", &self.wakedpd_en())
            .field("rtc1khz_en", &self.rtc1khz_en())
            .field("rtc_en", &self.rtc_en())
            .field("rtc_osc_pd", &self.rtc_osc_pd())
            .field("rtc_subsec_ena", &self.rtc_subsec_ena())
            .field("rtc_osc_loadcap", &self.rtc_osc_loadcap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software reset control"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SwresetW<CtrlSpec> {
        SwresetW::new(self, 0)
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    #[must_use]
    pub fn alarm1hz(&mut self) -> Alarm1hzW<CtrlSpec> {
        Alarm1hzW::new(self, 2)
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    #[must_use]
    pub fn wake1khz(&mut self) -> Wake1khzW<CtrlSpec> {
        Wake1khzW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    #[must_use]
    pub fn alarmdpd_en(&mut self) -> AlarmdpdEnW<CtrlSpec> {
        AlarmdpdEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    #[must_use]
    pub fn wakedpd_en(&mut self) -> WakedpdEnW<CtrlSpec> {
        WakedpdEnW::new(self, 5)
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    #[must_use]
    pub fn rtc1khz_en(&mut self) -> Rtc1khzEnW<CtrlSpec> {
        Rtc1khzEnW::new(self, 6)
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_en(&mut self) -> RtcEnW<CtrlSpec> {
        RtcEnW::new(self, 7)
    }
    #[doc = "Bit 8 - The RTC oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_osc_pd(&mut self) -> RtcOscPdW<CtrlSpec> {
        RtcOscPdW::new(self, 8)
    }
    #[doc = "Bit 10 - The 32 KHz sub-second counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_subsec_ena(&mut self) -> RtcSubsecEnaW<CtrlSpec> {
        RtcSubsecEnaW::new(self, 10)
    }
    #[doc = "Bits 28:31 - capacitive load selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_osc_loadcap(&mut self) -> RtcOscLoadcapW<CtrlSpec> {
        RtcOscLoadcapW::new(self, 28)
    }
}
#[doc = "RTC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x1000_0103"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x1000_0103;
}
