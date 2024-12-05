#[doc = "Register `FIFO_CTRL` reader"]
pub type R = crate::R<FifoCtrlSpec>;
#[doc = "Register `FIFO_CTRL` writer"]
pub type W = crate::W<FifoCtrlSpec>;
#[doc = "FIFO enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    Disabled = 0,
    #[doc = "1: FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - FIFO enable."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - FIFO enable."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
#[doc = "FIFO reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetn {
    #[doc = "0: Reset the FIFO."]
    Reset = 0,
    #[doc = "1: Normal operation"]
    Normal = 1,
}
impl From<Resetn> for bool {
    #[inline(always)]
    fn from(variant: Resetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETN` reader - FIFO reset."]
pub type ResetnR = crate::BitReader<Resetn>;
impl ResetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetn {
        match self.bits {
            false => Resetn::Reset,
            true => Resetn::Normal,
        }
    }
    #[doc = "Reset the FIFO."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Resetn::Reset
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Resetn::Normal
    }
}
#[doc = "Field `RESETN` writer - FIFO reset."]
pub type ResetnW<'a, REG> = crate::BitWriter<'a, REG, Resetn>;
impl<'a, REG> ResetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the FIFO."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Resetn::Reset)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Resetn::Normal)
    }
}
#[doc = "Interrupt enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: FIFO level interrupts are not enabled."]
    Disabled = 0,
    #[doc = "1: FIFO level interrupts are enabled."]
    Enabled = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Interrupt enable."]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::Disabled,
            true => Inten::Enabled,
        }
    }
    #[doc = "FIFO level interrupts are not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inten::Disabled
    }
    #[doc = "FIFO level interrupts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inten::Enabled
    }
}
#[doc = "Field `INTEN` writer - Interrupt enable."]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO level interrupts are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Disabled)
    }
    #[doc = "FIFO level interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Enabled)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA requests are not enabled."]
    Disabled = 0,
    #[doc = "1: DMA requests based on FIFO level are enabled."]
    Enabled = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disabled,
            true => Dmaen::Enabled,
        }
    }
    #[doc = "DMA requests are not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaen::Disabled
    }
    #[doc = "DMA requests based on FIFO level are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaen::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA requests are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disabled)
    }
    #[doc = "DMA requests based on FIFO level are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enabled)
    }
}
#[doc = "Field `TRIGLVL` reader - Trigger level for interrupt"]
pub type TriglvlR = crate::FieldReader;
#[doc = "Field `TRIGLVL` writer - Trigger level for interrupt"]
pub type TriglvlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&self) -> ResetnR {
        ResetnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Trigger level for interrupt"]
    #[inline(always)]
    pub fn triglvl(&self) -> TriglvlR {
        TriglvlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CTRL")
            .field("enable", &self.enable())
            .field("resetn", &self.resetn())
            .field("inten", &self.inten())
            .field("dmaen", &self.dmaen())
            .field("triglvl", &self.triglvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<FifoCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&mut self) -> ResetnW<FifoCtrlSpec> {
        ResetnW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<FifoCtrlSpec> {
        IntenW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<FifoCtrlSpec> {
        DmaenW::new(self, 3)
    }
    #[doc = "Bits 16:20 - Trigger level for interrupt"]
    #[inline(always)]
    pub fn triglvl(&mut self) -> TriglvlW<FifoCtrlSpec> {
        TriglvlW::new(self, 16)
    }
}
#[doc = "FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoCtrlSpec;
impl crate::RegisterSpec for FifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for FifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for FifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_CTRL to value 0"]
impl crate::Resettable for FifoCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
