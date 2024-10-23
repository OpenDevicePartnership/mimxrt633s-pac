#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CtcrSpec>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CtcrSpec>;
#[doc = "Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctmode {
    #[doc = "0: Timer Mode. Incremented every rising APB bus clock edge."]
    Timer = 0,
    #[doc = "1: Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    CounterRisingEdge = 1,
    #[doc = "2: Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    CounterFallingEdge = 2,
    #[doc = "3: Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    CounterDualEdge = 3,
}
impl From<Ctmode> for u8 {
    #[inline(always)]
    fn from(variant: Ctmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctmode {
    type Ux = u8;
}
impl crate::IsEnum for Ctmode {}
#[doc = "Field `CTMODE` reader - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CtmodeR = crate::FieldReader<Ctmode>;
impl CtmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctmode {
        match self.bits {
            0 => Ctmode::Timer,
            1 => Ctmode::CounterRisingEdge,
            2 => Ctmode::CounterFallingEdge,
            3 => Ctmode::CounterDualEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Ctmode::Timer
    }
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_counter_rising_edge(&self) -> bool {
        *self == Ctmode::CounterRisingEdge
    }
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_counter_falling_edge(&self) -> bool {
        *self == Ctmode::CounterFallingEdge
    }
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_counter_dual_edge(&self) -> bool {
        *self == Ctmode::CounterDualEdge
    }
}
#[doc = "Field `CTMODE` writer - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CtmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctmode, crate::Safe>;
impl<'a, REG> CtmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::Timer)
    }
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::CounterRisingEdge)
    }
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::CounterFallingEdge)
    }
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_dual_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::CounterDualEdge)
    }
}
#[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cinsel {
    #[doc = "0: Channel 0. CAPn.0 for CTIMERn"]
    Channel0 = 0,
    #[doc = "1: Channel 1. CAPn.1 for CTIMERn"]
    Channel1 = 1,
    #[doc = "2: Channel 2. CAPn.2 for CTIMERn"]
    Channel2 = 2,
    #[doc = "3: Channel 3. CAPn.3 for CTIMERn"]
    Channel3 = 3,
}
impl From<Cinsel> for u8 {
    #[inline(always)]
    fn from(variant: Cinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cinsel {
    type Ux = u8;
}
impl crate::IsEnum for Cinsel {}
#[doc = "Field `CINSEL` reader - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CinselR = crate::FieldReader<Cinsel>;
impl CinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinsel {
        match self.bits {
            0 => Cinsel::Channel0,
            1 => Cinsel::Channel1,
            2 => Cinsel::Channel2,
            3 => Cinsel::Channel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    #[inline(always)]
    pub fn is_channel_0(&self) -> bool {
        *self == Cinsel::Channel0
    }
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    #[inline(always)]
    pub fn is_channel_1(&self) -> bool {
        *self == Cinsel::Channel1
    }
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    #[inline(always)]
    pub fn is_channel_2(&self) -> bool {
        *self == Cinsel::Channel2
    }
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    #[inline(always)]
    pub fn is_channel_3(&self) -> bool {
        *self == Cinsel::Channel3
    }
}
#[doc = "Field `CINSEL` writer - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CinselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cinsel, crate::Safe>;
impl<'a, REG> CinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    #[inline(always)]
    pub fn channel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsel::Channel0)
    }
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    #[inline(always)]
    pub fn channel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsel::Channel1)
    }
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    #[inline(always)]
    pub fn channel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsel::Channel2)
    }
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    #[inline(always)]
    pub fn channel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsel::Channel3)
    }
}
#[doc = "Field `ENCC` reader - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type EnccR = crate::BitReader;
#[doc = "Field `ENCC` writer - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type EnccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selcc {
    #[doc = "0: Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    Channel0Rising = 0,
    #[doc = "1: Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    Channel0Falling = 1,
    #[doc = "2: Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    Channel1Rising = 2,
    #[doc = "3: Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    Channel1Falling = 3,
    #[doc = "4: Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    Channel2Rising = 4,
    #[doc = "5: Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    Channel2Falling = 5,
}
impl From<Selcc> for u8 {
    #[inline(always)]
    fn from(variant: Selcc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selcc {
    type Ux = u8;
}
impl crate::IsEnum for Selcc {}
#[doc = "Field `SELCC` reader - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
pub type SelccR = crate::FieldReader<Selcc>;
impl SelccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selcc> {
        match self.bits {
            0 => Some(Selcc::Channel0Rising),
            1 => Some(Selcc::Channel0Falling),
            2 => Some(Selcc::Channel1Rising),
            3 => Some(Selcc::Channel1Falling),
            4 => Some(Selcc::Channel2Rising),
            5 => Some(Selcc::Channel2Falling),
            _ => None,
        }
    }
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn is_channel_0_rising(&self) -> bool {
        *self == Selcc::Channel0Rising
    }
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn is_channel_0_falling(&self) -> bool {
        *self == Selcc::Channel0Falling
    }
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn is_channel_1_rising(&self) -> bool {
        *self == Selcc::Channel1Rising
    }
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn is_channel_1_falling(&self) -> bool {
        *self == Selcc::Channel1Falling
    }
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn is_channel_2_rising(&self) -> bool {
        *self == Selcc::Channel2Rising
    }
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn is_channel_2_falling(&self) -> bool {
        *self == Selcc::Channel2Falling
    }
}
#[doc = "Field `SELCC` writer - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
pub type SelccW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selcc>;
impl<'a, REG> SelccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_0_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Channel0Rising)
    }
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_0_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Channel0Falling)
    }
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_1_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Channel1Rising)
    }
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_1_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Channel1Falling)
    }
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_2_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Channel2Rising)
    }
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_2_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Selcc::Channel2Falling)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CtmodeR {
        CtmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CinselR {
        CinselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> EnccR {
        EnccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn selcc(&self) -> SelccR {
        SelccR::new(((self.bits >> 5) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTCR")
            .field("ctmode", &self.ctmode())
            .field("cinsel", &self.cinsel())
            .field("encc", &self.encc())
            .field("selcc", &self.selcc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    #[must_use]
    pub fn ctmode(&mut self) -> CtmodeW<CtcrSpec> {
        CtmodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    #[must_use]
    pub fn cinsel(&mut self) -> CinselW<CtcrSpec> {
        CinselW::new(self, 2)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    #[must_use]
    pub fn encc(&mut self) -> EnccW<CtcrSpec> {
        EnccW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn selcc(&mut self) -> SelccW<CtcrSpec> {
        SelccW::new(self, 5)
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcrSpec;
impl crate::RegisterSpec for CtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctcr::R`](R) reader structure"]
impl crate::Readable for CtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctcr::W`](W) writer structure"]
impl crate::Writable for CtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CtcrSpec {
    const RESET_VALUE: u32 = 0;
}
