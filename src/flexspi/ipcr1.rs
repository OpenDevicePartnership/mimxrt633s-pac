#[doc = "Register `IPCR1` reader"]
pub type R = crate::R<Ipcr1Spec>;
#[doc = "Register `IPCR1` writer"]
pub type W = crate::W<Ipcr1Spec>;
#[doc = "Field `IDATSZ` reader - Flash Read/Program Data Size (in Bytes) for IP command."]
pub type IdatszR = crate::FieldReader<u16>;
#[doc = "Field `IDATSZ` writer - Flash Read/Program Data Size (in Bytes) for IP command."]
pub type IdatszW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ISEQID` reader - Sequence Index in LUT for IP command."]
pub type IseqidR = crate::FieldReader;
#[doc = "Field `ISEQID` writer - Sequence Index in LUT for IP command."]
pub type IseqidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ISEQNUM` reader - Sequence Number for IP command: ISEQNUM+1."]
pub type IseqnumR = crate::FieldReader;
#[doc = "Field `ISEQNUM` writer - Sequence Number for IP command: ISEQNUM+1."]
pub type IseqnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Parallel mode Enabled for IP command.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iparen {
    #[doc = "0: Flash will be accessed in Individual mode."]
    Iparen0 = 0,
    #[doc = "1: Flash will be accessed in Parallel mode."]
    Iparen1 = 1,
}
impl From<Iparen> for bool {
    #[inline(always)]
    fn from(variant: Iparen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPAREN` reader - Parallel mode Enabled for IP command."]
pub type IparenR = crate::BitReader<Iparen>;
impl IparenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iparen {
        match self.bits {
            false => Iparen::Iparen0,
            true => Iparen::Iparen1,
        }
    }
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn is_iparen_0(&self) -> bool {
        *self == Iparen::Iparen0
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn is_iparen_1(&self) -> bool {
        *self == Iparen::Iparen1
    }
}
#[doc = "Field `IPAREN` writer - Parallel mode Enabled for IP command."]
pub type IparenW<'a, REG> = crate::BitWriter<'a, REG, Iparen>;
impl<'a, REG> IparenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn iparen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Iparen::Iparen0)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn iparen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Iparen::Iparen1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub fn idatsz(&self) -> IdatszR {
        IdatszR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub fn iseqid(&self) -> IseqidR {
        IseqidR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub fn iseqnum(&self) -> IseqnumR {
        IseqnumR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub fn iparen(&self) -> IparenR {
        IparenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCR1")
            .field("idatsz", &self.idatsz())
            .field("iseqid", &self.iseqid())
            .field("iseqnum", &self.iseqnum())
            .field("iparen", &self.iparen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn idatsz(&mut self) -> IdatszW<Ipcr1Spec> {
        IdatszW::new(self, 0)
    }
    #[doc = "Bits 16:20 - Sequence Index in LUT for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn iseqid(&mut self) -> IseqidW<Ipcr1Spec> {
        IseqidW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    #[must_use]
    pub fn iseqnum(&mut self) -> IseqnumW<Ipcr1Spec> {
        IseqnumW::new(self, 24)
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn iparen(&mut self) -> IparenW<Ipcr1Spec> {
        IparenW::new(self, 31)
    }
}
#[doc = "IP Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipcr1Spec;
impl crate::RegisterSpec for Ipcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcr1::R`](R) reader structure"]
impl crate::Readable for Ipcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ipcr1::W`](W) writer structure"]
impl crate::Writable for Ipcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCR1 to value 0"]
impl crate::Resettable for Ipcr1Spec {
    const RESET_VALUE: u32 = 0;
}
