#[doc = "Register `OSR` reader"]
pub type R = crate::R<OsrSpec>;
#[doc = "Register `OSR` writer"]
pub type W = crate::W<OsrSpec>;
#[doc = "Field `OSRVAL` reader - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
pub type OsrvalR = crate::FieldReader;
#[doc = "Field `OSRVAL` writer - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
pub type OsrvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn osrval(&self) -> OsrvalR {
        OsrvalR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSR")
            .field("osrval", &self.osrval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn osrval(&mut self) -> OsrvalW<OsrSpec> {
        OsrvalW::new(self, 0)
    }
}
#[doc = "Oversample selection register for asynchronous communication.\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsrSpec;
impl crate::RegisterSpec for OsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osr::R`](R) reader structure"]
impl crate::Readable for OsrSpec {}
#[doc = "`write(|w| ..)` method takes [`osr::W`](W) writer structure"]
impl crate::Writable for OsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSR to value 0x0f"]
impl crate::Resettable for OsrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
