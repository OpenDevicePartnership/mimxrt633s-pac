#[doc = "Register `GATE%s` reader"]
pub type R = crate::R<GateSpec>;
#[doc = "Register `GATE%s` writer"]
pub type W = crate::W<GateSpec>;
#[doc = "ate Finite State Machine. The hardware gate is maintained in a 16-state implementation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gtfsm {
    #[doc = "0: The gate is unlocked (free)."]
    Gtfsm0 = 0,
    #[doc = "1: The gate has been locked by processor 0."]
    Gtfsm1 = 1,
    #[doc = "2: The gate has been locked by processor 1."]
    Gtfsm2 = 2,
}
impl From<Gtfsm> for u8 {
    #[inline(always)]
    fn from(variant: Gtfsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gtfsm {
    type Ux = u8;
}
impl crate::IsEnum for Gtfsm {}
#[doc = "Field `GTFSM` reader - ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
pub type GtfsmR = crate::FieldReader<Gtfsm>;
impl GtfsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gtfsm> {
        match self.bits {
            0 => Some(Gtfsm::Gtfsm0),
            1 => Some(Gtfsm::Gtfsm1),
            2 => Some(Gtfsm::Gtfsm2),
            _ => None,
        }
    }
    #[doc = "The gate is unlocked (free)."]
    #[inline(always)]
    pub fn is_gtfsm_0(&self) -> bool {
        *self == Gtfsm::Gtfsm0
    }
    #[doc = "The gate has been locked by processor 0."]
    #[inline(always)]
    pub fn is_gtfsm_1(&self) -> bool {
        *self == Gtfsm::Gtfsm1
    }
    #[doc = "The gate has been locked by processor 1."]
    #[inline(always)]
    pub fn is_gtfsm_2(&self) -> bool {
        *self == Gtfsm::Gtfsm2
    }
}
#[doc = "Field `GTFSM` writer - ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
pub type GtfsmW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gtfsm>;
impl<'a, REG> GtfsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The gate is unlocked (free)."]
    #[inline(always)]
    pub fn gtfsm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gtfsm::Gtfsm0)
    }
    #[doc = "The gate has been locked by processor 0."]
    #[inline(always)]
    pub fn gtfsm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtfsm::Gtfsm1)
    }
    #[doc = "The gate has been locked by processor 1."]
    #[inline(always)]
    pub fn gtfsm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gtfsm::Gtfsm2)
    }
}
impl R {
    #[doc = "Bits 0:3 - ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
    #[inline(always)]
    pub fn gtfsm(&self) -> GtfsmR {
        GtfsmR::new(self.bits & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GATE")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
    #[inline(always)]
    pub fn gtfsm(&mut self) -> GtfsmW<GateSpec> {
        GtfsmW::new(self, 0)
    }
}
#[doc = "Semphores2 Gate n\n\nYou can [`read`](crate::Reg::read) this register and get [`gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GateSpec;
impl crate::RegisterSpec for GateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gate::R`](R) reader structure"]
impl crate::Readable for GateSpec {}
#[doc = "`write(|w| ..)` method takes [`gate::W`](W) writer structure"]
impl crate::Writable for GateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATE%s to value 0"]
impl crate::Resettable for GateSpec {
    const RESET_VALUE: u8 = 0;
}
