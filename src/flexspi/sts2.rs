#[doc = "Register `STS2` reader"]
pub type R = crate::R<Sts2Spec>;
#[doc = "Field `ASLVLOCK` reader - Flash A sample clock slave delay line locked."]
pub type AslvlockR = crate::BitReader;
#[doc = "Field `AREFLOCK` reader - Flash A sample clock reference delay line locked."]
pub type AreflockR = crate::BitReader;
#[doc = "Field `ASLVSEL` reader - Flash A sample clock slave delay line delay cell number selection ."]
pub type AslvselR = crate::FieldReader;
#[doc = "Field `AREFSEL` reader - Flash A sample clock reference delay line delay cell number selection."]
pub type ArefselR = crate::FieldReader;
#[doc = "Field `BSLVLOCK` reader - Flash B sample clock slave delay line locked."]
pub type BslvlockR = crate::BitReader;
#[doc = "Field `BREFLOCK` reader - Flash B sample clock reference delay line locked."]
pub type BreflockR = crate::BitReader;
#[doc = "Field `BSLVSEL` reader - Flash B sample clock slave delay line delay cell number selection."]
pub type BslvselR = crate::FieldReader;
#[doc = "Field `BREFSEL` reader - Flash B sample clock reference delay line delay cell number selection."]
pub type BrefselR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub fn aslvlock(&self) -> AslvlockR {
        AslvlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub fn areflock(&self) -> AreflockR {
        AreflockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub fn aslvsel(&self) -> AslvselR {
        AslvselR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn arefsel(&self) -> ArefselR {
        ArefselR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub fn bslvlock(&self) -> BslvlockR {
        BslvlockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub fn breflock(&self) -> BreflockR {
        BreflockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub fn bslvsel(&self) -> BslvselR {
        BslvselR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn brefsel(&self) -> BrefselR {
        BrefselR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS2")
            .field("aslvlock", &self.aslvlock())
            .field("areflock", &self.areflock())
            .field("aslvsel", &self.aslvsel())
            .field("arefsel", &self.arefsel())
            .field("bslvlock", &self.bslvlock())
            .field("breflock", &self.breflock())
            .field("bslvsel", &self.bslvsel())
            .field("brefsel", &self.brefsel())
            .finish()
    }
}
#[doc = "Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts2Spec;
impl crate::RegisterSpec for Sts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for Sts2Spec {}
#[doc = "`reset()` method sets STS2 to value 0x0100_0100"]
impl crate::Resettable for Sts2Spec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
