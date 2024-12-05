#[doc = "Register `FLSHCR4` reader"]
pub type R = crate::R<Flshcr4Spec>;
#[doc = "Register `FLSHCR4` writer"]
pub type W = crate::W<Flshcr4Spec>;
#[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wmopt1 {
    #[doc = "0: DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    Wmopt1_0 = 0,
    #[doc = "1: DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    Wmopt1_1 = 1,
}
impl From<Wmopt1> for bool {
    #[inline(always)]
    fn from(variant: Wmopt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WMOPT1` reader - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
pub type Wmopt1R = crate::BitReader<Wmopt1>;
impl Wmopt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wmopt1 {
        match self.bits {
            false => Wmopt1::Wmopt1_0,
            true => Wmopt1::Wmopt1_1,
        }
    }
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn is_wmopt1_0(&self) -> bool {
        *self == Wmopt1::Wmopt1_0
    }
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn is_wmopt1_1(&self) -> bool {
        *self == Wmopt1::Wmopt1_1
    }
}
#[doc = "Field `WMOPT1` writer - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
pub type Wmopt1W<'a, REG> = crate::BitWriter<'a, REG, Wmopt1>;
impl<'a, REG> Wmopt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn wmopt1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wmopt1::Wmopt1_0)
    }
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn wmopt1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wmopt1::Wmopt1_1)
    }
}
#[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wmena {
    #[doc = "0: Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    Wmena0 = 0,
    #[doc = "1: Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    Wmena1 = 1,
}
impl From<Wmena> for bool {
    #[inline(always)]
    fn from(variant: Wmena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WMENA` reader - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
pub type WmenaR = crate::BitReader<Wmena>;
impl WmenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wmena {
        match self.bits {
            false => Wmena::Wmena0,
            true => Wmena::Wmena1,
        }
    }
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn is_wmena_0(&self) -> bool {
        *self == Wmena::Wmena0
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn is_wmena_1(&self) -> bool {
        *self == Wmena::Wmena1
    }
}
#[doc = "Field `WMENA` writer - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
pub type WmenaW<'a, REG> = crate::BitWriter<'a, REG, Wmena>;
impl<'a, REG> WmenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn wmena_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wmena::Wmena0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn wmena_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wmena::Wmena1)
    }
}
#[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wmenb {
    #[doc = "0: Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    Wmenb0 = 0,
    #[doc = "1: Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    Wmenb1 = 1,
}
impl From<Wmenb> for bool {
    #[inline(always)]
    fn from(variant: Wmenb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WMENB` reader - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
pub type WmenbR = crate::BitReader<Wmenb>;
impl WmenbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wmenb {
        match self.bits {
            false => Wmenb::Wmenb0,
            true => Wmenb::Wmenb1,
        }
    }
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn is_wmenb_0(&self) -> bool {
        *self == Wmenb::Wmenb0
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn is_wmenb_1(&self) -> bool {
        *self == Wmenb::Wmenb1
    }
}
#[doc = "Field `WMENB` writer - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
pub type WmenbW<'a, REG> = crate::BitWriter<'a, REG, Wmenb>;
impl<'a, REG> WmenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn wmenb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wmenb::Wmenb0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn wmenb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wmenb::Wmenb1)
    }
}
impl R {
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub fn wmopt1(&self) -> Wmopt1R {
        Wmopt1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub fn wmena(&self) -> WmenaR {
        WmenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub fn wmenb(&self) -> WmenbR {
        WmenbR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLSHCR4")
            .field("wmopt1", &self.wmopt1())
            .field("wmena", &self.wmena())
            .field("wmenb", &self.wmenb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub fn wmopt1(&mut self) -> Wmopt1W<Flshcr4Spec> {
        Wmopt1W::new(self, 0)
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub fn wmena(&mut self) -> WmenaW<Flshcr4Spec> {
        WmenaW::new(self, 2)
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub fn wmenb(&mut self) -> WmenbW<Flshcr4Spec> {
        WmenbW::new(self, 3)
    }
}
#[doc = "Flash Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flshcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flshcr4Spec;
impl crate::RegisterSpec for Flshcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flshcr4::R`](R) reader structure"]
impl crate::Readable for Flshcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`flshcr4::W`](W) writer structure"]
impl crate::Writable for Flshcr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLSHCR4 to value 0"]
impl crate::Resettable for Flshcr4Spec {
    const RESET_VALUE: u32 = 0;
}
