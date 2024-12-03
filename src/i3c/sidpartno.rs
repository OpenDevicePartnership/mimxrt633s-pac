#[doc = "Register `SIDPARTNO` reader"]
pub type R = crate::R<SidpartnoSpec>;
#[doc = "Register `SIDPARTNO` writer"]
pub type W = crate::W<SidpartnoSpec>;
#[doc = "Field `PARTNO` reader - Part number"]
pub type PartnoR = crate::FieldReader<u32>;
#[doc = "Field `PARTNO` writer - Part number"]
pub type PartnoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Part number"]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIDPARTNO")
            .field("partno", &self.partno())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Part number"]
    #[inline(always)]
    pub fn partno(&mut self) -> PartnoW<SidpartnoSpec> {
        PartnoW::new(self, 0)
    }
}
#[doc = "Slave ID Part Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidpartno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidpartno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidpartnoSpec;
impl crate::RegisterSpec for SidpartnoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidpartno::R`](R) reader structure"]
impl crate::Readable for SidpartnoSpec {}
#[doc = "`write(|w| ..)` method takes [`sidpartno::W`](W) writer structure"]
impl crate::Writable for SidpartnoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIDPARTNO to value 0"]
impl crate::Resettable for SidpartnoSpec {
    const RESET_VALUE: u32 = 0;
}
