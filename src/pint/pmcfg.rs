#[doc = "Register `PMCFG` reader"]
pub type R = crate::R<PmcfgSpec>;
#[doc = "Register `PMCFG` writer"]
pub type W = crate::W<PmcfgSpec>;
#[doc = "Determines whether slice 0 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts0 {
    #[doc = "0: No effect. Slice 0 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts0> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS0` reader - Determines whether slice 0 is an endpoint."]
pub type ProdEndpts0R = crate::BitReader<ProdEndpts0>;
impl ProdEndpts0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts0 {
        match self.bits {
            false => ProdEndpts0::NoEffect,
            true => ProdEndpts0::Endpoint,
        }
    }
    #[doc = "No effect. Slice 0 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts0::NoEffect
    }
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts0::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS0` writer - Determines whether slice 0 is an endpoint."]
pub type ProdEndpts0W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts0>;
impl<'a, REG> ProdEndpts0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 0 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts0::NoEffect)
    }
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts0::Endpoint)
    }
}
#[doc = "Determines whether slice 1 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts1 {
    #[doc = "0: No effect. Slice 1 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts1> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS1` reader - Determines whether slice 1 is an endpoint."]
pub type ProdEndpts1R = crate::BitReader<ProdEndpts1>;
impl ProdEndpts1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts1 {
        match self.bits {
            false => ProdEndpts1::NoEffect,
            true => ProdEndpts1::Endpoint,
        }
    }
    #[doc = "No effect. Slice 1 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts1::NoEffect
    }
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts1::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS1` writer - Determines whether slice 1 is an endpoint."]
pub type ProdEndpts1W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts1>;
impl<'a, REG> ProdEndpts1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 1 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts1::NoEffect)
    }
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts1::Endpoint)
    }
}
#[doc = "Determines whether slice 2 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts2 {
    #[doc = "0: No effect. Slice 2 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts2> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS2` reader - Determines whether slice 2 is an endpoint."]
pub type ProdEndpts2R = crate::BitReader<ProdEndpts2>;
impl ProdEndpts2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts2 {
        match self.bits {
            false => ProdEndpts2::NoEffect,
            true => ProdEndpts2::Endpoint,
        }
    }
    #[doc = "No effect. Slice 2 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts2::NoEffect
    }
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts2::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS2` writer - Determines whether slice 2 is an endpoint."]
pub type ProdEndpts2W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts2>;
impl<'a, REG> ProdEndpts2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 2 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts2::NoEffect)
    }
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts2::Endpoint)
    }
}
#[doc = "Determines whether slice 3 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts3 {
    #[doc = "0: No effect. Slice 3 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts3> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS3` reader - Determines whether slice 3 is an endpoint."]
pub type ProdEndpts3R = crate::BitReader<ProdEndpts3>;
impl ProdEndpts3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts3 {
        match self.bits {
            false => ProdEndpts3::NoEffect,
            true => ProdEndpts3::Endpoint,
        }
    }
    #[doc = "No effect. Slice 3 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts3::NoEffect
    }
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts3::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS3` writer - Determines whether slice 3 is an endpoint."]
pub type ProdEndpts3W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts3>;
impl<'a, REG> ProdEndpts3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 3 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts3::NoEffect)
    }
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts3::Endpoint)
    }
}
#[doc = "Determines whether slice 4 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts4 {
    #[doc = "0: No effect. Slice 4 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts4> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS4` reader - Determines whether slice 4 is an endpoint."]
pub type ProdEndpts4R = crate::BitReader<ProdEndpts4>;
impl ProdEndpts4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts4 {
        match self.bits {
            false => ProdEndpts4::NoEffect,
            true => ProdEndpts4::Endpoint,
        }
    }
    #[doc = "No effect. Slice 4 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts4::NoEffect
    }
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts4::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS4` writer - Determines whether slice 4 is an endpoint."]
pub type ProdEndpts4W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts4>;
impl<'a, REG> ProdEndpts4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 4 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts4::NoEffect)
    }
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts4::Endpoint)
    }
}
#[doc = "Determines whether slice 5 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts5 {
    #[doc = "0: No effect. Slice 5 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts5> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS5` reader - Determines whether slice 5 is an endpoint."]
pub type ProdEndpts5R = crate::BitReader<ProdEndpts5>;
impl ProdEndpts5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts5 {
        match self.bits {
            false => ProdEndpts5::NoEffect,
            true => ProdEndpts5::Endpoint,
        }
    }
    #[doc = "No effect. Slice 5 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts5::NoEffect
    }
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts5::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS5` writer - Determines whether slice 5 is an endpoint."]
pub type ProdEndpts5W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts5>;
impl<'a, REG> ProdEndpts5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 5 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts5::NoEffect)
    }
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts5::Endpoint)
    }
}
#[doc = "Determines whether slice 6 is an endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProdEndpts6 {
    #[doc = "0: No effect. Slice 6 is not an endpoint."]
    NoEffect = 0,
    #[doc = "1: endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    Endpoint = 1,
}
impl From<ProdEndpts6> for bool {
    #[inline(always)]
    fn from(variant: ProdEndpts6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS6` reader - Determines whether slice 6 is an endpoint."]
pub type ProdEndpts6R = crate::BitReader<ProdEndpts6>;
impl ProdEndpts6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProdEndpts6 {
        match self.bits {
            false => ProdEndpts6::NoEffect,
            true => ProdEndpts6::Endpoint,
        }
    }
    #[doc = "No effect. Slice 6 is not an endpoint."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ProdEndpts6::NoEffect
    }
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == ProdEndpts6::Endpoint
    }
}
#[doc = "Field `PROD_ENDPTS6` writer - Determines whether slice 6 is an endpoint."]
pub type ProdEndpts6W<'a, REG> = crate::BitWriter<'a, REG, ProdEndpts6>;
impl<'a, REG> ProdEndpts6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Slice 6 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts6::NoEffect)
    }
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut crate::W<REG> {
        self.variant(ProdEndpts6::Endpoint)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg0 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg0> for u8 {
    #[inline(always)]
    fn from(variant: Cfg0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg0 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg0 {}
#[doc = "Field `CFG0` reader - Specifies the match contribution condition for bit slice 0."]
pub type Cfg0R = crate::FieldReader<Cfg0>;
impl Cfg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg0 {
        match self.bits {
            0 => Cfg0::ConstantHigh,
            1 => Cfg0::StickyRisingEdge,
            2 => Cfg0::StickyFallingEdge,
            3 => Cfg0::StickyRisingFallingEdge,
            4 => Cfg0::HighLevel,
            5 => Cfg0::LowLevel,
            6 => Cfg0::ConstantZero,
            7 => Cfg0::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg0::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg0::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg0::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg0::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg0::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg0::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg0::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg0::Event
    }
}
#[doc = "Field `CFG0` writer - Specifies the match contribution condition for bit slice 0."]
pub type Cfg0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg0, crate::Safe>;
impl<'a, REG> Cfg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg0::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg1 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg1> for u8 {
    #[inline(always)]
    fn from(variant: Cfg1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg1 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg1 {}
#[doc = "Field `CFG1` reader - Specifies the match contribution condition for bit slice 1."]
pub type Cfg1R = crate::FieldReader<Cfg1>;
impl Cfg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg1 {
        match self.bits {
            0 => Cfg1::ConstantHigh,
            1 => Cfg1::StickyRisingEdge,
            2 => Cfg1::StickyFallingEdge,
            3 => Cfg1::StickyRisingFallingEdge,
            4 => Cfg1::HighLevel,
            5 => Cfg1::LowLevel,
            6 => Cfg1::ConstantZero,
            7 => Cfg1::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg1::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg1::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg1::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg1::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg1::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg1::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg1::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg1::Event
    }
}
#[doc = "Field `CFG1` writer - Specifies the match contribution condition for bit slice 1."]
pub type Cfg1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg1, crate::Safe>;
impl<'a, REG> Cfg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg1::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 2.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg2 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg2> for u8 {
    #[inline(always)]
    fn from(variant: Cfg2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg2 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg2 {}
#[doc = "Field `CFG2` reader - Specifies the match contribution condition for bit slice 2."]
pub type Cfg2R = crate::FieldReader<Cfg2>;
impl Cfg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg2 {
        match self.bits {
            0 => Cfg2::ConstantHigh,
            1 => Cfg2::StickyRisingEdge,
            2 => Cfg2::StickyFallingEdge,
            3 => Cfg2::StickyRisingFallingEdge,
            4 => Cfg2::HighLevel,
            5 => Cfg2::LowLevel,
            6 => Cfg2::ConstantZero,
            7 => Cfg2::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg2::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg2::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg2::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg2::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg2::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg2::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg2::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg2::Event
    }
}
#[doc = "Field `CFG2` writer - Specifies the match contribution condition for bit slice 2."]
pub type Cfg2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg2, crate::Safe>;
impl<'a, REG> Cfg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg2::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 3.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg3 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg3> for u8 {
    #[inline(always)]
    fn from(variant: Cfg3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg3 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg3 {}
#[doc = "Field `CFG3` reader - Specifies the match contribution condition for bit slice 3."]
pub type Cfg3R = crate::FieldReader<Cfg3>;
impl Cfg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg3 {
        match self.bits {
            0 => Cfg3::ConstantHigh,
            1 => Cfg3::StickyRisingEdge,
            2 => Cfg3::StickyFallingEdge,
            3 => Cfg3::StickyRisingFallingEdge,
            4 => Cfg3::HighLevel,
            5 => Cfg3::LowLevel,
            6 => Cfg3::ConstantZero,
            7 => Cfg3::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg3::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg3::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg3::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg3::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg3::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg3::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg3::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg3::Event
    }
}
#[doc = "Field `CFG3` writer - Specifies the match contribution condition for bit slice 3."]
pub type Cfg3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg3, crate::Safe>;
impl<'a, REG> Cfg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg3::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 4.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg4 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg4> for u8 {
    #[inline(always)]
    fn from(variant: Cfg4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg4 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg4 {}
#[doc = "Field `CFG4` reader - Specifies the match contribution condition for bit slice 4."]
pub type Cfg4R = crate::FieldReader<Cfg4>;
impl Cfg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg4 {
        match self.bits {
            0 => Cfg4::ConstantHigh,
            1 => Cfg4::StickyRisingEdge,
            2 => Cfg4::StickyFallingEdge,
            3 => Cfg4::StickyRisingFallingEdge,
            4 => Cfg4::HighLevel,
            5 => Cfg4::LowLevel,
            6 => Cfg4::ConstantZero,
            7 => Cfg4::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg4::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg4::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg4::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg4::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg4::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg4::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg4::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg4::Event
    }
}
#[doc = "Field `CFG4` writer - Specifies the match contribution condition for bit slice 4."]
pub type Cfg4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg4, crate::Safe>;
impl<'a, REG> Cfg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg4::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 5.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg5 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg5> for u8 {
    #[inline(always)]
    fn from(variant: Cfg5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg5 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg5 {}
#[doc = "Field `CFG5` reader - Specifies the match contribution condition for bit slice 5."]
pub type Cfg5R = crate::FieldReader<Cfg5>;
impl Cfg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg5 {
        match self.bits {
            0 => Cfg5::ConstantHigh,
            1 => Cfg5::StickyRisingEdge,
            2 => Cfg5::StickyFallingEdge,
            3 => Cfg5::StickyRisingFallingEdge,
            4 => Cfg5::HighLevel,
            5 => Cfg5::LowLevel,
            6 => Cfg5::ConstantZero,
            7 => Cfg5::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg5::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg5::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg5::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg5::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg5::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg5::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg5::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg5::Event
    }
}
#[doc = "Field `CFG5` writer - Specifies the match contribution condition for bit slice 5."]
pub type Cfg5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg5, crate::Safe>;
impl<'a, REG> Cfg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg5::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 6.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg6 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg6> for u8 {
    #[inline(always)]
    fn from(variant: Cfg6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg6 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg6 {}
#[doc = "Field `CFG6` reader - Specifies the match contribution condition for bit slice 6."]
pub type Cfg6R = crate::FieldReader<Cfg6>;
impl Cfg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg6 {
        match self.bits {
            0 => Cfg6::ConstantHigh,
            1 => Cfg6::StickyRisingEdge,
            2 => Cfg6::StickyFallingEdge,
            3 => Cfg6::StickyRisingFallingEdge,
            4 => Cfg6::HighLevel,
            5 => Cfg6::LowLevel,
            6 => Cfg6::ConstantZero,
            7 => Cfg6::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg6::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg6::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg6::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg6::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg6::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg6::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg6::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg6::Event
    }
}
#[doc = "Field `CFG6` writer - Specifies the match contribution condition for bit slice 6."]
pub type Cfg6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg6, crate::Safe>;
impl<'a, REG> Cfg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg6::Event)
    }
}
#[doc = "Specifies the match contribution condition for bit slice 7.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg7 {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    ConstantHigh = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingEdge = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyFallingEdge = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    StickyRisingFallingEdge = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HighLevel = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LowLevel = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    ConstantZero = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    Event = 7,
}
impl From<Cfg7> for u8 {
    #[inline(always)]
    fn from(variant: Cfg7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg7 {
    type Ux = u8;
}
impl crate::IsEnum for Cfg7 {}
#[doc = "Field `CFG7` reader - Specifies the match contribution condition for bit slice 7."]
pub type Cfg7R = crate::FieldReader<Cfg7>;
impl Cfg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg7 {
        match self.bits {
            0 => Cfg7::ConstantHigh,
            1 => Cfg7::StickyRisingEdge,
            2 => Cfg7::StickyFallingEdge,
            3 => Cfg7::StickyRisingFallingEdge,
            4 => Cfg7::HighLevel,
            5 => Cfg7::LowLevel,
            6 => Cfg7::ConstantZero,
            7 => Cfg7::Event,
            _ => unreachable!(),
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == Cfg7::ConstantHigh
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == Cfg7::StickyRisingEdge
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == Cfg7::StickyFallingEdge
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == Cfg7::StickyRisingFallingEdge
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == Cfg7::HighLevel
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == Cfg7::LowLevel
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == Cfg7::ConstantZero
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Cfg7::Event
    }
}
#[doc = "Field `CFG7` writer - Specifies the match contribution condition for bit slice 7."]
pub type Cfg7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg7, crate::Safe>;
impl<'a, REG> Cfg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::ConstantHigh)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::StickyRisingEdge)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::StickyFallingEdge)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::StickyRisingFallingEdge)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::HighLevel)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::LowLevel)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::ConstantZero)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg7::Event)
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&self) -> ProdEndpts0R {
        ProdEndpts0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&self) -> ProdEndpts1R {
        ProdEndpts1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&self) -> ProdEndpts2R {
        ProdEndpts2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&self) -> ProdEndpts3R {
        ProdEndpts3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&self) -> ProdEndpts4R {
        ProdEndpts4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&self) -> ProdEndpts5R {
        ProdEndpts5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&self) -> ProdEndpts6R {
        ProdEndpts6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&self) -> Cfg0R {
        Cfg0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&self) -> Cfg1R {
        Cfg1R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&self) -> Cfg2R {
        Cfg2R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&self) -> Cfg3R {
        Cfg3R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&self) -> Cfg4R {
        Cfg4R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&self) -> Cfg5R {
        Cfg5R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&self) -> Cfg6R {
        Cfg6R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&self) -> Cfg7R {
        Cfg7R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCFG")
            .field("prod_endpts0", &self.prod_endpts0())
            .field("prod_endpts1", &self.prod_endpts1())
            .field("prod_endpts2", &self.prod_endpts2())
            .field("prod_endpts3", &self.prod_endpts3())
            .field("prod_endpts4", &self.prod_endpts4())
            .field("prod_endpts5", &self.prod_endpts5())
            .field("prod_endpts6", &self.prod_endpts6())
            .field("cfg0", &self.cfg0())
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .field("cfg5", &self.cfg5())
            .field("cfg6", &self.cfg6())
            .field("cfg7", &self.cfg7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&mut self) -> ProdEndpts0W<PmcfgSpec> {
        ProdEndpts0W::new(self, 0)
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&mut self) -> ProdEndpts1W<PmcfgSpec> {
        ProdEndpts1W::new(self, 1)
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&mut self) -> ProdEndpts2W<PmcfgSpec> {
        ProdEndpts2W::new(self, 2)
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&mut self) -> ProdEndpts3W<PmcfgSpec> {
        ProdEndpts3W::new(self, 3)
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&mut self) -> ProdEndpts4W<PmcfgSpec> {
        ProdEndpts4W::new(self, 4)
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&mut self) -> ProdEndpts5W<PmcfgSpec> {
        ProdEndpts5W::new(self, 5)
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&mut self) -> ProdEndpts6W<PmcfgSpec> {
        ProdEndpts6W::new(self, 6)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&mut self) -> Cfg0W<PmcfgSpec> {
        Cfg0W::new(self, 8)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&mut self) -> Cfg1W<PmcfgSpec> {
        Cfg1W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&mut self) -> Cfg2W<PmcfgSpec> {
        Cfg2W::new(self, 14)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&mut self) -> Cfg3W<PmcfgSpec> {
        Cfg3W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&mut self) -> Cfg4W<PmcfgSpec> {
        Cfg4W::new(self, 20)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&mut self) -> Cfg5W<PmcfgSpec> {
        Cfg5W::new(self, 23)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&mut self) -> Cfg6W<PmcfgSpec> {
        Cfg6W::new(self, 26)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&mut self) -> Cfg7W<PmcfgSpec> {
        Cfg7W::new(self, 29)
    }
}
#[doc = "Pattern match interrupt bit slice configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcfgSpec;
impl crate::RegisterSpec for PmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcfg::R`](R) reader structure"]
impl crate::Readable for PmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmcfg::W`](W) writer structure"]
impl crate::Writable for PmcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMCFG to value 0"]
impl crate::Resettable for PmcfgSpec {
    const RESET_VALUE: u32 = 0;
}
