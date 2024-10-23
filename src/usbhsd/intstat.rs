#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `EP0OUT` reader - Interrupt status register bit for the Control EP0 OUT direction."]
pub type Ep0outR = crate::BitReader;
#[doc = "Field `EP0OUT` writer - Interrupt status register bit for the Control EP0 OUT direction."]
pub type Ep0outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0IN` reader - Interrupt status register bit for the Control EP0 IN direction."]
pub type Ep0inR = crate::BitReader;
#[doc = "Field `EP0IN` writer - Interrupt status register bit for the Control EP0 IN direction."]
pub type Ep0inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1OUT` reader - Interrupt status register bit for the EP1 OUT direction."]
pub type Ep1outR = crate::BitReader;
#[doc = "Field `EP1OUT` writer - Interrupt status register bit for the EP1 OUT direction."]
pub type Ep1outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1IN` reader - Interrupt status register bit for the EP1 IN direction."]
pub type Ep1inR = crate::BitReader;
#[doc = "Field `EP1IN` writer - Interrupt status register bit for the EP1 IN direction."]
pub type Ep1inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2OUT` reader - Interrupt status register bit for the EP2 OUT direction."]
pub type Ep2outR = crate::BitReader;
#[doc = "Field `EP2OUT` writer - Interrupt status register bit for the EP2 OUT direction."]
pub type Ep2outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2IN` reader - Interrupt status register bit for the EP2 IN direction."]
pub type Ep2inR = crate::BitReader;
#[doc = "Field `EP2IN` writer - Interrupt status register bit for the EP2 IN direction."]
pub type Ep2inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3OUT` reader - Interrupt status register bit for the EP3 OUT direction."]
pub type Ep3outR = crate::BitReader;
#[doc = "Field `EP3OUT` writer - Interrupt status register bit for the EP3 OUT direction."]
pub type Ep3outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3IN` reader - Interrupt status register bit for the EP3 IN direction."]
pub type Ep3inR = crate::BitReader;
#[doc = "Field `EP3IN` writer - Interrupt status register bit for the EP3 IN direction."]
pub type Ep3inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4OUT` reader - Interrupt status register bit for the EP4 OUT direction."]
pub type Ep4outR = crate::BitReader;
#[doc = "Field `EP4OUT` writer - Interrupt status register bit for the EP4 OUT direction."]
pub type Ep4outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4IN` reader - Interrupt status register bit for the EP4 IN direction."]
pub type Ep4inR = crate::BitReader;
#[doc = "Field `EP4IN` writer - Interrupt status register bit for the EP4 IN direction."]
pub type Ep4inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5OUT` reader - Interrupt status register bit for the EP5 OUT direction."]
pub type Ep5outR = crate::BitReader;
#[doc = "Field `EP5OUT` writer - Interrupt status register bit for the EP5 OUT direction."]
pub type Ep5outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5IN` reader - Interrupt status register bit for the EP5 IN direction."]
pub type Ep5inR = crate::BitReader;
#[doc = "Field `EP5IN` writer - Interrupt status register bit for the EP5 IN direction."]
pub type Ep5inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_INT` reader - Frame interrupt."]
pub type FrameIntR = crate::BitReader;
#[doc = "Field `FRAME_INT` writer - Frame interrupt."]
pub type FrameIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_INT` reader - Device status interrupt."]
pub type DevIntR = crate::BitReader;
#[doc = "Field `DEV_INT` writer - Device status interrupt."]
pub type DevIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn ep0out(&self) -> Ep0outR {
        Ep0outR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn ep0in(&self) -> Ep0inR {
        Ep0inR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn ep1out(&self) -> Ep1outR {
        Ep1outR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn ep1in(&self) -> Ep1inR {
        Ep1inR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn ep2out(&self) -> Ep2outR {
        Ep2outR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn ep2in(&self) -> Ep2inR {
        Ep2inR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn ep3out(&self) -> Ep3outR {
        Ep3outR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn ep3in(&self) -> Ep3inR {
        Ep3inR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn ep4out(&self) -> Ep4outR {
        Ep4outR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn ep4in(&self) -> Ep4inR {
        Ep4inR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn ep5out(&self) -> Ep5outR {
        Ep5outR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn ep5in(&self) -> Ep5inR {
        Ep5inR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    pub fn frame_int(&self) -> FrameIntR {
        FrameIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    pub fn dev_int(&self) -> DevIntR {
        DevIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("ep0out", &self.ep0out())
            .field("ep0in", &self.ep0in())
            .field("ep1out", &self.ep1out())
            .field("ep1in", &self.ep1in())
            .field("ep2out", &self.ep2out())
            .field("ep2in", &self.ep2in())
            .field("ep3out", &self.ep3out())
            .field("ep3in", &self.ep3in())
            .field("ep4out", &self.ep4out())
            .field("ep4in", &self.ep4in())
            .field("ep5out", &self.ep5out())
            .field("ep5in", &self.ep5in())
            .field("frame_int", &self.frame_int())
            .field("dev_int", &self.dev_int())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep0out(&mut self) -> Ep0outW<IntstatSpec> {
        Ep0outW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep0in(&mut self) -> Ep0inW<IntstatSpec> {
        Ep0inW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep1out(&mut self) -> Ep1outW<IntstatSpec> {
        Ep1outW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep1in(&mut self) -> Ep1inW<IntstatSpec> {
        Ep1inW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep2out(&mut self) -> Ep2outW<IntstatSpec> {
        Ep2outW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep2in(&mut self) -> Ep2inW<IntstatSpec> {
        Ep2inW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep3out(&mut self) -> Ep3outW<IntstatSpec> {
        Ep3outW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep3in(&mut self) -> Ep3inW<IntstatSpec> {
        Ep3inW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep4out(&mut self) -> Ep4outW<IntstatSpec> {
        Ep4outW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep4in(&mut self) -> Ep4inW<IntstatSpec> {
        Ep4inW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep5out(&mut self) -> Ep5outW<IntstatSpec> {
        Ep5outW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep5in(&mut self) -> Ep5inW<IntstatSpec> {
        Ep5inW::new(self, 11)
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frame_int(&mut self) -> FrameIntW<IntstatSpec> {
        FrameIntW::new(self, 30)
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dev_int(&mut self) -> DevIntW<IntstatSpec> {
        DevIntW::new(self, 31)
    }
}
#[doc = "USB interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
