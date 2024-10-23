#[doc = "Register `FIFORDNOPOP` reader"]
pub type R = crate::R<FifordnopopSpec>;
#[doc = "Field `RXDATA` reader - Received data from the FIFO."]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `RXSSEL0_N` reader - Slave Select for receive."]
pub type Rxssel0NR = crate::BitReader;
#[doc = "Field `RXSSEL1_N` reader - Slave Select for receive."]
pub type Rxssel1NR = crate::BitReader;
#[doc = "Field `RXSSEL2_N` reader - Slave Select for receive."]
pub type Rxssel2NR = crate::BitReader;
#[doc = "Field `RXSSEL3_N` reader - Slave Select for receive."]
pub type Rxssel3NR = crate::BitReader;
#[doc = "Field `SOT` reader - Start of transfer flag."]
pub type SotR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> Rxssel0NR {
        Rxssel0NR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> Rxssel1NR {
        Rxssel1NR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> Rxssel2NR {
        Rxssel2NR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> Rxssel3NR {
        Rxssel3NR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Start of transfer flag."]
    #[inline(always)]
    pub fn sot(&self) -> SotR {
        SotR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORDNOPOP")
            .field("rxdata", &self.rxdata())
            .field("rxssel0_n", &self.rxssel0_n())
            .field("rxssel1_n", &self.rxssel1_n())
            .field("rxssel2_n", &self.rxssel2_n())
            .field("rxssel3_n", &self.rxssel3_n())
            .field("sot", &self.sot())
            .finish()
    }
}
#[doc = "FIFO data read with no FIFO pop.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifordnopop::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifordnopopSpec;
impl crate::RegisterSpec for FifordnopopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifordnopop::R`](R) reader structure"]
impl crate::Readable for FifordnopopSpec {}
#[doc = "`reset()` method sets FIFORDNOPOP to value 0"]
impl crate::Resettable for FifordnopopSpec {
    const RESET_VALUE: u32 = 0;
}
