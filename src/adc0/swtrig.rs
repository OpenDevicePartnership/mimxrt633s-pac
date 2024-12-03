#[doc = "Register `SWTRIG` reader"]
pub type R = crate::R<SwtrigSpec>;
#[doc = "Register `SWTRIG` writer"]
pub type W = crate::W<SwtrigSpec>;
#[doc = "Software trigger 0 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt0 {
    #[doc = "0: No trigger 0 event generated."]
    Swt0_0 = 0,
    #[doc = "1: Trigger 0 event generated."]
    Swt0_1 = 1,
}
impl From<Swt0> for bool {
    #[inline(always)]
    fn from(variant: Swt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT0` reader - Software trigger 0 event"]
pub type Swt0R = crate::BitReader<Swt0>;
impl Swt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt0 {
        match self.bits {
            false => Swt0::Swt0_0,
            true => Swt0::Swt0_1,
        }
    }
    #[doc = "No trigger 0 event generated."]
    #[inline(always)]
    pub fn is_swt0_0(&self) -> bool {
        *self == Swt0::Swt0_0
    }
    #[doc = "Trigger 0 event generated."]
    #[inline(always)]
    pub fn is_swt0_1(&self) -> bool {
        *self == Swt0::Swt0_1
    }
}
#[doc = "Field `SWT0` writer - Software trigger 0 event"]
pub type Swt0W<'a, REG> = crate::BitWriter<'a, REG, Swt0>;
impl<'a, REG> Swt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 0 event generated."]
    #[inline(always)]
    pub fn swt0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt0::Swt0_0)
    }
    #[doc = "Trigger 0 event generated."]
    #[inline(always)]
    pub fn swt0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt0::Swt0_1)
    }
}
#[doc = "Software trigger 1 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt1 {
    #[doc = "0: No trigger 1 event generated."]
    Swt1_0 = 0,
    #[doc = "1: Trigger 1 event generated."]
    Swt1_1 = 1,
}
impl From<Swt1> for bool {
    #[inline(always)]
    fn from(variant: Swt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT1` reader - Software trigger 1 event"]
pub type Swt1R = crate::BitReader<Swt1>;
impl Swt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt1 {
        match self.bits {
            false => Swt1::Swt1_0,
            true => Swt1::Swt1_1,
        }
    }
    #[doc = "No trigger 1 event generated."]
    #[inline(always)]
    pub fn is_swt1_0(&self) -> bool {
        *self == Swt1::Swt1_0
    }
    #[doc = "Trigger 1 event generated."]
    #[inline(always)]
    pub fn is_swt1_1(&self) -> bool {
        *self == Swt1::Swt1_1
    }
}
#[doc = "Field `SWT1` writer - Software trigger 1 event"]
pub type Swt1W<'a, REG> = crate::BitWriter<'a, REG, Swt1>;
impl<'a, REG> Swt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 1 event generated."]
    #[inline(always)]
    pub fn swt1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt1::Swt1_0)
    }
    #[doc = "Trigger 1 event generated."]
    #[inline(always)]
    pub fn swt1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt1::Swt1_1)
    }
}
#[doc = "Software trigger 2 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt2 {
    #[doc = "0: No trigger 2 event generated."]
    Swt2_0 = 0,
    #[doc = "1: Trigger 2 event generated."]
    Swt2_1 = 1,
}
impl From<Swt2> for bool {
    #[inline(always)]
    fn from(variant: Swt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT2` reader - Software trigger 2 event"]
pub type Swt2R = crate::BitReader<Swt2>;
impl Swt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt2 {
        match self.bits {
            false => Swt2::Swt2_0,
            true => Swt2::Swt2_1,
        }
    }
    #[doc = "No trigger 2 event generated."]
    #[inline(always)]
    pub fn is_swt2_0(&self) -> bool {
        *self == Swt2::Swt2_0
    }
    #[doc = "Trigger 2 event generated."]
    #[inline(always)]
    pub fn is_swt2_1(&self) -> bool {
        *self == Swt2::Swt2_1
    }
}
#[doc = "Field `SWT2` writer - Software trigger 2 event"]
pub type Swt2W<'a, REG> = crate::BitWriter<'a, REG, Swt2>;
impl<'a, REG> Swt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 2 event generated."]
    #[inline(always)]
    pub fn swt2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt2::Swt2_0)
    }
    #[doc = "Trigger 2 event generated."]
    #[inline(always)]
    pub fn swt2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt2::Swt2_1)
    }
}
#[doc = "Software trigger 3 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt3 {
    #[doc = "0: No trigger 3 event generated."]
    Swt3_0 = 0,
    #[doc = "1: Trigger 3 event generated."]
    Swt3_1 = 1,
}
impl From<Swt3> for bool {
    #[inline(always)]
    fn from(variant: Swt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT3` reader - Software trigger 3 event"]
pub type Swt3R = crate::BitReader<Swt3>;
impl Swt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt3 {
        match self.bits {
            false => Swt3::Swt3_0,
            true => Swt3::Swt3_1,
        }
    }
    #[doc = "No trigger 3 event generated."]
    #[inline(always)]
    pub fn is_swt3_0(&self) -> bool {
        *self == Swt3::Swt3_0
    }
    #[doc = "Trigger 3 event generated."]
    #[inline(always)]
    pub fn is_swt3_1(&self) -> bool {
        *self == Swt3::Swt3_1
    }
}
#[doc = "Field `SWT3` writer - Software trigger 3 event"]
pub type Swt3W<'a, REG> = crate::BitWriter<'a, REG, Swt3>;
impl<'a, REG> Swt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 3 event generated."]
    #[inline(always)]
    pub fn swt3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt3::Swt3_0)
    }
    #[doc = "Trigger 3 event generated."]
    #[inline(always)]
    pub fn swt3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt3::Swt3_1)
    }
}
#[doc = "Software trigger 4 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt4 {
    #[doc = "0: No trigger 4 event generated."]
    Swt4_0 = 0,
    #[doc = "1: Trigger 4 event generated."]
    Swt4_1 = 1,
}
impl From<Swt4> for bool {
    #[inline(always)]
    fn from(variant: Swt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT4` reader - Software trigger 4 event"]
pub type Swt4R = crate::BitReader<Swt4>;
impl Swt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt4 {
        match self.bits {
            false => Swt4::Swt4_0,
            true => Swt4::Swt4_1,
        }
    }
    #[doc = "No trigger 4 event generated."]
    #[inline(always)]
    pub fn is_swt4_0(&self) -> bool {
        *self == Swt4::Swt4_0
    }
    #[doc = "Trigger 4 event generated."]
    #[inline(always)]
    pub fn is_swt4_1(&self) -> bool {
        *self == Swt4::Swt4_1
    }
}
#[doc = "Field `SWT4` writer - Software trigger 4 event"]
pub type Swt4W<'a, REG> = crate::BitWriter<'a, REG, Swt4>;
impl<'a, REG> Swt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 4 event generated."]
    #[inline(always)]
    pub fn swt4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt4::Swt4_0)
    }
    #[doc = "Trigger 4 event generated."]
    #[inline(always)]
    pub fn swt4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt4::Swt4_1)
    }
}
#[doc = "Software trigger 5 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt5 {
    #[doc = "0: No trigger 5 event generated."]
    Swt5_0 = 0,
    #[doc = "1: Trigger 5 event generated."]
    Swt5_1 = 1,
}
impl From<Swt5> for bool {
    #[inline(always)]
    fn from(variant: Swt5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT5` reader - Software trigger 5 event"]
pub type Swt5R = crate::BitReader<Swt5>;
impl Swt5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt5 {
        match self.bits {
            false => Swt5::Swt5_0,
            true => Swt5::Swt5_1,
        }
    }
    #[doc = "No trigger 5 event generated."]
    #[inline(always)]
    pub fn is_swt5_0(&self) -> bool {
        *self == Swt5::Swt5_0
    }
    #[doc = "Trigger 5 event generated."]
    #[inline(always)]
    pub fn is_swt5_1(&self) -> bool {
        *self == Swt5::Swt5_1
    }
}
#[doc = "Field `SWT5` writer - Software trigger 5 event"]
pub type Swt5W<'a, REG> = crate::BitWriter<'a, REG, Swt5>;
impl<'a, REG> Swt5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 5 event generated."]
    #[inline(always)]
    pub fn swt5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt5::Swt5_0)
    }
    #[doc = "Trigger 5 event generated."]
    #[inline(always)]
    pub fn swt5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt5::Swt5_1)
    }
}
#[doc = "Software trigger 6 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt6 {
    #[doc = "0: No trigger 6 event generated."]
    Swt6_0 = 0,
    #[doc = "1: Trigger 6 event generated."]
    Swt6_1 = 1,
}
impl From<Swt6> for bool {
    #[inline(always)]
    fn from(variant: Swt6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT6` reader - Software trigger 6 event"]
pub type Swt6R = crate::BitReader<Swt6>;
impl Swt6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt6 {
        match self.bits {
            false => Swt6::Swt6_0,
            true => Swt6::Swt6_1,
        }
    }
    #[doc = "No trigger 6 event generated."]
    #[inline(always)]
    pub fn is_swt6_0(&self) -> bool {
        *self == Swt6::Swt6_0
    }
    #[doc = "Trigger 6 event generated."]
    #[inline(always)]
    pub fn is_swt6_1(&self) -> bool {
        *self == Swt6::Swt6_1
    }
}
#[doc = "Field `SWT6` writer - Software trigger 6 event"]
pub type Swt6W<'a, REG> = crate::BitWriter<'a, REG, Swt6>;
impl<'a, REG> Swt6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 6 event generated."]
    #[inline(always)]
    pub fn swt6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt6::Swt6_0)
    }
    #[doc = "Trigger 6 event generated."]
    #[inline(always)]
    pub fn swt6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt6::Swt6_1)
    }
}
#[doc = "Software trigger 7 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt7 {
    #[doc = "0: No trigger 7 event generated."]
    Swt7_0 = 0,
    #[doc = "1: Trigger 7 event generated."]
    Swt7_1 = 1,
}
impl From<Swt7> for bool {
    #[inline(always)]
    fn from(variant: Swt7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT7` reader - Software trigger 7 event"]
pub type Swt7R = crate::BitReader<Swt7>;
impl Swt7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt7 {
        match self.bits {
            false => Swt7::Swt7_0,
            true => Swt7::Swt7_1,
        }
    }
    #[doc = "No trigger 7 event generated."]
    #[inline(always)]
    pub fn is_swt7_0(&self) -> bool {
        *self == Swt7::Swt7_0
    }
    #[doc = "Trigger 7 event generated."]
    #[inline(always)]
    pub fn is_swt7_1(&self) -> bool {
        *self == Swt7::Swt7_1
    }
}
#[doc = "Field `SWT7` writer - Software trigger 7 event"]
pub type Swt7W<'a, REG> = crate::BitWriter<'a, REG, Swt7>;
impl<'a, REG> Swt7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 7 event generated."]
    #[inline(always)]
    pub fn swt7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt7::Swt7_0)
    }
    #[doc = "Trigger 7 event generated."]
    #[inline(always)]
    pub fn swt7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt7::Swt7_1)
    }
}
#[doc = "Software trigger 8 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt8 {
    #[doc = "0: No trigger 8 event generated."]
    Swt8_0 = 0,
    #[doc = "1: Trigger 8 event generated."]
    Swt8_1 = 1,
}
impl From<Swt8> for bool {
    #[inline(always)]
    fn from(variant: Swt8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT8` reader - Software trigger 8 event"]
pub type Swt8R = crate::BitReader<Swt8>;
impl Swt8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt8 {
        match self.bits {
            false => Swt8::Swt8_0,
            true => Swt8::Swt8_1,
        }
    }
    #[doc = "No trigger 8 event generated."]
    #[inline(always)]
    pub fn is_swt8_0(&self) -> bool {
        *self == Swt8::Swt8_0
    }
    #[doc = "Trigger 8 event generated."]
    #[inline(always)]
    pub fn is_swt8_1(&self) -> bool {
        *self == Swt8::Swt8_1
    }
}
#[doc = "Field `SWT8` writer - Software trigger 8 event"]
pub type Swt8W<'a, REG> = crate::BitWriter<'a, REG, Swt8>;
impl<'a, REG> Swt8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 8 event generated."]
    #[inline(always)]
    pub fn swt8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt8::Swt8_0)
    }
    #[doc = "Trigger 8 event generated."]
    #[inline(always)]
    pub fn swt8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt8::Swt8_1)
    }
}
#[doc = "Software trigger 9 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt9 {
    #[doc = "0: No trigger 9 event generated."]
    Swt9_0 = 0,
    #[doc = "1: Trigger 9 event generated."]
    Swt9_1 = 1,
}
impl From<Swt9> for bool {
    #[inline(always)]
    fn from(variant: Swt9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT9` reader - Software trigger 9 event"]
pub type Swt9R = crate::BitReader<Swt9>;
impl Swt9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt9 {
        match self.bits {
            false => Swt9::Swt9_0,
            true => Swt9::Swt9_1,
        }
    }
    #[doc = "No trigger 9 event generated."]
    #[inline(always)]
    pub fn is_swt9_0(&self) -> bool {
        *self == Swt9::Swt9_0
    }
    #[doc = "Trigger 9 event generated."]
    #[inline(always)]
    pub fn is_swt9_1(&self) -> bool {
        *self == Swt9::Swt9_1
    }
}
#[doc = "Field `SWT9` writer - Software trigger 9 event"]
pub type Swt9W<'a, REG> = crate::BitWriter<'a, REG, Swt9>;
impl<'a, REG> Swt9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 9 event generated."]
    #[inline(always)]
    pub fn swt9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt9::Swt9_0)
    }
    #[doc = "Trigger 9 event generated."]
    #[inline(always)]
    pub fn swt9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt9::Swt9_1)
    }
}
#[doc = "Software trigger 10 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt10 {
    #[doc = "0: No trigger 10 event generated."]
    Swt10_0 = 0,
    #[doc = "1: Trigger 10 event generated."]
    Swt10_1 = 1,
}
impl From<Swt10> for bool {
    #[inline(always)]
    fn from(variant: Swt10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT10` reader - Software trigger 10 event"]
pub type Swt10R = crate::BitReader<Swt10>;
impl Swt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt10 {
        match self.bits {
            false => Swt10::Swt10_0,
            true => Swt10::Swt10_1,
        }
    }
    #[doc = "No trigger 10 event generated."]
    #[inline(always)]
    pub fn is_swt10_0(&self) -> bool {
        *self == Swt10::Swt10_0
    }
    #[doc = "Trigger 10 event generated."]
    #[inline(always)]
    pub fn is_swt10_1(&self) -> bool {
        *self == Swt10::Swt10_1
    }
}
#[doc = "Field `SWT10` writer - Software trigger 10 event"]
pub type Swt10W<'a, REG> = crate::BitWriter<'a, REG, Swt10>;
impl<'a, REG> Swt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 10 event generated."]
    #[inline(always)]
    pub fn swt10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt10::Swt10_0)
    }
    #[doc = "Trigger 10 event generated."]
    #[inline(always)]
    pub fn swt10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt10::Swt10_1)
    }
}
#[doc = "Software trigger 11 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt11 {
    #[doc = "0: No trigger 11 event generated."]
    Swt11_0 = 0,
    #[doc = "1: Trigger 11 event generated."]
    Swt11_1 = 1,
}
impl From<Swt11> for bool {
    #[inline(always)]
    fn from(variant: Swt11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT11` reader - Software trigger 11 event"]
pub type Swt11R = crate::BitReader<Swt11>;
impl Swt11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt11 {
        match self.bits {
            false => Swt11::Swt11_0,
            true => Swt11::Swt11_1,
        }
    }
    #[doc = "No trigger 11 event generated."]
    #[inline(always)]
    pub fn is_swt11_0(&self) -> bool {
        *self == Swt11::Swt11_0
    }
    #[doc = "Trigger 11 event generated."]
    #[inline(always)]
    pub fn is_swt11_1(&self) -> bool {
        *self == Swt11::Swt11_1
    }
}
#[doc = "Field `SWT11` writer - Software trigger 11 event"]
pub type Swt11W<'a, REG> = crate::BitWriter<'a, REG, Swt11>;
impl<'a, REG> Swt11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 11 event generated."]
    #[inline(always)]
    pub fn swt11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt11::Swt11_0)
    }
    #[doc = "Trigger 11 event generated."]
    #[inline(always)]
    pub fn swt11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt11::Swt11_1)
    }
}
#[doc = "Software trigger 12 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt12 {
    #[doc = "0: No trigger 12 event generated."]
    Swt12_0 = 0,
    #[doc = "1: Trigger 12 event generated."]
    Swt12_1 = 1,
}
impl From<Swt12> for bool {
    #[inline(always)]
    fn from(variant: Swt12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT12` reader - Software trigger 12 event"]
pub type Swt12R = crate::BitReader<Swt12>;
impl Swt12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt12 {
        match self.bits {
            false => Swt12::Swt12_0,
            true => Swt12::Swt12_1,
        }
    }
    #[doc = "No trigger 12 event generated."]
    #[inline(always)]
    pub fn is_swt12_0(&self) -> bool {
        *self == Swt12::Swt12_0
    }
    #[doc = "Trigger 12 event generated."]
    #[inline(always)]
    pub fn is_swt12_1(&self) -> bool {
        *self == Swt12::Swt12_1
    }
}
#[doc = "Field `SWT12` writer - Software trigger 12 event"]
pub type Swt12W<'a, REG> = crate::BitWriter<'a, REG, Swt12>;
impl<'a, REG> Swt12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 12 event generated."]
    #[inline(always)]
    pub fn swt12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt12::Swt12_0)
    }
    #[doc = "Trigger 12 event generated."]
    #[inline(always)]
    pub fn swt12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt12::Swt12_1)
    }
}
#[doc = "Software trigger 13 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt13 {
    #[doc = "0: No trigger 13 event generated."]
    Swt13_0 = 0,
    #[doc = "1: Trigger 13 event generated."]
    Swt13_1 = 1,
}
impl From<Swt13> for bool {
    #[inline(always)]
    fn from(variant: Swt13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT13` reader - Software trigger 13 event"]
pub type Swt13R = crate::BitReader<Swt13>;
impl Swt13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt13 {
        match self.bits {
            false => Swt13::Swt13_0,
            true => Swt13::Swt13_1,
        }
    }
    #[doc = "No trigger 13 event generated."]
    #[inline(always)]
    pub fn is_swt13_0(&self) -> bool {
        *self == Swt13::Swt13_0
    }
    #[doc = "Trigger 13 event generated."]
    #[inline(always)]
    pub fn is_swt13_1(&self) -> bool {
        *self == Swt13::Swt13_1
    }
}
#[doc = "Field `SWT13` writer - Software trigger 13 event"]
pub type Swt13W<'a, REG> = crate::BitWriter<'a, REG, Swt13>;
impl<'a, REG> Swt13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 13 event generated."]
    #[inline(always)]
    pub fn swt13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt13::Swt13_0)
    }
    #[doc = "Trigger 13 event generated."]
    #[inline(always)]
    pub fn swt13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt13::Swt13_1)
    }
}
#[doc = "Software trigger 14 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt14 {
    #[doc = "0: No trigger 14 event generated."]
    Swt14_0 = 0,
    #[doc = "1: Trigger 14 event generated."]
    Swt14_1 = 1,
}
impl From<Swt14> for bool {
    #[inline(always)]
    fn from(variant: Swt14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT14` reader - Software trigger 14 event"]
pub type Swt14R = crate::BitReader<Swt14>;
impl Swt14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt14 {
        match self.bits {
            false => Swt14::Swt14_0,
            true => Swt14::Swt14_1,
        }
    }
    #[doc = "No trigger 14 event generated."]
    #[inline(always)]
    pub fn is_swt14_0(&self) -> bool {
        *self == Swt14::Swt14_0
    }
    #[doc = "Trigger 14 event generated."]
    #[inline(always)]
    pub fn is_swt14_1(&self) -> bool {
        *self == Swt14::Swt14_1
    }
}
#[doc = "Field `SWT14` writer - Software trigger 14 event"]
pub type Swt14W<'a, REG> = crate::BitWriter<'a, REG, Swt14>;
impl<'a, REG> Swt14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 14 event generated."]
    #[inline(always)]
    pub fn swt14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt14::Swt14_0)
    }
    #[doc = "Trigger 14 event generated."]
    #[inline(always)]
    pub fn swt14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt14::Swt14_1)
    }
}
#[doc = "Software trigger 15 event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swt15 {
    #[doc = "0: No trigger 15 event generated."]
    Swt15_0 = 0,
    #[doc = "1: Trigger 15 event generated."]
    Swt15_1 = 1,
}
impl From<Swt15> for bool {
    #[inline(always)]
    fn from(variant: Swt15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT15` reader - Software trigger 15 event"]
pub type Swt15R = crate::BitReader<Swt15>;
impl Swt15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swt15 {
        match self.bits {
            false => Swt15::Swt15_0,
            true => Swt15::Swt15_1,
        }
    }
    #[doc = "No trigger 15 event generated."]
    #[inline(always)]
    pub fn is_swt15_0(&self) -> bool {
        *self == Swt15::Swt15_0
    }
    #[doc = "Trigger 15 event generated."]
    #[inline(always)]
    pub fn is_swt15_1(&self) -> bool {
        *self == Swt15::Swt15_1
    }
}
#[doc = "Field `SWT15` writer - Software trigger 15 event"]
pub type Swt15W<'a, REG> = crate::BitWriter<'a, REG, Swt15>;
impl<'a, REG> Swt15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger 15 event generated."]
    #[inline(always)]
    pub fn swt15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Swt15::Swt15_0)
    }
    #[doc = "Trigger 15 event generated."]
    #[inline(always)]
    pub fn swt15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Swt15::Swt15_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline(always)]
    pub fn swt0(&self) -> Swt0R {
        Swt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline(always)]
    pub fn swt1(&self) -> Swt1R {
        Swt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline(always)]
    pub fn swt2(&self) -> Swt2R {
        Swt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline(always)]
    pub fn swt3(&self) -> Swt3R {
        Swt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline(always)]
    pub fn swt4(&self) -> Swt4R {
        Swt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline(always)]
    pub fn swt5(&self) -> Swt5R {
        Swt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline(always)]
    pub fn swt6(&self) -> Swt6R {
        Swt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline(always)]
    pub fn swt7(&self) -> Swt7R {
        Swt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline(always)]
    pub fn swt8(&self) -> Swt8R {
        Swt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline(always)]
    pub fn swt9(&self) -> Swt9R {
        Swt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline(always)]
    pub fn swt10(&self) -> Swt10R {
        Swt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline(always)]
    pub fn swt11(&self) -> Swt11R {
        Swt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline(always)]
    pub fn swt12(&self) -> Swt12R {
        Swt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline(always)]
    pub fn swt13(&self) -> Swt13R {
        Swt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline(always)]
    pub fn swt14(&self) -> Swt14R {
        Swt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline(always)]
    pub fn swt15(&self) -> Swt15R {
        Swt15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWTRIG")
            .field("swt0", &self.swt0())
            .field("swt1", &self.swt1())
            .field("swt2", &self.swt2())
            .field("swt3", &self.swt3())
            .field("swt4", &self.swt4())
            .field("swt5", &self.swt5())
            .field("swt6", &self.swt6())
            .field("swt7", &self.swt7())
            .field("swt8", &self.swt8())
            .field("swt9", &self.swt9())
            .field("swt10", &self.swt10())
            .field("swt11", &self.swt11())
            .field("swt12", &self.swt12())
            .field("swt13", &self.swt13())
            .field("swt14", &self.swt14())
            .field("swt15", &self.swt15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline(always)]
    pub fn swt0(&mut self) -> Swt0W<SwtrigSpec> {
        Swt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline(always)]
    pub fn swt1(&mut self) -> Swt1W<SwtrigSpec> {
        Swt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline(always)]
    pub fn swt2(&mut self) -> Swt2W<SwtrigSpec> {
        Swt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline(always)]
    pub fn swt3(&mut self) -> Swt3W<SwtrigSpec> {
        Swt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline(always)]
    pub fn swt4(&mut self) -> Swt4W<SwtrigSpec> {
        Swt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline(always)]
    pub fn swt5(&mut self) -> Swt5W<SwtrigSpec> {
        Swt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline(always)]
    pub fn swt6(&mut self) -> Swt6W<SwtrigSpec> {
        Swt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline(always)]
    pub fn swt7(&mut self) -> Swt7W<SwtrigSpec> {
        Swt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline(always)]
    pub fn swt8(&mut self) -> Swt8W<SwtrigSpec> {
        Swt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline(always)]
    pub fn swt9(&mut self) -> Swt9W<SwtrigSpec> {
        Swt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline(always)]
    pub fn swt10(&mut self) -> Swt10W<SwtrigSpec> {
        Swt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline(always)]
    pub fn swt11(&mut self) -> Swt11W<SwtrigSpec> {
        Swt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline(always)]
    pub fn swt12(&mut self) -> Swt12W<SwtrigSpec> {
        Swt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline(always)]
    pub fn swt13(&mut self) -> Swt13W<SwtrigSpec> {
        Swt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline(always)]
    pub fn swt14(&mut self) -> Swt14W<SwtrigSpec> {
        Swt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline(always)]
    pub fn swt15(&mut self) -> Swt15W<SwtrigSpec> {
        Swt15W::new(self, 15)
    }
}
#[doc = "Software Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigSpec;
impl crate::RegisterSpec for SwtrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrig::R`](R) reader structure"]
impl crate::Readable for SwtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrig::W`](W) writer structure"]
impl crate::Writable for SwtrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SwtrigSpec {
    const RESET_VALUE: u32 = 0;
}
