#[doc = "Register `P2DataOut` reader"]
pub type R = crate::R<P2dataOutSpec>;
#[doc = "Register `P2DataOut` writer"]
pub type W = crate::W<P2dataOutSpec>;
#[doc = "Field `DATA` reader - Data to send to Host. Application can tell when taken via the PnStatus register."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data to send to Host. Application can tell when taken via the PnStatus register."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2DataOut")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<P2dataOutSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p2data_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2data_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2dataOutSpec;
impl crate::RegisterSpec for P2dataOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p2data_out::R`](R) reader structure"]
impl crate::Readable for P2dataOutSpec {}
#[doc = "`write(|w| ..)` method takes [`p2data_out::W`](W) writer structure"]
impl crate::Writable for P2dataOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P2DataOut to value 0"]
impl crate::Resettable for P2dataOutSpec {
    const RESET_VALUE: u32 = 0;
}
