#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `IPCMDDONEEN` reader - IP triggered Command Sequences Execution finished interrupt enable."]
pub type IpcmddoneenR = crate::BitReader;
#[doc = "Field `IPCMDDONEEN` writer - IP triggered Command Sequences Execution finished interrupt enable."]
pub type IpcmddoneenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCMDGEEN` reader - IP triggered Command Sequences Grant Timeout interrupt enable."]
pub type IpcmdgeenR = crate::BitReader;
#[doc = "Field `IPCMDGEEN` writer - IP triggered Command Sequences Grant Timeout interrupt enable."]
pub type IpcmdgeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBCMDGEEN` reader - AHB triggered Command Sequences Grant Timeout interrupt enable."]
pub type AhbcmdgeenR = crate::BitReader;
#[doc = "Field `AHBCMDGEEN` writer - AHB triggered Command Sequences Grant Timeout interrupt enable."]
pub type AhbcmdgeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCMDERREN` reader - IP triggered Command Sequences Error Detected interrupt enable."]
pub type IpcmderrenR = crate::BitReader;
#[doc = "Field `IPCMDERREN` writer - IP triggered Command Sequences Error Detected interrupt enable."]
pub type IpcmderrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBCMDERREN` reader - AHB triggered Command Sequences Error Detected interrupt enable."]
pub type AhbcmderrenR = crate::BitReader;
#[doc = "Field `AHBCMDERREN` writer - AHB triggered Command Sequences Error Detected interrupt enable."]
pub type AhbcmderrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPRXWAEN` reader - IP RX FIFO WaterMark available interrupt enable."]
pub type IprxwaenR = crate::BitReader;
#[doc = "Field `IPRXWAEN` writer - IP RX FIFO WaterMark available interrupt enable."]
pub type IprxwaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPTXWEEN` reader - IP TX FIFO WaterMark empty interrupt enable."]
pub type IptxweenR = crate::BitReader;
#[doc = "Field `IPTXWEEN` writer - IP TX FIFO WaterMark empty interrupt enable."]
pub type IptxweenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATALEARNFAILEN` reader - Data Learning failed interrupt enable."]
pub type DatalearnfailenR = crate::BitReader;
#[doc = "Field `DATALEARNFAILEN` writer - Data Learning failed interrupt enable."]
pub type DatalearnfailenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKSTOPBYRDEN` reader - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
pub type SckstopbyrdenR = crate::BitReader;
#[doc = "Field `SCKSTOPBYRDEN` writer - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
pub type SckstopbyrdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKSTOPBYWREN` reader - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
pub type SckstopbywrenR = crate::BitReader;
#[doc = "Field `SCKSTOPBYWREN` writer - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
pub type SckstopbywrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBBUSTIMEOUTEN` reader - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AhbbustimeoutenR = crate::BitReader;
#[doc = "Field `AHBBUSTIMEOUTEN` writer - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AhbbustimeoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQTIMEOUTEN` reader - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
pub type SeqtimeoutenR = crate::BitReader;
#[doc = "Field `SEQTIMEOUTEN` writer - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
pub type SeqtimeoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub fn ipcmddoneen(&self) -> IpcmddoneenR {
        IpcmddoneenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ipcmdgeen(&self) -> IpcmdgeenR {
        IpcmdgeenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ahbcmdgeen(&self) -> AhbcmdgeenR {
        AhbcmdgeenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ipcmderren(&self) -> IpcmderrenR {
        IpcmderrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ahbcmderren(&self) -> AhbcmderrenR {
        AhbcmderrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub fn iprxwaen(&self) -> IprxwaenR {
        IprxwaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub fn iptxween(&self) -> IptxweenR {
        IptxweenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Learning failed interrupt enable."]
    #[inline(always)]
    pub fn datalearnfailen(&self) -> DatalearnfailenR {
        DatalearnfailenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub fn sckstopbyrden(&self) -> SckstopbyrdenR {
        SckstopbyrdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn sckstopbywren(&self) -> SckstopbywrenR {
        SckstopbywrenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeouten(&self) -> AhbbustimeoutenR {
        AhbbustimeoutenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn seqtimeouten(&self) -> SeqtimeoutenR {
        SeqtimeoutenR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("ipcmddoneen", &self.ipcmddoneen())
            .field("ipcmdgeen", &self.ipcmdgeen())
            .field("ahbcmdgeen", &self.ahbcmdgeen())
            .field("ipcmderren", &self.ipcmderren())
            .field("ahbcmderren", &self.ahbcmderren())
            .field("iprxwaen", &self.iprxwaen())
            .field("iptxween", &self.iptxween())
            .field("datalearnfailen", &self.datalearnfailen())
            .field("sckstopbyrden", &self.sckstopbyrden())
            .field("sckstopbywren", &self.sckstopbywren())
            .field("ahbbustimeouten", &self.ahbbustimeouten())
            .field("seqtimeouten", &self.seqtimeouten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddoneen(&mut self) -> IpcmddoneenW<IntenSpec> {
        IpcmddoneenW::new(self, 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmdgeen(&mut self) -> IpcmdgeenW<IntenSpec> {
        IpcmdgeenW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmdgeen(&mut self) -> AhbcmdgeenW<IntenSpec> {
        AhbcmdgeenW::new(self, 2)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderren(&mut self) -> IpcmderrenW<IntenSpec> {
        IpcmderrenW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmderren(&mut self) -> AhbcmderrenW<IntenSpec> {
        AhbcmderrenW::new(self, 4)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn iprxwaen(&mut self) -> IprxwaenW<IntenSpec> {
        IprxwaenW::new(self, 5)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn iptxween(&mut self) -> IptxweenW<IntenSpec> {
        IptxweenW::new(self, 6)
    }
    #[doc = "Bit 7 - Data Learning failed interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn datalearnfailen(&mut self) -> DatalearnfailenW<IntenSpec> {
        DatalearnfailenW::new(self, 7)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbyrden(&mut self) -> SckstopbyrdenW<IntenSpec> {
        SckstopbyrdenW::new(self, 8)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbywren(&mut self) -> SckstopbywrenW<IntenSpec> {
        SckstopbywrenW::new(self, 9)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    #[must_use]
    pub fn ahbbustimeouten(&mut self) -> AhbbustimeoutenW<IntenSpec> {
        AhbbustimeoutenW::new(self, 10)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    #[must_use]
    pub fn seqtimeouten(&mut self) -> SeqtimeoutenW<IntenSpec> {
        SeqtimeoutenW::new(self, 11)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
