#[doc = "Register `DATAIN` reader"]
pub type R = crate::R<DatainSpec>;
#[doc = "Field `DATA_LEN` reader - Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
pub type DataLenR = crate::FieldReader;
#[doc = "Field `DIR` reader - Direction of last access: 0 = Read By Host 1 = Write By Host"]
pub type DirR = crate::BitReader;
#[doc = "Field `IDX` reader - Index of last access (ie"]
pub type IdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[inline(always)]
    pub fn data_len(&self) -> DataLenR {
        DataLenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Index of last access (ie"]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAIN")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[doc = "Port Data from Host\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatainSpec;
impl crate::RegisterSpec for DatainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datain::R`](R) reader structure"]
impl crate::Readable for DatainSpec {}
#[doc = "`reset()` method sets DATAIN to value 0"]
impl crate::Resettable for DatainSpec {
    const RESET_VALUE: u32 = 0;
}
