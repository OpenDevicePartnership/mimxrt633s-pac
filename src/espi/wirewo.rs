#[doc = "Register `WIREWO` reader"]
pub type R = crate::R<WirewoSpec>;
#[doc = "Register `WIREWO` writer"]
pub type W = crate::W<WirewoSpec>;
#[doc = "Field `WIRES` reader - LPC: IRQ states to use"]
pub type WiresR = crate::FieldReader<u32>;
#[doc = "Field `WIRES` writer - LPC: IRQ states to use"]
pub type WiresW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `DONE` reader - Set to 1 after last write has been pushed out to Host"]
pub type DoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:17 - LPC: IRQ states to use"]
    #[inline(always)]
    pub fn wires(&self) -> WiresR {
        WiresR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 31 - Set to 1 after last write has been pushed out to Host"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIREWO")
            .field("wires", &self.wires())
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - LPC: IRQ states to use"]
    #[inline(always)]
    #[must_use]
    pub fn wires(&mut self) -> WiresW<WirewoSpec> {
        WiresW::new(self, 0)
    }
}
#[doc = "Wire states for Host to see; if LPC, this is the IRQ states.\n\nYou can [`read`](crate::Reg::read) this register and get [`wirewo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wirewo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WirewoSpec;
impl crate::RegisterSpec for WirewoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wirewo::R`](R) reader structure"]
impl crate::Readable for WirewoSpec {}
#[doc = "`write(|w| ..)` method takes [`wirewo::W`](W) writer structure"]
impl crate::Writable for WirewoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIREWO to value 0"]
impl crate::Resettable for WirewoSpec {
    const RESET_VALUE: u32 = 0;
}
