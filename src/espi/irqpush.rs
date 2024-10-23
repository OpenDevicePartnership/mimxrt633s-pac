#[doc = "Register `IRQPUSH` reader"]
pub type R = crate::R<IrqpushSpec>;
#[doc = "Register `IRQPUSH` writer"]
pub type W = crate::W<IrqpushSpec>;
#[doc = "Field `IRQ` reader - Set to the IRQ to push across to the Host."]
pub type IrqR = crate::FieldReader;
#[doc = "Field `IRQ` writer - Set to the IRQ to push across to the Host."]
pub type IrqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DONE` reader - Will go to 1 when complete; this is just the same bit as IrqUpd in MSTAT, which can interrupt using INTENSET"]
pub type DoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Set to the IRQ to push across to the Host."]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Will go to 1 when complete; this is just the same bit as IrqUpd in MSTAT, which can interrupt using INTENSET"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQPUSH")
            .field("irq", &self.irq())
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Set to the IRQ to push across to the Host."]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IrqW<IrqpushSpec> {
        IrqW::new(self, 0)
    }
}
#[doc = "IRQ to drive into Host (with eSPI)\n\nYou can [`read`](crate::Reg::read) this register and get [`irqpush::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqpush::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqpushSpec;
impl crate::RegisterSpec for IrqpushSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqpush::R`](R) reader structure"]
impl crate::Readable for IrqpushSpec {}
#[doc = "`write(|w| ..)` method takes [`irqpush::W`](W) writer structure"]
impl crate::Writable for IrqpushSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQPUSH to value 0"]
impl crate::Resettable for IrqpushSpec {
    const RESET_VALUE: u32 = 0;
}
