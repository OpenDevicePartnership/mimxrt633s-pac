#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `PortInt` reader - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortIntR = crate::FieldReader;
#[doc = "Field `PortInt` writer - If set to 1, corresponding port will interrupt main processor if matches IRule"]
pub type PortIntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
    #[doc = "Bits 0:4 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub fn port_int(&self) -> PortIntR {
        PortIntR::new((self.bits & 0x1f) as u8)
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
        f.debug_struct("INTSTAT")
            .field("port_int", &self.port_int())
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    #[must_use]
    pub fn port_int(&mut self) -> PortIntW<IntstatSpec> {
        PortIntW::new(self, 0)
    }
    #[doc = "Bit 8 - If set to 1, Port80 will interrupt main processor on update from Host."]
    #[inline(always)]
    #[must_use]
    pub fn p80int(&mut self) -> P80intW<IntstatSpec> {
        P80intW::new(self, 8)
    }
    #[doc = "Bit 9 - If 1, a change in Bus Reset status will interrupt main processor."]
    #[inline(always)]
    #[must_use]
    pub fn bus_rst(&mut self) -> BusRstW<IntstatSpec> {
        BusRstW::new(self, 9)
    }
    #[doc = "Bit 10 - If 1, completion of an IRQ update will interrupt main processor."]
    #[inline(always)]
    #[must_use]
    pub fn irq_upd(&mut self) -> IrqUpdW<IntstatSpec> {
        IrqUpdW::new(self, 10)
    }
    #[doc = "Bit 11 - If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[inline(always)]
    #[must_use]
    pub fn wire_chg(&mut self) -> WireChgW<IntstatSpec> {
        WireChgW::new(self, 11)
    }
    #[doc = "Bit 12 - If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[inline(always)]
    #[must_use]
    pub fn hstall(&mut self) -> HstallW<IntstatSpec> {
        HstallW::new(self, 12)
    }
    #[doc = "Bit 13 - If 1, when a CRC error detected, will interrupt main processor."]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CrcerrW<IntstatSpec> {
        CrcerrW::new(self, 13)
    }
    #[doc = "Bit 14 - If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GpioW<IntstatSpec> {
        GpioW::new(self, 14)
    }
}
#[doc = "Masked interrupt status (causes)\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
