#[doc = "Register `DMAC1_REQ_ENA1` reader"]
pub type R = crate::R<Dmac1ReqEna1Spec>;
#[doc = "Register `DMAC1_REQ_ENA1` writer"]
pub type W = crate::W<Dmac1ReqEna1Spec>;
#[doc = "ESPI Channel 1 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EspiCh1 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<EspiCh1> for bool {
    #[inline(always)]
    fn from(variant: EspiCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESPI_CH1` reader - ESPI Channel 1 enable"]
pub type EspiCh1R = crate::BitReader<EspiCh1>;
impl EspiCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EspiCh1 {
        match self.bits {
            false => EspiCh1::Disabled,
            true => EspiCh1::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EspiCh1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EspiCh1::Enabled
    }
}
#[doc = "Field `ESPI_CH1` writer - ESPI Channel 1 enable"]
pub type EspiCh1W<'a, REG> = crate::BitWriter<'a, REG, EspiCh1>;
impl<'a, REG> EspiCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh1::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ESPI Channel 1 enable"]
    #[inline(always)]
    pub fn espi_ch1(&self) -> EspiCh1R {
        EspiCh1R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1_REQ_ENA1")
            .field("espi_ch1", &self.espi_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ESPI Channel 1 enable"]
    #[inline(always)]
    pub fn espi_ch1(&mut self) -> EspiCh1W<Dmac1ReqEna1Spec> {
        EspiCh1W::new(self, 0)
    }
}
#[doc = "DMAC1 request enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_req_ena1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac1ReqEna1Spec;
impl crate::RegisterSpec for Dmac1ReqEna1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac1_req_ena1::R`](R) reader structure"]
impl crate::Readable for Dmac1ReqEna1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmac1_req_ena1::W`](W) writer structure"]
impl crate::Writable for Dmac1ReqEna1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC1_REQ_ENA1 to value 0x01"]
impl crate::Resettable for Dmac1ReqEna1Spec {
    const RESET_VALUE: u32 = 0x01;
}
