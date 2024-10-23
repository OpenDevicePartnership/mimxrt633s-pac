#[doc = "Register `PINT_SEL%s` reader"]
pub type R = crate::R<PintSelSpec>;
#[doc = "Register `PINT_SEL%s` writer"]
pub type W = crate::W<PintSelSpec>;
#[doc = "Field `PINT_SEL` reader - Port Input (PIOx.y) 64 to 8 Mux Select. . . Pin number select for pin interrupt or pattern match engine input. (For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
pub type PintSelR = crate::FieldReader;
#[doc = "Field `PINT_SEL` writer - Port Input (PIOx.y) 64 to 8 Mux Select. . . Pin number select for pin interrupt or pattern match engine input. (For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
pub type PintSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Input (PIOx.y) 64 to 8 Mux Select. . . Pin number select for pin interrupt or pattern match engine input. (For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub fn pint_sel(&self) -> PintSelR {
        PintSelR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINT_SEL")
            .field("pint_sel", &self.pint_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Input (PIOx.y) 64 to 8 Mux Select. . . Pin number select for pin interrupt or pattern match engine input. (For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    #[must_use]
    pub fn pint_sel(&mut self) -> PintSelW<PintSelSpec> {
        PintSelW::new(self, 0)
    }
}
#[doc = "GPIO Pin Input Multiplexer N\n\nYou can [`read`](crate::Reg::read) this register and get [`pint_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pint_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintSelSpec;
impl crate::RegisterSpec for PintSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pint_sel::R`](R) reader structure"]
impl crate::Readable for PintSelSpec {}
#[doc = "`write(|w| ..)` method takes [`pint_sel::W`](W) writer structure"]
impl crate::Writable for PintSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINT_SEL%s to value 0x7f"]
impl crate::Resettable for PintSelSpec {
    const RESET_VALUE: u32 = 0x7f;
}
