#[doc = "Register `PKRCNT98` reader"]
pub type R = crate::R<Pkrcnt98Spec>;
#[doc = "Field `PKR_8_CT` reader - Poker 8h Count"]
pub type Pkr8CtR = crate::FieldReader<u16>;
#[doc = "Field `PKR_9_CT` reader - Poker 9h Count"]
pub type Pkr9CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 8h Count"]
    #[inline(always)]
    pub fn pkr_8_ct(&self) -> Pkr8CtR {
        Pkr8CtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 9h Count"]
    #[inline(always)]
    pub fn pkr_9_ct(&self) -> Pkr9CtR {
        Pkr9CtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKRCNT98")
            .field("pkr_8_ct", &self.pkr_8_ct())
            .field("pkr_9_ct", &self.pkr_9_ct())
            .finish()
    }
}
#[doc = "Statistical Check Poker Count 9 and 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcnt98::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkrcnt98Spec;
impl crate::RegisterSpec for Pkrcnt98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkrcnt98::R`](R) reader structure"]
impl crate::Readable for Pkrcnt98Spec {}
#[doc = "`reset()` method sets PKRCNT98 to value 0"]
impl crate::Resettable for Pkrcnt98Spec {
    const RESET_VALUE: u32 = 0;
}
