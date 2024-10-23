#[doc = "Register `CTX_RGD_W1` reader"]
pub type R = crate::R<CtxRgdW1Spec>;
#[doc = "Register `CTX_RGD_W1` writer"]
pub type W = crate::W<CtxRgdW1Spec>;
#[doc = "Valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vld {
    #[doc = "0: Context is invalid."]
    Vld0 = 0,
    #[doc = "1: Context is valid."]
    Vld1 = 1,
}
impl From<Vld> for bool {
    #[inline(always)]
    fn from(variant: Vld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLD` reader - Valid"]
pub type VldR = crate::BitReader<Vld>;
impl VldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vld {
        match self.bits {
            false => Vld::Vld0,
            true => Vld::Vld1,
        }
    }
    #[doc = "Context is invalid."]
    #[inline(always)]
    pub fn is_vld_0(&self) -> bool {
        *self == Vld::Vld0
    }
    #[doc = "Context is valid."]
    #[inline(always)]
    pub fn is_vld_1(&self) -> bool {
        *self == Vld::Vld1
    }
}
#[doc = "Field `VLD` writer - Valid"]
pub type VldW<'a, REG> = crate::BitWriter<'a, REG, Vld>;
impl<'a, REG> VldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Context is invalid."]
    #[inline(always)]
    pub fn vld_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vld::Vld0)
    }
    #[doc = "Context is valid."]
    #[inline(always)]
    pub fn vld_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vld::Vld1)
    }
}
#[doc = "AES Decryption Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ade {
    #[doc = "0: Bypass the fetched data."]
    Ade0 = 0,
    #[doc = "1: Perform the CTR-AES128 mode decryption on the fetched data."]
    Ade1 = 1,
}
impl From<Ade> for bool {
    #[inline(always)]
    fn from(variant: Ade) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADE` reader - AES Decryption Enable."]
pub type AdeR = crate::BitReader<Ade>;
impl AdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ade {
        match self.bits {
            false => Ade::Ade0,
            true => Ade::Ade1,
        }
    }
    #[doc = "Bypass the fetched data."]
    #[inline(always)]
    pub fn is_ade_0(&self) -> bool {
        *self == Ade::Ade0
    }
    #[doc = "Perform the CTR-AES128 mode decryption on the fetched data."]
    #[inline(always)]
    pub fn is_ade_1(&self) -> bool {
        *self == Ade::Ade1
    }
}
#[doc = "Field `ADE` writer - AES Decryption Enable."]
pub type AdeW<'a, REG> = crate::BitWriter<'a, REG, Ade>;
impl<'a, REG> AdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass the fetched data."]
    #[inline(always)]
    pub fn ade_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ade::Ade0)
    }
    #[doc = "Perform the CTR-AES128 mode decryption on the fetched data."]
    #[inline(always)]
    pub fn ade_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ade::Ade1)
    }
}
#[doc = "Read-Only\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ro {
    #[doc = "0: The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
    Ro0 = 0,
    #[doc = "1: The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    Ro1 = 1,
}
impl From<Ro> for bool {
    #[inline(always)]
    fn from(variant: Ro) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RO` reader - Read-Only"]
pub type RoR = crate::BitReader<Ro>;
impl RoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ro {
        match self.bits {
            false => Ro::Ro0,
            true => Ro::Ro1,
        }
    }
    #[doc = "The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
    #[inline(always)]
    pub fn is_ro_0(&self) -> bool {
        *self == Ro::Ro0
    }
    #[doc = "The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    #[inline(always)]
    pub fn is_ro_1(&self) -> bool {
        *self == Ro::Ro1
    }
}
#[doc = "Field `RO` writer - Read-Only"]
pub type RoW<'a, REG> = crate::BitWriter<'a, REG, Ro>;
impl<'a, REG> RoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
    #[inline(always)]
    pub fn ro_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ro::Ro0)
    }
    #[doc = "The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    #[inline(always)]
    pub fn ro_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ro::Ro1)
    }
}
#[doc = "Field `ENDADDR` reader - End Address"]
pub type EndaddrR = crate::FieldReader<u32>;
#[doc = "Field `ENDADDR` writer - End Address"]
pub type EndaddrW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VldR {
        VldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES Decryption Enable."]
    #[inline(always)]
    pub fn ade(&self) -> AdeR {
        AdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read-Only"]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 10:31 - End Address"]
    #[inline(always)]
    pub fn endaddr(&self) -> EndaddrR {
        EndaddrR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTX_RGD_W1")
            .field("vld", &self.vld())
            .field("ade", &self.ade())
            .field("ro", &self.ro())
            .field("endaddr", &self.endaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn vld(&mut self) -> VldW<CtxRgdW1Spec> {
        VldW::new(self, 0)
    }
    #[doc = "Bit 1 - AES Decryption Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ade(&mut self) -> AdeW<CtxRgdW1Spec> {
        AdeW::new(self, 1)
    }
    #[doc = "Bit 2 - Read-Only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RoW<CtxRgdW1Spec> {
        RoW::new(self, 2)
    }
    #[doc = "Bits 10:31 - End Address"]
    #[inline(always)]
    #[must_use]
    pub fn endaddr(&mut self) -> EndaddrW<CtxRgdW1Spec> {
        EndaddrW::new(self, 10)
    }
}
#[doc = "AES Region Descriptor Word1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_rgd_w1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_rgd_w1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxRgdW1Spec;
impl crate::RegisterSpec for CtxRgdW1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_rgd_w1::R`](R) reader structure"]
impl crate::Readable for CtxRgdW1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_rgd_w1::W`](W) writer structure"]
impl crate::Writable for CtxRgdW1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTX_RGD_W1 to value 0x03f8"]
impl crate::Resettable for CtxRgdW1Spec {
    const RESET_VALUE: u32 = 0x03f8;
}
