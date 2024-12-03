#[doc = "Register `SCMISC` reader"]
pub type R = crate::R<ScmiscSpec>;
#[doc = "Register `SCMISC` writer"]
pub type W = crate::W<ScmiscSpec>;
#[doc = "Field `LRUN_MAX` reader - LONG RUN MAX LIMIT"]
pub type LrunMaxR = crate::FieldReader;
#[doc = "Field `LRUN_MAX` writer - LONG RUN MAX LIMIT"]
pub type LrunMaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTY_CT` reader - RETRY COUNT"]
pub type RtyCtR = crate::FieldReader;
#[doc = "Field `RTY_CT` writer - RETRY COUNT"]
pub type RtyCtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&self) -> LrunMaxR {
        LrunMaxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&self) -> RtyCtR {
        RtyCtR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCMISC")
            .field("lrun_max", &self.lrun_max())
            .field("rty_ct", &self.rty_ct())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&mut self) -> LrunMaxW<ScmiscSpec> {
        LrunMaxW::new(self, 0)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&mut self) -> RtyCtW<ScmiscSpec> {
        RtyCtW::new(self, 16)
    }
}
#[doc = "Statistical Check Miscellaneous Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scmisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmiscSpec;
impl crate::RegisterSpec for ScmiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmisc::R`](R) reader structure"]
impl crate::Readable for ScmiscSpec {}
#[doc = "`write(|w| ..)` method takes [`scmisc::W`](W) writer structure"]
impl crate::Writable for ScmiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCMISC to value 0x0001_0022"]
impl crate::Resettable for ScmiscSpec {
    const RESET_VALUE: u32 = 0x0001_0022;
}
