#[doc = "Register `ESPIMISC` reader"]
pub type R = crate::R<EspimiscSpec>;
#[doc = "Register `ESPIMISC` writer"]
pub type W = crate::W<EspimiscSpec>;
#[doc = "Field `GPIO_OE` reader - Set to 1 to make the Alert/Reset pin an output GPIO, else is input (High-Z)"]
pub type GpioOeR = crate::BitReader;
#[doc = "Field `GPIO_OE` writer - Set to 1 to make the Alert/Reset pin an output GPIO, else is input (High-Z)"]
pub type GpioOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_OD` reader - Set to 1 to make the Alert/Reset pin act open-drain when GPIO_OE=1"]
pub type GpioOdR = crate::BitReader;
#[doc = "Field `GPIO_OD` writer - Set to 1 to make the Alert/Reset pin act open-drain when GPIO_OE=1"]
pub type GpioOdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_OUT` reader - Used when GPIO_OE=1"]
pub type GpioOutR = crate::BitReader;
#[doc = "Field `GPIO_OUT` writer - Used when GPIO_OE=1"]
pub type GpioOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_IN` reader - Is the current state of the Alert/Reset pin, whether in input mode or not"]
pub type GpioInR = crate::BitReader;
#[doc = "Field `RISGP` reader - Reset# is a GPIO"]
pub type RisgpR = crate::BitReader;
#[doc = "Field `RISGP` writer - Reset# is a GPIO"]
pub type RisgpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSAV` reader - This bit, when set, will employ clock gating for the eSPI side"]
pub type PwrsavR = crate::BitReader;
#[doc = "Field `PWRSAV` writer - This bit, when set, will employ clock gating for the eSPI side"]
pub type PwrsavW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to 1 to make the Alert/Reset pin an output GPIO, else is input (High-Z)"]
    #[inline(always)]
    pub fn gpio_oe(&self) -> GpioOeR {
        GpioOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to make the Alert/Reset pin act open-drain when GPIO_OE=1"]
    #[inline(always)]
    pub fn gpio_od(&self) -> GpioOdR {
        GpioOdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Used when GPIO_OE=1"]
    #[inline(always)]
    pub fn gpio_out(&self) -> GpioOutR {
        GpioOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Is the current state of the Alert/Reset pin, whether in input mode or not"]
    #[inline(always)]
    pub fn gpio_in(&self) -> GpioInR {
        GpioInR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset# is a GPIO"]
    #[inline(always)]
    pub fn risgp(&self) -> RisgpR {
        RisgpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit, when set, will employ clock gating for the eSPI side"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PwrsavR {
        PwrsavR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPIMISC")
            .field("gpio_oe", &self.gpio_oe())
            .field("gpio_od", &self.gpio_od())
            .field("gpio_out", &self.gpio_out())
            .field("gpio_in", &self.gpio_in())
            .field("risgp", &self.risgp())
            .field("pwrsav", &self.pwrsav())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to make the Alert/Reset pin an output GPIO, else is input (High-Z)"]
    #[inline(always)]
    pub fn gpio_oe(&mut self) -> GpioOeW<EspimiscSpec> {
        GpioOeW::new(self, 0)
    }
    #[doc = "Bit 1 - Set to 1 to make the Alert/Reset pin act open-drain when GPIO_OE=1"]
    #[inline(always)]
    pub fn gpio_od(&mut self) -> GpioOdW<EspimiscSpec> {
        GpioOdW::new(self, 1)
    }
    #[doc = "Bit 2 - Used when GPIO_OE=1"]
    #[inline(always)]
    pub fn gpio_out(&mut self) -> GpioOutW<EspimiscSpec> {
        GpioOutW::new(self, 2)
    }
    #[doc = "Bit 4 - Reset# is a GPIO"]
    #[inline(always)]
    pub fn risgp(&mut self) -> RisgpW<EspimiscSpec> {
        RisgpW::new(self, 4)
    }
    #[doc = "Bit 31 - This bit, when set, will employ clock gating for the eSPI side"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PwrsavW<EspimiscSpec> {
        PwrsavW::new(self, 31)
    }
}
#[doc = "Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert).\n\nYou can [`read`](crate::Reg::read) this register and get [`espimisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espimisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspimiscSpec;
impl crate::RegisterSpec for EspimiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espimisc::R`](R) reader structure"]
impl crate::Readable for EspimiscSpec {}
#[doc = "`write(|w| ..)` method takes [`espimisc::W`](W) writer structure"]
impl crate::Writable for EspimiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPIMISC to value 0"]
impl crate::Resettable for EspimiscSpec {
    const RESET_VALUE: u32 = 0;
}
