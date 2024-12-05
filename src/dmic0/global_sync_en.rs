#[doc = "Register `GLOBAL_SYNC_EN` reader"]
pub type R = crate::R<GlobalSyncEnSpec>;
#[doc = "Register `GLOBAL_SYNC_EN` writer"]
pub type W = crate::W<GlobalSyncEnSpec>;
#[doc = "Field `CH_SYNC_EN` reader - Choose which channels to sync to global sync (7:0) corresponds to the 8 channels"]
pub type ChSyncEnR = crate::FieldReader;
#[doc = "Field `CH_SYNC_EN` writer - Choose which channels to sync to global sync (7:0) corresponds to the 8 channels"]
pub type ChSyncEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Choose which channels to sync to global sync (7:0) corresponds to the 8 channels"]
    #[inline(always)]
    pub fn ch_sync_en(&self) -> ChSyncEnR {
        ChSyncEnR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLOBAL_SYNC_EN")
            .field("ch_sync_en", &self.ch_sync_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Choose which channels to sync to global sync (7:0) corresponds to the 8 channels"]
    #[inline(always)]
    pub fn ch_sync_en(&mut self) -> ChSyncEnW<GlobalSyncEnSpec> {
        ChSyncEnW::new(self, 0)
    }
}
#[doc = "global sync enable\n\nYou can [`read`](crate::Reg::read) this register and get [`global_sync_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`global_sync_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobalSyncEnSpec;
impl crate::RegisterSpec for GlobalSyncEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`global_sync_en::R`](R) reader structure"]
impl crate::Readable for GlobalSyncEnSpec {}
#[doc = "`write(|w| ..)` method takes [`global_sync_en::W`](W) writer structure"]
impl crate::Writable for GlobalSyncEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBAL_SYNC_EN to value 0"]
impl crate::Resettable for GlobalSyncEnSpec {
    const RESET_VALUE: u32 = 0;
}
