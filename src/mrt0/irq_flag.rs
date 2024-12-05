#[doc = "Register `IRQ_FLAG` reader"]
pub type R = crate::R<IrqFlagSpec>;
#[doc = "Register `IRQ_FLAG` writer"]
pub type W = crate::W<IrqFlagSpec>;
#[doc = "Monitors the interrupt flag of TIMER0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gflag0 {
    #[doc = "0: No pending interrupt. Writing a zero is equivalent to no operation."]
    NoPendingInterrupt = 0,
    #[doc = "1: Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PendingInterrupt = 1,
}
impl From<Gflag0> for bool {
    #[inline(always)]
    fn from(variant: Gflag0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFLAG0` reader - Monitors the interrupt flag of TIMER0."]
pub type Gflag0R = crate::BitReader<Gflag0>;
impl Gflag0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gflag0 {
        match self.bits {
            false => Gflag0::NoPendingInterrupt,
            true => Gflag0::PendingInterrupt,
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == Gflag0::NoPendingInterrupt
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == Gflag0::PendingInterrupt
    }
}
#[doc = "Field `GFLAG0` writer - Monitors the interrupt flag of TIMER0."]
pub type Gflag0W<'a, REG> = crate::BitWriter<'a, REG, Gflag0>;
impl<'a, REG> Gflag0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Gflag0::NoPendingInterrupt)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Gflag0::PendingInterrupt)
    }
}
#[doc = "Field `GFLAG1` reader - Monitors the interrupt flag of TIMER1. See description of channel 0."]
pub type Gflag1R = crate::BitReader;
#[doc = "Field `GFLAG1` writer - Monitors the interrupt flag of TIMER1. See description of channel 0."]
pub type Gflag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFLAG2` reader - Monitors the interrupt flag of TIMER2. See description of channel 0."]
pub type Gflag2R = crate::BitReader;
#[doc = "Field `GFLAG2` writer - Monitors the interrupt flag of TIMER2. See description of channel 0."]
pub type Gflag2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFLAG3` reader - Monitors the interrupt flag of TIMER3. See description of channel 0."]
pub type Gflag3R = crate::BitReader;
#[doc = "Field `GFLAG3` writer - Monitors the interrupt flag of TIMER3. See description of channel 0."]
pub type Gflag3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&self) -> Gflag0R {
        Gflag0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&self) -> Gflag1R {
        Gflag1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&self) -> Gflag2R {
        Gflag2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&self) -> Gflag3R {
        Gflag3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_FLAG")
            .field("gflag0", &self.gflag0())
            .field("gflag1", &self.gflag1())
            .field("gflag2", &self.gflag2())
            .field("gflag3", &self.gflag3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&mut self) -> Gflag0W<IrqFlagSpec> {
        Gflag0W::new(self, 0)
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&mut self) -> Gflag1W<IrqFlagSpec> {
        Gflag1W::new(self, 1)
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&mut self) -> Gflag2W<IrqFlagSpec> {
        Gflag2W::new(self, 2)
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&mut self) -> Gflag3W<IrqFlagSpec> {
        Gflag3W::new(self, 3)
    }
}
#[doc = "Global interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqFlagSpec;
impl crate::RegisterSpec for IrqFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_flag::R`](R) reader structure"]
impl crate::Readable for IrqFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_flag::W`](W) writer structure"]
impl crate::Writable for IrqFlagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_FLAG to value 0"]
impl crate::Resettable for IrqFlagSpec {
    const RESET_VALUE: u32 = 0;
}
