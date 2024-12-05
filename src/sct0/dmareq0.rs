#[doc = "Register `DMAREQ0` reader"]
pub type R = crate::R<Dmareq0Spec>;
#[doc = "Register `DMAREQ0` writer"]
pub type W = crate::W<Dmareq0Spec>;
#[doc = "Field `DEV_0` reader - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type Dev0R = crate::FieldReader<u16>;
#[doc = "Field `DEV_0` writer - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type Dev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DRL0` reader - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
pub type Drl0R = crate::BitReader;
#[doc = "Field `DRL0` writer - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
pub type Drl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQ0` reader - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type Drq0R = crate::BitReader;
#[doc = "Field `DRQ0` writer - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type Drq0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&self) -> Dev0R {
        Dev0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&self) -> Drl0R {
        Drl0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&self) -> Drq0R {
        Drq0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAREQ0")
            .field("dev_0", &self.dev_0())
            .field("drl0", &self.drl0())
            .field("drq0", &self.drq0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&mut self) -> Dev0W<Dmareq0Spec> {
        Dev0W::new(self, 0)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&mut self) -> Drl0W<Dmareq0Spec> {
        Drl0W::new(self, 30)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&mut self) -> Drq0W<Dmareq0Spec> {
        Drq0W::new(self, 31)
    }
}
#[doc = "SCT DMA request 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmareq0Spec;
impl crate::RegisterSpec for Dmareq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmareq0::R`](R) reader structure"]
impl crate::Readable for Dmareq0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmareq0::W`](W) writer structure"]
impl crate::Writable for Dmareq0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAREQ0 to value 0"]
impl crate::Resettable for Dmareq0Spec {
    const RESET_VALUE: u32 = 0;
}
