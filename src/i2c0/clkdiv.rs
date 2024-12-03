#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `DIVVAL` reader - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
pub type DivvalR = crate::FieldReader<u16>;
#[doc = "Field `DIVVAL` writer - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
pub type DivvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub fn divval(&self) -> DivvalR {
        DivvalR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("divval", &self.divval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub fn divval(&mut self) -> DivvalW<ClkdivSpec> {
        DivvalW::new(self, 0)
    }
}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0;
}
