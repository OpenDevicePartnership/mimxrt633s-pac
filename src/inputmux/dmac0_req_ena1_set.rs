#[doc = "Register `DMAC0_REQ_ENA1_SET` writer"]
pub type W = crate::W<Dmac0ReqEna1SetSpec>;
#[doc = "ESPI Channel 1 enable set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EspiCh1 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the ENA1 Bit"]
    SetEna0Bit = 1,
}
impl From<EspiCh1> for bool {
    #[inline(always)]
    fn from(variant: EspiCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESPI_CH1` writer - ESPI Channel 1 enable set"]
pub type EspiCh1W<'a, REG> = crate::BitWriter<'a, REG, EspiCh1>;
impl<'a, REG> EspiCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh1::NoEffect)
    }
    #[doc = "Sets the ENA1 Bit"]
    #[inline(always)]
    pub fn set_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh1::SetEna0Bit)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dmac0ReqEna1SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ESPI Channel 1 enable set"]
    #[inline(always)]
    pub fn espi_ch1(&mut self) -> EspiCh1W<Dmac0ReqEna1SetSpec> {
        EspiCh1W::new(self, 0)
    }
}
#[doc = "DMAC0 request enable set 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0ReqEna1SetSpec;
impl crate::RegisterSpec for Dmac0ReqEna1SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac0_req_ena1_set::W`](W) writer structure"]
impl crate::Writable for Dmac0ReqEna1SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_REQ_ENA1_SET to value 0"]
impl crate::Resettable for Dmac0ReqEna1SetSpec {
    const RESET_VALUE: u32 = 0;
}
