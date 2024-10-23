#[doc = "Register `ALLOW` reader"]
pub type R = crate::R<AllowSpec>;
#[doc = "Field `ALLOWENROLL` reader - Allow Enroll"]
pub type AllowenrollR = crate::BitReader;
#[doc = "Field `ALLOWSTART` reader - Allow Start"]
pub type AllowstartR = crate::BitReader;
#[doc = "Field `ALLOWSETKEY` reader - Allow Set Key"]
pub type AllowsetkeyR = crate::BitReader;
#[doc = "Field `ALLOWGETKEY` reader - Allow Get Key"]
pub type AllowgetkeyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Allow Enroll"]
    #[inline(always)]
    pub fn allowenroll(&self) -> AllowenrollR {
        AllowenrollR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow Start"]
    #[inline(always)]
    pub fn allowstart(&self) -> AllowstartR {
        AllowstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow Set Key"]
    #[inline(always)]
    pub fn allowsetkey(&self) -> AllowsetkeyR {
        AllowsetkeyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow Get Key"]
    #[inline(always)]
    pub fn allowgetkey(&self) -> AllowgetkeyR {
        AllowgetkeyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALLOW")
            .field("allowenroll", &self.allowenroll())
            .field("allowstart", &self.allowstart())
            .field("allowsetkey", &self.allowsetkey())
            .field("allowgetkey", &self.allowgetkey())
            .finish()
    }
}
#[doc = "PUF Allow\n\nYou can [`read`](crate::Reg::read) this register and get [`allow::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AllowSpec;
impl crate::RegisterSpec for AllowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`allow::R`](R) reader structure"]
impl crate::Readable for AllowSpec {}
#[doc = "`reset()` method sets ALLOW to value 0"]
impl crate::Resettable for AllowSpec {
    const RESET_VALUE: u32 = 0;
}
