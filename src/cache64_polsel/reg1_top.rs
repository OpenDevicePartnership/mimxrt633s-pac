#[doc = "Register `REG1_TOP` reader"]
pub type R = crate::R<Reg1TopSpec>;
#[doc = "Register `REG1_TOP` writer"]
pub type W = crate::W<Reg1TopSpec>;
#[doc = "Field `REG1_TOP` reader - Upper limit of Region 1"]
pub type Reg1TopR = crate::FieldReader<u32>;
#[doc = "Field `REG1_TOP` writer - Upper limit of Region 1"]
pub type Reg1TopW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 10:26 - Upper limit of Region 1"]
    #[inline(always)]
    pub fn reg1_top(&self) -> Reg1TopR {
        Reg1TopR::new((self.bits >> 10) & 0x0001_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG1_TOP")
            .field("reg1_top", &self.reg1_top())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:26 - Upper limit of Region 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg1_top(&mut self) -> Reg1TopW<Reg1TopSpec> {
        Reg1TopW::new(self, 10)
    }
}
#[doc = "Region 1 Top Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1_top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg1TopSpec;
impl crate::RegisterSpec for Reg1TopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg1_top::R`](R) reader structure"]
impl crate::Readable for Reg1TopSpec {}
#[doc = "`write(|w| ..)` method takes [`reg1_top::W`](W) writer structure"]
impl crate::Writable for Reg1TopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG1_TOP to value 0x0555_5400"]
impl crate::Resettable for Reg1TopSpec {
    const RESET_VALUE: u32 = 0x0555_5400;
}
