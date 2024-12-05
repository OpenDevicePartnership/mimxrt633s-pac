#[doc = "Register `FIFOCFG` reader"]
pub type R = crate::R<FifocfgSpec>;
#[doc = "Register `FIFOCFG` writer"]
pub type W = crate::W<FifocfgSpec>;
#[doc = "Enable the transmit FIFO.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enabletx {
    #[doc = "0: The transmit FIFO is not enabled."]
    Disabled = 0,
    #[doc = "1: The transmit FIFO is enabled."]
    Enabled = 1,
}
impl From<Enabletx> for bool {
    #[inline(always)]
    fn from(variant: Enabletx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLETX` reader - Enable the transmit FIFO."]
pub type EnabletxR = crate::BitReader<Enabletx>;
impl EnabletxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enabletx {
        match self.bits {
            false => Enabletx::Disabled,
            true => Enabletx::Enabled,
        }
    }
    #[doc = "The transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enabletx::Disabled
    }
    #[doc = "The transmit FIFO is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enabletx::Enabled
    }
}
#[doc = "Field `ENABLETX` writer - Enable the transmit FIFO."]
pub type EnabletxW<'a, REG> = crate::BitWriter<'a, REG, Enabletx>;
impl<'a, REG> EnabletxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabletx::Disabled)
    }
    #[doc = "The transmit FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabletx::Enabled)
    }
}
#[doc = "Enable the receive FIFO.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enablerx {
    #[doc = "0: The receive FIFO is not enabled."]
    Disabled = 0,
    #[doc = "1: The receive FIFO is enabled."]
    Enabled = 1,
}
impl From<Enablerx> for bool {
    #[inline(always)]
    fn from(variant: Enablerx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLERX` reader - Enable the receive FIFO."]
pub type EnablerxR = crate::BitReader<Enablerx>;
impl EnablerxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enablerx {
        match self.bits {
            false => Enablerx::Disabled,
            true => Enablerx::Enabled,
        }
    }
    #[doc = "The receive FIFO is not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enablerx::Disabled
    }
    #[doc = "The receive FIFO is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enablerx::Enabled
    }
}
#[doc = "Field `ENABLERX` writer - Enable the receive FIFO."]
pub type EnablerxW<'a, REG> = crate::BitWriter<'a, REG, Enablerx>;
impl<'a, REG> EnablerxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receive FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enablerx::Disabled)
    }
    #[doc = "The receive FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enablerx::Enabled)
    }
}
#[doc = "Field `SIZE` reader - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
pub type SizeR = crate::FieldReader;
#[doc = "DMA configuration for transmit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatx {
    #[doc = "0: DMA is not used for the transmit function."]
    Disabled = 0,
    #[doc = "1: Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    Enabled = 1,
}
impl From<Dmatx> for bool {
    #[inline(always)]
    fn from(variant: Dmatx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATX` reader - DMA configuration for transmit."]
pub type DmatxR = crate::BitReader<Dmatx>;
impl DmatxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatx {
        match self.bits {
            false => Dmatx::Disabled,
            true => Dmatx::Enabled,
        }
    }
    #[doc = "DMA is not used for the transmit function."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatx::Disabled
    }
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatx::Enabled
    }
}
#[doc = "Field `DMATX` writer - DMA configuration for transmit."]
pub type DmatxW<'a, REG> = crate::BitWriter<'a, REG, Dmatx>;
impl<'a, REG> DmatxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not used for the transmit function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatx::Disabled)
    }
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatx::Enabled)
    }
}
#[doc = "DMA configuration for receive.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarx {
    #[doc = "0: DMA is not used for the receive function."]
    Disabled = 0,
    #[doc = "1: Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    Enabled = 1,
}
impl From<Dmarx> for bool {
    #[inline(always)]
    fn from(variant: Dmarx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARX` reader - DMA configuration for receive."]
pub type DmarxR = crate::BitReader<Dmarx>;
impl DmarxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarx {
        match self.bits {
            false => Dmarx::Disabled,
            true => Dmarx::Enabled,
        }
    }
    #[doc = "DMA is not used for the receive function."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarx::Disabled
    }
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarx::Enabled
    }
}
#[doc = "Field `DMARX` writer - DMA configuration for receive."]
pub type DmarxW<'a, REG> = crate::BitWriter<'a, REG, Dmarx>;
impl<'a, REG> DmarxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not used for the receive function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarx::Disabled)
    }
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarx::Enabled)
    }
}
#[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waketx {
    #[doc = "0: Only enabled interrupts will wake up the device form reduced power modes."]
    Disabled = 0,
    #[doc = "1: A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    Enabled = 1,
}
impl From<Waketx> for bool {
    #[inline(always)]
    fn from(variant: Waketx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKETX` reader - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WaketxR = crate::BitReader<Waketx>;
impl WaketxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Waketx {
        match self.bits {
            false => Waketx::Disabled,
            true => Waketx::Enabled,
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Waketx::Disabled
    }
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Waketx::Enabled
    }
}
#[doc = "Field `WAKETX` writer - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WaketxW<'a, REG> = crate::BitWriter<'a, REG, Waketx>;
impl<'a, REG> WaketxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Waketx::Disabled)
    }
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Waketx::Enabled)
    }
}
#[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakerx {
    #[doc = "0: Only enabled interrupts will wake up the device form reduced power modes."]
    Disabled = 0,
    #[doc = "1: A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    Enabled = 1,
}
impl From<Wakerx> for bool {
    #[inline(always)]
    fn from(variant: Wakerx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKERX` reader - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WakerxR = crate::BitReader<Wakerx>;
impl WakerxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakerx {
        match self.bits {
            false => Wakerx::Disabled,
            true => Wakerx::Enabled,
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wakerx::Disabled
    }
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wakerx::Enabled
    }
}
#[doc = "Field `WAKERX` writer - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WakerxW<'a, REG> = crate::BitWriter<'a, REG, Wakerx>;
impl<'a, REG> WakerxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wakerx::Disabled)
    }
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wakerx::Enabled)
    }
}
#[doc = "Field `EMPTYTX` reader - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
pub type EmptytxR = crate::BitReader;
#[doc = "Field `EMPTYTX` writer - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
pub type EmptytxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTYRX` reader - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
pub type EmptyrxR = crate::BitReader;
#[doc = "Field `EMPTYRX` writer - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
pub type EmptyrxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&self) -> EnabletxR {
        EnabletxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&self) -> EnablerxR {
        EnablerxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&self) -> DmatxR {
        DmatxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&self) -> DmarxR {
        DmarxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&self) -> WaketxR {
        WaketxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&self) -> WakerxR {
        WakerxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&self) -> EmptytxR {
        EmptytxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&self) -> EmptyrxR {
        EmptyrxR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOCFG")
            .field("enabletx", &self.enabletx())
            .field("enablerx", &self.enablerx())
            .field("size", &self.size())
            .field("dmatx", &self.dmatx())
            .field("dmarx", &self.dmarx())
            .field("waketx", &self.waketx())
            .field("wakerx", &self.wakerx())
            .field("emptytx", &self.emptytx())
            .field("emptyrx", &self.emptyrx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&mut self) -> EnabletxW<FifocfgSpec> {
        EnabletxW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&mut self) -> EnablerxW<FifocfgSpec> {
        EnablerxW::new(self, 1)
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&mut self) -> DmatxW<FifocfgSpec> {
        DmatxW::new(self, 12)
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&mut self) -> DmarxW<FifocfgSpec> {
        DmarxW::new(self, 13)
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&mut self) -> WaketxW<FifocfgSpec> {
        WaketxW::new(self, 14)
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&mut self) -> WakerxW<FifocfgSpec> {
        WakerxW::new(self, 15)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&mut self) -> EmptytxW<FifocfgSpec> {
        EmptytxW::new(self, 16)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&mut self) -> EmptyrxW<FifocfgSpec> {
        EmptyrxW::new(self, 17)
    }
}
#[doc = "FIFO configuration and enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifocfgSpec;
impl crate::RegisterSpec for FifocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocfg::R`](R) reader structure"]
impl crate::Readable for FifocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fifocfg::W`](W) writer structure"]
impl crate::Writable for FifocfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCFG to value 0"]
impl crate::Resettable for FifocfgSpec {
    const RESET_VALUE: u32 = 0;
}
