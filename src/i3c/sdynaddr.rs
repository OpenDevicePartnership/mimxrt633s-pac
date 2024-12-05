#[doc = "Register `SDYNADDR` reader"]
pub type R = crate::R<SdynaddrSpec>;
#[doc = "Register `SDYNADDR` writer"]
pub type W = crate::W<SdynaddrSpec>;
#[doc = "DAVALID\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Davalid {
    #[doc = "0: DANOTASSIGNED: a Dynamic Address is not assigned"]
    Danotassigned = 0,
    #[doc = "1: DAASSIGNED: a Dynamic Address is assigned"]
    Daassigned = 1,
}
impl From<Davalid> for bool {
    #[inline(always)]
    fn from(variant: Davalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAVALID` reader - DAVALID"]
pub type DavalidR = crate::BitReader<Davalid>;
impl DavalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Davalid {
        match self.bits {
            false => Davalid::Danotassigned,
            true => Davalid::Daassigned,
        }
    }
    #[doc = "DANOTASSIGNED: a Dynamic Address is not assigned"]
    #[inline(always)]
    pub fn is_danotassigned(&self) -> bool {
        *self == Davalid::Danotassigned
    }
    #[doc = "DAASSIGNED: a Dynamic Address is assigned"]
    #[inline(always)]
    pub fn is_daassigned(&self) -> bool {
        *self == Davalid::Daassigned
    }
}
#[doc = "Field `DAVALID` writer - DAVALID"]
pub type DavalidW<'a, REG> = crate::BitWriter<'a, REG, Davalid>;
impl<'a, REG> DavalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DANOTASSIGNED: a Dynamic Address is not assigned"]
    #[inline(always)]
    pub fn danotassigned(self) -> &'a mut crate::W<REG> {
        self.variant(Davalid::Danotassigned)
    }
    #[doc = "DAASSIGNED: a Dynamic Address is assigned"]
    #[inline(always)]
    pub fn daassigned(self) -> &'a mut crate::W<REG> {
        self.variant(Davalid::Daassigned)
    }
}
#[doc = "Field `DADDR` reader - Dynamic address"]
pub type DaddrR = crate::FieldReader;
#[doc = "Field `DADDR` writer - Dynamic address"]
pub type DaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MAPIDX` writer - Mapped Dynamic Address"]
pub type MapidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAPSA` writer - Map a Static Address"]
pub type MapsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - Key"]
pub type KeyR = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - DAVALID"]
    #[inline(always)]
    pub fn davalid(&self) -> DavalidR {
        DavalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    pub fn daddr(&self) -> DaddrR {
        DaddrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDYNADDR")
            .field("davalid", &self.davalid())
            .field("daddr", &self.daddr())
            .field("key", &self.key())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DAVALID"]
    #[inline(always)]
    pub fn davalid(&mut self) -> DavalidW<SdynaddrSpec> {
        DavalidW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    pub fn daddr(&mut self) -> DaddrW<SdynaddrSpec> {
        DaddrW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Mapped Dynamic Address"]
    #[inline(always)]
    pub fn mapidx(&mut self) -> MapidxW<SdynaddrSpec> {
        MapidxW::new(self, 8)
    }
    #[doc = "Bit 12 - Map a Static Address"]
    #[inline(always)]
    pub fn mapsa(&mut self) -> MapsaW<SdynaddrSpec> {
        MapsaW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<SdynaddrSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Slave Dynamic Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdynaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdynaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdynaddrSpec;
impl crate::RegisterSpec for SdynaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdynaddr::R`](R) reader structure"]
impl crate::Readable for SdynaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdynaddr::W`](W) writer structure"]
impl crate::Writable for SdynaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDYNADDR to value 0"]
impl crate::Resettable for SdynaddrSpec {
    const RESET_VALUE: u32 = 0;
}
