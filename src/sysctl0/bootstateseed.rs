#[doc = "Register `BOOTSTATESEED%s` reader"]
pub type R = crate::R<BootstateseedSpec>;
#[doc = "Register `BOOTSTATESEED%s` writer"]
pub type W = crate::W<BootstateseedSpec>;
#[doc = "Field `BOOTSTATESEED` reader - A 256-bit random number set by boot ROM on each restart"]
pub type BootstateseedR = crate::FieldReader<u32>;
#[doc = "Field `BOOTSTATESEED` writer - A 256-bit random number set by boot ROM on each restart"]
pub type BootstateseedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A 256-bit random number set by boot ROM on each restart"]
    #[inline(always)]
    pub fn bootstateseed(&self) -> BootstateseedR {
        BootstateseedR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTSTATESEED")
            .field("bootstateseed", &self.bootstateseed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - A 256-bit random number set by boot ROM on each restart"]
    #[inline(always)]
    #[must_use]
    pub fn bootstateseed(&mut self) -> BootstateseedW<BootstateseedSpec> {
        BootstateseedW::new(self, 0)
    }
}
#[doc = "boot state seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`bootstateseed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootstateseed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootstateseedSpec;
impl crate::RegisterSpec for BootstateseedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootstateseed::R`](R) reader structure"]
impl crate::Readable for BootstateseedSpec {}
#[doc = "`write(|w| ..)` method takes [`bootstateseed::W`](W) writer structure"]
impl crate::Writable for BootstateseedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTSTATESEED%s to value 0"]
impl crate::Resettable for BootstateseedSpec {
    const RESET_VALUE: u32 = 0;
}
