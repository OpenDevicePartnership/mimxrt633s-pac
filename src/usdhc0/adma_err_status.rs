#[doc = "Register `ADMA_ERR_STATUS` reader"]
pub type R = crate::R<AdmaErrStatusSpec>;
#[doc = "Field `ADMAES` reader - ADMA Error State (when ADMA Error is occurred)"]
pub type AdmaesR = crate::FieldReader;
#[doc = "ADMA Length Mismatch Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admalme {
    #[doc = "0: No Error"]
    Admalme0 = 0,
    #[doc = "1: Error"]
    Admalme1 = 1,
}
impl From<Admalme> for bool {
    #[inline(always)]
    fn from(variant: Admalme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMALME` reader - ADMA Length Mismatch Error"]
pub type AdmalmeR = crate::BitReader<Admalme>;
impl AdmalmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admalme {
        match self.bits {
            false => Admalme::Admalme0,
            true => Admalme::Admalme1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_admalme_0(&self) -> bool {
        *self == Admalme::Admalme0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_admalme_1(&self) -> bool {
        *self == Admalme::Admalme1
    }
}
#[doc = "ADMA Descriptor Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admadce {
    #[doc = "0: No Error"]
    Admadce0 = 0,
    #[doc = "1: Error"]
    Admadce1 = 1,
}
impl From<Admadce> for bool {
    #[inline(always)]
    fn from(variant: Admadce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMADCE` reader - ADMA Descriptor Error"]
pub type AdmadceR = crate::BitReader<Admadce>;
impl AdmadceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admadce {
        match self.bits {
            false => Admadce::Admadce0,
            true => Admadce::Admadce1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_admadce_0(&self) -> bool {
        *self == Admadce::Admadce0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_admadce_1(&self) -> bool {
        *self == Admadce::Admadce1
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State (when ADMA Error is occurred)"]
    #[inline(always)]
    pub fn admaes(&self) -> AdmaesR {
        AdmaesR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> AdmalmeR {
        AdmalmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADMA Descriptor Error"]
    #[inline(always)]
    pub fn admadce(&self) -> AdmadceR {
        AdmadceR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADMA_ERR_STATUS")
            .field("admaes", &self.admaes())
            .field("admalme", &self.admalme())
            .field("admadce", &self.admadce())
            .finish()
    }
}
#[doc = "ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_err_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaErrStatusSpec;
impl crate::RegisterSpec for AdmaErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma_err_status::R`](R) reader structure"]
impl crate::Readable for AdmaErrStatusSpec {}
#[doc = "`reset()` method sets ADMA_ERR_STATUS to value 0"]
impl crate::Resettable for AdmaErrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
