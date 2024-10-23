#[doc = "Register `SEC_VIO_MISC_INFO[%s]` reader"]
pub type R = crate::R<SecVioMiscInfoSpec>;
#[doc = "Field `SEC_VIO_INFO_WRITE` reader - security violation access read/write indicator, 0: read, 1: write"]
pub type SecVioInfoWriteR = crate::BitReader;
#[doc = "Field `SEC_VIO_INFO_DATA_ACCESS` reader - security violation access data/code indicator, 0: code, 1"]
pub type SecVioInfoDataAccessR = crate::BitReader;
#[doc = "Field `SEC_VIO_INFO_MASTER_SEC_LEVEL` reader - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
pub type SecVioInfoMasterSecLevelR = crate::FieldReader;
#[doc = "Field `SEC_VIO_INFO_MASTER` reader - security violation master number"]
pub type SecVioInfoMasterR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - security violation access read/write indicator, 0: read, 1: write"]
    #[inline(always)]
    pub fn sec_vio_info_write(&self) -> SecVioInfoWriteR {
        SecVioInfoWriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - security violation access data/code indicator, 0: code, 1"]
    #[inline(always)]
    pub fn sec_vio_info_data_access(&self) -> SecVioInfoDataAccessR {
        SecVioInfoDataAccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
    #[inline(always)]
    pub fn sec_vio_info_master_sec_level(&self) -> SecVioInfoMasterSecLevelR {
        SecVioInfoMasterSecLevelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - security violation master number"]
    #[inline(always)]
    pub fn sec_vio_info_master(&self) -> SecVioInfoMasterR {
        SecVioInfoMasterR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_VIO_MISC_INFO")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[doc = "most recent security violation miscellaneous information for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_misc_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecVioMiscInfoSpec;
impl crate::RegisterSpec for SecVioMiscInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_vio_misc_info::R`](R) reader structure"]
impl crate::Readable for SecVioMiscInfoSpec {}
#[doc = "`reset()` method sets SEC_VIO_MISC_INFO[%s]
to value 0"]
impl crate::Resettable for SecVioMiscInfoSpec {
    const RESET_VALUE: u32 = 0;
}
