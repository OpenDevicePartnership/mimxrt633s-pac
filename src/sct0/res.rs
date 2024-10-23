#[doc = "Register `RES` reader"]
pub type R = crate::R<ResSpec>;
#[doc = "Register `RES` writer"]
pub type W = crate::W<ResSpec>;
#[doc = "Effect of simultaneous set and clear on output 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O0res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR0 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O0res> for u8 {
    #[inline(always)]
    fn from(variant: O0res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O0res {
    type Ux = u8;
}
impl crate::IsEnum for O0res {}
#[doc = "Field `O0RES` reader - Effect of simultaneous set and clear on output 0."]
pub type O0resR = crate::FieldReader<O0res>;
impl O0resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O0res {
        match self.bits {
            0 => O0res::NoChange,
            1 => O0res::Set,
            2 => O0res::Clear,
            3 => O0res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O0res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O0res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O0res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0res::ToggleOutput
    }
}
#[doc = "Field `O0RES` writer - Effect of simultaneous set and clear on output 0."]
pub type O0resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O0res, crate::Safe>;
impl<'a, REG> O0resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O0res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O0res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O0res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O0res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O1res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR1 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O1res> for u8 {
    #[inline(always)]
    fn from(variant: O1res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O1res {
    type Ux = u8;
}
impl crate::IsEnum for O1res {}
#[doc = "Field `O1RES` reader - Effect of simultaneous set and clear on output 1."]
pub type O1resR = crate::FieldReader<O1res>;
impl O1resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O1res {
        match self.bits {
            0 => O1res::NoChange,
            1 => O1res::Set,
            2 => O1res::Clear,
            3 => O1res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O1res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O1res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O1res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1res::ToggleOutput
    }
}
#[doc = "Field `O1RES` writer - Effect of simultaneous set and clear on output 1."]
pub type O1resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O1res, crate::Safe>;
impl<'a, REG> O1resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O1res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O1res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O1res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O1res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 2.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O2res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output n (or set based on the SETCLR2 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O2res> for u8 {
    #[inline(always)]
    fn from(variant: O2res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O2res {
    type Ux = u8;
}
impl crate::IsEnum for O2res {}
#[doc = "Field `O2RES` reader - Effect of simultaneous set and clear on output 2."]
pub type O2resR = crate::FieldReader<O2res>;
impl O2resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O2res {
        match self.bits {
            0 => O2res::NoChange,
            1 => O2res::Set,
            2 => O2res::Clear,
            3 => O2res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O2res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O2res::Set
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O2res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2res::ToggleOutput
    }
}
#[doc = "Field `O2RES` writer - Effect of simultaneous set and clear on output 2."]
pub type O2resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O2res, crate::Safe>;
impl<'a, REG> O2resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O2res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O2res::Set)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O2res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O2res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 3.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O3res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR3 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O3res> for u8 {
    #[inline(always)]
    fn from(variant: O3res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O3res {
    type Ux = u8;
}
impl crate::IsEnum for O3res {}
#[doc = "Field `O3RES` reader - Effect of simultaneous set and clear on output 3."]
pub type O3resR = crate::FieldReader<O3res>;
impl O3resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O3res {
        match self.bits {
            0 => O3res::NoChange,
            1 => O3res::Set,
            2 => O3res::Clear,
            3 => O3res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O3res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O3res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O3res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3res::ToggleOutput
    }
}
#[doc = "Field `O3RES` writer - Effect of simultaneous set and clear on output 3."]
pub type O3resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O3res, crate::Safe>;
impl<'a, REG> O3resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O3res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O3res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O3res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O3res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 4.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O4res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR4 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O4res> for u8 {
    #[inline(always)]
    fn from(variant: O4res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O4res {
    type Ux = u8;
}
impl crate::IsEnum for O4res {}
#[doc = "Field `O4RES` reader - Effect of simultaneous set and clear on output 4."]
pub type O4resR = crate::FieldReader<O4res>;
impl O4resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O4res {
        match self.bits {
            0 => O4res::NoChange,
            1 => O4res::Set,
            2 => O4res::Clear,
            3 => O4res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O4res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O4res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O4res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O4res::ToggleOutput
    }
}
#[doc = "Field `O4RES` writer - Effect of simultaneous set and clear on output 4."]
pub type O4resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O4res, crate::Safe>;
impl<'a, REG> O4resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O4res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O4res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O4res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O4res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 5.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O5res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR5 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O5res> for u8 {
    #[inline(always)]
    fn from(variant: O5res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O5res {
    type Ux = u8;
}
impl crate::IsEnum for O5res {}
#[doc = "Field `O5RES` reader - Effect of simultaneous set and clear on output 5."]
pub type O5resR = crate::FieldReader<O5res>;
impl O5resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O5res {
        match self.bits {
            0 => O5res::NoChange,
            1 => O5res::Set,
            2 => O5res::Clear,
            3 => O5res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O5res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O5res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O5res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O5res::ToggleOutput
    }
}
#[doc = "Field `O5RES` writer - Effect of simultaneous set and clear on output 5."]
pub type O5resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O5res, crate::Safe>;
impl<'a, REG> O5resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O5res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O5res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O5res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O5res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 6.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O6res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR6 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O6res> for u8 {
    #[inline(always)]
    fn from(variant: O6res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O6res {
    type Ux = u8;
}
impl crate::IsEnum for O6res {}
#[doc = "Field `O6RES` reader - Effect of simultaneous set and clear on output 6."]
pub type O6resR = crate::FieldReader<O6res>;
impl O6resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O6res {
        match self.bits {
            0 => O6res::NoChange,
            1 => O6res::Set,
            2 => O6res::Clear,
            3 => O6res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O6res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O6res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O6res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O6res::ToggleOutput
    }
}
#[doc = "Field `O6RES` writer - Effect of simultaneous set and clear on output 6."]
pub type O6resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O6res, crate::Safe>;
impl<'a, REG> O6resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O6res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O6res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O6res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O6res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 7.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O7res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output n (or set based on the SETCLR7 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O7res> for u8 {
    #[inline(always)]
    fn from(variant: O7res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O7res {
    type Ux = u8;
}
impl crate::IsEnum for O7res {}
#[doc = "Field `O7RES` reader - Effect of simultaneous set and clear on output 7."]
pub type O7resR = crate::FieldReader<O7res>;
impl O7resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O7res {
        match self.bits {
            0 => O7res::NoChange,
            1 => O7res::Set,
            2 => O7res::Clear,
            3 => O7res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O7res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O7res::Set
    }
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O7res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O7res::ToggleOutput
    }
}
#[doc = "Field `O7RES` writer - Effect of simultaneous set and clear on output 7."]
pub type O7resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O7res, crate::Safe>;
impl<'a, REG> O7resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O7res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O7res::Set)
    }
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O7res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O7res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 8.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O8res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR8 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O8res> for u8 {
    #[inline(always)]
    fn from(variant: O8res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O8res {
    type Ux = u8;
}
impl crate::IsEnum for O8res {}
#[doc = "Field `O8RES` reader - Effect of simultaneous set and clear on output 8."]
pub type O8resR = crate::FieldReader<O8res>;
impl O8resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O8res {
        match self.bits {
            0 => O8res::NoChange,
            1 => O8res::Set,
            2 => O8res::Clear,
            3 => O8res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O8res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O8res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O8res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O8res::ToggleOutput
    }
}
#[doc = "Field `O8RES` writer - Effect of simultaneous set and clear on output 8."]
pub type O8resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O8res, crate::Safe>;
impl<'a, REG> O8resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O8res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O8res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O8res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O8res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 9.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O9res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR9 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O9res> for u8 {
    #[inline(always)]
    fn from(variant: O9res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O9res {
    type Ux = u8;
}
impl crate::IsEnum for O9res {}
#[doc = "Field `O9RES` reader - Effect of simultaneous set and clear on output 9."]
pub type O9resR = crate::FieldReader<O9res>;
impl O9resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O9res {
        match self.bits {
            0 => O9res::NoChange,
            1 => O9res::Set,
            2 => O9res::Clear,
            3 => O9res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O9res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O9res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O9res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O9res::ToggleOutput
    }
}
#[doc = "Field `O9RES` writer - Effect of simultaneous set and clear on output 9."]
pub type O9resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O9res, crate::Safe>;
impl<'a, REG> O9resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O9res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O9res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O9res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O9res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 10.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O10res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR10 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O10res> for u8 {
    #[inline(always)]
    fn from(variant: O10res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O10res {
    type Ux = u8;
}
impl crate::IsEnum for O10res {}
#[doc = "Field `O10RES` reader - Effect of simultaneous set and clear on output 10."]
pub type O10resR = crate::FieldReader<O10res>;
impl O10resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O10res {
        match self.bits {
            0 => O10res::NoChange,
            1 => O10res::Set,
            2 => O10res::Clear,
            3 => O10res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O10res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O10res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O10res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O10res::ToggleOutput
    }
}
#[doc = "Field `O10RES` writer - Effect of simultaneous set and clear on output 10."]
pub type O10resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O10res, crate::Safe>;
impl<'a, REG> O10resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O10res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O10res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O10res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O10res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 11.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O11res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR11 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O11res> for u8 {
    #[inline(always)]
    fn from(variant: O11res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O11res {
    type Ux = u8;
}
impl crate::IsEnum for O11res {}
#[doc = "Field `O11RES` reader - Effect of simultaneous set and clear on output 11."]
pub type O11resR = crate::FieldReader<O11res>;
impl O11resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O11res {
        match self.bits {
            0 => O11res::NoChange,
            1 => O11res::Set,
            2 => O11res::Clear,
            3 => O11res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O11res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O11res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O11res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O11res::ToggleOutput
    }
}
#[doc = "Field `O11RES` writer - Effect of simultaneous set and clear on output 11."]
pub type O11resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O11res, crate::Safe>;
impl<'a, REG> O11resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O11res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O11res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O11res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O11res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 12.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O12res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR12 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O12res> for u8 {
    #[inline(always)]
    fn from(variant: O12res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O12res {
    type Ux = u8;
}
impl crate::IsEnum for O12res {}
#[doc = "Field `O12RES` reader - Effect of simultaneous set and clear on output 12."]
pub type O12resR = crate::FieldReader<O12res>;
impl O12resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O12res {
        match self.bits {
            0 => O12res::NoChange,
            1 => O12res::Set,
            2 => O12res::Clear,
            3 => O12res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O12res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O12res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O12res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O12res::ToggleOutput
    }
}
#[doc = "Field `O12RES` writer - Effect of simultaneous set and clear on output 12."]
pub type O12resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O12res, crate::Safe>;
impl<'a, REG> O12resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O12res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O12res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O12res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O12res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 13.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O13res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR13 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O13res> for u8 {
    #[inline(always)]
    fn from(variant: O13res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O13res {
    type Ux = u8;
}
impl crate::IsEnum for O13res {}
#[doc = "Field `O13RES` reader - Effect of simultaneous set and clear on output 13."]
pub type O13resR = crate::FieldReader<O13res>;
impl O13resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O13res {
        match self.bits {
            0 => O13res::NoChange,
            1 => O13res::Set,
            2 => O13res::Clear,
            3 => O13res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O13res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O13res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O13res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O13res::ToggleOutput
    }
}
#[doc = "Field `O13RES` writer - Effect of simultaneous set and clear on output 13."]
pub type O13resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O13res, crate::Safe>;
impl<'a, REG> O13resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O13res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O13res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O13res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O13res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 14.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O14res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR14 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O14res> for u8 {
    #[inline(always)]
    fn from(variant: O14res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O14res {
    type Ux = u8;
}
impl crate::IsEnum for O14res {}
#[doc = "Field `O14RES` reader - Effect of simultaneous set and clear on output 14."]
pub type O14resR = crate::FieldReader<O14res>;
impl O14resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O14res {
        match self.bits {
            0 => O14res::NoChange,
            1 => O14res::Set,
            2 => O14res::Clear,
            3 => O14res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O14res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O14res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O14res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O14res::ToggleOutput
    }
}
#[doc = "Field `O14RES` writer - Effect of simultaneous set and clear on output 14."]
pub type O14resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O14res, crate::Safe>;
impl<'a, REG> O14resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O14res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O14res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O14res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O14res::ToggleOutput)
    }
}
#[doc = "Effect of simultaneous set and clear on output 15.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum O15res {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    Set = 1,
    #[doc = "2: Clear output (or set based on the SETCLR15 field)."]
    Clear = 2,
    #[doc = "3: Toggle output."]
    ToggleOutput = 3,
}
impl From<O15res> for u8 {
    #[inline(always)]
    fn from(variant: O15res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for O15res {
    type Ux = u8;
}
impl crate::IsEnum for O15res {}
#[doc = "Field `O15RES` reader - Effect of simultaneous set and clear on output 15."]
pub type O15resR = crate::FieldReader<O15res>;
impl O15resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O15res {
        match self.bits {
            0 => O15res::NoChange,
            1 => O15res::Set,
            2 => O15res::Clear,
            3 => O15res::ToggleOutput,
            _ => unreachable!(),
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O15res::NoChange
    }
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O15res::Set
    }
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O15res::Clear
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O15res::ToggleOutput
    }
}
#[doc = "Field `O15RES` writer - Effect of simultaneous set and clear on output 15."]
pub type O15resW<'a, REG> = crate::FieldWriter<'a, REG, 2, O15res, crate::Safe>;
impl<'a, REG> O15resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(O15res::NoChange)
    }
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(O15res::Set)
    }
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(O15res::Clear)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut crate::W<REG> {
        self.variant(O15res::ToggleOutput)
    }
}
impl R {
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&self) -> O0resR {
        O0resR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&self) -> O1resR {
        O1resR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&self) -> O2resR {
        O2resR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&self) -> O3resR {
        O3resR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub fn o4res(&self) -> O4resR {
        O4resR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub fn o5res(&self) -> O5resR {
        O5resR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub fn o6res(&self) -> O6resR {
        O6resR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub fn o7res(&self) -> O7resR {
        O7resR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub fn o8res(&self) -> O8resR {
        O8resR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub fn o9res(&self) -> O9resR {
        O9resR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub fn o10res(&self) -> O10resR {
        O10resR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub fn o11res(&self) -> O11resR {
        O11resR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub fn o12res(&self) -> O12resR {
        O12resR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub fn o13res(&self) -> O13resR {
        O13resR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub fn o14res(&self) -> O14resR {
        O14resR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub fn o15res(&self) -> O15resR {
        O15resR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES")
            .field("o0res", &self.o0res())
            .field("o1res", &self.o1res())
            .field("o2res", &self.o2res())
            .field("o3res", &self.o3res())
            .field("o4res", &self.o4res())
            .field("o5res", &self.o5res())
            .field("o6res", &self.o6res())
            .field("o7res", &self.o7res())
            .field("o8res", &self.o8res())
            .field("o9res", &self.o9res())
            .field("o10res", &self.o10res())
            .field("o11res", &self.o11res())
            .field("o12res", &self.o12res())
            .field("o13res", &self.o13res())
            .field("o14res", &self.o14res())
            .field("o15res", &self.o15res())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    #[must_use]
    pub fn o0res(&mut self) -> O0resW<ResSpec> {
        O0resW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    #[must_use]
    pub fn o1res(&mut self) -> O1resW<ResSpec> {
        O1resW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    #[must_use]
    pub fn o2res(&mut self) -> O2resW<ResSpec> {
        O2resW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    #[must_use]
    pub fn o3res(&mut self) -> O3resW<ResSpec> {
        O3resW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    #[must_use]
    pub fn o4res(&mut self) -> O4resW<ResSpec> {
        O4resW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    #[must_use]
    pub fn o5res(&mut self) -> O5resW<ResSpec> {
        O5resW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    #[must_use]
    pub fn o6res(&mut self) -> O6resW<ResSpec> {
        O6resW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    #[must_use]
    pub fn o7res(&mut self) -> O7resW<ResSpec> {
        O7resW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    #[must_use]
    pub fn o8res(&mut self) -> O8resW<ResSpec> {
        O8resW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    #[must_use]
    pub fn o9res(&mut self) -> O9resW<ResSpec> {
        O9resW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    #[must_use]
    pub fn o10res(&mut self) -> O10resW<ResSpec> {
        O10resW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    #[must_use]
    pub fn o11res(&mut self) -> O11resW<ResSpec> {
        O11resW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    #[must_use]
    pub fn o12res(&mut self) -> O12resW<ResSpec> {
        O12resW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    #[must_use]
    pub fn o13res(&mut self) -> O13resW<ResSpec> {
        O13resW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    #[must_use]
    pub fn o14res(&mut self) -> O14resW<ResSpec> {
        O14resW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    #[must_use]
    pub fn o15res(&mut self) -> O15resW<ResSpec> {
        O15resW::new(self, 30)
    }
}
#[doc = "SCT conflict resolution register\n\nYou can [`read`](crate::Reg::read) this register and get [`res::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResSpec;
impl crate::RegisterSpec for ResSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res::R`](R) reader structure"]
impl crate::Readable for ResSpec {}
#[doc = "`write(|w| ..)` method takes [`res::W`](W) writer structure"]
impl crate::Writable for ResSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES to value 0"]
impl crate::Resettable for ResSpec {
    const RESET_VALUE: u32 = 0;
}
