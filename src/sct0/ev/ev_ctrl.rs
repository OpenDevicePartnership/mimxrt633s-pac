#[doc = "Register `EV_CTRL` reader"]
pub type R = crate::R<EvCtrlSpec>;
#[doc = "Register `EV_CTRL` writer"]
pub type W = crate::W<EvCtrlSpec>;
#[doc = "Field `MATCHSEL` reader - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
pub type MatchselR = crate::FieldReader;
#[doc = "Field `MATCHSEL` writer - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
pub type MatchselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Select L/H counter. Do not set this bit if UNIFY = 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hevent {
    #[doc = "0: Selects the L state and the L match register selected by MATCHSEL."]
    LCounter = 0,
    #[doc = "1: Selects the H state and the H match register selected by MATCHSEL."]
    HCounter = 1,
}
impl From<Hevent> for bool {
    #[inline(always)]
    fn from(variant: Hevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEVENT` reader - Select L/H counter. Do not set this bit if UNIFY = 1."]
pub type HeventR = crate::BitReader<Hevent>;
impl HeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hevent {
        match self.bits {
            false => Hevent::LCounter,
            true => Hevent::HCounter,
        }
    }
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn is_l_counter(&self) -> bool {
        *self == Hevent::LCounter
    }
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn is_h_counter(&self) -> bool {
        *self == Hevent::HCounter
    }
}
#[doc = "Field `HEVENT` writer - Select L/H counter. Do not set this bit if UNIFY = 1."]
pub type HeventW<'a, REG> = crate::BitWriter<'a, REG, Hevent>;
impl<'a, REG> HeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn l_counter(self) -> &'a mut crate::W<REG> {
        self.variant(Hevent::LCounter)
    }
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn h_counter(self) -> &'a mut crate::W<REG> {
        self.variant(Hevent::HCounter)
    }
}
#[doc = "Input/output select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outsel {
    #[doc = "0: Selects the inputs selected by IOSEL."]
    Input = 0,
    #[doc = "1: Selects the outputs selected by IOSEL."]
    Output = 1,
}
impl From<Outsel> for bool {
    #[inline(always)]
    fn from(variant: Outsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` reader - Input/output select"]
pub type OutselR = crate::BitReader<Outsel>;
impl OutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outsel {
        match self.bits {
            false => Outsel::Input,
            true => Outsel::Output,
        }
    }
    #[doc = "Selects the inputs selected by IOSEL."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Outsel::Input
    }
    #[doc = "Selects the outputs selected by IOSEL."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Outsel::Output
    }
}
#[doc = "Field `OUTSEL` writer - Input/output select"]
pub type OutselW<'a, REG> = crate::BitWriter<'a, REG, Outsel>;
impl<'a, REG> OutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects the inputs selected by IOSEL."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Outsel::Input)
    }
    #[doc = "Selects the outputs selected by IOSEL."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Outsel::Output)
    }
}
#[doc = "Field `IOSEL` reader - Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
pub type IoselR = crate::FieldReader;
#[doc = "Field `IOSEL` writer - Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
pub type IoselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iocond {
    #[doc = "0: LOW"]
    Low = 0,
    #[doc = "1: Rise"]
    Rise = 1,
    #[doc = "2: Fall"]
    Fall = 2,
    #[doc = "3: HIGH"]
    High = 3,
}
impl From<Iocond> for u8 {
    #[inline(always)]
    fn from(variant: Iocond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iocond {
    type Ux = u8;
}
impl crate::IsEnum for Iocond {}
#[doc = "Field `IOCOND` reader - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
pub type IocondR = crate::FieldReader<Iocond>;
impl IocondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocond {
        match self.bits {
            0 => Iocond::Low,
            1 => Iocond::Rise,
            2 => Iocond::Fall,
            3 => Iocond::High,
            _ => unreachable!(),
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iocond::Low
    }
    #[doc = "Rise"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Iocond::Rise
    }
    #[doc = "Fall"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Iocond::Fall
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iocond::High
    }
}
#[doc = "Field `IOCOND` writer - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
pub type IocondW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iocond, crate::Safe>;
impl<'a, REG> IocondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iocond::Low)
    }
    #[doc = "Rise"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Iocond::Rise)
    }
    #[doc = "Fall"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Iocond::Fall)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iocond::High)
    }
}
#[doc = "Selects how the specified match and I/O condition are used and combined.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Combmode {
    #[doc = "0: OR. The event occurs when either the specified match or I/O condition occurs."]
    Or = 0,
    #[doc = "1: MATCH. Uses the specified match only."]
    Match = 1,
    #[doc = "2: IO. Uses the specified I/O condition only."]
    Io = 2,
    #[doc = "3: AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    And = 3,
}
impl From<Combmode> for u8 {
    #[inline(always)]
    fn from(variant: Combmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Combmode {
    type Ux = u8;
}
impl crate::IsEnum for Combmode {}
#[doc = "Field `COMBMODE` reader - Selects how the specified match and I/O condition are used and combined."]
pub type CombmodeR = crate::FieldReader<Combmode>;
impl CombmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Combmode {
        match self.bits {
            0 => Combmode::Or,
            1 => Combmode::Match,
            2 => Combmode::Io,
            3 => Combmode::And,
            _ => unreachable!(),
        }
    }
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == Combmode::Or
    }
    #[doc = "MATCH. Uses the specified match only."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Combmode::Match
    }
    #[doc = "IO. Uses the specified I/O condition only."]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Combmode::Io
    }
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == Combmode::And
    }
}
#[doc = "Field `COMBMODE` writer - Selects how the specified match and I/O condition are used and combined."]
pub type CombmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Combmode, crate::Safe>;
impl<'a, REG> CombmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    #[inline(always)]
    pub fn or(self) -> &'a mut crate::W<REG> {
        self.variant(Combmode::Or)
    }
    #[doc = "MATCH. Uses the specified match only."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Combmode::Match)
    }
    #[doc = "IO. Uses the specified I/O condition only."]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Combmode::Io)
    }
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    #[inline(always)]
    pub fn and(self) -> &'a mut crate::W<REG> {
        self.variant(Combmode::And)
    }
}
#[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateld {
    #[doc = "0: STATEV value is added into STATE (the carry-out is ignored)."]
    Add = 0,
    #[doc = "1: STATEV value is loaded into STATE."]
    Load = 1,
}
impl From<Stateld> for bool {
    #[inline(always)]
    fn from(variant: Stateld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATELD` reader - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
pub type StateldR = crate::BitReader<Stateld>;
impl StateldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateld {
        match self.bits {
            false => Stateld::Add,
            true => Stateld::Load,
        }
    }
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == Stateld::Add
    }
    #[doc = "STATEV value is loaded into STATE."]
    #[inline(always)]
    pub fn is_load(&self) -> bool {
        *self == Stateld::Load
    }
}
#[doc = "Field `STATELD` writer - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
pub type StateldW<'a, REG> = crate::BitWriter<'a, REG, Stateld>;
impl<'a, REG> StateldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    #[inline(always)]
    pub fn add(self) -> &'a mut crate::W<REG> {
        self.variant(Stateld::Add)
    }
    #[doc = "STATEV value is loaded into STATE."]
    #[inline(always)]
    pub fn load(self) -> &'a mut crate::W<REG> {
        self.variant(Stateld::Load)
    }
}
#[doc = "Field `STATEV` reader - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
pub type StatevR = crate::FieldReader;
#[doc = "Field `STATEV` writer - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
pub type StatevW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MATCHMEM` reader - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
pub type MatchmemR = crate::BitReader;
#[doc = "Field `MATCHMEM` writer - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
pub type MatchmemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    #[doc = "0: Direction independent. This event is triggered regardless of the count direction."]
    DirectionIndependent = 0,
    #[doc = "1: Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    CountingUp = 1,
    #[doc = "2: Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    CountingDown = 2,
}
impl From<Direction> for u8 {
    #[inline(always)]
    fn from(variant: Direction) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Direction {
    type Ux = u8;
}
impl crate::IsEnum for Direction {}
#[doc = "Field `DIRECTION` reader - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
pub type DirectionR = crate::FieldReader<Direction>;
impl DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Direction> {
        match self.bits {
            0 => Some(Direction::DirectionIndependent),
            1 => Some(Direction::CountingUp),
            2 => Some(Direction::CountingDown),
            _ => None,
        }
    }
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    #[inline(always)]
    pub fn is_direction_independent(&self) -> bool {
        *self == Direction::DirectionIndependent
    }
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn is_counting_up(&self) -> bool {
        *self == Direction::CountingUp
    }
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn is_counting_down(&self) -> bool {
        *self == Direction::CountingDown
    }
}
#[doc = "Field `DIRECTION` writer - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
pub type DirectionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Direction>;
impl<'a, REG> DirectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    #[inline(always)]
    pub fn direction_independent(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::DirectionIndependent)
    }
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_up(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::CountingUp)
    }
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_down(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::CountingDown)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub fn matchsel(&self) -> MatchselR {
        MatchselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub fn hevent(&self) -> HeventR {
        HeventR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline(always)]
    pub fn outsel(&self) -> OutselR {
        OutselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub fn iosel(&self) -> IoselR {
        IoselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub fn iocond(&self) -> IocondR {
        IocondR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub fn combmode(&self) -> CombmodeR {
        CombmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub fn stateld(&self) -> StateldR {
        StateldR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub fn statev(&self) -> StatevR {
        StatevR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub fn matchmem(&self) -> MatchmemR {
        MatchmemR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 21) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EV_CTRL")
            .field("matchsel", &self.matchsel())
            .field("hevent", &self.hevent())
            .field("outsel", &self.outsel())
            .field("iosel", &self.iosel())
            .field("iocond", &self.iocond())
            .field("combmode", &self.combmode())
            .field("stateld", &self.stateld())
            .field("statev", &self.statev())
            .field("matchmem", &self.matchmem())
            .field("direction", &self.direction())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub fn matchsel(&mut self) -> MatchselW<EvCtrlSpec> {
        MatchselW::new(self, 0)
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub fn hevent(&mut self) -> HeventW<EvCtrlSpec> {
        HeventW::new(self, 4)
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OutselW<EvCtrlSpec> {
        OutselW::new(self, 5)
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub fn iosel(&mut self) -> IoselW<EvCtrlSpec> {
        IoselW::new(self, 6)
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub fn iocond(&mut self) -> IocondW<EvCtrlSpec> {
        IocondW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub fn combmode(&mut self) -> CombmodeW<EvCtrlSpec> {
        CombmodeW::new(self, 12)
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub fn stateld(&mut self) -> StateldW<EvCtrlSpec> {
        StateldW::new(self, 14)
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub fn statev(&mut self) -> StatevW<EvCtrlSpec> {
        StatevW::new(self, 15)
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub fn matchmem(&mut self) -> MatchmemW<EvCtrlSpec> {
        MatchmemW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn direction(&mut self) -> DirectionW<EvCtrlSpec> {
        DirectionW::new(self, 21)
    }
}
#[doc = "SCT event control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvCtrlSpec;
impl crate::RegisterSpec for EvCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_ctrl::R`](R) reader structure"]
impl crate::Readable for EvCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_ctrl::W`](W) writer structure"]
impl crate::Writable for EvCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EV_CTRL to value 0"]
impl crate::Resettable for EvCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
