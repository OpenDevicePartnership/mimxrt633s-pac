#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Indicates if should interrupt when waiting for data input.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waiting {
    #[doc = "0: Will not interrupt when waiting."]
    NoInterrupt = 0,
    #[doc = "1: Will interrupt when waiting"]
    Interrupt = 1,
}
impl From<Waiting> for bool {
    #[inline(always)]
    fn from(variant: Waiting) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITING` reader - Indicates if should interrupt when waiting for data input."]
pub type WaitingR = crate::BitReader<Waiting>;
impl WaitingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Waiting {
        match self.bits {
            false => Waiting::NoInterrupt,
            true => Waiting::Interrupt,
        }
    }
    #[doc = "Will not interrupt when waiting."]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Waiting::NoInterrupt
    }
    #[doc = "Will interrupt when waiting"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Waiting::Interrupt
    }
}
#[doc = "Field `WAITING` writer - Indicates if should interrupt when waiting for data input."]
pub type WaitingW<'a, REG> = crate::BitWriter<'a, REG, Waiting>;
impl<'a, REG> WaitingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Will not interrupt when waiting."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Waiting::NoInterrupt)
    }
    #[doc = "Will interrupt when waiting"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Waiting::Interrupt)
    }
}
#[doc = "Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digest {
    #[doc = "0: Will not interrupt when Digest is ready"]
    NoInterrupt = 0,
    #[doc = "1: Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    Interrupt = 1,
}
impl From<Digest> for bool {
    #[inline(always)]
    fn from(variant: Digest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGEST` reader - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
pub type DigestR = crate::BitReader<Digest>;
impl DigestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digest {
        match self.bits {
            false => Digest::NoInterrupt,
            true => Digest::Interrupt,
        }
    }
    #[doc = "Will not interrupt when Digest is ready"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Digest::NoInterrupt
    }
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Digest::Interrupt
    }
}
#[doc = "Field `DIGEST` writer - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
pub type DigestW<'a, REG> = crate::BitWriter<'a, REG, Digest>;
impl<'a, REG> DigestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Will not interrupt when Digest is ready"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Digest::NoInterrupt)
    }
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Digest::Interrupt)
    }
}
#[doc = "Indicates if should interrupt on an ERROR (as defined in Status)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Will not interrupt on Error."]
    NoInterrupt = 0,
    #[doc = "1: Will interrupt on Error (until cleared)."]
    Interrupt = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Indicates if should interrupt on an ERROR (as defined in Status)"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::NoInterrupt,
            true => Error::Interrupt,
        }
    }
    #[doc = "Will not interrupt on Error."]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Error::NoInterrupt
    }
    #[doc = "Will interrupt on Error (until cleared)."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Error::Interrupt
    }
}
#[doc = "Field `ERROR` writer - Indicates if should interrupt on an ERROR (as defined in Status)"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Will not interrupt on Error."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Error::NoInterrupt)
    }
    #[doc = "Will interrupt on Error (until cleared)."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Interrupt)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if should interrupt when waiting for data input."]
    #[inline(always)]
    pub fn waiting(&self) -> WaitingR {
        WaitingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline(always)]
    pub fn digest(&self) -> DigestR {
        DigestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("waiting", &self.waiting())
            .field("digest", &self.digest())
            .field("error", &self.error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if should interrupt when waiting for data input."]
    #[inline(always)]
    pub fn waiting(&mut self) -> WaitingW<IntensetSpec> {
        WaitingW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline(always)]
    pub fn digest(&mut self) -> DigestW<IntensetSpec> {
        DigestW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IntensetSpec> {
        ErrorW::new(self, 2)
    }
}
#[doc = "Write 1 to enable interrupts; reads back with which are set.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
