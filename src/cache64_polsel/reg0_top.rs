#[doc = "Register `REG0_TOP` reader"]
pub type R = crate::R<Reg0TopSpec>;
#[doc = "Register `REG0_TOP` writer"]
pub type W = crate::W<Reg0TopSpec>;
#[doc = "Field `REG0_TOP` reader - Upper limit of Region 0"]
pub type Reg0TopR = crate::FieldReader<u32>;
#[doc = "Field `REG0_TOP` writer - Upper limit of Region 0"]
pub type Reg0TopW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 10:26 - Upper limit of Region 0"]
    #[inline(always)]
    pub fn reg0_top(&self) -> Reg0TopR {
        Reg0TopR::new((self.bits >> 10) & 0x0001_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG0_TOP")
            .field("reg0_top", &self.reg0_top())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:26 - Upper limit of Region 0"]
    #[inline(always)]
    pub fn reg0_top(&mut self) -> Reg0TopW<Reg0TopSpec> {
        Reg0TopW::new(self, 10)
    }
}
#[doc = "Region 0 Top Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0_top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0_top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg0TopSpec;
impl crate::RegisterSpec for Reg0TopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg0_top::R`](R) reader structure"]
impl crate::Readable for Reg0TopSpec {}
#[doc = "`write(|w| ..)` method takes [`reg0_top::W`](W) writer structure"]
impl crate::Writable for Reg0TopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG0_TOP to value 0x02aa_a800"]
impl crate::Resettable for Reg0TopSpec {
    const RESET_VALUE: u32 = 0x02aa_a800;
}
