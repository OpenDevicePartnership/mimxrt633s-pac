#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Result FIFO Ready Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: Result FIFO data level not above watermark level."]
    Rdy0 = 0,
    #[doc = "1: Result FIFO holding data above watermark level."]
    Rdy1 = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Result FIFO Ready Flag"]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::Rdy0,
            true => Rdy::Rdy1,
        }
    }
    #[doc = "Result FIFO data level not above watermark level."]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        *self == Rdy::Rdy0
    }
    #[doc = "Result FIFO holding data above watermark level."]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        *self == Rdy::Rdy1
    }
}
#[doc = "Result FIFO Overflow Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fof {
    #[doc = "0: No result FIFO overflow has occurred since the last time the flag was cleared."]
    Fof0 = 0,
    #[doc = "1: At least one result FIFO overflow has occurred since the last time the flag was cleared."]
    Fof1 = 1,
}
impl From<Fof> for bool {
    #[inline(always)]
    fn from(variant: Fof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOF` reader - Result FIFO Overflow Flag"]
pub type FofR = crate::BitReader<Fof>;
impl FofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fof {
        match self.bits {
            false => Fof::Fof0,
            true => Fof::Fof1,
        }
    }
    #[doc = "No result FIFO overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fof_0(&self) -> bool {
        *self == Fof::Fof0
    }
    #[doc = "At least one result FIFO overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fof_1(&self) -> bool {
        *self == Fof::Fof1
    }
}
#[doc = "Field `FOF` writer - Result FIFO Overflow Flag"]
pub type FofW<'a, REG> = crate::BitWriter1C<'a, REG, Fof>;
impl<'a, REG> FofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result FIFO overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fof::Fof0)
    }
    #[doc = "At least one result FIFO overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fof::Fof1)
    }
}
#[doc = "Trigger Active\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgact {
    #[doc = "0: Command (sequence) associated with Trigger 0 currently being executed."]
    Trgact0 = 0,
    #[doc = "1: Command (sequence) associated with Trigger 1 currently being executed."]
    Trgact1 = 1,
    #[doc = "15: Command (sequence) associated with Trigger 15 currently being executed."]
    Trgact15 = 15,
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(variant: Trgact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgact {
    type Ux = u8;
}
impl crate::IsEnum for Trgact {}
#[doc = "Field `TRGACT` reader - Trigger Active"]
pub type TrgactR = crate::FieldReader<Trgact>;
impl TrgactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgact> {
        match self.bits {
            0 => Some(Trgact::Trgact0),
            1 => Some(Trgact::Trgact1),
            15 => Some(Trgact::Trgact15),
            _ => None,
        }
    }
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    #[inline(always)]
    pub fn is_trgact_0(&self) -> bool {
        *self == Trgact::Trgact0
    }
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    #[inline(always)]
    pub fn is_trgact_1(&self) -> bool {
        *self == Trgact::Trgact1
    }
    #[doc = "Command (sequence) associated with Trigger 15 currently being executed."]
    #[inline(always)]
    pub fn is_trgact_15(&self) -> bool {
        *self == Trgact::Trgact15
    }
}
#[doc = "Command Active\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdact {
    #[doc = "0: No command is currently in progress."]
    Cmdact0 = 0,
    #[doc = "1: Command 1 currently being executed."]
    Cmdact1 = 1,
    #[doc = "2: Command 2 currently being executed."]
    Cmdact2 = 2,
    #[doc = "15: Command 15 currently being executed."]
    Cmdact15 = 15,
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(variant: Cmdact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdact {
    type Ux = u8;
}
impl crate::IsEnum for Cmdact {}
#[doc = "Field `CMDACT` reader - Command Active"]
pub type CmdactR = crate::FieldReader<Cmdact>;
impl CmdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdact> {
        match self.bits {
            0 => Some(Cmdact::Cmdact0),
            1 => Some(Cmdact::Cmdact1),
            2 => Some(Cmdact::Cmdact2),
            15 => Some(Cmdact::Cmdact15),
            _ => None,
        }
    }
    #[doc = "No command is currently in progress."]
    #[inline(always)]
    pub fn is_cmdact_0(&self) -> bool {
        *self == Cmdact::Cmdact0
    }
    #[doc = "Command 1 currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_1(&self) -> bool {
        *self == Cmdact::Cmdact1
    }
    #[doc = "Command 2 currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_2(&self) -> bool {
        *self == Cmdact::Cmdact2
    }
    #[doc = "Command 15 currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_15(&self) -> bool {
        *self == Cmdact::Cmdact15
    }
}
impl R {
    #[doc = "Bit 0 - Result FIFO Ready Flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result FIFO Overflow Flag"]
    #[inline(always)]
    pub fn fof(&self) -> FofR {
        FofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Trigger Active"]
    #[inline(always)]
    pub fn trgact(&self) -> TrgactR {
        TrgactR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Active"]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("rdy", &self.rdy())
            .field("fof", &self.fof())
            .field("trgact", &self.trgact())
            .field("cmdact", &self.cmdact())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Result FIFO Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fof(&mut self) -> FofW<StatSpec> {
        FofW::new(self, 1)
    }
}
#[doc = "ADC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
