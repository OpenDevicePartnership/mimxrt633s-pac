#[doc = "Register `DMAREQ1` reader"]
pub type R = crate::R<Dmareq1Spec>;
#[doc = "Register `DMAREQ1` writer"]
pub type W = crate::W<Dmareq1Spec>;
#[doc = "Field `DEV_1` reader - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type Dev1R = crate::FieldReader<u16>;
#[doc = "Field `DEV_1` writer - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type Dev1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DRL1` reader - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
pub type Drl1R = crate::BitReader;
#[doc = "Field `DRL1` writer - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
pub type Drl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQ1` reader - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type Drq1R = crate::BitReader;
#[doc = "Field `DRQ1` writer - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type Drq1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_1(&self) -> Dev1R {
        Dev1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub fn drl1(&self) -> Drl1R {
        Drl1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq1(&self) -> Drq1R {
        Drq1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAREQ1")
            .field("dev_1", &self.dev_1())
            .field("drl1", &self.drl1())
            .field("drq1", &self.drq1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_1(&mut self) -> Dev1W<Dmareq1Spec> {
        Dev1W::new(self, 0)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub fn drl1(&mut self) -> Drl1W<Dmareq1Spec> {
        Drl1W::new(self, 30)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq1(&mut self) -> Drq1W<Dmareq1Spec> {
        Drq1W::new(self, 31)
    }
}
#[doc = "SCT DMA request 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmareq1Spec;
impl crate::RegisterSpec for Dmareq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmareq1::R`](R) reader structure"]
impl crate::Readable for Dmareq1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmareq1::W`](W) writer structure"]
impl crate::Writable for Dmareq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAREQ1 to value 0"]
impl crate::Resettable for Dmareq1Spec {
    const RESET_VALUE: u32 = 0;
}
