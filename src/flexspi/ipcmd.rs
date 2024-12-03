#[doc = "Register `IPCMD` reader"]
pub type R = crate::R<IpcmdSpec>;
#[doc = "Register `IPCMD` writer"]
pub type W = crate::W<IpcmdSpec>;
#[doc = "Field `TRG` reader - Setting this bit will trigger an IP Command."]
pub type TrgR = crate::BitReader;
#[doc = "Field `TRG` writer - Setting this bit will trigger an IP Command."]
pub type TrgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub fn trg(&self) -> TrgR {
        TrgR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCMD").field("trg", &self.trg()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub fn trg(&mut self) -> TrgW<IpcmdSpec> {
        TrgW::new(self, 0)
    }
}
#[doc = "IP Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcmdSpec;
impl crate::RegisterSpec for IpcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcmd::R`](R) reader structure"]
impl crate::Readable for IpcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ipcmd::W`](W) writer structure"]
impl crate::Writable for IpcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCMD to value 0"]
impl crate::Resettable for IpcmdSpec {
    const RESET_VALUE: u32 = 0;
}
