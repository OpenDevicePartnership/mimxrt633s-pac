#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `SSA` writer - Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
pub type SsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSD` writer - Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
pub type SsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED` reader - Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
pub type StalledR = crate::BitReader;
#[doc = "Field `ENDTRANSFER` reader - End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
pub type EndtransferR = crate::BitReader;
#[doc = "Field `ENDTRANSFER` writer - End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
pub type EndtransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTIDLE` reader - Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
pub type MstidleR = crate::BitReader;
impl R {
    #[doc = "Bit 6 - Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[inline(always)]
    pub fn stalled(&self) -> StalledR {
        StalledR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[inline(always)]
    pub fn endtransfer(&self) -> EndtransferR {
        EndtransferR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[inline(always)]
    pub fn mstidle(&self) -> MstidleR {
        MstidleR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("stalled", &self.stalled())
            .field("endtransfer", &self.endtransfer())
            .field("mstidle", &self.mstidle())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ssa(&mut self) -> SsaW<StatSpec> {
        SsaW::new(self, 4)
    }
    #[doc = "Bit 5 - Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SsdW<StatSpec> {
        SsdW::new(self, 5)
    }
    #[doc = "Bit 7 - End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[inline(always)]
    #[must_use]
    pub fn endtransfer(&mut self) -> EndtransferW<StatSpec> {
        EndtransferW::new(self, 7)
    }
}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0100"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0100;
}
