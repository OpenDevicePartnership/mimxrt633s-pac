#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "ADC Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcen {
    #[doc = "0: ADC is disabled."]
    Adcen0 = 0,
    #[doc = "1: ADC is enabled."]
    Adcen1 = 1,
}
impl From<Adcen> for bool {
    #[inline(always)]
    fn from(variant: Adcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC Enable"]
pub type AdcenR = crate::BitReader<Adcen>;
impl AdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcen {
        match self.bits {
            false => Adcen::Adcen0,
            true => Adcen::Adcen1,
        }
    }
    #[doc = "ADC is disabled."]
    #[inline(always)]
    pub fn is_adcen_0(&self) -> bool {
        *self == Adcen::Adcen0
    }
    #[doc = "ADC is enabled."]
    #[inline(always)]
    pub fn is_adcen_1(&self) -> bool {
        *self == Adcen::Adcen1
    }
}
#[doc = "Field `ADCEN` writer - ADC Enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG, Adcen>;
impl<'a, REG> AdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is disabled."]
    #[inline(always)]
    pub fn adcen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::Adcen0)
    }
    #[doc = "ADC is enabled."]
    #[inline(always)]
    pub fn adcen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::Adcen1)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rst {
    #[doc = "0: ADC logic is not reset."]
    Rst0 = 0,
    #[doc = "1: ADC logic is reset."]
    Rst1 = 1,
}
impl From<Rst> for bool {
    #[inline(always)]
    fn from(variant: Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub type RstR = crate::BitReader<Rst>;
impl RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rst {
        match self.bits {
            false => Rst::Rst0,
            true => Rst::Rst1,
        }
    }
    #[doc = "ADC logic is not reset."]
    #[inline(always)]
    pub fn is_rst_0(&self) -> bool {
        *self == Rst::Rst0
    }
    #[doc = "ADC logic is reset."]
    #[inline(always)]
    pub fn is_rst_1(&self) -> bool {
        *self == Rst::Rst1
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rst>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC logic is not reset."]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Rst0)
    }
    #[doc = "ADC logic is reset."]
    #[inline(always)]
    pub fn rst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Rst1)
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dozen {
    #[doc = "0: ADC is enabled in Doze mode."]
    Dozen0 = 0,
    #[doc = "1: ADC is disabled in Doze mode."]
    Dozen1 = 1,
}
impl From<Dozen> for bool {
    #[inline(always)]
    fn from(variant: Dozen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub type DozenR = crate::BitReader<Dozen>;
impl DozenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dozen {
        match self.bits {
            false => Dozen::Dozen0,
            true => Dozen::Dozen1,
        }
    }
    #[doc = "ADC is enabled in Doze mode."]
    #[inline(always)]
    pub fn is_dozen_0(&self) -> bool {
        *self == Dozen::Dozen0
    }
    #[doc = "ADC is disabled in Doze mode."]
    #[inline(always)]
    pub fn is_dozen_1(&self) -> bool {
        *self == Dozen::Dozen1
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub type DozenW<'a, REG> = crate::BitWriter<'a, REG, Dozen>;
impl<'a, REG> DozenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is enabled in Doze mode."]
    #[inline(always)]
    pub fn dozen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dozen::Dozen0)
    }
    #[doc = "ADC is disabled in Doze mode."]
    #[inline(always)]
    pub fn dozen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dozen::Dozen1)
    }
}
#[doc = "Reset FIFO\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfifo {
    #[doc = "0: No effect."]
    Rstfifo0 = 0,
    #[doc = "1: FIFO is reset."]
    Rstfifo1 = 1,
}
impl From<Rstfifo> for bool {
    #[inline(always)]
    fn from(variant: Rstfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFIFO` reader - Reset FIFO"]
pub type RstfifoR = crate::BitReader<Rstfifo>;
impl RstfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfifo {
        match self.bits {
            false => Rstfifo::Rstfifo0,
            true => Rstfifo::Rstfifo1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_rstfifo_0(&self) -> bool {
        *self == Rstfifo::Rstfifo0
    }
    #[doc = "FIFO is reset."]
    #[inline(always)]
    pub fn is_rstfifo_1(&self) -> bool {
        *self == Rstfifo::Rstfifo1
    }
}
#[doc = "Field `RSTFIFO` writer - Reset FIFO"]
pub type RstfifoW<'a, REG> = crate::BitWriter<'a, REG, Rstfifo>;
impl<'a, REG> RstfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn rstfifo_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfifo::Rstfifo0)
    }
    #[doc = "FIFO is reset."]
    #[inline(always)]
    pub fn rstfifo_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfifo::Rstfifo1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DozenR {
        DozenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset FIFO"]
    #[inline(always)]
    pub fn rstfifo(&self) -> RstfifoR {
        RstfifoR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("adcen", &self.adcen())
            .field("rst", &self.rst())
            .field("dozen", &self.dozen())
            .field("rstfifo", &self.rstfifo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<CtrlSpec> {
        AdcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CtrlSpec> {
        RstW::new(self, 1)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozen(&mut self) -> DozenW<CtrlSpec> {
        DozenW::new(self, 2)
    }
    #[doc = "Bit 8 - Reset FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rstfifo(&mut self) -> RstfifoW<CtrlSpec> {
        RstfifoW::new(self, 8)
    }
}
#[doc = "ADC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
