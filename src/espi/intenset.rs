#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `PORT_INT0` reader - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt0R = crate::BitReader;
#[doc = "Field `PORT_INT0` writer - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_INT1` reader - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt1R = crate::BitReader;
#[doc = "Field `PORT_INT1` writer - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_INT2` reader - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt2R = crate::BitReader;
#[doc = "Field `PORT_INT2` writer - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_INT3` reader - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt3R = crate::BitReader;
#[doc = "Field `PORT_INT3` writer - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_INT4` reader - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt4R = crate::BitReader;
#[doc = "Field `PORT_INT4` writer - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortInt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P80Int` reader - If set to 1, Port80 will interrupt main processor on update from Host."]
pub type P80intR = crate::BitReader;
#[doc = "Field `P80Int` writer - If set to 1, Port80 will interrupt main processor on update from Host."]
pub type P80intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BusRst` reader - If 1, a change in Bus Reset status will interrupt main processor."]
pub type BusRstR = crate::BitReader;
#[doc = "Field `BusRst` writer - If 1, a change in Bus Reset status will interrupt main processor."]
pub type BusRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IrqUpd` reader - If 1, completion of an IRQ update will interrupt main processor."]
pub type IrqUpdR = crate::BitReader;
#[doc = "Field `IrqUpd` writer - If 1, completion of an IRQ update will interrupt main processor."]
pub type IrqUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WireChg` reader - If 1, when one or more VWire input has changed, will interrupt main processor."]
pub type WireChgR = crate::BitReader;
#[doc = "Field `WireChg` writer - If 1, when one or more VWire input has changed, will interrupt main processor."]
pub type WireChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Hstall` reader - If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
pub type HstallR = crate::BitReader;
#[doc = "Field `Hstall` writer - If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
pub type HstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERR` reader - If 1, when a CRC error detected, will interrupt main processor."]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - If 1, when a CRC error detected, will interrupt main processor."]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int0(&self) -> PortInt0R {
        PortInt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int1(&self) -> PortInt1R {
        PortInt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int2(&self) -> PortInt2R {
        PortInt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int3(&self) -> PortInt3R {
        PortInt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int4(&self) -> PortInt4R {
        PortInt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - If set to 1, Port80 will interrupt main processor on update from Host."]
    #[inline(always)]
    pub fn p80int(&self) -> P80intR {
        P80intR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If 1, a change in Bus Reset status will interrupt main processor."]
    #[inline(always)]
    pub fn bus_rst(&self) -> BusRstR {
        BusRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If 1, completion of an IRQ update will interrupt main processor."]
    #[inline(always)]
    pub fn irq_upd(&self) -> IrqUpdR {
        IrqUpdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[inline(always)]
    pub fn wire_chg(&self) -> WireChgR {
        WireChgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[inline(always)]
    pub fn hstall(&self) -> HstallR {
        HstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If 1, when a CRC error detected, will interrupt main processor."]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("port_int0", &self.port_int0())
            .field("port_int1", &self.port_int1())
            .field("port_int2", &self.port_int2())
            .field("port_int3", &self.port_int3())
            .field("port_int4", &self.port_int4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int0(&mut self) -> PortInt0W<IntensetSpec> {
        PortInt0W::new(self, 0)
    }
    #[doc = "Bit 1 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int1(&mut self) -> PortInt1W<IntensetSpec> {
        PortInt1W::new(self, 1)
    }
    #[doc = "Bit 2 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int2(&mut self) -> PortInt2W<IntensetSpec> {
        PortInt2W::new(self, 2)
    }
    #[doc = "Bit 3 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int3(&mut self) -> PortInt3W<IntensetSpec> {
        PortInt3W::new(self, 3)
    }
    #[doc = "Bit 4 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int4(&mut self) -> PortInt4W<IntensetSpec> {
        PortInt4W::new(self, 4)
    }
    #[doc = "Bit 8 - If set to 1, Port80 will interrupt main processor on update from Host."]
    #[inline(always)]
    pub fn p80int(&mut self) -> P80intW<IntensetSpec> {
        P80intW::new(self, 8)
    }
    #[doc = "Bit 9 - If 1, a change in Bus Reset status will interrupt main processor."]
    #[inline(always)]
    pub fn bus_rst(&mut self) -> BusRstW<IntensetSpec> {
        BusRstW::new(self, 9)
    }
    #[doc = "Bit 10 - If 1, completion of an IRQ update will interrupt main processor."]
    #[inline(always)]
    pub fn irq_upd(&mut self) -> IrqUpdW<IntensetSpec> {
        IrqUpdW::new(self, 10)
    }
    #[doc = "Bit 11 - If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[inline(always)]
    pub fn wire_chg(&mut self) -> WireChgW<IntensetSpec> {
        WireChgW::new(self, 11)
    }
    #[doc = "Bit 12 - If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[inline(always)]
    pub fn hstall(&mut self) -> HstallW<IntensetSpec> {
        HstallW::new(self, 12)
    }
    #[doc = "Bit 13 - If 1, when a CRC error detected, will interrupt main processor."]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<IntensetSpec> {
        CrcerrW::new(self, 13)
    }
    #[doc = "Bit 14 - If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<IntensetSpec> {
        GpioW::new(self, 14)
    }
}
#[doc = "Interrupt Set (enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
