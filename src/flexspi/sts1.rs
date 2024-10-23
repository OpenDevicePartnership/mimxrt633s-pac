#[doc = "Register `STS1` reader"]
pub type R = crate::R<Sts1Spec>;
#[doc = "Field `AHBCMDERRID` reader - Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
pub type AhbcmderridR = crate::FieldReader;
#[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ahbcmderrcode {
    #[doc = "0: No error."]
    Ahbcmderrcode0 = 0,
    #[doc = "2: AHB Write command with JMP_ON_CS instruction used in the sequence."]
    Ahbcmderrcode2 = 2,
    #[doc = "3: There is unknown instruction opcode in the sequence."]
    Ahbcmderrcode3 = 3,
    #[doc = "4: Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    Ahbcmderrcode4 = 4,
    #[doc = "5: Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    Ahbcmderrcode5 = 5,
    #[doc = "14: Sequence execution timeout."]
    Ahbcmderrcode14 = 14,
}
impl From<Ahbcmderrcode> for u8 {
    #[inline(always)]
    fn from(variant: Ahbcmderrcode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ahbcmderrcode {
    type Ux = u8;
}
impl crate::IsEnum for Ahbcmderrcode {}
#[doc = "Field `AHBCMDERRCODE` reader - Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
pub type AhbcmderrcodeR = crate::FieldReader<Ahbcmderrcode>;
impl AhbcmderrcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ahbcmderrcode> {
        match self.bits {
            0 => Some(Ahbcmderrcode::Ahbcmderrcode0),
            2 => Some(Ahbcmderrcode::Ahbcmderrcode2),
            3 => Some(Ahbcmderrcode::Ahbcmderrcode3),
            4 => Some(Ahbcmderrcode::Ahbcmderrcode4),
            5 => Some(Ahbcmderrcode::Ahbcmderrcode5),
            14 => Some(Ahbcmderrcode::Ahbcmderrcode14),
            _ => None,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_ahbcmderrcode_0(&self) -> bool {
        *self == Ahbcmderrcode::Ahbcmderrcode0
    }
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence."]
    #[inline(always)]
    pub fn is_ahbcmderrcode_2(&self) -> bool {
        *self == Ahbcmderrcode::Ahbcmderrcode2
    }
    #[doc = "There is unknown instruction opcode in the sequence."]
    #[inline(always)]
    pub fn is_ahbcmderrcode_3(&self) -> bool {
        *self == Ahbcmderrcode::Ahbcmderrcode3
    }
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    #[inline(always)]
    pub fn is_ahbcmderrcode_4(&self) -> bool {
        *self == Ahbcmderrcode::Ahbcmderrcode4
    }
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    #[inline(always)]
    pub fn is_ahbcmderrcode_5(&self) -> bool {
        *self == Ahbcmderrcode::Ahbcmderrcode5
    }
    #[doc = "Sequence execution timeout."]
    #[inline(always)]
    pub fn is_ahbcmderrcode_14(&self) -> bool {
        *self == Ahbcmderrcode::Ahbcmderrcode14
    }
}
#[doc = "Field `IPCMDERRID` reader - Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
pub type IpcmderridR = crate::FieldReader;
#[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipcmderrcode {
    #[doc = "0: No error."]
    Ipcmderrcode0 = 0,
    #[doc = "2: IP command with JMP_ON_CS instruction used in the sequence."]
    Ipcmderrcode2 = 2,
    #[doc = "3: There is unknown instruction opcode in the sequence."]
    Ipcmderrcode3 = 3,
    #[doc = "4: Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    Ipcmderrcode4 = 4,
    #[doc = "5: Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    Ipcmderrcode5 = 5,
    #[doc = "6: Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    Ipcmderrcode6 = 6,
    #[doc = "14: Sequence execution timeout."]
    Ipcmderrcode14 = 14,
    #[doc = "15: Flash boundary crossed."]
    Ipcmderrcode15 = 15,
}
impl From<Ipcmderrcode> for u8 {
    #[inline(always)]
    fn from(variant: Ipcmderrcode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipcmderrcode {
    type Ux = u8;
}
impl crate::IsEnum for Ipcmderrcode {}
#[doc = "Field `IPCMDERRCODE` reader - Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
pub type IpcmderrcodeR = crate::FieldReader<Ipcmderrcode>;
impl IpcmderrcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ipcmderrcode> {
        match self.bits {
            0 => Some(Ipcmderrcode::Ipcmderrcode0),
            2 => Some(Ipcmderrcode::Ipcmderrcode2),
            3 => Some(Ipcmderrcode::Ipcmderrcode3),
            4 => Some(Ipcmderrcode::Ipcmderrcode4),
            5 => Some(Ipcmderrcode::Ipcmderrcode5),
            6 => Some(Ipcmderrcode::Ipcmderrcode6),
            14 => Some(Ipcmderrcode::Ipcmderrcode14),
            15 => Some(Ipcmderrcode::Ipcmderrcode15),
            _ => None,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_ipcmderrcode_0(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode0
    }
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence."]
    #[inline(always)]
    pub fn is_ipcmderrcode_2(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode2
    }
    #[doc = "There is unknown instruction opcode in the sequence."]
    #[inline(always)]
    pub fn is_ipcmderrcode_3(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode3
    }
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    #[inline(always)]
    pub fn is_ipcmderrcode_4(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode4
    }
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    #[inline(always)]
    pub fn is_ipcmderrcode_5(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode5
    }
    #[doc = "Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    #[inline(always)]
    pub fn is_ipcmderrcode_6(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode6
    }
    #[doc = "Sequence execution timeout."]
    #[inline(always)]
    pub fn is_ipcmderrcode_14(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode14
    }
    #[doc = "Flash boundary crossed."]
    #[inline(always)]
    pub fn is_ipcmderrcode_15(&self) -> bool {
        *self == Ipcmderrcode::Ipcmderrcode15
    }
}
impl R {
    #[doc = "Bits 0:4 - Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ahbcmderrid(&self) -> AhbcmderridR {
        AhbcmderridR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ahbcmderrcode(&self) -> AhbcmderrcodeR {
        AhbcmderrcodeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ipcmderrid(&self) -> IpcmderridR {
        IpcmderridR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ipcmderrcode(&self) -> IpcmderrcodeR {
        IpcmderrcodeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS1")
            .field("ahbcmderrid", &self.ahbcmderrid())
            .field("ahbcmderrcode", &self.ahbcmderrcode())
            .field("ipcmderrid", &self.ipcmderrid())
            .field("ipcmderrcode", &self.ipcmderrcode())
            .finish()
    }
}
#[doc = "Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts1Spec;
impl crate::RegisterSpec for Sts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for Sts1Spec {}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for Sts1Spec {
    const RESET_VALUE: u32 = 0;
}
