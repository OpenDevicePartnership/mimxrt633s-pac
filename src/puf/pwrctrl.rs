#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PwrctrlSpec>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PwrctrlSpec>;
#[doc = "RAM Power On\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamOn {
    #[doc = "0: Power Off"]
    PowerOff = 0,
    #[doc = "1: Power On"]
    PowerOn = 1,
}
impl From<RamOn> for bool {
    #[inline(always)]
    fn from(variant: RamOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_ON` reader - RAM Power On"]
pub type RamOnR = crate::BitReader<RamOn>;
impl RamOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamOn {
        match self.bits {
            false => RamOn::PowerOff,
            true => RamOn::PowerOn,
        }
    }
    #[doc = "Power Off"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == RamOn::PowerOff
    }
    #[doc = "Power On"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == RamOn::PowerOn
    }
}
#[doc = "Field `RAM_ON` writer - RAM Power On"]
pub type RamOnW<'a, REG> = crate::BitWriter<'a, REG, RamOn>;
impl<'a, REG> RamOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power Off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut crate::W<REG> {
        self.variant(RamOn::PowerOff)
    }
    #[doc = "Power On"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut crate::W<REG> {
        self.variant(RamOn::PowerOn)
    }
}
#[doc = "PUF Clock control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CkDis {
    #[doc = "0: PUF RAM clock is disabled."]
    ClockOff = 0,
    #[doc = "1: PUF RAM clock is enabled."]
    ClockOn = 1,
}
impl From<CkDis> for bool {
    #[inline(always)]
    fn from(variant: CkDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK_DIS` reader - PUF Clock control."]
pub type CkDisR = crate::BitReader<CkDis>;
impl CkDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CkDis {
        match self.bits {
            false => CkDis::ClockOff,
            true => CkDis::ClockOn,
        }
    }
    #[doc = "PUF RAM clock is disabled."]
    #[inline(always)]
    pub fn is_clock_off(&self) -> bool {
        *self == CkDis::ClockOff
    }
    #[doc = "PUF RAM clock is enabled."]
    #[inline(always)]
    pub fn is_clock_on(&self) -> bool {
        *self == CkDis::ClockOn
    }
}
#[doc = "Field `CK_DIS` writer - PUF Clock control."]
pub type CkDisW<'a, REG> = crate::BitWriter<'a, REG, CkDis>;
impl<'a, REG> CkDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PUF RAM clock is disabled."]
    #[inline(always)]
    pub fn clock_off(self) -> &'a mut crate::W<REG> {
        self.variant(CkDis::ClockOff)
    }
    #[doc = "PUF RAM clock is enabled."]
    #[inline(always)]
    pub fn clock_on(self) -> &'a mut crate::W<REG> {
        self.variant(CkDis::ClockOn)
    }
}
impl R {
    #[doc = "Bit 0 - RAM Power On"]
    #[inline(always)]
    pub fn ram_on(&self) -> RamOnR {
        RamOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PUF Clock control."]
    #[inline(always)]
    pub fn ck_dis(&self) -> CkDisR {
        CkDisR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCTRL")
            .field("ram_on", &self.ram_on())
            .field("ck_dis", &self.ck_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Power On"]
    #[inline(always)]
    pub fn ram_on(&mut self) -> RamOnW<PwrctrlSpec> {
        RamOnW::new(self, 0)
    }
    #[doc = "Bit 2 - PUF Clock control."]
    #[inline(always)]
    pub fn ck_dis(&mut self) -> CkDisW<PwrctrlSpec> {
        CkDisW::new(self, 2)
    }
}
#[doc = "PUF Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctrlSpec;
impl crate::RegisterSpec for PwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0x01"]
impl crate::Resettable for PwrctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
