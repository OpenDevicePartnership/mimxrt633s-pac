#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "SCT operation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unify {
    #[doc = "0: The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DualCounter = 0,
    #[doc = "1: The SCT operates as a unified 32-bit counter."]
    UnifiedCounter = 1,
}
impl From<Unify> for bool {
    #[inline(always)]
    fn from(variant: Unify) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNIFY` reader - SCT operation"]
pub type UnifyR = crate::BitReader<Unify>;
impl UnifyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unify {
        match self.bits {
            false => Unify::DualCounter,
            true => Unify::UnifiedCounter,
        }
    }
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline(always)]
    pub fn is_dual_counter(&self) -> bool {
        *self == Unify::DualCounter
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn is_unified_counter(&self) -> bool {
        *self == Unify::UnifiedCounter
    }
}
#[doc = "Field `UNIFY` writer - SCT operation"]
pub type UnifyW<'a, REG> = crate::BitWriter<'a, REG, Unify>;
impl<'a, REG> UnifyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline(always)]
    pub fn dual_counter(self) -> &'a mut crate::W<REG> {
        self.variant(Unify::DualCounter)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn unified_counter(self) -> &'a mut crate::W<REG> {
        self.variant(Unify::UnifiedCounter)
    }
}
#[doc = "SCT clock mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkmode {
    #[doc = "0: System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SystemClockMode = 0,
    #[doc = "1: Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SampledSystemClockMode = 1,
    #[doc = "2: SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SctInputClockMode = 2,
    #[doc = "3: Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    AsynchronousMode = 3,
}
impl From<Clkmode> for u8 {
    #[inline(always)]
    fn from(variant: Clkmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkmode {
    type Ux = u8;
}
impl crate::IsEnum for Clkmode {}
#[doc = "Field `CLKMODE` reader - SCT clock mode"]
pub type ClkmodeR = crate::FieldReader<Clkmode>;
impl ClkmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkmode {
        match self.bits {
            0 => Clkmode::SystemClockMode,
            1 => Clkmode::SampledSystemClockMode,
            2 => Clkmode::SctInputClockMode,
            3 => Clkmode::AsynchronousMode,
            _ => unreachable!(),
        }
    }
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    #[inline(always)]
    pub fn is_system_clock_mode(&self) -> bool {
        *self == Clkmode::SystemClockMode
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline(always)]
    pub fn is_sampled_system_clock_mode(&self) -> bool {
        *self == Clkmode::SampledSystemClockMode
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline(always)]
    pub fn is_sct_input_clock_mode(&self) -> bool {
        *self == Clkmode::SctInputClockMode
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == Clkmode::AsynchronousMode
    }
}
#[doc = "Field `CLKMODE` writer - SCT clock mode"]
pub type ClkmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clkmode, crate::Safe>;
impl<'a, REG> ClkmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    #[inline(always)]
    pub fn system_clock_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmode::SystemClockMode)
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline(always)]
    pub fn sampled_system_clock_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmode::SampledSystemClockMode)
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline(always)]
    pub fn sct_input_clock_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmode::SctInputClockMode)
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmode::AsynchronousMode)
    }
}
#[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cksel {
    #[doc = "0: Rising edges on input 0."]
    Input0RisingEdges = 0,
    #[doc = "1: Falling edges on input 0."]
    Input0FallingEdge = 1,
    #[doc = "2: Rising edges on input 1."]
    Input1RisingEdges = 2,
    #[doc = "3: Falling edges on input 1."]
    Input1FallingEdge = 3,
    #[doc = "4: Rising edges on input 2."]
    Input2RisingEdges = 4,
    #[doc = "5: Falling edges on input 2."]
    Input2FallingEdge = 5,
    #[doc = "6: Rising edges on input 3."]
    Input3RisingEdges = 6,
    #[doc = "7: Falling edges on input 3."]
    Input3FallingEdge = 7,
    #[doc = "8: Rising edges on input 4."]
    Input4RisingEdges = 8,
    #[doc = "9: Falling edges on input 4."]
    Input4FallingEdge = 9,
    #[doc = "10: Rising edges on input 5."]
    Input5RisingEdges = 10,
    #[doc = "11: Falling edges on input 5."]
    Input5FallingEdge = 11,
    #[doc = "12: Rising edges on input 6."]
    Input6RisingEdges = 12,
    #[doc = "13: Falling edges on input 6."]
    Input6FallingEdge = 13,
    #[doc = "14: Rising edges on input 7."]
    Input7RisingEdges = 14,
    #[doc = "15: Falling edges on input 7."]
    Input7FallingEdge = 15,
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(variant: Cksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cksel {
    type Ux = u8;
}
impl crate::IsEnum for Cksel {}
#[doc = "Field `CKSEL` reader - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
pub type CkselR = crate::FieldReader<Cksel>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cksel {
        match self.bits {
            0 => Cksel::Input0RisingEdges,
            1 => Cksel::Input0FallingEdge,
            2 => Cksel::Input1RisingEdges,
            3 => Cksel::Input1FallingEdge,
            4 => Cksel::Input2RisingEdges,
            5 => Cksel::Input2FallingEdge,
            6 => Cksel::Input3RisingEdges,
            7 => Cksel::Input3FallingEdge,
            8 => Cksel::Input4RisingEdges,
            9 => Cksel::Input4FallingEdge,
            10 => Cksel::Input5RisingEdges,
            11 => Cksel::Input5FallingEdge,
            12 => Cksel::Input6RisingEdges,
            13 => Cksel::Input6FallingEdge,
            14 => Cksel::Input7RisingEdges,
            15 => Cksel::Input7FallingEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edges on input 0."]
    #[inline(always)]
    pub fn is_input_0_rising_edges(&self) -> bool {
        *self == Cksel::Input0RisingEdges
    }
    #[doc = "Falling edges on input 0."]
    #[inline(always)]
    pub fn is_input_0_falling_edge(&self) -> bool {
        *self == Cksel::Input0FallingEdge
    }
    #[doc = "Rising edges on input 1."]
    #[inline(always)]
    pub fn is_input_1_rising_edges(&self) -> bool {
        *self == Cksel::Input1RisingEdges
    }
    #[doc = "Falling edges on input 1."]
    #[inline(always)]
    pub fn is_input_1_falling_edge(&self) -> bool {
        *self == Cksel::Input1FallingEdge
    }
    #[doc = "Rising edges on input 2."]
    #[inline(always)]
    pub fn is_input_2_rising_edges(&self) -> bool {
        *self == Cksel::Input2RisingEdges
    }
    #[doc = "Falling edges on input 2."]
    #[inline(always)]
    pub fn is_input_2_falling_edge(&self) -> bool {
        *self == Cksel::Input2FallingEdge
    }
    #[doc = "Rising edges on input 3."]
    #[inline(always)]
    pub fn is_input_3_rising_edges(&self) -> bool {
        *self == Cksel::Input3RisingEdges
    }
    #[doc = "Falling edges on input 3."]
    #[inline(always)]
    pub fn is_input_3_falling_edge(&self) -> bool {
        *self == Cksel::Input3FallingEdge
    }
    #[doc = "Rising edges on input 4."]
    #[inline(always)]
    pub fn is_input_4_rising_edges(&self) -> bool {
        *self == Cksel::Input4RisingEdges
    }
    #[doc = "Falling edges on input 4."]
    #[inline(always)]
    pub fn is_input_4_falling_edge(&self) -> bool {
        *self == Cksel::Input4FallingEdge
    }
    #[doc = "Rising edges on input 5."]
    #[inline(always)]
    pub fn is_input_5_rising_edges(&self) -> bool {
        *self == Cksel::Input5RisingEdges
    }
    #[doc = "Falling edges on input 5."]
    #[inline(always)]
    pub fn is_input_5_falling_edge(&self) -> bool {
        *self == Cksel::Input5FallingEdge
    }
    #[doc = "Rising edges on input 6."]
    #[inline(always)]
    pub fn is_input_6_rising_edges(&self) -> bool {
        *self == Cksel::Input6RisingEdges
    }
    #[doc = "Falling edges on input 6."]
    #[inline(always)]
    pub fn is_input_6_falling_edge(&self) -> bool {
        *self == Cksel::Input6FallingEdge
    }
    #[doc = "Rising edges on input 7."]
    #[inline(always)]
    pub fn is_input_7_rising_edges(&self) -> bool {
        *self == Cksel::Input7RisingEdges
    }
    #[doc = "Falling edges on input 7."]
    #[inline(always)]
    pub fn is_input_7_falling_edge(&self) -> bool {
        *self == Cksel::Input7FallingEdge
    }
}
#[doc = "Field `CKSEL` writer - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
pub type CkselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cksel, crate::Safe>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges on input 0."]
    #[inline(always)]
    pub fn input_0_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input0RisingEdges)
    }
    #[doc = "Falling edges on input 0."]
    #[inline(always)]
    pub fn input_0_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input0FallingEdge)
    }
    #[doc = "Rising edges on input 1."]
    #[inline(always)]
    pub fn input_1_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input1RisingEdges)
    }
    #[doc = "Falling edges on input 1."]
    #[inline(always)]
    pub fn input_1_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input1FallingEdge)
    }
    #[doc = "Rising edges on input 2."]
    #[inline(always)]
    pub fn input_2_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input2RisingEdges)
    }
    #[doc = "Falling edges on input 2."]
    #[inline(always)]
    pub fn input_2_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input2FallingEdge)
    }
    #[doc = "Rising edges on input 3."]
    #[inline(always)]
    pub fn input_3_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input3RisingEdges)
    }
    #[doc = "Falling edges on input 3."]
    #[inline(always)]
    pub fn input_3_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input3FallingEdge)
    }
    #[doc = "Rising edges on input 4."]
    #[inline(always)]
    pub fn input_4_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input4RisingEdges)
    }
    #[doc = "Falling edges on input 4."]
    #[inline(always)]
    pub fn input_4_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input4FallingEdge)
    }
    #[doc = "Rising edges on input 5."]
    #[inline(always)]
    pub fn input_5_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input5RisingEdges)
    }
    #[doc = "Falling edges on input 5."]
    #[inline(always)]
    pub fn input_5_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input5FallingEdge)
    }
    #[doc = "Rising edges on input 6."]
    #[inline(always)]
    pub fn input_6_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input6RisingEdges)
    }
    #[doc = "Falling edges on input 6."]
    #[inline(always)]
    pub fn input_6_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input6FallingEdge)
    }
    #[doc = "Rising edges on input 7."]
    #[inline(always)]
    pub fn input_7_rising_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input7RisingEdges)
    }
    #[doc = "Falling edges on input 7."]
    #[inline(always)]
    pub fn input_7_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Input7FallingEdge)
    }
}
#[doc = "Field `NORELOAD_L` reader - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type NoreloadLR = crate::BitReader;
#[doc = "Field `NORELOAD_L` writer - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type NoreloadLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NORELOAD_H` reader - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type NoreloadHR = crate::BitReader;
#[doc = "Field `NORELOAD_H` writer - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type NoreloadHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSYNC` reader - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
pub type InsyncR = crate::FieldReader;
#[doc = "Field `INSYNC` writer - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
pub type InsyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AUTOLIMIT_L` reader - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type AutolimitLR = crate::BitReader;
#[doc = "Field `AUTOLIMIT_L` writer - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type AutolimitLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOLIMIT_H` reader - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type AutolimitHR = crate::BitReader;
#[doc = "Field `AUTOLIMIT_H` writer - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type AutolimitHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&self) -> UnifyR {
        UnifyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&self) -> ClkmodeR {
        ClkmodeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_l(&self) -> NoreloadLR {
        NoreloadLR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&self) -> NoreloadHR {
        NoreloadHR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&self) -> InsyncR {
        InsyncR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&self) -> AutolimitLR {
        AutolimitLR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&self) -> AutolimitHR {
        AutolimitHR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("unify", &self.unify())
            .field("clkmode", &self.clkmode())
            .field("cksel", &self.cksel())
            .field("noreload_l", &self.noreload_l())
            .field("noreload_h", &self.noreload_h())
            .field("insync", &self.insync())
            .field("autolimit_l", &self.autolimit_l())
            .field("autolimit_h", &self.autolimit_h())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&mut self) -> UnifyW<ConfigSpec> {
        UnifyW::new(self, 0)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&mut self) -> ClkmodeW<ConfigSpec> {
        ClkmodeW::new(self, 1)
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<ConfigSpec> {
        CkselW::new(self, 3)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_l(&mut self) -> NoreloadLW<ConfigSpec> {
        NoreloadLW::new(self, 7)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&mut self) -> NoreloadHW<ConfigSpec> {
        NoreloadHW::new(self, 8)
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&mut self) -> InsyncW<ConfigSpec> {
        InsyncW::new(self, 9)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&mut self) -> AutolimitLW<ConfigSpec> {
        AutolimitLW::new(self, 17)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&mut self) -> AutolimitHW<ConfigSpec> {
        AutolimitHW::new(self, 18)
    }
}
#[doc = "SCT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x1e00"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x1e00;
}
