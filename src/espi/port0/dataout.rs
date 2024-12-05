#[doc = "Register `DATAOUT` reader"]
pub type R = crate::R<DataoutSpec>;
#[doc = "Register `DATAOUT` writer"]
pub type W = crate::W<DataoutSpec>;
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
        f.debug_struct("DATAOUT")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DataoutSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Port Data to Host (for Endpoint and Index/Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutSpec;
impl crate::RegisterSpec for DataoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataout::R`](R) reader structure"]
impl crate::Readable for DataoutSpec {}
#[doc = "`write(|w| ..)` method takes [`dataout::W`](W) writer structure"]
impl crate::Writable for DataoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAOUT to value 0"]
impl crate::Resettable for DataoutSpec {
    const RESET_VALUE: u32 = 0;
}
