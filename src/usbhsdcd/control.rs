#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Interrupt Acknowledge\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iack {
    #[doc = "0: Do not clear the interrupt."]
    IntNoclear = 0,
    #[doc = "1: Clear the IF bit (interrupt flag)."]
    IntClear = 1,
}
impl From<Iack> for bool {
    #[inline(always)]
    fn from(variant: Iack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IACK` reader - Interrupt Acknowledge"]
pub type IackR = crate::BitReader<Iack>;
impl IackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iack {
        match self.bits {
            false => Iack::IntNoclear,
            true => Iack::IntClear,
        }
    }
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn is_int_noclear(&self) -> bool {
        *self == Iack::IntNoclear
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline(always)]
    pub fn is_int_clear(&self) -> bool {
        *self == Iack::IntClear
    }
}
#[doc = "Field `IACK` writer - Interrupt Acknowledge"]
pub type IackW<'a, REG> = crate::BitWriter<'a, REG, Iack>;
impl<'a, REG> IackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn int_noclear(self) -> &'a mut crate::W<REG> {
        self.variant(Iack::IntNoclear)
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline(always)]
    pub fn int_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Iack::IntClear)
    }
}
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum If {
    #[doc = "0: No interrupt is pending."]
    IntPend = 0,
    #[doc = "1: An interrupt is pending."]
    IntNopend = 1,
}
impl From<If> for bool {
    #[inline(always)]
    fn from(variant: If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF` reader - Interrupt Flag"]
pub type IfR = crate::BitReader<If>;
impl IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> If {
        match self.bits {
            false => If::IntPend,
            true => If::IntNopend,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_int_pend(&self) -> bool {
        *self == If::IntPend
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_int_nopend(&self) -> bool {
        *self == If::IntNopend
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ie {
    #[doc = "0: Disable interrupts to the system."]
    DisInt = 0,
    #[doc = "1: Enable interrupts to the system."]
    EnInt = 1,
}
impl From<Ie> for bool {
    #[inline(always)]
    fn from(variant: Ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IeR = crate::BitReader<Ie>;
impl IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ie {
        match self.bits {
            false => Ie::DisInt,
            true => Ie::EnInt,
        }
    }
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn is_dis_int(&self) -> bool {
        *self == Ie::DisInt
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn is_en_int(&self) -> bool {
        *self == Ie::EnInt
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG, Ie>;
impl<'a, REG> IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn dis_int(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::DisInt)
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn en_int(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::EnInt)
    }
}
#[doc = "BC12\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc12 {
    #[doc = "0: Compatible with BC1.1 (default)"]
    Bc11 = 0,
    #[doc = "1: Compatible with BC1.2"]
    Bc12 = 1,
}
impl From<Bc12> for bool {
    #[inline(always)]
    fn from(variant: Bc12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC12` reader - BC12"]
pub type Bc12R = crate::BitReader<Bc12>;
impl Bc12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc12 {
        match self.bits {
            false => Bc12::Bc11,
            true => Bc12::Bc12,
        }
    }
    #[doc = "Compatible with BC1.1 (default)"]
    #[inline(always)]
    pub fn is_bc11(&self) -> bool {
        *self == Bc12::Bc11
    }
    #[doc = "Compatible with BC1.2"]
    #[inline(always)]
    pub fn is_bc12(&self) -> bool {
        *self == Bc12::Bc12
    }
}
#[doc = "Field `BC12` writer - BC12"]
pub type Bc12W<'a, REG> = crate::BitWriter<'a, REG, Bc12>;
impl<'a, REG> Bc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compatible with BC1.1 (default)"]
    #[inline(always)]
    pub fn bc11(self) -> &'a mut crate::W<REG> {
        self.variant(Bc12::Bc11)
    }
    #[doc = "Compatible with BC1.2"]
    #[inline(always)]
    pub fn bc12(self) -> &'a mut crate::W<REG> {
        self.variant(Bc12::Bc12)
    }
}
#[doc = "Start Change Detection Sequence\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Do not start the sequence. Writes of this value have no effect."]
    NoStart = 0,
    #[doc = "1: Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    Start = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start Change Detection Sequence"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::NoStart,
            true => Start::Start,
        }
    }
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == Start::NoStart
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Start::Start
    }
}
#[doc = "Field `START` writer - Start Change Detection Sequence"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::NoStart)
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr {
    #[doc = "0: Do not perform a software reset."]
    NoReset = 0,
    #[doc = "1: Perform a software reset."]
    SwReset = 1,
}
impl From<Sr> for bool {
    #[inline(always)]
    fn from(variant: Sr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Software Reset"]
pub type SrR = crate::BitReader<Sr>;
impl SrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr {
        match self.bits {
            false => Sr::NoReset,
            true => Sr::SwReset,
        }
    }
    #[doc = "Do not perform a software reset."]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == Sr::NoReset
    }
    #[doc = "Perform a software reset."]
    #[inline(always)]
    pub fn is_sw_reset(&self) -> bool {
        *self == Sr::SwReset
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG, Sr>;
impl<'a, REG> SrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not perform a software reset."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::NoReset)
    }
    #[doc = "Perform a software reset."]
    #[inline(always)]
    pub fn sw_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::SwReset)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    pub fn iack(&self) -> IackR {
        IackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IfR {
        IfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BC12"]
    #[inline(always)]
    pub fn bc12(&self) -> Bc12R {
        Bc12R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field("iack", &self.iack())
            .field("if_", &self.if_())
            .field("ie", &self.ie())
            .field("bc12", &self.bc12())
            .field("start", &self.start())
            .field("sr", &self.sr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn iack(&mut self) -> IackW<ControlSpec> {
        IackW::new(self, 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<ControlSpec> {
        IeW::new(self, 16)
    }
    #[doc = "Bit 17 - BC12"]
    #[inline(always)]
    #[must_use]
    pub fn bc12(&mut self) -> Bc12W<ControlSpec> {
        Bc12W::new(self, 17)
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<ControlSpec> {
        StartW::new(self, 24)
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<ControlSpec> {
        SrW::new(self, 25)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0x0001_0000"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
