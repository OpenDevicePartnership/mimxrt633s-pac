#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Block Enroll and Set Key Operation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockenrollSetkey {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<BlockenrollSetkey> for bool {
    #[inline(always)]
    fn from(variant: BlockenrollSetkey) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` reader - Block Enroll and Set Key Operation"]
pub type BlockenrollSetkeyR = crate::BitReader<BlockenrollSetkey>;
impl BlockenrollSetkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockenrollSetkey {
        match self.bits {
            false => BlockenrollSetkey::Disabled,
            true => BlockenrollSetkey::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BlockenrollSetkey::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BlockenrollSetkey::Enabled
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` writer - Block Enroll and Set Key Operation"]
pub type BlockenrollSetkeyW<'a, REG> = crate::BitWriter<'a, REG, BlockenrollSetkey>;
impl<'a, REG> BlockenrollSetkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BlockenrollSetkey::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BlockenrollSetkey::Enabled)
    }
}
#[doc = "Block Key Output Data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockkeyoutput {
    #[doc = "0: Disabled. BLOCKKEYOUTPUT is cleared on reset."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<Blockkeyoutput> for bool {
    #[inline(always)]
    fn from(variant: Blockkeyoutput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKKEYOUTPUT` reader - Block Key Output Data"]
pub type BlockkeyoutputR = crate::BitReader<Blockkeyoutput>;
impl BlockkeyoutputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockkeyoutput {
        match self.bits {
            false => Blockkeyoutput::Disabled,
            true => Blockkeyoutput::Enabled,
        }
    }
    #[doc = "Disabled. BLOCKKEYOUTPUT is cleared on reset."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Blockkeyoutput::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Blockkeyoutput::Enabled
    }
}
#[doc = "Field `BLOCKKEYOUTPUT` writer - Block Key Output Data"]
pub type BlockkeyoutputW<'a, REG> = crate::BitWriter<'a, REG, Blockkeyoutput>;
impl<'a, REG> BlockkeyoutputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. BLOCKKEYOUTPUT is cleared on reset."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Blockkeyoutput::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Blockkeyoutput::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Block Enroll and Set Key Operation"]
    #[inline(always)]
    pub fn blockenroll_setkey(&self) -> BlockenrollSetkeyR {
        BlockenrollSetkeyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Key Output Data"]
    #[inline(always)]
    pub fn blockkeyoutput(&self) -> BlockkeyoutputR {
        BlockkeyoutputR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("blockenroll_setkey", &self.blockenroll_setkey())
            .field("blockkeyoutput", &self.blockkeyoutput())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Block Enroll and Set Key Operation"]
    #[inline(always)]
    pub fn blockenroll_setkey(&mut self) -> BlockenrollSetkeyW<CfgSpec> {
        BlockenrollSetkeyW::new(self, 0)
    }
    #[doc = "Bit 1 - Block Key Output Data"]
    #[inline(always)]
    pub fn blockkeyoutput(&mut self) -> BlockkeyoutputW<CfgSpec> {
        BlockkeyoutputW::new(self, 1)
    }
}
#[doc = "PUF Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
