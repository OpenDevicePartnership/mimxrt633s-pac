#[doc = "Register `SHAREDCTRLSET%s` reader"]
pub type R = crate::R<SharedctrlsetSpec>;
#[doc = "Register `SHAREDCTRLSET%s` writer"]
pub type W = crate::W<SharedctrlsetSpec>;
#[doc = "Shared SCK Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sharedscksel {
    #[doc = "0: FLEXCOMM0"]
    Flexcomm0 = 0,
    #[doc = "1: FLEXCOMM1"]
    Flexcomm1 = 1,
    #[doc = "2: FLEXCOMM2"]
    Flexcomm2 = 2,
    #[doc = "3: FLEXCOMM3"]
    Flexcomm3 = 3,
    #[doc = "4: FLEXCOMM4"]
    Flexcomm4 = 4,
    #[doc = "5: FLEXCOMM5"]
    Flexcomm5 = 5,
    #[doc = "6: FLEXCOMM6"]
    Flexcomm6 = 6,
    #[doc = "7: FLEXCOMM7"]
    Flexcomm7 = 7,
}
impl From<Sharedscksel> for u8 {
    #[inline(always)]
    fn from(variant: Sharedscksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sharedscksel {
    type Ux = u8;
}
impl crate::IsEnum for Sharedscksel {}
#[doc = "Field `SHAREDSCKSEL` reader - Shared SCK Select. . ."]
pub type SharedsckselR = crate::FieldReader<Sharedscksel>;
impl SharedsckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sharedscksel {
        match self.bits {
            0 => Sharedscksel::Flexcomm0,
            1 => Sharedscksel::Flexcomm1,
            2 => Sharedscksel::Flexcomm2,
            3 => Sharedscksel::Flexcomm3,
            4 => Sharedscksel::Flexcomm4,
            5 => Sharedscksel::Flexcomm5,
            6 => Sharedscksel::Flexcomm6,
            7 => Sharedscksel::Flexcomm7,
            _ => unreachable!(),
        }
    }
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == Sharedscksel::Flexcomm0
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == Sharedscksel::Flexcomm1
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == Sharedscksel::Flexcomm2
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == Sharedscksel::Flexcomm3
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == Sharedscksel::Flexcomm4
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == Sharedscksel::Flexcomm5
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == Sharedscksel::Flexcomm6
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == Sharedscksel::Flexcomm7
    }
}
#[doc = "Field `SHAREDSCKSEL` writer - Shared SCK Select. . ."]
pub type SharedsckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sharedscksel, crate::Safe>;
impl<'a, REG> SharedsckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm7)
    }
}
#[doc = "Shared WS Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sharedwssel {
    #[doc = "0: FLEXCOMM0"]
    Flexcomm0 = 0,
    #[doc = "1: FLEXCOMM1"]
    Flexcomm1 = 1,
    #[doc = "2: FLEXCOMM2"]
    Flexcomm2 = 2,
    #[doc = "3: FLEXCOMM3"]
    Flexcomm3 = 3,
    #[doc = "4: FLEXCOMM4"]
    Flexcomm4 = 4,
    #[doc = "5: FLEXCOMM5"]
    Flexcomm5 = 5,
    #[doc = "6: FLEXCOMM6"]
    Flexcomm6 = 6,
    #[doc = "7: FLEXCOMM7"]
    Flexcomm7 = 7,
}
impl From<Sharedwssel> for u8 {
    #[inline(always)]
    fn from(variant: Sharedwssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sharedwssel {
    type Ux = u8;
}
impl crate::IsEnum for Sharedwssel {}
#[doc = "Field `SHAREDWSSEL` reader - Shared WS Select. . ."]
pub type SharedwsselR = crate::FieldReader<Sharedwssel>;
impl SharedwsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sharedwssel {
        match self.bits {
            0 => Sharedwssel::Flexcomm0,
            1 => Sharedwssel::Flexcomm1,
            2 => Sharedwssel::Flexcomm2,
            3 => Sharedwssel::Flexcomm3,
            4 => Sharedwssel::Flexcomm4,
            5 => Sharedwssel::Flexcomm5,
            6 => Sharedwssel::Flexcomm6,
            7 => Sharedwssel::Flexcomm7,
            _ => unreachable!(),
        }
    }
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == Sharedwssel::Flexcomm0
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == Sharedwssel::Flexcomm1
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == Sharedwssel::Flexcomm2
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == Sharedwssel::Flexcomm3
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == Sharedwssel::Flexcomm4
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == Sharedwssel::Flexcomm5
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == Sharedwssel::Flexcomm6
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == Sharedwssel::Flexcomm7
    }
}
#[doc = "Field `SHAREDWSSEL` writer - Shared WS Select. . ."]
pub type SharedwsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sharedwssel, crate::Safe>;
impl<'a, REG> SharedwsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm7)
    }
}
#[doc = "Shared DATA Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shareddatasel {
    #[doc = "0: FLEXCOMM0"]
    Flexcomm0 = 0,
    #[doc = "1: FLEXCOMM1"]
    Flexcomm1 = 1,
    #[doc = "2: FLEXCOMM2"]
    Flexcomm2 = 2,
    #[doc = "3: FLEXCOMM3"]
    Flexcomm3 = 3,
    #[doc = "4: FLEXCOMM4"]
    Flexcomm4 = 4,
    #[doc = "5: FLEXCOMM5"]
    Flexcomm5 = 5,
    #[doc = "6: FLEXCOMM6"]
    Flexcomm6 = 6,
    #[doc = "7: FLEXCOMM7"]
    Flexcomm7 = 7,
}
impl From<Shareddatasel> for u8 {
    #[inline(always)]
    fn from(variant: Shareddatasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Shareddatasel {
    type Ux = u8;
}
impl crate::IsEnum for Shareddatasel {}
#[doc = "Field `SHAREDDATASEL` reader - Shared DATA Select. . ."]
pub type ShareddataselR = crate::FieldReader<Shareddatasel>;
impl ShareddataselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shareddatasel {
        match self.bits {
            0 => Shareddatasel::Flexcomm0,
            1 => Shareddatasel::Flexcomm1,
            2 => Shareddatasel::Flexcomm2,
            3 => Shareddatasel::Flexcomm3,
            4 => Shareddatasel::Flexcomm4,
            5 => Shareddatasel::Flexcomm5,
            6 => Shareddatasel::Flexcomm6,
            7 => Shareddatasel::Flexcomm7,
            _ => unreachable!(),
        }
    }
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == Shareddatasel::Flexcomm0
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == Shareddatasel::Flexcomm1
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == Shareddatasel::Flexcomm2
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == Shareddatasel::Flexcomm3
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == Shareddatasel::Flexcomm4
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == Shareddatasel::Flexcomm5
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == Shareddatasel::Flexcomm6
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == Shareddatasel::Flexcomm7
    }
}
#[doc = "Field `SHAREDDATASEL` writer - Shared DATA Select. . ."]
pub type ShareddataselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Shareddatasel, crate::Safe>;
impl<'a, REG> ShareddataselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm7)
    }
}
#[doc = "FLEXCOMM0 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc0dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc0dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0DATAOUTEN` reader - FLEXCOMM0 DATAOUT OUTPUT ENABLE"]
pub type Fc0dataoutenR = crate::BitReader<Fc0dataouten>;
impl Fc0dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc0dataouten {
        match self.bits {
            false => Fc0dataouten::Input,
            true => Fc0dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc0dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc0dataouten::Output
    }
}
#[doc = "Field `FC0DATAOUTEN` writer - FLEXCOMM0 DATAOUT OUTPUT ENABLE"]
pub type Fc0dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc0dataouten>;
impl<'a, REG> Fc0dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0dataouten::Output)
    }
}
#[doc = "FLEXCOMM1 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc1dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc1dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1DATAOUTEN` reader - FLEXCOMM1 DATAOUT OUTPUT ENABLE"]
pub type Fc1dataoutenR = crate::BitReader<Fc1dataouten>;
impl Fc1dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc1dataouten {
        match self.bits {
            false => Fc1dataouten::Input,
            true => Fc1dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc1dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc1dataouten::Output
    }
}
#[doc = "Field `FC1DATAOUTEN` writer - FLEXCOMM1 DATAOUT OUTPUT ENABLE"]
pub type Fc1dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc1dataouten>;
impl<'a, REG> Fc1dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1dataouten::Output)
    }
}
#[doc = "FLEXCOMM2 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F20dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<F20dataouten> for bool {
    #[inline(always)]
    fn from(variant: F20dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F20DATAOUTEN` reader - FLEXCOMM2 DATAOUT OUTPUT ENABLE"]
pub type F20dataoutenR = crate::BitReader<F20dataouten>;
impl F20dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F20dataouten {
        match self.bits {
            false => F20dataouten::Input,
            true => F20dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == F20dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == F20dataouten::Output
    }
}
#[doc = "Field `F20DATAOUTEN` writer - FLEXCOMM2 DATAOUT OUTPUT ENABLE"]
pub type F20dataoutenW<'a, REG> = crate::BitWriter<'a, REG, F20dataouten>;
impl<'a, REG> F20dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(F20dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(F20dataouten::Output)
    }
}
#[doc = "FLEXCOMM3 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc3dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc3dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc3dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3DATAOUTEN` reader - FLEXCOMM3 DATAOUT OUTPUT ENABLE"]
pub type Fc3dataoutenR = crate::BitReader<Fc3dataouten>;
impl Fc3dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc3dataouten {
        match self.bits {
            false => Fc3dataouten::Input,
            true => Fc3dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc3dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc3dataouten::Output
    }
}
#[doc = "Field `FC3DATAOUTEN` writer - FLEXCOMM3 DATAOUT OUTPUT ENABLE"]
pub type Fc3dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc3dataouten>;
impl<'a, REG> Fc3dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3dataouten::Output)
    }
}
#[doc = "FLEXCOMM4 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc4dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc4dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4DATAOUTEN` reader - FLEXCOMM4 DATAOUT OUTPUT ENABLE"]
pub type Fc4dataoutenR = crate::BitReader<Fc4dataouten>;
impl Fc4dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc4dataouten {
        match self.bits {
            false => Fc4dataouten::Input,
            true => Fc4dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc4dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc4dataouten::Output
    }
}
#[doc = "Field `FC4DATAOUTEN` writer - FLEXCOMM4 DATAOUT OUTPUT ENABLE"]
pub type Fc4dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc4dataouten>;
impl<'a, REG> Fc4dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4dataouten::Output)
    }
}
#[doc = "FLEXCOMM5 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc5dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc5dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5DATAOUTEN` reader - FLEXCOMM5 DATAOUT OUTPUT ENABLE"]
pub type Fc5dataoutenR = crate::BitReader<Fc5dataouten>;
impl Fc5dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc5dataouten {
        match self.bits {
            false => Fc5dataouten::Input,
            true => Fc5dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc5dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc5dataouten::Output
    }
}
#[doc = "Field `FC5DATAOUTEN` writer - FLEXCOMM5 DATAOUT OUTPUT ENABLE"]
pub type Fc5dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc5dataouten>;
impl<'a, REG> Fc5dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5dataouten::Output)
    }
}
#[doc = "FLEXCOMM6 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc6dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc6dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6DATAOUTEN` reader - FLEXCOMM6 DATAOUT OUTPUT ENABLE"]
pub type Fc6dataoutenR = crate::BitReader<Fc6dataouten>;
impl Fc6dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc6dataouten {
        match self.bits {
            false => Fc6dataouten::Input,
            true => Fc6dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc6dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc6dataouten::Output
    }
}
#[doc = "Field `FC6DATAOUTEN` writer - FLEXCOMM6 DATAOUT OUTPUT ENABLE"]
pub type Fc6dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc6dataouten>;
impl<'a, REG> Fc6dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6dataouten::Output)
    }
}
#[doc = "FLEXCOMM7 DATAOUT OUTPUT ENABLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7dataouten {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<Fc7dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc7dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7DATAOUTEN` reader - FLEXCOMM7 DATAOUT OUTPUT ENABLE"]
pub type Fc7dataoutenR = crate::BitReader<Fc7dataouten>;
impl Fc7dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc7dataouten {
        match self.bits {
            false => Fc7dataouten::Input,
            true => Fc7dataouten::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc7dataouten::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc7dataouten::Output
    }
}
#[doc = "Field `FC7DATAOUTEN` writer - FLEXCOMM7 DATAOUT OUTPUT ENABLE"]
pub type Fc7dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc7dataouten>;
impl<'a, REG> Fc7dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7dataouten::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7dataouten::Output)
    }
}
impl R {
    #[doc = "Bits 0:2 - Shared SCK Select. . ."]
    #[inline(always)]
    pub fn sharedscksel(&self) -> SharedsckselR {
        SharedsckselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Shared WS Select. . ."]
    #[inline(always)]
    pub fn sharedwssel(&self) -> SharedwsselR {
        SharedwsselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Shared DATA Select. . ."]
    #[inline(always)]
    pub fn shareddatasel(&self) -> ShareddataselR {
        ShareddataselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - FLEXCOMM0 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc0dataouten(&self) -> Fc0dataoutenR {
        Fc0dataoutenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLEXCOMM1 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc1dataouten(&self) -> Fc1dataoutenR {
        Fc1dataoutenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLEXCOMM2 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn f20dataouten(&self) -> F20dataoutenR {
        F20dataoutenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FLEXCOMM3 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc3dataouten(&self) -> Fc3dataoutenR {
        Fc3dataoutenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FLEXCOMM4 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc4dataouten(&self) -> Fc4dataoutenR {
        Fc4dataoutenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FLEXCOMM5 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc5dataouten(&self) -> Fc5dataoutenR {
        Fc5dataoutenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FLEXCOMM6 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc6dataouten(&self) -> Fc6dataoutenR {
        Fc6dataoutenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FLEXCOMM7 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub fn fc7dataouten(&self) -> Fc7dataoutenR {
        Fc7dataoutenR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHAREDCTRLSET")
            .field("sharedscksel", &self.sharedscksel())
            .field("sharedwssel", &self.sharedwssel())
            .field("shareddatasel", &self.shareddatasel())
            .field("fc0dataouten", &self.fc0dataouten())
            .field("fc1dataouten", &self.fc1dataouten())
            .field("f20dataouten", &self.f20dataouten())
            .field("fc3dataouten", &self.fc3dataouten())
            .field("fc4dataouten", &self.fc4dataouten())
            .field("fc5dataouten", &self.fc5dataouten())
            .field("fc6dataouten", &self.fc6dataouten())
            .field("fc7dataouten", &self.fc7dataouten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Shared SCK Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sharedscksel(&mut self) -> SharedsckselW<SharedctrlsetSpec> {
        SharedsckselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Shared WS Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sharedwssel(&mut self) -> SharedwsselW<SharedctrlsetSpec> {
        SharedwsselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Shared DATA Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn shareddatasel(&mut self) -> ShareddataselW<SharedctrlsetSpec> {
        ShareddataselW::new(self, 8)
    }
    #[doc = "Bit 16 - FLEXCOMM0 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc0dataouten(&mut self) -> Fc0dataoutenW<SharedctrlsetSpec> {
        Fc0dataoutenW::new(self, 16)
    }
    #[doc = "Bit 17 - FLEXCOMM1 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc1dataouten(&mut self) -> Fc1dataoutenW<SharedctrlsetSpec> {
        Fc1dataoutenW::new(self, 17)
    }
    #[doc = "Bit 18 - FLEXCOMM2 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn f20dataouten(&mut self) -> F20dataoutenW<SharedctrlsetSpec> {
        F20dataoutenW::new(self, 18)
    }
    #[doc = "Bit 19 - FLEXCOMM3 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc3dataouten(&mut self) -> Fc3dataoutenW<SharedctrlsetSpec> {
        Fc3dataoutenW::new(self, 19)
    }
    #[doc = "Bit 20 - FLEXCOMM4 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc4dataouten(&mut self) -> Fc4dataoutenW<SharedctrlsetSpec> {
        Fc4dataoutenW::new(self, 20)
    }
    #[doc = "Bit 21 - FLEXCOMM5 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc5dataouten(&mut self) -> Fc5dataoutenW<SharedctrlsetSpec> {
        Fc5dataoutenW::new(self, 21)
    }
    #[doc = "Bit 22 - FLEXCOMM6 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc6dataouten(&mut self) -> Fc6dataoutenW<SharedctrlsetSpec> {
        Fc6dataoutenW::new(self, 22)
    }
    #[doc = "Bit 23 - FLEXCOMM7 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn fc7dataouten(&mut self) -> Fc7dataoutenW<SharedctrlsetSpec> {
        Fc7dataoutenW::new(self, 23)
    }
}
#[doc = "shared control set N\n\nYou can [`read`](crate::Reg::read) this register and get [`sharedctrlset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharedctrlset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedctrlsetSpec;
impl crate::RegisterSpec for SharedctrlsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharedctrlset::R`](R) reader structure"]
impl crate::Readable for SharedctrlsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sharedctrlset::W`](W) writer structure"]
impl crate::Writable for SharedctrlsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHAREDCTRLSET%s to value 0"]
impl crate::Resettable for SharedctrlsetSpec {
    const RESET_VALUE: u32 = 0;
}
