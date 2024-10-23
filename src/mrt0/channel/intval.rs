#[doc = "Register `INTVAL` reader"]
pub type R = crate::R<IntvalSpec>;
#[doc = "Register `INTVAL` writer"]
pub type W = crate::W<IntvalSpec>;
#[doc = "Field `IVALUE` reader - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
pub type IvalueR = crate::FieldReader<u32>;
#[doc = "Field `IVALUE` writer - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
pub type IvalueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Load {
    #[doc = "0: No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NoForceLoad = 0,
    #[doc = "1: Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    ForceLoad = 1,
}
impl From<Load> for bool {
    #[inline(always)]
    fn from(variant: Load) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOAD` reader - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
pub type LoadR = crate::BitReader<Load>;
impl LoadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Load {
        match self.bits {
            false => Load::NoForceLoad,
            true => Load::ForceLoad,
        }
    }
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    #[inline(always)]
    pub fn is_no_force_load(&self) -> bool {
        *self == Load::NoForceLoad
    }
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    #[inline(always)]
    pub fn is_force_load(&self) -> bool {
        *self == Load::ForceLoad
    }
}
#[doc = "Field `LOAD` writer - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG, Load>;
impl<'a, REG> LoadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    #[inline(always)]
    pub fn no_force_load(self) -> &'a mut crate::W<REG> {
        self.variant(Load::NoForceLoad)
    }
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    #[inline(always)]
    pub fn force_load(self) -> &'a mut crate::W<REG> {
        self.variant(Load::ForceLoad)
    }
}
impl R {
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn ivalue(&self) -> IvalueR {
        IvalueR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTVAL")
            .field("ivalue", &self.ivalue())
            .field("load", &self.load())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    #[must_use]
    pub fn ivalue(&mut self) -> IvalueW<IntvalSpec> {
        IvalueW::new(self, 0)
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<IntvalSpec> {
        LoadW::new(self, 31)
    }
}
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntvalSpec;
impl crate::RegisterSpec for IntvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intval::R`](R) reader structure"]
impl crate::Readable for IntvalSpec {}
#[doc = "`write(|w| ..)` method takes [`intval::W`](W) writer structure"]
impl crate::Writable for IntvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTVAL to value 0"]
impl crate::Resettable for IntvalSpec {
    const RESET_VALUE: u32 = 0;
}
