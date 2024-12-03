#[doc = "Register `WAKECLK32KHZDIV` reader"]
pub type R = crate::R<Wakeclk32khzdivSpec>;
#[doc = "Register `WAKECLK32KHZDIV` writer"]
pub type W = crate::W<Wakeclk32khzdivSpec>;
#[doc = "Field `HALT` reader - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKECLK32KHZDIV")
            .field("halt", &self.halt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<Wakeclk32khzdivSpec> {
        HaltW::new(self, 30)
    }
}
#[doc = "32k wake clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeclk32khzdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeclk32khzdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wakeclk32khzdivSpec;
impl crate::RegisterSpec for Wakeclk32khzdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeclk32khzdiv::R`](R) reader structure"]
impl crate::Readable for Wakeclk32khzdivSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeclk32khzdiv::W`](W) writer structure"]
impl crate::Writable for Wakeclk32khzdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKECLK32KHZDIV to value 0x1f"]
impl crate::Resettable for Wakeclk32khzdivSpec {
    const RESET_VALUE: u32 = 0x1f;
}
