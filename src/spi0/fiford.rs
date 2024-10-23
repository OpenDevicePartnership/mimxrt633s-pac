#[doc = "Register `FIFORD` reader"]
pub type R = crate::R<FifordSpec>;
#[doc = "Field `RXDATA` reader - Received data from the FIFO."]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `RXSSEL0_N` reader - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type Rxssel0NR = crate::BitReader;
#[doc = "Field `RXSSEL1_N` reader - Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type Rxssel1NR = crate::BitReader;
#[doc = "Field `RXSSEL2_N` reader - Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type Rxssel2NR = crate::BitReader;
#[doc = "Field `RXSSEL3_N` reader - Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
pub type Rxssel3NR = crate::BitReader;
#[doc = "Field `SOT` reader - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
pub type SotR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> Rxssel0NR {
        Rxssel0NR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> Rxssel1NR {
        Rxssel1NR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> Rxssel2NR {
        Rxssel2NR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> Rxssel3NR {
        Rxssel3NR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub fn sot(&self) -> SotR {
        SotR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORD")
            .field("rxdata", &self.rxdata())
            .field("rxssel0_n", &self.rxssel0_n())
            .field("rxssel1_n", &self.rxssel1_n())
            .field("rxssel2_n", &self.rxssel2_n())
            .field("rxssel3_n", &self.rxssel3_n())
            .field("sot", &self.sot())
            .finish()
    }
}
#[doc = "FIFO read data.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifordSpec;
impl crate::RegisterSpec for FifordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiford::R`](R) reader structure"]
impl crate::Readable for FifordSpec {}
#[doc = "`reset()` method sets FIFORD to value 0"]
impl crate::Resettable for FifordSpec {
    const RESET_VALUE: u32 = 0;
}
