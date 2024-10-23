#[doc = "Register `PMSRC` reader"]
pub type R = crate::R<PmsrcSpec>;
#[doc = "Register `PMSRC` writer"]
pub type W = crate::W<PmsrcSpec>;
#[doc = "Selects the input source for bit slice 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src0 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    Input7 = 7,
}
impl From<Src0> for u8 {
    #[inline(always)]
    fn from(variant: Src0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src0 {
    type Ux = u8;
}
impl crate::IsEnum for Src0 {}
#[doc = "Field `SRC0` reader - Selects the input source for bit slice 0"]
pub type Src0R = crate::FieldReader<Src0>;
impl Src0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src0 {
        match self.bits {
            0 => Src0::Input0,
            1 => Src0::Input1,
            2 => Src0::Input2,
            3 => Src0::Input3,
            4 => Src0::Input4,
            5 => Src0::Input5,
            6 => Src0::Input6,
            7 => Src0::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src0::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src0::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src0::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src0::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src0::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src0::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src0::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src0::Input7
    }
}
#[doc = "Field `SRC0` writer - Selects the input source for bit slice 0"]
pub type Src0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src0, crate::Safe>;
impl<'a, REG> Src0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Input7)
    }
}
#[doc = "Selects the input source for bit slice 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src1 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    Input7 = 7,
}
impl From<Src1> for u8 {
    #[inline(always)]
    fn from(variant: Src1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src1 {
    type Ux = u8;
}
impl crate::IsEnum for Src1 {}
#[doc = "Field `SRC1` reader - Selects the input source for bit slice 1"]
pub type Src1R = crate::FieldReader<Src1>;
impl Src1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src1 {
        match self.bits {
            0 => Src1::Input0,
            1 => Src1::Input1,
            2 => Src1::Input2,
            3 => Src1::Input3,
            4 => Src1::Input4,
            5 => Src1::Input5,
            6 => Src1::Input6,
            7 => Src1::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src1::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src1::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src1::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src1::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src1::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src1::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src1::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src1::Input7
    }
}
#[doc = "Field `SRC1` writer - Selects the input source for bit slice 1"]
pub type Src1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src1, crate::Safe>;
impl<'a, REG> Src1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Input7)
    }
}
#[doc = "Selects the input source for bit slice 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src2 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    Input7 = 7,
}
impl From<Src2> for u8 {
    #[inline(always)]
    fn from(variant: Src2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src2 {
    type Ux = u8;
}
impl crate::IsEnum for Src2 {}
#[doc = "Field `SRC2` reader - Selects the input source for bit slice 2"]
pub type Src2R = crate::FieldReader<Src2>;
impl Src2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src2 {
        match self.bits {
            0 => Src2::Input0,
            1 => Src2::Input1,
            2 => Src2::Input2,
            3 => Src2::Input3,
            4 => Src2::Input4,
            5 => Src2::Input5,
            6 => Src2::Input6,
            7 => Src2::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src2::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src2::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src2::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src2::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src2::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src2::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src2::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src2::Input7
    }
}
#[doc = "Field `SRC2` writer - Selects the input source for bit slice 2"]
pub type Src2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src2, crate::Safe>;
impl<'a, REG> Src2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Input7)
    }
}
#[doc = "Selects the input source for bit slice 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src3 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    Input7 = 7,
}
impl From<Src3> for u8 {
    #[inline(always)]
    fn from(variant: Src3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src3 {
    type Ux = u8;
}
impl crate::IsEnum for Src3 {}
#[doc = "Field `SRC3` reader - Selects the input source for bit slice 3"]
pub type Src3R = crate::FieldReader<Src3>;
impl Src3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src3 {
        match self.bits {
            0 => Src3::Input0,
            1 => Src3::Input1,
            2 => Src3::Input2,
            3 => Src3::Input3,
            4 => Src3::Input4,
            5 => Src3::Input5,
            6 => Src3::Input6,
            7 => Src3::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src3::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src3::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src3::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src3::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src3::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src3::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src3::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src3::Input7
    }
}
#[doc = "Field `SRC3` writer - Selects the input source for bit slice 3"]
pub type Src3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src3, crate::Safe>;
impl<'a, REG> Src3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Input7)
    }
}
#[doc = "Selects the input source for bit slice 4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src4 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    Input7 = 7,
}
impl From<Src4> for u8 {
    #[inline(always)]
    fn from(variant: Src4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src4 {
    type Ux = u8;
}
impl crate::IsEnum for Src4 {}
#[doc = "Field `SRC4` reader - Selects the input source for bit slice 4"]
pub type Src4R = crate::FieldReader<Src4>;
impl Src4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src4 {
        match self.bits {
            0 => Src4::Input0,
            1 => Src4::Input1,
            2 => Src4::Input2,
            3 => Src4::Input3,
            4 => Src4::Input4,
            5 => Src4::Input5,
            6 => Src4::Input6,
            7 => Src4::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src4::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src4::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src4::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src4::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src4::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src4::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src4::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src4::Input7
    }
}
#[doc = "Field `SRC4` writer - Selects the input source for bit slice 4"]
pub type Src4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src4, crate::Safe>;
impl<'a, REG> Src4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Input7)
    }
}
#[doc = "Selects the input source for bit slice 5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src5 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    Input7 = 7,
}
impl From<Src5> for u8 {
    #[inline(always)]
    fn from(variant: Src5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src5 {
    type Ux = u8;
}
impl crate::IsEnum for Src5 {}
#[doc = "Field `SRC5` reader - Selects the input source for bit slice 5"]
pub type Src5R = crate::FieldReader<Src5>;
impl Src5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src5 {
        match self.bits {
            0 => Src5::Input0,
            1 => Src5::Input1,
            2 => Src5::Input2,
            3 => Src5::Input3,
            4 => Src5::Input4,
            5 => Src5::Input5,
            6 => Src5::Input6,
            7 => Src5::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src5::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src5::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src5::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src5::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src5::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src5::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src5::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src5::Input7
    }
}
#[doc = "Field `SRC5` writer - Selects the input source for bit slice 5"]
pub type Src5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src5, crate::Safe>;
impl<'a, REG> Src5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Input7)
    }
}
#[doc = "Selects the input source for bit slice 6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src6 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    Input7 = 7,
}
impl From<Src6> for u8 {
    #[inline(always)]
    fn from(variant: Src6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src6 {
    type Ux = u8;
}
impl crate::IsEnum for Src6 {}
#[doc = "Field `SRC6` reader - Selects the input source for bit slice 6"]
pub type Src6R = crate::FieldReader<Src6>;
impl Src6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src6 {
        match self.bits {
            0 => Src6::Input0,
            1 => Src6::Input1,
            2 => Src6::Input2,
            3 => Src6::Input3,
            4 => Src6::Input4,
            5 => Src6::Input5,
            6 => Src6::Input6,
            7 => Src6::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src6::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src6::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src6::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src6::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src6::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src6::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src6::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src6::Input7
    }
}
#[doc = "Field `SRC6` writer - Selects the input source for bit slice 6"]
pub type Src6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src6, crate::Safe>;
impl<'a, REG> Src6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Input7)
    }
}
#[doc = "Selects the input source for bit slice 7\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src7 {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    Input0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    Input1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    Input2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    Input3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    Input4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    Input5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    Input6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    Input7 = 7,
}
impl From<Src7> for u8 {
    #[inline(always)]
    fn from(variant: Src7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src7 {
    type Ux = u8;
}
impl crate::IsEnum for Src7 {}
#[doc = "Field `SRC7` reader - Selects the input source for bit slice 7"]
pub type Src7R = crate::FieldReader<Src7>;
impl Src7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src7 {
        match self.bits {
            0 => Src7::Input0,
            1 => Src7::Input1,
            2 => Src7::Input2,
            3 => Src7::Input3,
            4 => Src7::Input4,
            5 => Src7::Input5,
            6 => Src7::Input6,
            7 => Src7::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == Src7::Input0
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Src7::Input1
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == Src7::Input2
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Src7::Input3
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == Src7::Input4
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Src7::Input5
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == Src7::Input6
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Src7::Input7
    }
}
#[doc = "Field `SRC7` writer - Selects the input source for bit slice 7"]
pub type Src7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Src7, crate::Safe>;
impl<'a, REG> Src7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Input7)
    }
}
impl R {
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline(always)]
    pub fn src0(&self) -> Src0R {
        Src0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline(always)]
    pub fn src1(&self) -> Src1R {
        Src1R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline(always)]
    pub fn src2(&self) -> Src2R {
        Src2R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline(always)]
    pub fn src3(&self) -> Src3R {
        Src3R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline(always)]
    pub fn src4(&self) -> Src4R {
        Src4R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline(always)]
    pub fn src5(&self) -> Src5R {
        Src5R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline(always)]
    pub fn src6(&self) -> Src6R {
        Src6R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline(always)]
    pub fn src7(&self) -> Src7R {
        Src7R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMSRC")
            .field("src0", &self.src0())
            .field("src1", &self.src1())
            .field("src2", &self.src2())
            .field("src3", &self.src3())
            .field("src4", &self.src4())
            .field("src5", &self.src5())
            .field("src6", &self.src6())
            .field("src7", &self.src7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline(always)]
    #[must_use]
    pub fn src0(&mut self) -> Src0W<PmsrcSpec> {
        Src0W::new(self, 8)
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline(always)]
    #[must_use]
    pub fn src1(&mut self) -> Src1W<PmsrcSpec> {
        Src1W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline(always)]
    #[must_use]
    pub fn src2(&mut self) -> Src2W<PmsrcSpec> {
        Src2W::new(self, 14)
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline(always)]
    #[must_use]
    pub fn src3(&mut self) -> Src3W<PmsrcSpec> {
        Src3W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline(always)]
    #[must_use]
    pub fn src4(&mut self) -> Src4W<PmsrcSpec> {
        Src4W::new(self, 20)
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline(always)]
    #[must_use]
    pub fn src5(&mut self) -> Src5W<PmsrcSpec> {
        Src5W::new(self, 23)
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline(always)]
    #[must_use]
    pub fn src6(&mut self) -> Src6W<PmsrcSpec> {
        Src6W::new(self, 26)
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline(always)]
    #[must_use]
    pub fn src7(&mut self) -> Src7W<PmsrcSpec> {
        Src7W::new(self, 29)
    }
}
#[doc = "Pattern match interrupt bit-slice source register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmsrcSpec;
impl crate::RegisterSpec for PmsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmsrc::R`](R) reader structure"]
impl crate::Readable for PmsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`pmsrc::W`](W) writer structure"]
impl crate::Writable for PmsrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMSRC to value 0"]
impl crate::Resettable for PmsrcSpec {
    const RESET_VALUE: u32 = 0;
}
