#[doc = "Register `P3CFG` reader"]
pub type R = crate::R<P3cfgSpec>;
#[doc = "Register `P3CFG` writer"]
pub type W = crate::W<P3cfgSpec>;
#[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "0: Unconfigured (reset condition)"]
    Unconfigured = 0,
    #[doc = "1: ACPI style Endpoint"]
    AcpiEnd = 1,
    #[doc = "2: ACPI style Index/Data"]
    AcpiIndex = 2,
    #[doc = "4: Bus Master Mem Single"]
    BusMMemS = 4,
    #[doc = "5: Bus Master Flash Single"]
    BusMFlashS = 5,
    #[doc = "8: Mailbox Shared"]
    MailboxShared = 8,
    #[doc = "9: Mailbox Single"]
    MailboxSingle = 9,
    #[doc = "10: Mailbox Split"]
    MailboxSplit = 10,
    #[doc = "11: Mailbox OOB Split"]
    MailboxOobSplit = 11,
    #[doc = "12: Mailbox OEM"]
    MailboxOem = 12,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `Type` reader - The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Type> {
        match self.bits {
            0 => Some(Type::Unconfigured),
            1 => Some(Type::AcpiEnd),
            2 => Some(Type::AcpiIndex),
            4 => Some(Type::BusMMemS),
            5 => Some(Type::BusMFlashS),
            8 => Some(Type::MailboxShared),
            9 => Some(Type::MailboxSingle),
            10 => Some(Type::MailboxSplit),
            11 => Some(Type::MailboxOobSplit),
            12 => Some(Type::MailboxOem),
            _ => None,
        }
    }
    #[doc = "Unconfigured (reset condition)"]
    #[inline(always)]
    pub fn is_unconfigured(&self) -> bool {
        *self == Type::Unconfigured
    }
    #[doc = "ACPI style Endpoint"]
    #[inline(always)]
    pub fn is_acpi_end(&self) -> bool {
        *self == Type::AcpiEnd
    }
    #[doc = "ACPI style Index/Data"]
    #[inline(always)]
    pub fn is_acpi_index(&self) -> bool {
        *self == Type::AcpiIndex
    }
    #[doc = "Bus Master Mem Single"]
    #[inline(always)]
    pub fn is_bus_m_mem_s(&self) -> bool {
        *self == Type::BusMMemS
    }
    #[doc = "Bus Master Flash Single"]
    #[inline(always)]
    pub fn is_bus_m_flash_s(&self) -> bool {
        *self == Type::BusMFlashS
    }
    #[doc = "Mailbox Shared"]
    #[inline(always)]
    pub fn is_mailbox_shared(&self) -> bool {
        *self == Type::MailboxShared
    }
    #[doc = "Mailbox Single"]
    #[inline(always)]
    pub fn is_mailbox_single(&self) -> bool {
        *self == Type::MailboxSingle
    }
    #[doc = "Mailbox Split"]
    #[inline(always)]
    pub fn is_mailbox_split(&self) -> bool {
        *self == Type::MailboxSplit
    }
    #[doc = "Mailbox OOB Split"]
    #[inline(always)]
    pub fn is_mailbox_oob_split(&self) -> bool {
        *self == Type::MailboxOobSplit
    }
    #[doc = "Mailbox OEM"]
    #[inline(always)]
    pub fn is_mailbox_oem(&self) -> bool {
        *self == Type::MailboxOem
    }
}
#[doc = "Field `Type` writer - The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Type>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unconfigured (reset condition)"]
    #[inline(always)]
    pub fn unconfigured(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Unconfigured)
    }
    #[doc = "ACPI style Endpoint"]
    #[inline(always)]
    pub fn acpi_end(self) -> &'a mut crate::W<REG> {
        self.variant(Type::AcpiEnd)
    }
    #[doc = "ACPI style Index/Data"]
    #[inline(always)]
    pub fn acpi_index(self) -> &'a mut crate::W<REG> {
        self.variant(Type::AcpiIndex)
    }
    #[doc = "Bus Master Mem Single"]
    #[inline(always)]
    pub fn bus_m_mem_s(self) -> &'a mut crate::W<REG> {
        self.variant(Type::BusMMemS)
    }
    #[doc = "Bus Master Flash Single"]
    #[inline(always)]
    pub fn bus_m_flash_s(self) -> &'a mut crate::W<REG> {
        self.variant(Type::BusMFlashS)
    }
    #[doc = "Mailbox Shared"]
    #[inline(always)]
    pub fn mailbox_shared(self) -> &'a mut crate::W<REG> {
        self.variant(Type::MailboxShared)
    }
    #[doc = "Mailbox Single"]
    #[inline(always)]
    pub fn mailbox_single(self) -> &'a mut crate::W<REG> {
        self.variant(Type::MailboxSingle)
    }
    #[doc = "Mailbox Split"]
    #[inline(always)]
    pub fn mailbox_split(self) -> &'a mut crate::W<REG> {
        self.variant(Type::MailboxSplit)
    }
    #[doc = "Mailbox OOB Split"]
    #[inline(always)]
    pub fn mailbox_oob_split(self) -> &'a mut crate::W<REG> {
        self.variant(Type::MailboxOobSplit)
    }
    #[doc = "Mailbox OEM"]
    #[inline(always)]
    pub fn mailbox_oem(self) -> &'a mut crate::W<REG> {
        self.variant(Type::MailboxOem)
    }
}
#[doc = "Field `Direction` reader - Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
pub type DirectionR = crate::FieldReader;
#[doc = "Field `Direction` writer - Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
pub type DirectionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBIntAll` reader - Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
pub type MbintAllR = crate::BitReader;
#[doc = "Field `MBIntAll` writer - Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
pub type MbintAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StallRd` reader - Stall on any Read of Index/Data and Mailbox (only)"]
pub type StallRdR = crate::BitReader;
#[doc = "Field `StallRd` writer - Stall on any Read of Index/Data and Mailbox (only)"]
pub type StallRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StallWr` reader - Stall on any Write of Index/Data and Mailbox"]
pub type StallWrR = crate::BitReader;
#[doc = "Field `StallWr` writer - Stall on any Write of Index/Data and Mailbox"]
pub type StallWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ErrorIgn` reader - If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
pub type ErrorIgnR = crate::BitReader;
#[doc = "Field `ErrorIgn` writer - If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
pub type ErrorIgnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    pub fn mbint_all(&self) -> MbintAllR {
        MbintAllR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    pub fn stall_rd(&self) -> StallRdR {
        StallRdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    pub fn stall_wr(&self) -> StallWrR {
        StallWrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    pub fn error_ign(&self) -> ErrorIgnR {
        ErrorIgnR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3CFG")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbint_all", &self.mbint_all())
            .field("stall_rd", &self.stall_rd())
            .field("stall_wr", &self.stall_wr())
            .field("error_ign", &self.error_ign())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<P3cfgSpec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    #[must_use]
    pub fn direction(&mut self) -> DirectionW<P3cfgSpec> {
        DirectionW::new(self, 5)
    }
    #[doc = "Bit 7 - Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    #[must_use]
    pub fn mbint_all(&mut self) -> MbintAllW<P3cfgSpec> {
        MbintAllW::new(self, 7)
    }
    #[doc = "Bit 8 - Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    #[must_use]
    pub fn stall_rd(&mut self) -> StallRdW<P3cfgSpec> {
        StallRdW::new(self, 8)
    }
    #[doc = "Bit 9 - Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn stall_wr(&mut self) -> StallWrW<P3cfgSpec> {
        StallWrW::new(self, 9)
    }
    #[doc = "Bit 10 - If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    #[must_use]
    pub fn error_ign(&mut self) -> ErrorIgnW<P3cfgSpec> {
        ErrorIgnW::new(self, 10)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p3cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3cfgSpec;
impl crate::RegisterSpec for P3cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p3cfg::R`](R) reader structure"]
impl crate::Readable for P3cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`p3cfg::W`](W) writer structure"]
impl crate::Writable for P3cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P3CFG to value 0"]
impl crate::Resettable for P3cfgSpec {
    const RESET_VALUE: u32 = 0;
}
