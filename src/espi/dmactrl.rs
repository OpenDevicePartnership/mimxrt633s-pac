#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DmactrlSpec>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DmactrlSpec>;
#[doc = "Enables the DMA use on the 1st channel for eSPI and selects what triggers it\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0en {
    #[doc = "0: Disabled. The DMA channel is not used"]
    Disabled = 0,
    #[doc = "1: Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory"]
    TriggerdOnHostRead = 1,
    #[doc = "2: Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    TriggersOnHostWrite = 2,
}
impl From<Dma0en> for u8 {
    #[inline(always)]
    fn from(variant: Dma0en) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0en {
    type Ux = u8;
}
impl crate::IsEnum for Dma0en {}
#[doc = "Field `DMA0EN` reader - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
pub type Dma0enR = crate::FieldReader<Dma0en>;
impl Dma0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dma0en> {
        match self.bits {
            0 => Some(Dma0en::Disabled),
            1 => Some(Dma0en::TriggerdOnHostRead),
            2 => Some(Dma0en::TriggersOnHostWrite),
            _ => None,
        }
    }
    #[doc = "Disabled. The DMA channel is not used"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dma0en::Disabled
    }
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory"]
    #[inline(always)]
    pub fn is_triggerd_on_host_read(&self) -> bool {
        *self == Dma0en::TriggerdOnHostRead
    }
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    #[inline(always)]
    pub fn is_triggers_on_host_write(&self) -> bool {
        *self == Dma0en::TriggersOnHostWrite
    }
}
#[doc = "Field `DMA0EN` writer - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
pub type Dma0enW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma0en>;
impl<'a, REG> Dma0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled. The DMA channel is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0en::Disabled)
    }
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory"]
    #[inline(always)]
    pub fn triggerd_on_host_read(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0en::TriggerdOnHostRead)
    }
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    #[inline(always)]
    pub fn triggers_on_host_write(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0en::TriggersOnHostWrite)
    }
}
#[doc = "Enables the DMA use on the 1st channel for eSPI and selects what triggers it\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1en {
    #[doc = "0: Disabled. The DMA channel is not used"]
    Disabled = 0,
    #[doc = "1: Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory"]
    TriggerdOnHostRead = 1,
    #[doc = "2: Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    TriggersOnHostWrite = 2,
}
impl From<Dma1en> for u8 {
    #[inline(always)]
    fn from(variant: Dma1en) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1en {
    type Ux = u8;
}
impl crate::IsEnum for Dma1en {}
#[doc = "Field `DMA1EN` reader - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
pub type Dma1enR = crate::FieldReader<Dma1en>;
impl Dma1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dma1en> {
        match self.bits {
            0 => Some(Dma1en::Disabled),
            1 => Some(Dma1en::TriggerdOnHostRead),
            2 => Some(Dma1en::TriggersOnHostWrite),
            _ => None,
        }
    }
    #[doc = "Disabled. The DMA channel is not used"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dma1en::Disabled
    }
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory"]
    #[inline(always)]
    pub fn is_triggerd_on_host_read(&self) -> bool {
        *self == Dma1en::TriggerdOnHostRead
    }
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    #[inline(always)]
    pub fn is_triggers_on_host_write(&self) -> bool {
        *self == Dma1en::TriggersOnHostWrite
    }
}
#[doc = "Field `DMA1EN` writer - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
pub type Dma1enW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma1en>;
impl<'a, REG> Dma1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled. The DMA channel is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::Disabled)
    }
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory"]
    #[inline(always)]
    pub fn triggerd_on_host_read(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::TriggerdOnHostRead)
    }
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    #[inline(always)]
    pub fn triggers_on_host_write(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::TriggersOnHostWrite)
    }
}
#[doc = "Field `DMA0PORT` reader - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
pub type Dma0portR = crate::FieldReader;
#[doc = "Field `DMA0PORT` writer - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
pub type Dma0portW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMA1PORT` reader - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
pub type Dma1portR = crate::FieldReader;
#[doc = "Field `DMA1PORT` writer - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
pub type Dma1portW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNT0` reader - Used with Mailbox and Bus master to allow RAM contents to be copied"]
pub type Cnt0R = crate::FieldReader;
#[doc = "Field `CNT0` writer - Used with Mailbox and Bus master to allow RAM contents to be copied"]
pub type Cnt0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CNT1` reader - Used with Mailbox and Bus master to allow RAM contents to be copied"]
pub type Cnt1R = crate::FieldReader;
#[doc = "Field `CNT1` writer - Used with Mailbox and Bus master to allow RAM contents to be copied"]
pub type Cnt1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[inline(always)]
    pub fn dma0en(&self) -> Dma0enR {
        Dma0enR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[inline(always)]
    pub fn dma0port(&self) -> Dma0portR {
        Dma0portR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[inline(always)]
    pub fn dma1port(&self) -> Dma1portR {
        Dma1portR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[inline(always)]
    pub fn cnt0(&self) -> Cnt0R {
        Cnt0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[inline(always)]
    pub fn cnt1(&self) -> Cnt1R {
        Cnt1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTRL")
            .field("dma0en", &self.dma0en())
            .field("dma1en", &self.dma1en())
            .field("dma0port", &self.dma0port())
            .field("dma1port", &self.dma1port())
            .field("cnt0", &self.cnt0())
            .field("cnt1", &self.cnt1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[inline(always)]
    pub fn dma0en(&mut self) -> Dma0enW<DmactrlSpec> {
        Dma0enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<DmactrlSpec> {
        Dma1enW::new(self, 2)
    }
    #[doc = "Bits 8:11 - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[inline(always)]
    pub fn dma0port(&mut self) -> Dma0portW<DmactrlSpec> {
        Dma0portW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[inline(always)]
    pub fn dma1port(&mut self) -> Dma1portW<DmactrlSpec> {
        Dma1portW::new(self, 12)
    }
    #[doc = "Bits 16:22 - Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[inline(always)]
    pub fn cnt0(&mut self) -> Cnt0W<DmactrlSpec> {
        Cnt0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[inline(always)]
    pub fn cnt1(&mut self) -> Cnt1W<DmactrlSpec> {
        Cnt1W::new(self, 24)
    }
}
#[doc = "Selects DMA for Ports (if used)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DmactrlSpec {
    const RESET_VALUE: u32 = 0;
}
