#[doc = "Register `P1RAMUse` reader"]
pub type R = crate::R<P1ramuseSpec>;
#[doc = "Register `P1RAMUse` writer"]
pub type W = crate::W<P1ramuseSpec>;
#[doc = "Field `OFF` reader - This is the word offset into the RAM"]
pub type OffR = crate::FieldReader<u16>;
#[doc = "Field `OFF` writer - This is the word offset into the RAM"]
pub type OffW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LEN` reader - This is the length of the mailbox or mastering area as 4&lt;&lt;LEN per direction"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - This is the length of the mailbox or mastering area as 4&lt;&lt;LEN per direction"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - This is the word offset into the RAM"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - This is the length of the mailbox or mastering area as 4&lt;&lt;LEN per direction"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1RAMUse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This is the word offset into the RAM"]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OffW<P1ramuseSpec> {
        OffW::new(self, 0)
    }
    #[doc = "Bits 16:18 - This is the length of the mailbox or mastering area as 4&lt;&lt;LEN per direction"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<P1ramuseSpec> {
        LenW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ramuse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ramuse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ramuseSpec;
impl crate::RegisterSpec for P1ramuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p1ramuse::R`](R) reader structure"]
impl crate::Readable for P1ramuseSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ramuse::W`](W) writer structure"]
impl crate::Writable for P1ramuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P1RAMUse to value 0"]
impl crate::Resettable for P1ramuseSpec {
    const RESET_VALUE: u32 = 0;
}
