#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `IPCMDDONE` reader - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
pub type IpcmddoneR = crate::BitReader;
#[doc = "Field `IPCMDDONE` writer - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
pub type IpcmddoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IPCMDGE` reader - IP triggered Command Sequences Grant Timeout interrupt."]
pub type IpcmdgeR = crate::BitReader;
#[doc = "Field `IPCMDGE` writer - IP triggered Command Sequences Grant Timeout interrupt."]
pub type IpcmdgeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AHBCMDGE` reader - AHB triggered Command Sequences Grant Timeout interrupt."]
pub type AhbcmdgeR = crate::BitReader;
#[doc = "Field `AHBCMDGE` writer - AHB triggered Command Sequences Grant Timeout interrupt."]
pub type AhbcmdgeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IPCMDERR` reader - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
pub type IpcmderrR = crate::BitReader;
#[doc = "Field `IPCMDERR` writer - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
pub type IpcmderrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AHBCMDERR` reader - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
pub type AhbcmderrR = crate::BitReader;
#[doc = "Field `AHBCMDERR` writer - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
pub type AhbcmderrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IPRXWA` reader - IP RX FIFO watermark available interrupt."]
pub type IprxwaR = crate::BitReader;
#[doc = "Field `IPRXWA` writer - IP RX FIFO watermark available interrupt."]
pub type IprxwaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IPTXWE` reader - IP TX FIFO watermark empty interrupt."]
pub type IptxweR = crate::BitReader;
#[doc = "Field `IPTXWE` writer - IP TX FIFO watermark empty interrupt."]
pub type IptxweW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DATALEARNFAIL` reader - Data Learning failed interrupt."]
pub type DatalearnfailR = crate::BitReader;
#[doc = "Field `DATALEARNFAIL` writer - Data Learning failed interrupt."]
pub type DatalearnfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCKSTOPBYRD` reader - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
pub type SckstopbyrdR = crate::BitReader;
#[doc = "Field `SCKSTOPBYRD` writer - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
pub type SckstopbyrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCKSTOPBYWR` reader - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
pub type SckstopbywrR = crate::BitReader;
#[doc = "Field `SCKSTOPBYWR` writer - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
pub type SckstopbywrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AHBBUSTIMEOUT` reader - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AhbbustimeoutR = crate::BitReader;
#[doc = "Field `AHBBUSTIMEOUT` writer - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AhbbustimeoutW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SEQTIMEOUT` reader - Sequence execution timeout interrupt."]
pub type SeqtimeoutR = crate::BitReader;
#[doc = "Field `SEQTIMEOUT` writer - Sequence execution timeout interrupt."]
pub type SeqtimeoutW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IpcmddoneR {
        IpcmddoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ipcmdge(&self) -> IpcmdgeR {
        IpcmdgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ahbcmdge(&self) -> AhbcmdgeR {
        AhbcmdgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IpcmderrR {
        IpcmderrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ahbcmderr(&self) -> AhbcmderrR {
        AhbcmderrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub fn iprxwa(&self) -> IprxwaR {
        IprxwaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub fn iptxwe(&self) -> IptxweR {
        IptxweR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Learning failed interrupt."]
    #[inline(always)]
    pub fn datalearnfail(&self) -> DatalearnfailR {
        DatalearnfailR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub fn sckstopbyrd(&self) -> SckstopbyrdR {
        SckstopbyrdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub fn sckstopbywr(&self) -> SckstopbywrR {
        SckstopbywrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeout(&self) -> AhbbustimeoutR {
        AhbbustimeoutR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt."]
    #[inline(always)]
    pub fn seqtimeout(&self) -> SeqtimeoutR {
        SeqtimeoutR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR")
            .field("ipcmddone", &self.ipcmddone())
            .field("ipcmdge", &self.ipcmdge())
            .field("ahbcmdge", &self.ahbcmdge())
            .field("ipcmderr", &self.ipcmderr())
            .field("ahbcmderr", &self.ahbcmderr())
            .field("iprxwa", &self.iprxwa())
            .field("iptxwe", &self.iptxwe())
            .field("datalearnfail", &self.datalearnfail())
            .field("sckstopbyrd", &self.sckstopbyrd())
            .field("sckstopbywr", &self.sckstopbywr())
            .field("ahbbustimeout", &self.ahbbustimeout())
            .field("seqtimeout", &self.seqtimeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub fn ipcmddone(&mut self) -> IpcmddoneW<IntrSpec> {
        IpcmddoneW::new(self, 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ipcmdge(&mut self) -> IpcmdgeW<IntrSpec> {
        IpcmdgeW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ahbcmdge(&mut self) -> AhbcmdgeW<IntrSpec> {
        AhbcmdgeW::new(self, 2)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ipcmderr(&mut self) -> IpcmderrW<IntrSpec> {
        IpcmderrW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ahbcmderr(&mut self) -> AhbcmderrW<IntrSpec> {
        AhbcmderrW::new(self, 4)
    }
    #[doc = "Bit 5 - IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub fn iprxwa(&mut self) -> IprxwaW<IntrSpec> {
        IprxwaW::new(self, 5)
    }
    #[doc = "Bit 6 - IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub fn iptxwe(&mut self) -> IptxweW<IntrSpec> {
        IptxweW::new(self, 6)
    }
    #[doc = "Bit 7 - Data Learning failed interrupt."]
    #[inline(always)]
    pub fn datalearnfail(&mut self) -> DatalearnfailW<IntrSpec> {
        DatalearnfailW::new(self, 7)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub fn sckstopbyrd(&mut self) -> SckstopbyrdW<IntrSpec> {
        SckstopbyrdW::new(self, 8)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub fn sckstopbywr(&mut self) -> SckstopbywrW<IntrSpec> {
        SckstopbywrW::new(self, 9)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeout(&mut self) -> AhbbustimeoutW<IntrSpec> {
        AhbbustimeoutW::new(self, 10)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt."]
    #[inline(always)]
    pub fn seqtimeout(&mut self) -> SeqtimeoutW<IntrSpec> {
        SeqtimeoutW::new(self, 11)
    }
}
#[doc = "Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
