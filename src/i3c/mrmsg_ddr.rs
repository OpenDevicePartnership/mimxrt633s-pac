#[doc = "Register `MRMSG_DDR` reader"]
pub type R = crate::R<MrmsgDdrSpec>;
#[doc = "Register `MRMSG_DDR` writer"]
pub type W = crate::W<MrmsgDdrSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLEN` reader - Current length"]
pub type ClenR = crate::FieldReader<u16>;
#[doc = "Field `CLEN` writer - Current length"]
pub type ClenW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Current length"]
    #[inline(always)]
    pub fn clen(&self) -> ClenR {
        ClenR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRMSG_DDR")
            .field("data", &self.data())
            .field("clen", &self.clen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<MrmsgDdrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Current length"]
    #[inline(always)]
    pub fn clen(&mut self) -> ClenW<MrmsgDdrSpec> {
        ClenW::new(self, 16)
    }
}
#[doc = "Master Read Message in DDR mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mrmsg_ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrmsg_ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrmsgDdrSpec;
impl crate::RegisterSpec for MrmsgDdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrmsg_ddr::R`](R) reader structure"]
impl crate::Readable for MrmsgDdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrmsg_ddr::W`](W) writer structure"]
impl crate::Writable for MrmsgDdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRMSG_DDR to value 0"]
impl crate::Resettable for MrmsgDdrSpec {
    const RESET_VALUE: u32 = 0;
}
