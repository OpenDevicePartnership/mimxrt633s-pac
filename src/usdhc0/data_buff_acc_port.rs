#[doc = "Register `DATA_BUFF_ACC_PORT` reader"]
pub type R = crate::R<DataBuffAccPortSpec>;
#[doc = "Register `DATA_BUFF_ACC_PORT` writer"]
pub type W = crate::W<DataBuffAccPortSpec>;
#[doc = "Field `DATCONT` reader - Data Content"]
pub type DatcontR = crate::FieldReader<u32>;
#[doc = "Field `DATCONT` writer - Data Content"]
pub type DatcontW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Content"]
    #[inline(always)]
    pub fn datcont(&self) -> DatcontR {
        DatcontR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_BUFF_ACC_PORT")
            .field("datcont", &self.datcont())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Content"]
    #[inline(always)]
    #[must_use]
    pub fn datcont(&mut self) -> DatcontW<DataBuffAccPortSpec> {
        DatcontW::new(self, 0)
    }
}
#[doc = "Data Buffer Access Port\n\nYou can [`read`](crate::Reg::read) this register and get [`data_buff_acc_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_buff_acc_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataBuffAccPortSpec;
impl crate::RegisterSpec for DataBuffAccPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buff_acc_port::R`](R) reader structure"]
impl crate::Readable for DataBuffAccPortSpec {}
#[doc = "`write(|w| ..)` method takes [`data_buff_acc_port::W`](W) writer structure"]
impl crate::Writable for DataBuffAccPortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_BUFF_ACC_PORT to value 0"]
impl crate::Resettable for DataBuffAccPortSpec {
    const RESET_VALUE: u32 = 0;
}
