#[doc = "Register `DEBUG1_TOG` reader"]
pub type R = crate::R<Debug1TogSpec>;
#[doc = "Register `DEBUG1_TOG` writer"]
pub type W = crate::W<Debug1TogSpec>;
#[doc = "Delay increment of the rise of squelch:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Entailadjvd {
    #[doc = "0: Delay is nominal"]
    Entailadjvd0 = 0,
    #[doc = "1: Delay is +20%"]
    Entailadjvd1 = 1,
    #[doc = "2: Delay is -20%"]
    Entailadjvd2 = 2,
    #[doc = "3: Delay is -40%"]
    Entailadjvd3 = 3,
}
impl From<Entailadjvd> for u8 {
    #[inline(always)]
    fn from(variant: Entailadjvd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Entailadjvd {
    type Ux = u8;
}
impl crate::IsEnum for Entailadjvd {}
#[doc = "Field `ENTAILADJVD` reader - Delay increment of the rise of squelch:"]
pub type EntailadjvdR = crate::FieldReader<Entailadjvd>;
impl EntailadjvdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Entailadjvd {
        match self.bits {
            0 => Entailadjvd::Entailadjvd0,
            1 => Entailadjvd::Entailadjvd1,
            2 => Entailadjvd::Entailadjvd2,
            3 => Entailadjvd::Entailadjvd3,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay is nominal"]
    #[inline(always)]
    pub fn is_entailadjvd_0(&self) -> bool {
        *self == Entailadjvd::Entailadjvd0
    }
    #[doc = "Delay is +20%"]
    #[inline(always)]
    pub fn is_entailadjvd_1(&self) -> bool {
        *self == Entailadjvd::Entailadjvd1
    }
    #[doc = "Delay is -20%"]
    #[inline(always)]
    pub fn is_entailadjvd_2(&self) -> bool {
        *self == Entailadjvd::Entailadjvd2
    }
    #[doc = "Delay is -40%"]
    #[inline(always)]
    pub fn is_entailadjvd_3(&self) -> bool {
        *self == Entailadjvd::Entailadjvd3
    }
}
#[doc = "Field `ENTAILADJVD` writer - Delay increment of the rise of squelch:"]
pub type EntailadjvdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Entailadjvd, crate::Safe>;
impl<'a, REG> EntailadjvdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay is nominal"]
    #[inline(always)]
    pub fn entailadjvd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Entailadjvd::Entailadjvd0)
    }
    #[doc = "Delay is +20%"]
    #[inline(always)]
    pub fn entailadjvd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Entailadjvd::Entailadjvd1)
    }
    #[doc = "Delay is -20%"]
    #[inline(always)]
    pub fn entailadjvd_2(self) -> &'a mut crate::W<REG> {
        self.variant(Entailadjvd::Entailadjvd2)
    }
    #[doc = "Delay is -40%"]
    #[inline(always)]
    pub fn entailadjvd_3(self) -> &'a mut crate::W<REG> {
        self.variant(Entailadjvd::Entailadjvd3)
    }
}
#[doc = "Field `USB2_REFBIAS_VBGADJ` reader - Adjustment bits on bandgap"]
pub type Usb2RefbiasVbgadjR = crate::FieldReader;
#[doc = "Field `USB2_REFBIAS_VBGADJ` writer - Adjustment bits on bandgap"]
pub type Usb2RefbiasVbgadjW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USB2_REFBIAS_TST` reader - Bias current control for usb2_phy"]
pub type Usb2RefbiasTstR = crate::FieldReader;
#[doc = "Field `USB2_REFBIAS_TST` writer - Bias current control for usb2_phy"]
pub type Usb2RefbiasTstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub fn entailadjvd(&self) -> EntailadjvdR {
        EntailadjvdR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Adjustment bits on bandgap"]
    #[inline(always)]
    pub fn usb2_refbias_vbgadj(&self) -> Usb2RefbiasVbgadjR {
        Usb2RefbiasVbgadjR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Bias current control for usb2_phy"]
    #[inline(always)]
    pub fn usb2_refbias_tst(&self) -> Usb2RefbiasTstR {
        Usb2RefbiasTstR::new(((self.bits >> 21) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG1_TOG")
            .field("entailadjvd", &self.entailadjvd())
            .field("usb2_refbias_vbgadj", &self.usb2_refbias_vbgadj())
            .field("usb2_refbias_tst", &self.usb2_refbias_tst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline(always)]
    #[must_use]
    pub fn entailadjvd(&mut self) -> EntailadjvdW<Debug1TogSpec> {
        EntailadjvdW::new(self, 13)
    }
    #[doc = "Bits 18:20 - Adjustment bits on bandgap"]
    #[inline(always)]
    #[must_use]
    pub fn usb2_refbias_vbgadj(&mut self) -> Usb2RefbiasVbgadjW<Debug1TogSpec> {
        Usb2RefbiasVbgadjW::new(self, 18)
    }
    #[doc = "Bits 21:22 - Bias current control for usb2_phy"]
    #[inline(always)]
    #[must_use]
    pub fn usb2_refbias_tst(&mut self) -> Usb2RefbiasTstW<Debug1TogSpec> {
        Usb2RefbiasTstW::new(self, 21)
    }
}
#[doc = "UTMI Debug Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug1_tog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug1_tog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Debug1TogSpec;
impl crate::RegisterSpec for Debug1TogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug1_tog::R`](R) reader structure"]
impl crate::Readable for Debug1TogSpec {}
#[doc = "`write(|w| ..)` method takes [`debug1_tog::W`](W) writer structure"]
impl crate::Writable for Debug1TogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG1_TOG to value 0x1000"]
impl crate::Resettable for Debug1TogSpec {
    const RESET_VALUE: u32 = 0x1000;
}
