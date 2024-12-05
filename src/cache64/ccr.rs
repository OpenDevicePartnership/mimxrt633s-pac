#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Cache enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encache {
    #[doc = "0: Cache disabled"]
    Disabled = 0,
    #[doc = "1: Cache enabled"]
    Enabled = 1,
}
impl From<Encache> for bool {
    #[inline(always)]
    fn from(variant: Encache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENCACHE` reader - Cache enable"]
pub type EncacheR = crate::BitReader<Encache>;
impl EncacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Encache {
        match self.bits {
            false => Encache::Disabled,
            true => Encache::Enabled,
        }
    }
    #[doc = "Cache disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Encache::Disabled
    }
    #[doc = "Cache enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Encache::Enabled
    }
}
#[doc = "Field `ENCACHE` writer - Cache enable"]
pub type EncacheW<'a, REG> = crate::BitWriter<'a, REG, Encache>;
impl<'a, REG> EncacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Encache::Disabled)
    }
    #[doc = "Cache enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Encache::Enabled)
    }
}
#[doc = "Enable Write Buffer\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enwrbuf {
    #[doc = "0: Write buffer disabled"]
    Disabled = 0,
    #[doc = "1: Write buffer enabled"]
    Enabled = 1,
}
impl From<Enwrbuf> for bool {
    #[inline(always)]
    fn from(variant: Enwrbuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENWRBUF` reader - Enable Write Buffer"]
pub type EnwrbufR = crate::BitReader<Enwrbuf>;
impl EnwrbufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enwrbuf {
        match self.bits {
            false => Enwrbuf::Disabled,
            true => Enwrbuf::Enabled,
        }
    }
    #[doc = "Write buffer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enwrbuf::Disabled
    }
    #[doc = "Write buffer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enwrbuf::Enabled
    }
}
#[doc = "Field `ENWRBUF` writer - Enable Write Buffer"]
pub type EnwrbufW<'a, REG> = crate::BitWriter<'a, REG, Enwrbuf>;
impl<'a, REG> EnwrbufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enwrbuf::Disabled)
    }
    #[doc = "Write buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enwrbuf::Enabled)
    }
}
#[doc = "Invalidate Way 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invw0 {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: When setting the GO bit, invalidate all lines in way 0."]
    Invw0 = 1,
}
impl From<Invw0> for bool {
    #[inline(always)]
    fn from(variant: Invw0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVW0` reader - Invalidate Way 0"]
pub type Invw0R = crate::BitReader<Invw0>;
impl Invw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invw0 {
        match self.bits {
            false => Invw0::NoOperation,
            true => Invw0::Invw0,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Invw0::NoOperation
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    #[inline(always)]
    pub fn is_invw0(&self) -> bool {
        *self == Invw0::Invw0
    }
}
#[doc = "Field `INVW0` writer - Invalidate Way 0"]
pub type Invw0W<'a, REG> = crate::BitWriter<'a, REG, Invw0>;
impl<'a, REG> Invw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Invw0::NoOperation)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    #[inline(always)]
    pub fn invw0(self) -> &'a mut crate::W<REG> {
        self.variant(Invw0::Invw0)
    }
}
#[doc = "Push Way 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pushw0 {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: When setting the GO bit, push all modified lines in way 0"]
    Pushw0 = 1,
}
impl From<Pushw0> for bool {
    #[inline(always)]
    fn from(variant: Pushw0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUSHW0` reader - Push Way 0"]
pub type Pushw0R = crate::BitReader<Pushw0>;
impl Pushw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pushw0 {
        match self.bits {
            false => Pushw0::NoOperation,
            true => Pushw0::Pushw0,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Pushw0::NoOperation
    }
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    #[inline(always)]
    pub fn is_pushw0(&self) -> bool {
        *self == Pushw0::Pushw0
    }
}
#[doc = "Field `PUSHW0` writer - Push Way 0"]
pub type Pushw0W<'a, REG> = crate::BitWriter<'a, REG, Pushw0>;
impl<'a, REG> Pushw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Pushw0::NoOperation)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    #[inline(always)]
    pub fn pushw0(self) -> &'a mut crate::W<REG> {
        self.variant(Pushw0::Pushw0)
    }
}
#[doc = "Invalidate Way 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invw1 {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: When setting the GO bit, invalidate all lines in way 1"]
    Invw1 = 1,
}
impl From<Invw1> for bool {
    #[inline(always)]
    fn from(variant: Invw1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVW1` reader - Invalidate Way 1"]
pub type Invw1R = crate::BitReader<Invw1>;
impl Invw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invw1 {
        match self.bits {
            false => Invw1::NoOperation,
            true => Invw1::Invw1,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Invw1::NoOperation
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    #[inline(always)]
    pub fn is_invw1(&self) -> bool {
        *self == Invw1::Invw1
    }
}
#[doc = "Field `INVW1` writer - Invalidate Way 1"]
pub type Invw1W<'a, REG> = crate::BitWriter<'a, REG, Invw1>;
impl<'a, REG> Invw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Invw1::NoOperation)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    #[inline(always)]
    pub fn invw1(self) -> &'a mut crate::W<REG> {
        self.variant(Invw1::Invw1)
    }
}
#[doc = "Push Way 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pushw1 {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: When setting the GO bit, push all modified lines in way 1"]
    Pushw1 = 1,
}
impl From<Pushw1> for bool {
    #[inline(always)]
    fn from(variant: Pushw1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUSHW1` reader - Push Way 1"]
pub type Pushw1R = crate::BitReader<Pushw1>;
impl Pushw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pushw1 {
        match self.bits {
            false => Pushw1::NoOperation,
            true => Pushw1::Pushw1,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Pushw1::NoOperation
    }
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    #[inline(always)]
    pub fn is_pushw1(&self) -> bool {
        *self == Pushw1::Pushw1
    }
}
#[doc = "Field `PUSHW1` writer - Push Way 1"]
pub type Pushw1W<'a, REG> = crate::BitWriter<'a, REG, Pushw1>;
impl<'a, REG> Pushw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Pushw1::NoOperation)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    #[inline(always)]
    pub fn pushw1(self) -> &'a mut crate::W<REG> {
        self.variant(Pushw1::Pushw1)
    }
}
#[doc = "Initiate Cache Command\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Go {
    #[doc = "0: Write: no effect. Read: no cache command active."]
    NoEffect = 0,
    #[doc = "1: Write: initiate command indicated by bits 27-24. Read: cache command active."]
    InitCmd = 1,
}
impl From<Go> for bool {
    #[inline(always)]
    fn from(variant: Go) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GO` reader - Initiate Cache Command"]
pub type GoR = crate::BitReader<Go>;
impl GoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Go {
        match self.bits {
            false => Go::NoEffect,
            true => Go::InitCmd,
        }
    }
    #[doc = "Write: no effect. Read: no cache command active."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Go::NoEffect
    }
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == Go::InitCmd
    }
}
#[doc = "Field `GO` writer - Initiate Cache Command"]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG, Go>;
impl<'a, REG> GoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: no effect. Read: no cache command active."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Go::NoEffect)
    }
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Go::InitCmd)
    }
}
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn encache(&self) -> EncacheR {
        EncacheR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Buffer"]
    #[inline(always)]
    pub fn enwrbuf(&self) -> EnwrbufR {
        EnwrbufR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&self) -> Invw0R {
        Invw0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&self) -> Pushw0R {
        Pushw0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&self) -> Invw1R {
        Invw1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&self) -> Pushw1R {
        Pushw1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&self) -> GoR {
        GoR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("encache", &self.encache())
            .field("enwrbuf", &self.enwrbuf())
            .field("invw0", &self.invw0())
            .field("pushw0", &self.pushw0())
            .field("invw1", &self.invw1())
            .field("pushw1", &self.pushw1())
            .field("go", &self.go())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn encache(&mut self) -> EncacheW<CcrSpec> {
        EncacheW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Buffer"]
    #[inline(always)]
    pub fn enwrbuf(&mut self) -> EnwrbufW<CcrSpec> {
        EnwrbufW::new(self, 1)
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&mut self) -> Invw0W<CcrSpec> {
        Invw0W::new(self, 24)
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&mut self) -> Pushw0W<CcrSpec> {
        Pushw0W::new(self, 25)
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&mut self) -> Invw1W<CcrSpec> {
        Invw1W::new(self, 26)
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&mut self) -> Pushw1W<CcrSpec> {
        Pushw1W::new(self, 27)
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&mut self) -> GoW<CcrSpec> {
        GoW::new(self, 31)
    }
}
#[doc = "Cache control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
