#[doc = "Register `SLVADR[%s]` reader"]
pub type R = crate::R<SlvadrSpec>;
#[doc = "Register `SLVADR[%s]` writer"]
pub type W = crate::W<SlvadrSpec>;
#[doc = "Slave Address n Disable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadisable {
    #[doc = "0: Enabled. Slave Address n is enabled."]
    Enabled = 0,
    #[doc = "1: Ignored Slave Address n is ignored."]
    Disabled = 1,
}
impl From<Sadisable> for bool {
    #[inline(always)]
    fn from(variant: Sadisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SADISABLE` reader - Slave Address n Disable."]
pub type SadisableR = crate::BitReader<Sadisable>;
impl SadisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sadisable {
        match self.bits {
            false => Sadisable::Enabled,
            true => Sadisable::Disabled,
        }
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sadisable::Enabled
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sadisable::Disabled
    }
}
#[doc = "Field `SADISABLE` writer - Slave Address n Disable."]
pub type SadisableW<'a, REG> = crate::BitWriter<'a, REG, Sadisable>;
impl<'a, REG> SadisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sadisable::Enabled)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sadisable::Disabled)
    }
}
#[doc = "Field `SLVADR` reader - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SlvadrR = crate::FieldReader;
#[doc = "Field `SLVADR` writer - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SlvadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autonack {
    #[doc = "0: Normal operation, matching I2C addresses are not ignored."]
    Normal = 0,
    #[doc = "1: Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    Automatic = 1,
}
impl From<Autonack> for bool {
    #[inline(always)]
    fn from(variant: Autonack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTONACK` reader - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
pub type AutonackR = crate::BitReader<Autonack>;
impl AutonackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autonack {
        match self.bits {
            false => Autonack::Normal,
            true => Autonack::Automatic,
        }
    }
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Autonack::Normal
    }
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Autonack::Automatic
    }
}
#[doc = "Field `AUTONACK` writer - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
pub type AutonackW<'a, REG> = crate::BitWriter<'a, REG, Autonack>;
impl<'a, REG> AutonackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Autonack::Normal)
    }
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Autonack::Automatic)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SadisableR {
        SadisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SlvadrR {
        SlvadrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub fn autonack(&self) -> AutonackR {
        AutonackR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVADR")
            .field("sadisable", &self.sadisable())
            .field("slvadr", &self.slvadr())
            .field("autonack", &self.autonack())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    #[must_use]
    pub fn sadisable(&mut self) -> SadisableW<SlvadrSpec> {
        SadisableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn slvadr(&mut self) -> SlvadrW<SlvadrSpec> {
        SlvadrW::new(self, 1)
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    #[must_use]
    pub fn autonack(&mut self) -> AutonackW<SlvadrSpec> {
        AutonackW::new(self, 15)
    }
}
#[doc = "Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvadrSpec;
impl crate::RegisterSpec for SlvadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvadr::R`](R) reader structure"]
impl crate::Readable for SlvadrSpec {}
#[doc = "`write(|w| ..)` method takes [`slvadr::W`](W) writer structure"]
impl crate::Writable for SlvadrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLVADR[%s]
to value 0x01"]
impl crate::Resettable for SlvadrSpec {
    const RESET_VALUE: u32 = 0x01;
}
