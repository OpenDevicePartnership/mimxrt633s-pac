#[doc = "Register `FREQMECTRL_W` writer"]
pub type W = crate::W<FreqmectrlWSpec>;
#[doc = "Reference Clock Scaling Factor\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RefScale {
    #[doc = "0: Count cycle = 2^0 = 1"]
    Countcycle1 = 0,
    #[doc = "1: Count cycle = 2^1 = 2"]
    Countcycle2 = 1,
    #[doc = "2: Count cycle = 2^4 = 4"]
    Countcycle4 = 2,
    #[doc = "31: Count cycle = 2^31 = 2,147,483,648"]
    Countcycle31 = 31,
}
impl From<RefScale> for u8 {
    #[inline(always)]
    fn from(variant: RefScale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RefScale {
    type Ux = u8;
}
impl crate::IsEnum for RefScale {}
#[doc = "Field `REF_SCALE` writer - Reference Clock Scaling Factor"]
pub type RefScaleW<'a, REG> = crate::FieldWriter<'a, REG, 5, RefScale>;
impl<'a, REG> RefScaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Count cycle = 2^0 = 1"]
    #[inline(always)]
    pub fn countcycle_1(self) -> &'a mut crate::W<REG> {
        self.variant(RefScale::Countcycle1)
    }
    #[doc = "Count cycle = 2^1 = 2"]
    #[inline(always)]
    pub fn countcycle_2(self) -> &'a mut crate::W<REG> {
        self.variant(RefScale::Countcycle2)
    }
    #[doc = "Count cycle = 2^4 = 4"]
    #[inline(always)]
    pub fn countcycle_4(self) -> &'a mut crate::W<REG> {
        self.variant(RefScale::Countcycle4)
    }
    #[doc = "Count cycle = 2^31 = 2,147,483,648"]
    #[inline(always)]
    pub fn countcycle_31(self) -> &'a mut crate::W<REG> {
        self.variant(RefScale::Countcycle31)
    }
}
#[doc = "Pulse Width Measurement mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PulseMode {
    #[doc = "0: Frequency Measurement Mode. FREQMECTRL works in a Frequency Measurement mode. Once the measurement starts (real count start is aligned at rising edge arrival on reference clock), the target counter increments by the target clock until the reference counter running by the reference clock reaches the count end point selected by REF_SCALE."]
    FreqMeMode = 0,
    #[doc = "1: Pulse Width Measurement mode. FREQMECTRL works in a Pulse Width Measurement mode, measuring the high or low period of reference clock input selected by PULSE_POL. The target counter starts incrementing by the target clock once a corresponding trigger edge (rising edge for high period measurement and falling edge for low period) occurs."]
    PulseMeMode = 1,
}
impl From<PulseMode> for bool {
    #[inline(always)]
    fn from(variant: PulseMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSE_MODE` writer - Pulse Width Measurement mode select"]
pub type PulseModeW<'a, REG> = crate::BitWriter<'a, REG, PulseMode>;
impl<'a, REG> PulseModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frequency Measurement Mode. FREQMECTRL works in a Frequency Measurement mode. Once the measurement starts (real count start is aligned at rising edge arrival on reference clock), the target counter increments by the target clock until the reference counter running by the reference clock reaches the count end point selected by REF_SCALE."]
    #[inline(always)]
    pub fn freq_me_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PulseMode::FreqMeMode)
    }
    #[doc = "Pulse Width Measurement mode. FREQMECTRL works in a Pulse Width Measurement mode, measuring the high or low period of reference clock input selected by PULSE_POL. The target counter starts incrementing by the target clock once a corresponding trigger edge (rising edge for high period measurement and falling edge for low period) occurs."]
    #[inline(always)]
    pub fn pulse_me_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PulseMode::PulseMeMode)
    }
}
#[doc = "Pulse Polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PulsePol {
    #[doc = "0: High Period. High period of reference clock is measured in Pulse Width Measurement mode triggered by the rising edge on the reference clock input."]
    HighPeriod = 0,
    #[doc = "1: Low Period. Low period of reference clock is measured in Pulse Width Measurement mode triggered by the falling edge on the reference clock input."]
    LowPeriod = 1,
}
impl From<PulsePol> for bool {
    #[inline(always)]
    fn from(variant: PulsePol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSE_POL` writer - Pulse Polarity"]
pub type PulsePolW<'a, REG> = crate::BitWriter<'a, REG, PulsePol>;
impl<'a, REG> PulsePolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Period. High period of reference clock is measured in Pulse Width Measurement mode triggered by the rising edge on the reference clock input."]
    #[inline(always)]
    pub fn high_period(self) -> &'a mut crate::W<REG> {
        self.variant(PulsePol::HighPeriod)
    }
    #[doc = "Low Period. Low period of reference clock is measured in Pulse Width Measurement mode triggered by the falling edge on the reference clock input."]
    #[inline(always)]
    pub fn low_period(self) -> &'a mut crate::W<REG> {
        self.variant(PulsePol::LowPeriod)
    }
}
#[doc = "Measure in Progress\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasureInProgress {
    #[doc = "0: Force Terminate. Forces the termination of any measurement cycle currently in progress and resets RESULT or just resets RESULT if in idle."]
    ForceTerminate = 0,
    #[doc = "1: Initiates Measurement Cycle. Initiates frequency or pulse width measurement process. Hardware clears the MEASURE_IN_PROGRESS bit when the measurement cycle completes. A new measurement starts if there is an active measurement in progress."]
    InitiateAFreqmeCycle = 1,
}
impl From<MeasureInProgress> for bool {
    #[inline(always)]
    fn from(variant: MeasureInProgress) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEASURE_IN_PROGRESS` writer - Measure in Progress"]
pub type MeasureInProgressW<'a, REG> = crate::BitWriter<'a, REG, MeasureInProgress>;
impl<'a, REG> MeasureInProgressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force Terminate. Forces the termination of any measurement cycle currently in progress and resets RESULT or just resets RESULT if in idle."]
    #[inline(always)]
    pub fn force_terminate(self) -> &'a mut crate::W<REG> {
        self.variant(MeasureInProgress::ForceTerminate)
    }
    #[doc = "Initiates Measurement Cycle. Initiates frequency or pulse width measurement process. Hardware clears the MEASURE_IN_PROGRESS bit when the measurement cycle completes. A new measurement starts if there is an active measurement in progress."]
    #[inline(always)]
    pub fn initiate_a_freqme_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(MeasureInProgress::InitiateAFreqmeCycle)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FreqmectrlWSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference Clock Scaling Factor"]
    #[inline(always)]
    #[must_use]
    pub fn ref_scale(&mut self) -> RefScaleW<FreqmectrlWSpec> {
        RefScaleW::new(self, 0)
    }
    #[doc = "Bit 8 - Pulse Width Measurement mode select"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_mode(&mut self) -> PulseModeW<FreqmectrlWSpec> {
        PulseModeW::new(self, 8)
    }
    #[doc = "Bit 9 - Pulse Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_pol(&mut self) -> PulsePolW<FreqmectrlWSpec> {
        PulsePolW::new(self, 9)
    }
    #[doc = "Bit 31 - Measure in Progress"]
    #[inline(always)]
    #[must_use]
    pub fn measure_in_progress(&mut self) -> MeasureInProgressW<FreqmectrlWSpec> {
        MeasureInProgressW::new(self, 31)
    }
}
#[doc = "Freqeuncy Measurement (in Write mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqmectrl_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqmectrlWSpec;
impl crate::RegisterSpec for FreqmectrlWSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`freqmectrl_w::W`](W) writer structure"]
impl crate::Writable for FreqmectrlWSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQMECTRL_W to value 0"]
impl crate::Resettable for FreqmectrlWSpec {
    const RESET_VALUE: u32 = 0;
}
