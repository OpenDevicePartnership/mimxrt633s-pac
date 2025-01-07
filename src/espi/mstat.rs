#[doc = "Register `MSTAT` reader"]
pub type R = crate::R<MstatSpec>;
#[doc = "Register `MSTAT` writer"]
pub type W = crate::W<MstatSpec>;
#[doc = "Field `PortInt` reader - Corresponding port is pending interrupt service"]
pub type PortIntR = crate::FieldReader;
#[doc = "Field `P80Int` reader - Port80 has had a request and is pending service."]
pub type P80intR = crate::BitReader;
#[doc = "Field `P80Int` writer - Port80 has had a request and is pending service."]
pub type P80intW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BusRst` reader - If 1, the entered or left reset. Sticky - must clear."]
pub type BusRstR = crate::BitReader;
#[doc = "Field `BusRst` writer - If 1, the entered or left reset. Sticky - must clear."]
pub type BusRstW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IrqUpd` reader - If 1, the bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done)"]
pub type IrqUpdR = crate::BitReader;
#[doc = "Field `IrqUpd` writer - If 1, the bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done)"]
pub type IrqUpdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WireChg` reader - If 1, one or more input VWire has changed since last cleared for eSPI; for LPC, SERIRQ started"]
pub type WireChgR = crate::BitReader;
#[doc = "Field `WireChg` writer - If 1, one or more input VWire has changed since last cleared for eSPI; for LPC, SERIRQ started"]
pub type WireChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `Hstall` reader - If 1, the Host is stalled on a read from or write to a port that has the StallRd or StallWr bit set in the PnCFG register"]
pub type HstallR = crate::BitReader;
#[doc = "Field `Hstall` writer - If 1, the Host is stalled on a read from or write to a port that has the StallRd or StallWr bit set in the PnCFG register"]
pub type HstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERR` reader - If 1, the CRC from the Master did not match the computed CRC"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - If 1, the CRC from the Master did not match the computed CRC"]
pub type CrcerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GPIO` reader - If 1, the GPIO in ESPIMISC has had an input change"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - If 1, the GPIO in ESPIMISC has had an input change"]
pub type GpioW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `Busy` reader - If 1, the bus is busy."]
pub type BusyR = crate::BitReader;
#[doc = "Field `InRst` reader - If 1, the bus in reset."]
pub type InRstR = crate::BitReader;
#[doc = "Field `CompPend` reader - If 1, completions are pending for eSPI; indicates quiet mode for LPC."]
pub type CompPendR = crate::BitReader;
#[doc = "Field `MastPend` reader - If 1, Mastering is pending (flash or memory)"]
pub type MastPendR = crate::BitReader;
#[doc = "Field `AlertPend` reader - If 1, the Alert request pin is pending (whether separate pin or MISO)"]
pub type AlertPendR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Corresponding port is pending interrupt service"]
    #[inline(always)]
    pub fn port_int(&self) -> PortIntR {
        PortIntR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Port80 has had a request and is pending service."]
    #[inline(always)]
    pub fn p80int(&self) -> P80intR {
        P80intR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If 1, the entered or left reset. Sticky - must clear."]
    #[inline(always)]
    pub fn bus_rst(&self) -> BusRstR {
        BusRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If 1, the bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done)"]
    #[inline(always)]
    pub fn irq_upd(&self) -> IrqUpdR {
        IrqUpdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If 1, one or more input VWire has changed since last cleared for eSPI; for LPC, SERIRQ started"]
    #[inline(always)]
    pub fn wire_chg(&self) -> WireChgR {
        WireChgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If 1, the Host is stalled on a read from or write to a port that has the StallRd or StallWr bit set in the PnCFG register"]
    #[inline(always)]
    pub fn hstall(&self) -> HstallR {
        HstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If 1, the CRC from the Master did not match the computed CRC"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If 1, the GPIO in ESPIMISC has had an input change"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - If 1, the bus is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If 1, the bus in reset."]
    #[inline(always)]
    pub fn in_rst(&self) -> InRstR {
        InRstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If 1, completions are pending for eSPI; indicates quiet mode for LPC."]
    #[inline(always)]
    pub fn comp_pend(&self) -> CompPendR {
        CompPendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - If 1, Mastering is pending (flash or memory)"]
    #[inline(always)]
    pub fn mast_pend(&self) -> MastPendR {
        MastPendR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If 1, the Alert request pin is pending (whether separate pin or MISO)"]
    #[inline(always)]
    pub fn alert_pend(&self) -> AlertPendR {
        AlertPendR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTAT")
            .field("port_int", &self.port_int())
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("busy", &self.busy())
            .field("in_rst", &self.in_rst())
            .field("comp_pend", &self.comp_pend())
            .field("mast_pend", &self.mast_pend())
            .field("alert_pend", &self.alert_pend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Port80 has had a request and is pending service."]
    #[inline(always)]
    pub fn p80int(&mut self) -> P80intW<MstatSpec> {
        P80intW::new(self, 8)
    }
    #[doc = "Bit 9 - If 1, the entered or left reset. Sticky - must clear."]
    #[inline(always)]
    pub fn bus_rst(&mut self) -> BusRstW<MstatSpec> {
        BusRstW::new(self, 9)
    }
    #[doc = "Bit 10 - If 1, the bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done)"]
    #[inline(always)]
    pub fn irq_upd(&mut self) -> IrqUpdW<MstatSpec> {
        IrqUpdW::new(self, 10)
    }
    #[doc = "Bit 11 - If 1, one or more input VWire has changed since last cleared for eSPI; for LPC, SERIRQ started"]
    #[inline(always)]
    pub fn wire_chg(&mut self) -> WireChgW<MstatSpec> {
        WireChgW::new(self, 11)
    }
    #[doc = "Bit 12 - If 1, the Host is stalled on a read from or write to a port that has the StallRd or StallWr bit set in the PnCFG register"]
    #[inline(always)]
    pub fn hstall(&mut self) -> HstallW<MstatSpec> {
        HstallW::new(self, 12)
    }
    #[doc = "Bit 13 - If 1, the CRC from the Master did not match the computed CRC"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<MstatSpec> {
        CrcerrW::new(self, 13)
    }
    #[doc = "Bit 14 - If 1, the GPIO in ESPIMISC has had an input change"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<MstatSpec> {
        GpioW::new(self, 14)
    }
}
#[doc = "Master Status of whole peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`mstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstatSpec;
impl crate::RegisterSpec for MstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstat::R`](R) reader structure"]
impl crate::Readable for MstatSpec {}
#[doc = "`write(|w| ..)` method takes [`mstat::W`](W) writer structure"]
impl crate::Writable for MstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x6f00;
}
#[doc = "`reset()` method sets MSTAT to value 0"]
impl crate::Resettable for MstatSpec {
    const RESET_VALUE: u32 = 0;
}
