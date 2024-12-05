#[doc = "Register `RAMUSE` reader"]
pub type R = crate::R<RamuseSpec>;
#[doc = "Register `RAMUSE` writer"]
pub type W = crate::W<RamuseSpec>;
#[doc = "Field `OFF` reader - This is the word offset into the RAM"]
pub type OffR = crate::FieldReader<u16>;
#[doc = "Field `OFF` writer - This is the word offset into the RAM"]
pub type OffW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LEN` reader - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - This is the word offset into the RAM"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMUSE")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This is the word offset into the RAM"]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<RamuseSpec> {
        OffW::new(self, 0)
    }
    #[doc = "Bits 16:18 - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<RamuseSpec> {
        LenW::new(self, 16)
    }
}
#[doc = "Port RAM base and size (for Mailbox and Bus Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`ramuse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramuse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamuseSpec;
impl crate::RegisterSpec for RamuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramuse::R`](R) reader structure"]
impl crate::Readable for RamuseSpec {}
#[doc = "`write(|w| ..)` method takes [`ramuse::W`](W) writer structure"]
impl crate::Writable for RamuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMUSE to value 0"]
impl crate::Resettable for RamuseSpec {
    const RESET_VALUE: u32 = 0;
}
