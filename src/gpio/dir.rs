#[doc = "Register `DIR[%s]` reader"]
pub type R = crate::R<DirSpec>;
#[doc = "Register `DIR[%s]` writer"]
pub type W = crate::W<DirSpec>;
#[doc = "Field `DIRP` reader - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
pub type DirpR = crate::FieldReader<u32>;
#[doc = "Field `DIRP` writer - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
pub type DirpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&self) -> DirpR {
        DirpR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR").field("dirp", &self.dirp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&mut self) -> DirpW<DirSpec> {
        DirpW::new(self, 0)
    }
}
#[doc = "Direction registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirSpec;
impl crate::RegisterSpec for DirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DirSpec {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIR[%s]
to value 0"]
impl crate::Resettable for DirSpec {
    const RESET_VALUE: u32 = 0;
}
