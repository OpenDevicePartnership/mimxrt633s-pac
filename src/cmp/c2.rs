#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Field `ACOn` reader - ACOn"]
pub type AconR = crate::FieldReader;
#[doc = "Field `ACOn` writer - ACOn"]
pub type AconW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INITMOD` reader - Comparator and DAC initialization delay modulus."]
pub type InitmodR = crate::FieldReader;
#[doc = "Field `INITMOD` writer - Comparator and DAC initialization delay modulus."]
pub type InitmodW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Number of sample clocks\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsam {
    #[doc = "0: The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    Nsam0 = 0,
    #[doc = "1: The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    Nsam1 = 1,
    #[doc = "2: The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    Nsam2 = 2,
    #[doc = "3: The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    Nsam3 = 3,
}
impl From<Nsam> for u8 {
    #[inline(always)]
    fn from(variant: Nsam) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsam {
    type Ux = u8;
}
impl crate::IsEnum for Nsam {}
#[doc = "Field `NSAM` reader - Number of sample clocks"]
pub type NsamR = crate::FieldReader<Nsam>;
impl NsamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nsam {
        match self.bits {
            0 => Nsam::Nsam0,
            1 => Nsam::Nsam1,
            2 => Nsam::Nsam2,
            3 => Nsam::Nsam3,
            _ => unreachable!(),
        }
    }
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    #[inline(always)]
    pub fn is_nsam_0(&self) -> bool {
        *self == Nsam::Nsam0
    }
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn is_nsam_1(&self) -> bool {
        *self == Nsam::Nsam1
    }
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn is_nsam_2(&self) -> bool {
        *self == Nsam::Nsam2
    }
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn is_nsam_3(&self) -> bool {
        *self == Nsam::Nsam3
    }
}
#[doc = "Field `NSAM` writer - Number of sample clocks"]
pub type NsamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nsam, crate::Safe>;
impl<'a, REG> NsamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    #[inline(always)]
    pub fn nsam_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nsam::Nsam0)
    }
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nsam::Nsam1)
    }
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_2(self) -> &'a mut crate::W<REG> {
        self.variant(Nsam::Nsam2)
    }
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_3(self) -> &'a mut crate::W<REG> {
        self.variant(Nsam::Nsam3)
    }
}
#[doc = "Field `CH0F` reader - CH0F"]
pub type Ch0fR = crate::BitReader;
#[doc = "Field `CH0F` writer - CH0F"]
pub type Ch0fW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CH1F` reader - CH1F"]
pub type Ch1fR = crate::BitReader;
#[doc = "Field `CH1F` writer - CH1F"]
pub type Ch1fW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CH2F` reader - CH2F"]
pub type Ch2fR = crate::BitReader;
#[doc = "Field `CH2F` writer - CH2F"]
pub type Ch2fW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CH3F` reader - CH3F"]
pub type Ch3fR = crate::BitReader;
#[doc = "Field `CH3F` writer - CH3F"]
pub type Ch3fW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CH4F` reader - CH4F"]
pub type Ch4fR = crate::BitReader;
#[doc = "Field `CH4F` writer - CH4F"]
pub type Ch4fW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CH5F` reader - CH5F"]
pub type Ch5fR = crate::BitReader;
#[doc = "Field `CH5F` writer - CH5F"]
pub type Ch5fW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Fixed channel selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fxmxch {
    #[doc = "0: External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
    Fxmxch0 = 0,
    #[doc = "1: External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
    Fxmxch1 = 1,
    #[doc = "2: External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
    Fxmxch2 = 2,
    #[doc = "3: External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
    Fxmxch3 = 3,
    #[doc = "4: External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
    Fxmxch4 = 4,
    #[doc = "5: External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
    Fxmxch5 = 5,
    #[doc = "7: The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
    Fxmxch7 = 7,
}
impl From<Fxmxch> for u8 {
    #[inline(always)]
    fn from(variant: Fxmxch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fxmxch {
    type Ux = u8;
}
impl crate::IsEnum for Fxmxch {}
#[doc = "Field `FXMXCH` reader - Fixed channel selection"]
pub type FxmxchR = crate::FieldReader<Fxmxch>;
impl FxmxchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fxmxch> {
        match self.bits {
            0 => Some(Fxmxch::Fxmxch0),
            1 => Some(Fxmxch::Fxmxch1),
            2 => Some(Fxmxch::Fxmxch2),
            3 => Some(Fxmxch::Fxmxch3),
            4 => Some(Fxmxch::Fxmxch4),
            5 => Some(Fxmxch::Fxmxch5),
            7 => Some(Fxmxch::Fxmxch7),
            _ => None,
        }
    }
    #[doc = "External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_0(&self) -> bool {
        *self == Fxmxch::Fxmxch0
    }
    #[doc = "External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_1(&self) -> bool {
        *self == Fxmxch::Fxmxch1
    }
    #[doc = "External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_2(&self) -> bool {
        *self == Fxmxch::Fxmxch2
    }
    #[doc = "External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_3(&self) -> bool {
        *self == Fxmxch::Fxmxch3
    }
    #[doc = "External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_4(&self) -> bool {
        *self == Fxmxch::Fxmxch4
    }
    #[doc = "External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_5(&self) -> bool {
        *self == Fxmxch::Fxmxch5
    }
    #[doc = "The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn is_fxmxch_7(&self) -> bool {
        *self == Fxmxch::Fxmxch7
    }
}
#[doc = "Field `FXMXCH` writer - Fixed channel selection"]
pub type FxmxchW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fxmxch>;
impl<'a, REG> FxmxchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch0)
    }
    #[doc = "External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch1)
    }
    #[doc = "External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch2)
    }
    #[doc = "External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch3)
    }
    #[doc = "External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch4)
    }
    #[doc = "External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch5)
    }
    #[doc = "The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmxch::Fxmxch7)
    }
}
#[doc = "Fixed MUX Port\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fxmp {
    #[doc = "0: The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    Fxmp0 = 0,
    #[doc = "1: The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    Fxmp1 = 1,
}
impl From<Fxmp> for bool {
    #[inline(always)]
    fn from(variant: Fxmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FXMP` reader - Fixed MUX Port"]
pub type FxmpR = crate::BitReader<Fxmp>;
impl FxmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fxmp {
        match self.bits {
            false => Fxmp::Fxmp0,
            true => Fxmp::Fxmp1,
        }
    }
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    #[inline(always)]
    pub fn is_fxmp_0(&self) -> bool {
        *self == Fxmp::Fxmp0
    }
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    #[inline(always)]
    pub fn is_fxmp_1(&self) -> bool {
        *self == Fxmp::Fxmp1
    }
}
#[doc = "Field `FXMP` writer - Fixed MUX Port"]
pub type FxmpW<'a, REG> = crate::BitWriter<'a, REG, Fxmp>;
impl<'a, REG> FxmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    #[inline(always)]
    pub fn fxmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmp::Fxmp0)
    }
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    #[inline(always)]
    pub fn fxmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fxmp::Fxmp1)
    }
}
#[doc = "Round-Robin interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrie {
    #[doc = "0: The round-robin interrupt is disabled."]
    Rrie0 = 0,
    #[doc = "1: The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    Rrie1 = 1,
}
impl From<Rrie> for bool {
    #[inline(always)]
    fn from(variant: Rrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRIE` reader - Round-Robin interrupt enable"]
pub type RrieR = crate::BitReader<Rrie>;
impl RrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrie {
        match self.bits {
            false => Rrie::Rrie0,
            true => Rrie::Rrie1,
        }
    }
    #[doc = "The round-robin interrupt is disabled."]
    #[inline(always)]
    pub fn is_rrie_0(&self) -> bool {
        *self == Rrie::Rrie0
    }
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    #[inline(always)]
    pub fn is_rrie_1(&self) -> bool {
        *self == Rrie::Rrie1
    }
}
#[doc = "Field `RRIE` writer - Round-Robin interrupt enable"]
pub type RrieW<'a, REG> = crate::BitWriter<'a, REG, Rrie>;
impl<'a, REG> RrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The round-robin interrupt is disabled."]
    #[inline(always)]
    pub fn rrie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrie::Rrie0)
    }
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    #[inline(always)]
    pub fn rrie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrie::Rrie1)
    }
}
impl R {
    #[doc = "Bits 0:5 - ACOn"]
    #[inline(always)]
    pub fn acon(&self) -> AconR {
        AconR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub fn initmod(&self) -> InitmodR {
        InitmodR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    pub fn nsam(&self) -> NsamR {
        NsamR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - CH0F"]
    #[inline(always)]
    pub fn ch0f(&self) -> Ch0fR {
        Ch0fR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> Ch1fR {
        Ch1fR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> Ch2fR {
        Ch2fR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> Ch3fR {
        Ch3fR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CH4F"]
    #[inline(always)]
    pub fn ch4f(&self) -> Ch4fR {
        Ch4fR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH5F"]
    #[inline(always)]
    pub fn ch5f(&self) -> Ch5fR {
        Ch5fR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    pub fn fxmxch(&self) -> FxmxchR {
        FxmxchR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    pub fn fxmp(&self) -> FxmpR {
        FxmpR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RrieR {
        RrieR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2")
            .field("acon", &self.acon())
            .field("initmod", &self.initmod())
            .field("nsam", &self.nsam())
            .field("ch0f", &self.ch0f())
            .field("ch1f", &self.ch1f())
            .field("ch2f", &self.ch2f())
            .field("ch3f", &self.ch3f())
            .field("ch4f", &self.ch4f())
            .field("ch5f", &self.ch5f())
            .field("fxmxch", &self.fxmxch())
            .field("fxmp", &self.fxmp())
            .field("rrie", &self.rrie())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - ACOn"]
    #[inline(always)]
    #[must_use]
    pub fn acon(&mut self) -> AconW<C2Spec> {
        AconW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    #[must_use]
    pub fn initmod(&mut self) -> InitmodW<C2Spec> {
        InitmodW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    #[must_use]
    pub fn nsam(&mut self) -> NsamW<C2Spec> {
        NsamW::new(self, 14)
    }
    #[doc = "Bit 16 - CH0F"]
    #[inline(always)]
    #[must_use]
    pub fn ch0f(&mut self) -> Ch0fW<C2Spec> {
        Ch0fW::new(self, 16)
    }
    #[doc = "Bit 17 - CH1F"]
    #[inline(always)]
    #[must_use]
    pub fn ch1f(&mut self) -> Ch1fW<C2Spec> {
        Ch1fW::new(self, 17)
    }
    #[doc = "Bit 18 - CH2F"]
    #[inline(always)]
    #[must_use]
    pub fn ch2f(&mut self) -> Ch2fW<C2Spec> {
        Ch2fW::new(self, 18)
    }
    #[doc = "Bit 19 - CH3F"]
    #[inline(always)]
    #[must_use]
    pub fn ch3f(&mut self) -> Ch3fW<C2Spec> {
        Ch3fW::new(self, 19)
    }
    #[doc = "Bit 20 - CH4F"]
    #[inline(always)]
    #[must_use]
    pub fn ch4f(&mut self) -> Ch4fW<C2Spec> {
        Ch4fW::new(self, 20)
    }
    #[doc = "Bit 21 - CH5F"]
    #[inline(always)]
    #[must_use]
    pub fn ch5f(&mut self) -> Ch5fW<C2Spec> {
        Ch5fW::new(self, 21)
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn fxmxch(&mut self) -> FxmxchW<C2Spec> {
        FxmxchW::new(self, 25)
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    #[must_use]
    pub fn fxmp(&mut self) -> FxmpW<C2Spec> {
        FxmpW::new(self, 29)
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RrieW<C2Spec> {
        RrieW::new(self, 30)
    }
}
#[doc = "CMP Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x003f_0000;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u32 = 0;
}
