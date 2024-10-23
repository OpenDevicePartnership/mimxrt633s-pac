#[doc = "Register `SMSGMAPADDR` reader"]
pub type R = crate::R<SmsgmapaddrSpec>;
#[doc = "Field `MAPLAST` reader - Matched address index"]
pub type MaplastR = crate::FieldReader;
#[doc = "Field `MAPLASTM1` reader - Previous match index 1"]
pub type Maplastm1R = crate::FieldReader;
#[doc = "Field `MAPLASTM2` reader - Previous match index 2"]
pub type Maplastm2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Matched address index"]
    #[inline(always)]
    pub fn maplast(&self) -> MaplastR {
        MaplastR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Previous match index 1"]
    #[inline(always)]
    pub fn maplastm1(&self) -> Maplastm1R {
        Maplastm1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Previous match index 2"]
    #[inline(always)]
    pub fn maplastm2(&self) -> Maplastm2R {
        Maplastm2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMSGMAPADDR")
            .field("maplast", &self.maplast())
            .field("maplastm1", &self.maplastm1())
            .field("maplastm2", &self.maplastm2())
            .finish()
    }
}
#[doc = "Slave Message-Mapped Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smsgmapaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmsgmapaddrSpec;
impl crate::RegisterSpec for SmsgmapaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smsgmapaddr::R`](R) reader structure"]
impl crate::Readable for SmsgmapaddrSpec {}
#[doc = "`reset()` method sets SMSGMAPADDR to value 0x0214"]
impl crate::Resettable for SmsgmapaddrSpec {
    const RESET_VALUE: u32 = 0x0214;
}
