#[doc = "Register `OUTPUTDIRCTRL` reader"]
pub type R = crate::R<OutputdirctrlSpec>;
#[doc = "Register `OUTPUTDIRCTRL` writer"]
pub type W = crate::W<OutputdirctrlSpec>;
#[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr0 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr0> for u8 {
    #[inline(always)]
    fn from(variant: Setclr0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr0 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr0 {}
#[doc = "Field `SETCLR0` reader - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub type Setclr0R = crate::FieldReader<Setclr0>;
impl Setclr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr0> {
        match self.bits {
            0 => Some(Setclr0::Independent),
            1 => Some(Setclr0::LReversed),
            2 => Some(Setclr0::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr0::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr0::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr0::HReversed
    }
}
#[doc = "Field `SETCLR0` writer - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub type Setclr0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr0>;
impl<'a, REG> Setclr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr0::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr0::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr0::HReversed)
    }
}
#[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr1 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr1> for u8 {
    #[inline(always)]
    fn from(variant: Setclr1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr1 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr1 {}
#[doc = "Field `SETCLR1` reader - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub type Setclr1R = crate::FieldReader<Setclr1>;
impl Setclr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr1> {
        match self.bits {
            0 => Some(Setclr1::Independent),
            1 => Some(Setclr1::LReversed),
            2 => Some(Setclr1::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr1::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr1::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr1::HReversed
    }
}
#[doc = "Field `SETCLR1` writer - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub type Setclr1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr1>;
impl<'a, REG> Setclr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr1::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr1::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr1::HReversed)
    }
}
#[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr2 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr2> for u8 {
    #[inline(always)]
    fn from(variant: Setclr2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr2 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr2 {}
#[doc = "Field `SETCLR2` reader - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub type Setclr2R = crate::FieldReader<Setclr2>;
impl Setclr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr2> {
        match self.bits {
            0 => Some(Setclr2::Independent),
            1 => Some(Setclr2::LReversed),
            2 => Some(Setclr2::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr2::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr2::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr2::HReversed
    }
}
#[doc = "Field `SETCLR2` writer - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub type Setclr2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr2>;
impl<'a, REG> Setclr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr2::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr2::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr2::HReversed)
    }
}
#[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr3 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr3> for u8 {
    #[inline(always)]
    fn from(variant: Setclr3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr3 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr3 {}
#[doc = "Field `SETCLR3` reader - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub type Setclr3R = crate::FieldReader<Setclr3>;
impl Setclr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr3> {
        match self.bits {
            0 => Some(Setclr3::Independent),
            1 => Some(Setclr3::LReversed),
            2 => Some(Setclr3::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr3::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr3::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr3::HReversed
    }
}
#[doc = "Field `SETCLR3` writer - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub type Setclr3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr3>;
impl<'a, REG> Setclr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr3::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr3::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr3::HReversed)
    }
}
#[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr4 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr4> for u8 {
    #[inline(always)]
    fn from(variant: Setclr4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr4 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr4 {}
#[doc = "Field `SETCLR4` reader - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
pub type Setclr4R = crate::FieldReader<Setclr4>;
impl Setclr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr4> {
        match self.bits {
            0 => Some(Setclr4::Independent),
            1 => Some(Setclr4::LReversed),
            2 => Some(Setclr4::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr4::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr4::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr4::HReversed
    }
}
#[doc = "Field `SETCLR4` writer - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
pub type Setclr4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr4>;
impl<'a, REG> Setclr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr4::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr4::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr4::HReversed)
    }
}
#[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr5 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr5> for u8 {
    #[inline(always)]
    fn from(variant: Setclr5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr5 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr5 {}
#[doc = "Field `SETCLR5` reader - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
pub type Setclr5R = crate::FieldReader<Setclr5>;
impl Setclr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr5> {
        match self.bits {
            0 => Some(Setclr5::Independent),
            1 => Some(Setclr5::LReversed),
            2 => Some(Setclr5::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr5::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr5::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr5::HReversed
    }
}
#[doc = "Field `SETCLR5` writer - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
pub type Setclr5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr5>;
impl<'a, REG> Setclr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr5::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr5::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr5::HReversed)
    }
}
#[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr6 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr6> for u8 {
    #[inline(always)]
    fn from(variant: Setclr6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr6 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr6 {}
#[doc = "Field `SETCLR6` reader - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
pub type Setclr6R = crate::FieldReader<Setclr6>;
impl Setclr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr6> {
        match self.bits {
            0 => Some(Setclr6::Independent),
            1 => Some(Setclr6::LReversed),
            2 => Some(Setclr6::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr6::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr6::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr6::HReversed
    }
}
#[doc = "Field `SETCLR6` writer - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
pub type Setclr6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr6>;
impl<'a, REG> Setclr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr6::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr6::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr6::HReversed)
    }
}
#[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr7 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr7> for u8 {
    #[inline(always)]
    fn from(variant: Setclr7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr7 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr7 {}
#[doc = "Field `SETCLR7` reader - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
pub type Setclr7R = crate::FieldReader<Setclr7>;
impl Setclr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr7> {
        match self.bits {
            0 => Some(Setclr7::Independent),
            1 => Some(Setclr7::LReversed),
            2 => Some(Setclr7::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr7::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr7::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr7::HReversed
    }
}
#[doc = "Field `SETCLR7` writer - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
pub type Setclr7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr7>;
impl<'a, REG> Setclr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr7::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr7::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr7::HReversed)
    }
}
#[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr8 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr8> for u8 {
    #[inline(always)]
    fn from(variant: Setclr8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr8 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr8 {}
#[doc = "Field `SETCLR8` reader - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
pub type Setclr8R = crate::FieldReader<Setclr8>;
impl Setclr8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr8> {
        match self.bits {
            0 => Some(Setclr8::Independent),
            1 => Some(Setclr8::LReversed),
            2 => Some(Setclr8::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr8::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr8::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr8::HReversed
    }
}
#[doc = "Field `SETCLR8` writer - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
pub type Setclr8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr8>;
impl<'a, REG> Setclr8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr8::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr8::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr8::HReversed)
    }
}
#[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr9 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr9> for u8 {
    #[inline(always)]
    fn from(variant: Setclr9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr9 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr9 {}
#[doc = "Field `SETCLR9` reader - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
pub type Setclr9R = crate::FieldReader<Setclr9>;
impl Setclr9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr9> {
        match self.bits {
            0 => Some(Setclr9::Independent),
            1 => Some(Setclr9::LReversed),
            2 => Some(Setclr9::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr9::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr9::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr9::HReversed
    }
}
#[doc = "Field `SETCLR9` writer - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
pub type Setclr9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr9>;
impl<'a, REG> Setclr9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr9::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr9::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr9::HReversed)
    }
}
#[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr10 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr10> for u8 {
    #[inline(always)]
    fn from(variant: Setclr10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr10 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr10 {}
#[doc = "Field `SETCLR10` reader - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
pub type Setclr10R = crate::FieldReader<Setclr10>;
impl Setclr10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr10> {
        match self.bits {
            0 => Some(Setclr10::Independent),
            1 => Some(Setclr10::LReversed),
            2 => Some(Setclr10::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr10::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr10::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr10::HReversed
    }
}
#[doc = "Field `SETCLR10` writer - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
pub type Setclr10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr10>;
impl<'a, REG> Setclr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr10::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr10::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr10::HReversed)
    }
}
#[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr11 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr11> for u8 {
    #[inline(always)]
    fn from(variant: Setclr11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr11 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr11 {}
#[doc = "Field `SETCLR11` reader - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
pub type Setclr11R = crate::FieldReader<Setclr11>;
impl Setclr11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr11> {
        match self.bits {
            0 => Some(Setclr11::Independent),
            1 => Some(Setclr11::LReversed),
            2 => Some(Setclr11::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr11::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr11::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr11::HReversed
    }
}
#[doc = "Field `SETCLR11` writer - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
pub type Setclr11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr11>;
impl<'a, REG> Setclr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr11::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr11::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr11::HReversed)
    }
}
#[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr12 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr12> for u8 {
    #[inline(always)]
    fn from(variant: Setclr12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr12 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr12 {}
#[doc = "Field `SETCLR12` reader - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
pub type Setclr12R = crate::FieldReader<Setclr12>;
impl Setclr12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr12> {
        match self.bits {
            0 => Some(Setclr12::Independent),
            1 => Some(Setclr12::LReversed),
            2 => Some(Setclr12::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr12::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr12::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr12::HReversed
    }
}
#[doc = "Field `SETCLR12` writer - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
pub type Setclr12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr12>;
impl<'a, REG> Setclr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr12::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr12::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr12::HReversed)
    }
}
#[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr13 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr13> for u8 {
    #[inline(always)]
    fn from(variant: Setclr13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr13 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr13 {}
#[doc = "Field `SETCLR13` reader - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
pub type Setclr13R = crate::FieldReader<Setclr13>;
impl Setclr13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr13> {
        match self.bits {
            0 => Some(Setclr13::Independent),
            1 => Some(Setclr13::LReversed),
            2 => Some(Setclr13::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr13::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr13::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr13::HReversed
    }
}
#[doc = "Field `SETCLR13` writer - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
pub type Setclr13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr13>;
impl<'a, REG> Setclr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr13::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr13::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr13::HReversed)
    }
}
#[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr14 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr14> for u8 {
    #[inline(always)]
    fn from(variant: Setclr14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr14 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr14 {}
#[doc = "Field `SETCLR14` reader - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
pub type Setclr14R = crate::FieldReader<Setclr14>;
impl Setclr14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr14> {
        match self.bits {
            0 => Some(Setclr14::Independent),
            1 => Some(Setclr14::LReversed),
            2 => Some(Setclr14::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr14::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr14::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr14::HReversed
    }
}
#[doc = "Field `SETCLR14` writer - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
pub type Setclr14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr14>;
impl<'a, REG> Setclr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr14::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr14::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr14::HReversed)
    }
}
#[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setclr15 {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    Independent = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 2,
}
impl From<Setclr15> for u8 {
    #[inline(always)]
    fn from(variant: Setclr15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setclr15 {
    type Ux = u8;
}
impl crate::IsEnum for Setclr15 {}
#[doc = "Field `SETCLR15` reader - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
pub type Setclr15R = crate::FieldReader<Setclr15>;
impl Setclr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setclr15> {
        match self.bits {
            0 => Some(Setclr15::Independent),
            1 => Some(Setclr15::LReversed),
            2 => Some(Setclr15::HReversed),
            _ => None,
        }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Setclr15::Independent
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == Setclr15::LReversed
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == Setclr15::HReversed
    }
}
#[doc = "Field `SETCLR15` writer - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
pub type Setclr15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Setclr15>;
impl<'a, REG> Setclr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr15::Independent)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr15::LReversed)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Setclr15::HReversed)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&self) -> Setclr0R {
        Setclr0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&self) -> Setclr1R {
        Setclr1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&self) -> Setclr2R {
        Setclr2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&self) -> Setclr3R {
        Setclr3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr4(&self) -> Setclr4R {
        Setclr4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr5(&self) -> Setclr5R {
        Setclr5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr6(&self) -> Setclr6R {
        Setclr6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr7(&self) -> Setclr7R {
        Setclr7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr8(&self) -> Setclr8R {
        Setclr8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr9(&self) -> Setclr9R {
        Setclr9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr10(&self) -> Setclr10R {
        Setclr10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr11(&self) -> Setclr11R {
        Setclr11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr12(&self) -> Setclr12R {
        Setclr12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr13(&self) -> Setclr13R {
        Setclr13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr14(&self) -> Setclr14R {
        Setclr14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr15(&self) -> Setclr15R {
        Setclr15R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTDIRCTRL")
            .field("setclr0", &self.setclr0())
            .field("setclr1", &self.setclr1())
            .field("setclr2", &self.setclr2())
            .field("setclr3", &self.setclr3())
            .field("setclr4", &self.setclr4())
            .field("setclr5", &self.setclr5())
            .field("setclr6", &self.setclr6())
            .field("setclr7", &self.setclr7())
            .field("setclr8", &self.setclr8())
            .field("setclr9", &self.setclr9())
            .field("setclr10", &self.setclr10())
            .field("setclr11", &self.setclr11())
            .field("setclr12", &self.setclr12())
            .field("setclr13", &self.setclr13())
            .field("setclr14", &self.setclr14())
            .field("setclr15", &self.setclr15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr0(&mut self) -> Setclr0W<OutputdirctrlSpec> {
        Setclr0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr1(&mut self) -> Setclr1W<OutputdirctrlSpec> {
        Setclr1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr2(&mut self) -> Setclr2W<OutputdirctrlSpec> {
        Setclr2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr3(&mut self) -> Setclr3W<OutputdirctrlSpec> {
        Setclr3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr4(&mut self) -> Setclr4W<OutputdirctrlSpec> {
        Setclr4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr5(&mut self) -> Setclr5W<OutputdirctrlSpec> {
        Setclr5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr6(&mut self) -> Setclr6W<OutputdirctrlSpec> {
        Setclr6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr7(&mut self) -> Setclr7W<OutputdirctrlSpec> {
        Setclr7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr8(&mut self) -> Setclr8W<OutputdirctrlSpec> {
        Setclr8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr9(&mut self) -> Setclr9W<OutputdirctrlSpec> {
        Setclr9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr10(&mut self) -> Setclr10W<OutputdirctrlSpec> {
        Setclr10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr11(&mut self) -> Setclr11W<OutputdirctrlSpec> {
        Setclr11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr12(&mut self) -> Setclr12W<OutputdirctrlSpec> {
        Setclr12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr13(&mut self) -> Setclr13W<OutputdirctrlSpec> {
        Setclr13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr14(&mut self) -> Setclr14W<OutputdirctrlSpec> {
        Setclr14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr15(&mut self) -> Setclr15W<OutputdirctrlSpec> {
        Setclr15W::new(self, 30)
    }
}
#[doc = "SCT output counter direction control register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputdirctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputdirctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputdirctrlSpec;
impl crate::RegisterSpec for OutputdirctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputdirctrl::R`](R) reader structure"]
impl crate::Readable for OutputdirctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`outputdirctrl::W`](W) writer structure"]
impl crate::Writable for OutputdirctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTPUTDIRCTRL to value 0"]
impl crate::Resettable for OutputdirctrlSpec {
    const RESET_VALUE: u32 = 0;
}
