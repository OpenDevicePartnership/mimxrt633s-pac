#[doc = "Register `PHY_CTRL` reader"]
pub type R = crate::R<PhyCtrlSpec>;
#[doc = "Register `PHY_CTRL` writer"]
pub type W = crate::W<PhyCtrlSpec>;
#[doc = "Capture DMIC on Falling edge (0 means on rising)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyFall {
    #[doc = "0: Capture PDM_DATA on the rising edge of PDM_CLK."]
    RisingEdge = 0,
    #[doc = "1: Capture PDM_DATA on the falling edge of PDM_CLK."]
    FallingEdge = 1,
}
impl From<PhyFall> for bool {
    #[inline(always)]
    fn from(variant: PhyFall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_FALL` reader - Capture DMIC on Falling edge (0 means on rising)"]
pub type PhyFallR = crate::BitReader<PhyFall>;
impl PhyFallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyFall {
        match self.bits {
            false => PhyFall::RisingEdge,
            true => PhyFall::FallingEdge,
        }
    }
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PhyFall::RisingEdge
    }
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PhyFall::FallingEdge
    }
}
#[doc = "Field `PHY_FALL` writer - Capture DMIC on Falling edge (0 means on rising)"]
pub type PhyFallW<'a, REG> = crate::BitWriter<'a, REG, PhyFall>;
impl<'a, REG> PhyFallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PhyFall::RisingEdge)
    }
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PhyFall::FallingEdge)
    }
}
#[doc = "Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyHalf {
    #[doc = "0: Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    Standard = 0,
    #[doc = "1: Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    HalfRate = 1,
}
impl From<PhyHalf> for bool {
    #[inline(always)]
    fn from(variant: PhyHalf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_HALF` reader - Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)"]
pub type PhyHalfR = crate::BitReader<PhyHalf>;
impl PhyHalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyHalf {
        match self.bits {
            false => PhyHalf::Standard,
            true => PhyHalf::HalfRate,
        }
    }
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == PhyHalf::Standard
    }
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    #[inline(always)]
    pub fn is_half_rate(&self) -> bool {
        *self == PhyHalf::HalfRate
    }
}
#[doc = "Field `PHY_HALF` writer - Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)"]
pub type PhyHalfW<'a, REG> = crate::BitWriter<'a, REG, PhyHalf>;
impl<'a, REG> PhyHalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(PhyHalf::Standard)
    }
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    #[inline(always)]
    pub fn half_rate(self) -> &'a mut crate::W<REG> {
        self.variant(PhyHalf::HalfRate)
    }
}
impl R {
    #[doc = "Bit 0 - Capture DMIC on Falling edge (0 means on rising)"]
    #[inline(always)]
    pub fn phy_fall(&self) -> PhyFallR {
        PhyFallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)"]
    #[inline(always)]
    pub fn phy_half(&self) -> PhyHalfR {
        PhyHalfR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_CTRL")
            .field("phy_fall", &self.phy_fall())
            .field("phy_half", &self.phy_half())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture DMIC on Falling edge (0 means on rising)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_fall(&mut self) -> PhyFallW<PhyCtrlSpec> {
        PhyFallW::new(self, 0)
    }
    #[doc = "Bit 1 - Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_half(&mut self) -> PhyHalfW<PhyCtrlSpec> {
        PhyHalfW::new(self, 1)
    }
}
#[doc = "Phy Ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCtrlSpec;
impl crate::RegisterSpec for PhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_ctrl::R`](R) reader structure"]
impl crate::Readable for PhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_ctrl::W`](W) writer structure"]
impl crate::Writable for PhyCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_CTRL to value 0"]
impl crate::Resettable for PhyCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
