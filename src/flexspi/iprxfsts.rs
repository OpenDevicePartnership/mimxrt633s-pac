#[doc = "Register `IPRXFSTS` reader"]
pub type R = crate::R<IprxfstsSpec>;
#[doc = "Field `FILL` reader - Fill level of IP RX FIFO."]
pub type FillR = crate::FieldReader;
#[doc = "Field `RDCNTR` reader - Total Read Data Counter: RDCNTR * 64 Bits."]
pub type RdcntrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Fill level of IP RX FIFO."]
    #[inline(always)]
    pub fn fill(&self) -> FillR {
        FillR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Total Read Data Counter: RDCNTR * 64 Bits."]
    #[inline(always)]
    pub fn rdcntr(&self) -> RdcntrR {
        RdcntrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPRXFSTS")
            .field("fill", &self.fill())
            .field("rdcntr", &self.rdcntr())
            .finish()
    }
}
#[doc = "IP RX FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IprxfstsSpec;
impl crate::RegisterSpec for IprxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprxfsts::R`](R) reader structure"]
impl crate::Readable for IprxfstsSpec {}
#[doc = "`reset()` method sets IPRXFSTS to value 0"]
impl crate::Resettable for IprxfstsSpec {
    const RESET_VALUE: u32 = 0;
}
