#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Field `FRAME_NR` reader - Frame number."]
pub type FrameNrR = crate::FieldReader<u16>;
#[doc = "Field `ERR_CODE` reader - The error code which last occurred:."]
pub type ErrCodeR = crate::FieldReader;
#[doc = "Field `Minrev` reader - Minor revision."]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `Majrev` reader - Major revision."]
pub type MajrevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - Frame number."]
    #[inline(always)]
    pub fn frame_nr(&self) -> FrameNrR {
        FrameNrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - The error code which last occurred:."]
    #[inline(always)]
    pub fn err_code(&self) -> ErrCodeR {
        ErrCodeR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Minor revision."]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major revision."]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFO")
            .field("frame_nr", &self.frame_nr())
            .field("err_code", &self.err_code())
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
#[doc = "USB Info register\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`reset()` method sets INFO to value 0x0200_0000"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
