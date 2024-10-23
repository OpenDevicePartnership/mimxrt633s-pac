#[doc = "Register `LUT[%s]` reader"]
pub type R = crate::R<LutSpec>;
#[doc = "Register `LUT[%s]` writer"]
pub type W = crate::W<LutSpec>;
#[doc = "Field `OPERAND0` reader - OPERAND0"]
pub type Operand0R = crate::FieldReader;
#[doc = "Field `OPERAND0` writer - OPERAND0"]
pub type Operand0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUM_PADS0` reader - NUM_PADS0"]
pub type NumPads0R = crate::FieldReader;
#[doc = "Field `NUM_PADS0` writer - NUM_PADS0"]
pub type NumPads0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPCODE0` reader - OPCODE"]
pub type Opcode0R = crate::FieldReader;
#[doc = "Field `OPCODE0` writer - OPCODE"]
pub type Opcode0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OPERAND1` reader - OPERAND1"]
pub type Operand1R = crate::FieldReader;
#[doc = "Field `OPERAND1` writer - OPERAND1"]
pub type Operand1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUM_PADS1` reader - NUM_PADS1"]
pub type NumPads1R = crate::FieldReader;
#[doc = "Field `NUM_PADS1` writer - NUM_PADS1"]
pub type NumPads1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPCODE1` reader - OPCODE1"]
pub type Opcode1R = crate::FieldReader;
#[doc = "Field `OPCODE1` writer - OPCODE1"]
pub type Opcode1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline(always)]
    pub fn operand0(&self) -> Operand0R {
        Operand0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline(always)]
    pub fn num_pads0(&self) -> NumPads0R {
        NumPads0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline(always)]
    pub fn opcode0(&self) -> Opcode0R {
        Opcode0R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline(always)]
    pub fn operand1(&self) -> Operand1R {
        Operand1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline(always)]
    pub fn num_pads1(&self) -> NumPads1R {
        NumPads1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline(always)]
    pub fn opcode1(&self) -> Opcode1R {
        Opcode1R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUT")
            .field("operand0", &self.operand0())
            .field("num_pads0", &self.num_pads0())
            .field("opcode0", &self.opcode0())
            .field("operand1", &self.operand1())
            .field("num_pads1", &self.num_pads1())
            .field("opcode1", &self.opcode1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline(always)]
    #[must_use]
    pub fn operand0(&mut self) -> Operand0W<LutSpec> {
        Operand0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline(always)]
    #[must_use]
    pub fn num_pads0(&mut self) -> NumPads0W<LutSpec> {
        NumPads0W::new(self, 8)
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0(&mut self) -> Opcode0W<LutSpec> {
        Opcode0W::new(self, 10)
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline(always)]
    #[must_use]
    pub fn operand1(&mut self) -> Operand1W<LutSpec> {
        Operand1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline(always)]
    #[must_use]
    pub fn num_pads1(&mut self) -> NumPads1W<LutSpec> {
        NumPads1W::new(self, 24)
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline(always)]
    #[must_use]
    pub fn opcode1(&mut self) -> Opcode1W<LutSpec> {
        Opcode1W::new(self, 26)
    }
}
#[doc = "LUT x\n\nYou can [`read`](crate::Reg::read) this register and get [`lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutSpec;
impl crate::RegisterSpec for LutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut::R`](R) reader structure"]
impl crate::Readable for LutSpec {}
#[doc = "`write(|w| ..)` method takes [`lut::W`](W) writer structure"]
impl crate::Writable for LutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUT[%s]
to value 0"]
impl crate::Resettable for LutSpec {
    const RESET_VALUE: u32 = 0;
}
