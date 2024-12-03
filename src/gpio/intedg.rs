#[doc = "Register `INTEDG[%s]` reader"]
pub type R = crate::R<IntedgSpec>;
#[doc = "Register `INTEDG[%s]` writer"]
pub type W = crate::W<IntedgSpec>;
#[doc = "choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Edge {
    #[doc = "0: level"]
    Level = 0,
    #[doc = "1: edge"]
    Edge = 1,
}
impl From<Edge> for u32 {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edge {
    type Ux = u32;
}
impl crate::IsEnum for Edge {}
#[doc = "Field `EDGE` reader - choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)"]
pub type EdgeR = crate::FieldReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Edge> {
        match self.bits {
            0 => Some(Edge::Level),
            1 => Some(Edge::Edge),
            _ => None,
        }
    }
    #[doc = "level"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Edge::Level
    }
    #[doc = "edge"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Edge::Edge
    }
}
#[doc = "Field `EDGE` writer - choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)"]
pub type EdgeW<'a, REG> = crate::FieldWriter<'a, REG, 32, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Level)
    }
    #[doc = "edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Edge)
    }
}
impl R {
    #[doc = "Bits 0:31 - choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEDG")
            .field("edge", &self.edge())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<IntedgSpec> {
        EdgeW::new(self, 0)
    }
}
#[doc = "choose edge or level for interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intedg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intedg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntedgSpec;
impl crate::RegisterSpec for IntedgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intedg::R`](R) reader structure"]
impl crate::Readable for IntedgSpec {}
#[doc = "`write(|w| ..)` method takes [`intedg::W`](W) writer structure"]
impl crate::Writable for IntedgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEDG[%s]
to value 0"]
impl crate::Resettable for IntedgSpec {
    const RESET_VALUE: u32 = 0;
}
