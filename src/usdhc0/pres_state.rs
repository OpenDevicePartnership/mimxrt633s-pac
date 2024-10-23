#[doc = "Register `PRES_STATE` reader"]
pub type R = crate::R<PresStateSpec>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cihb {
    #[doc = "0: Can issue command using only CMD line"]
    Cihb0 = 0,
    #[doc = "1: Cannot issue command"]
    Cihb1 = 1,
}
impl From<Cihb> for bool {
    #[inline(always)]
    fn from(variant: Cihb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIHB` reader - Command Inhibit (CMD)"]
pub type CihbR = crate::BitReader<Cihb>;
impl CihbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cihb {
        match self.bits {
            false => Cihb::Cihb0,
            true => Cihb::Cihb1,
        }
    }
    #[doc = "Can issue command using only CMD line"]
    #[inline(always)]
    pub fn is_cihb_0(&self) -> bool {
        *self == Cihb::Cihb0
    }
    #[doc = "Cannot issue command"]
    #[inline(always)]
    pub fn is_cihb_1(&self) -> bool {
        *self == Cihb::Cihb1
    }
}
#[doc = "Command Inhibit (DATA)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdihb {
    #[doc = "0: Can issue command which uses the DATA line"]
    Cdihb0 = 0,
    #[doc = "1: Cannot issue command which uses the DATA line"]
    Cdihb1 = 1,
}
impl From<Cdihb> for bool {
    #[inline(always)]
    fn from(variant: Cdihb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDIHB` reader - Command Inhibit (DATA)"]
pub type CdihbR = crate::BitReader<Cdihb>;
impl CdihbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdihb {
        match self.bits {
            false => Cdihb::Cdihb0,
            true => Cdihb::Cdihb1,
        }
    }
    #[doc = "Can issue command which uses the DATA line"]
    #[inline(always)]
    pub fn is_cdihb_0(&self) -> bool {
        *self == Cdihb::Cdihb0
    }
    #[doc = "Cannot issue command which uses the DATA line"]
    #[inline(always)]
    pub fn is_cdihb_1(&self) -> bool {
        *self == Cdihb::Cdihb1
    }
}
#[doc = "Data Line Active\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dla {
    #[doc = "0: DATA Line Inactive"]
    Dla0 = 0,
    #[doc = "1: DATA Line Active"]
    Dla1 = 1,
}
impl From<Dla> for bool {
    #[inline(always)]
    fn from(variant: Dla) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLA` reader - Data Line Active"]
pub type DlaR = crate::BitReader<Dla>;
impl DlaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dla {
        match self.bits {
            false => Dla::Dla0,
            true => Dla::Dla1,
        }
    }
    #[doc = "DATA Line Inactive"]
    #[inline(always)]
    pub fn is_dla_0(&self) -> bool {
        *self == Dla::Dla0
    }
    #[doc = "DATA Line Active"]
    #[inline(always)]
    pub fn is_dla_1(&self) -> bool {
        *self == Dla::Dla1
    }
}
#[doc = "SD Clock Stable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdstb {
    #[doc = "0: Clock is changing frequency and not stable."]
    Sdstb0 = 0,
    #[doc = "1: Clock is stable."]
    Sdstb1 = 1,
}
impl From<Sdstb> for bool {
    #[inline(always)]
    fn from(variant: Sdstb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDSTB` reader - SD Clock Stable"]
pub type SdstbR = crate::BitReader<Sdstb>;
impl SdstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdstb {
        match self.bits {
            false => Sdstb::Sdstb0,
            true => Sdstb::Sdstb1,
        }
    }
    #[doc = "Clock is changing frequency and not stable."]
    #[inline(always)]
    pub fn is_sdstb_0(&self) -> bool {
        *self == Sdstb::Sdstb0
    }
    #[doc = "Clock is stable."]
    #[inline(always)]
    pub fn is_sdstb_1(&self) -> bool {
        *self == Sdstb::Sdstb1
    }
}
#[doc = "IPG_CLK Gated Off Internally\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipgoff {
    #[doc = "0: IPG_CLK is active."]
    Ipgoff0 = 0,
    #[doc = "1: IPG_CLK is gated off."]
    Ipgoff1 = 1,
}
impl From<Ipgoff> for bool {
    #[inline(always)]
    fn from(variant: Ipgoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPGOFF` reader - IPG_CLK Gated Off Internally"]
pub type IpgoffR = crate::BitReader<Ipgoff>;
impl IpgoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipgoff {
        match self.bits {
            false => Ipgoff::Ipgoff0,
            true => Ipgoff::Ipgoff1,
        }
    }
    #[doc = "IPG_CLK is active."]
    #[inline(always)]
    pub fn is_ipgoff_0(&self) -> bool {
        *self == Ipgoff::Ipgoff0
    }
    #[doc = "IPG_CLK is gated off."]
    #[inline(always)]
    pub fn is_ipgoff_1(&self) -> bool {
        *self == Ipgoff::Ipgoff1
    }
}
#[doc = "HCLK Gated Off Internally\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hckoff {
    #[doc = "0: HCLK is active."]
    Hckoff0 = 0,
    #[doc = "1: HCLK is gated off."]
    Hckoff1 = 1,
}
impl From<Hckoff> for bool {
    #[inline(always)]
    fn from(variant: Hckoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCKOFF` reader - HCLK Gated Off Internally"]
pub type HckoffR = crate::BitReader<Hckoff>;
impl HckoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hckoff {
        match self.bits {
            false => Hckoff::Hckoff0,
            true => Hckoff::Hckoff1,
        }
    }
    #[doc = "HCLK is active."]
    #[inline(always)]
    pub fn is_hckoff_0(&self) -> bool {
        *self == Hckoff::Hckoff0
    }
    #[doc = "HCLK is gated off."]
    #[inline(always)]
    pub fn is_hckoff_1(&self) -> bool {
        *self == Hckoff::Hckoff1
    }
}
#[doc = "IPG_PERCLK Gated Off Internally\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peroff {
    #[doc = "0: IPG_PERCLK is active."]
    Peroff0 = 0,
    #[doc = "1: IPG_PERCLK is gated off."]
    Peroff1 = 1,
}
impl From<Peroff> for bool {
    #[inline(always)]
    fn from(variant: Peroff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEROFF` reader - IPG_PERCLK Gated Off Internally"]
pub type PeroffR = crate::BitReader<Peroff>;
impl PeroffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peroff {
        match self.bits {
            false => Peroff::Peroff0,
            true => Peroff::Peroff1,
        }
    }
    #[doc = "IPG_PERCLK is active."]
    #[inline(always)]
    pub fn is_peroff_0(&self) -> bool {
        *self == Peroff::Peroff0
    }
    #[doc = "IPG_PERCLK is gated off."]
    #[inline(always)]
    pub fn is_peroff_1(&self) -> bool {
        *self == Peroff::Peroff1
    }
}
#[doc = "SD Clock Gated Off Internally\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdoff {
    #[doc = "0: SD Clock is active."]
    Sdoff0 = 0,
    #[doc = "1: SD Clock is gated off."]
    Sdoff1 = 1,
}
impl From<Sdoff> for bool {
    #[inline(always)]
    fn from(variant: Sdoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDOFF` reader - SD Clock Gated Off Internally"]
pub type SdoffR = crate::BitReader<Sdoff>;
impl SdoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdoff {
        match self.bits {
            false => Sdoff::Sdoff0,
            true => Sdoff::Sdoff1,
        }
    }
    #[doc = "SD Clock is active."]
    #[inline(always)]
    pub fn is_sdoff_0(&self) -> bool {
        *self == Sdoff::Sdoff0
    }
    #[doc = "SD Clock is gated off."]
    #[inline(always)]
    pub fn is_sdoff_1(&self) -> bool {
        *self == Sdoff::Sdoff1
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wta {
    #[doc = "0: No valid data"]
    Wta0 = 0,
    #[doc = "1: Transferring data"]
    Wta1 = 1,
}
impl From<Wta> for bool {
    #[inline(always)]
    fn from(variant: Wta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTA` reader - Write Transfer Active"]
pub type WtaR = crate::BitReader<Wta>;
impl WtaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wta {
        match self.bits {
            false => Wta::Wta0,
            true => Wta::Wta1,
        }
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_wta_0(&self) -> bool {
        *self == Wta::Wta0
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_wta_1(&self) -> bool {
        *self == Wta::Wta1
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rta {
    #[doc = "0: No valid data"]
    Rta0 = 0,
    #[doc = "1: Transferring data"]
    Rta1 = 1,
}
impl From<Rta> for bool {
    #[inline(always)]
    fn from(variant: Rta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTA` reader - Read Transfer Active"]
pub type RtaR = crate::BitReader<Rta>;
impl RtaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rta {
        match self.bits {
            false => Rta::Rta0,
            true => Rta::Rta1,
        }
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_rta_0(&self) -> bool {
        *self == Rta::Rta0
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_rta_1(&self) -> bool {
        *self == Rta::Rta1
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwen {
    #[doc = "0: Write disable"]
    Bwen0 = 0,
    #[doc = "1: Write enable"]
    Bwen1 = 1,
}
impl From<Bwen> for bool {
    #[inline(always)]
    fn from(variant: Bwen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWEN` reader - Buffer Write Enable"]
pub type BwenR = crate::BitReader<Bwen>;
impl BwenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwen {
        match self.bits {
            false => Bwen::Bwen0,
            true => Bwen::Bwen1,
        }
    }
    #[doc = "Write disable"]
    #[inline(always)]
    pub fn is_bwen_0(&self) -> bool {
        *self == Bwen::Bwen0
    }
    #[doc = "Write enable"]
    #[inline(always)]
    pub fn is_bwen_1(&self) -> bool {
        *self == Bwen::Bwen1
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bren {
    #[doc = "0: Read disable"]
    Bren0 = 0,
    #[doc = "1: Read enable"]
    Bren1 = 1,
}
impl From<Bren> for bool {
    #[inline(always)]
    fn from(variant: Bren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREN` reader - Buffer Read Enable"]
pub type BrenR = crate::BitReader<Bren>;
impl BrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bren {
        match self.bits {
            false => Bren::Bren0,
            true => Bren::Bren1,
        }
    }
    #[doc = "Read disable"]
    #[inline(always)]
    pub fn is_bren_0(&self) -> bool {
        *self == Bren::Bren0
    }
    #[doc = "Read enable"]
    #[inline(always)]
    pub fn is_bren_1(&self) -> bool {
        *self == Bren::Bren1
    }
}
#[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtr {
    #[doc = "0: Fixed or well tuned sampling clock"]
    Rtr0 = 0,
    #[doc = "1: Sampling clock needs re-tuning"]
    Rtr1 = 1,
}
impl From<Rtr> for bool {
    #[inline(always)]
    fn from(variant: Rtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type RtrR = crate::BitReader<Rtr>;
impl RtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtr {
        match self.bits {
            false => Rtr::Rtr0,
            true => Rtr::Rtr1,
        }
    }
    #[doc = "Fixed or well tuned sampling clock"]
    #[inline(always)]
    pub fn is_rtr_0(&self) -> bool {
        *self == Rtr::Rtr0
    }
    #[doc = "Sampling clock needs re-tuning"]
    #[inline(always)]
    pub fn is_rtr_1(&self) -> bool {
        *self == Rtr::Rtr1
    }
}
#[doc = "Tape Select Change Done\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tscd {
    #[doc = "0: Delay cell select change is not finished."]
    Tscd0 = 0,
    #[doc = "1: Delay cell select change is finished."]
    Tscd1 = 1,
}
impl From<Tscd> for bool {
    #[inline(always)]
    fn from(variant: Tscd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCD` reader - Tape Select Change Done"]
pub type TscdR = crate::BitReader<Tscd>;
impl TscdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tscd {
        match self.bits {
            false => Tscd::Tscd0,
            true => Tscd::Tscd1,
        }
    }
    #[doc = "Delay cell select change is not finished."]
    #[inline(always)]
    pub fn is_tscd_0(&self) -> bool {
        *self == Tscd::Tscd0
    }
    #[doc = "Delay cell select change is finished."]
    #[inline(always)]
    pub fn is_tscd_1(&self) -> bool {
        *self == Tscd::Tscd1
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cinst {
    #[doc = "0: Power on Reset or No Card"]
    Cinst0 = 0,
    #[doc = "1: Card Inserted"]
    Cinst1 = 1,
}
impl From<Cinst> for bool {
    #[inline(always)]
    fn from(variant: Cinst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINST` reader - Card Inserted"]
pub type CinstR = crate::BitReader<Cinst>;
impl CinstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinst {
        match self.bits {
            false => Cinst::Cinst0,
            true => Cinst::Cinst1,
        }
    }
    #[doc = "Power on Reset or No Card"]
    #[inline(always)]
    pub fn is_cinst_0(&self) -> bool {
        *self == Cinst::Cinst0
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_cinst_1(&self) -> bool {
        *self == Cinst::Cinst1
    }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdpl {
    #[doc = "0: No card present (CD_B = 1)"]
    Cdpl0 = 0,
    #[doc = "1: Card present (CD_B = 0)"]
    Cdpl1 = 1,
}
impl From<Cdpl> for bool {
    #[inline(always)]
    fn from(variant: Cdpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDPL` reader - Card Detect Pin Level"]
pub type CdplR = crate::BitReader<Cdpl>;
impl CdplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdpl {
        match self.bits {
            false => Cdpl::Cdpl0,
            true => Cdpl::Cdpl1,
        }
    }
    #[doc = "No card present (CD_B = 1)"]
    #[inline(always)]
    pub fn is_cdpl_0(&self) -> bool {
        *self == Cdpl::Cdpl0
    }
    #[doc = "Card present (CD_B = 0)"]
    #[inline(always)]
    pub fn is_cdpl_1(&self) -> bool {
        *self == Cdpl::Cdpl1
    }
}
#[doc = "Write Protect Switch Pin Level\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpspl {
    #[doc = "0: Write protected (WP = 1)"]
    Wpspl0 = 0,
    #[doc = "1: Write enabled (WP = 0)"]
    Wpspl1 = 1,
}
impl From<Wpspl> for bool {
    #[inline(always)]
    fn from(variant: Wpspl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSPL` reader - Write Protect Switch Pin Level"]
pub type WpsplR = crate::BitReader<Wpspl>;
impl WpsplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpspl {
        match self.bits {
            false => Wpspl::Wpspl0,
            true => Wpspl::Wpspl1,
        }
    }
    #[doc = "Write protected (WP = 1)"]
    #[inline(always)]
    pub fn is_wpspl_0(&self) -> bool {
        *self == Wpspl::Wpspl0
    }
    #[doc = "Write enabled (WP = 0)"]
    #[inline(always)]
    pub fn is_wpspl_1(&self) -> bool {
        *self == Wpspl::Wpspl1
    }
}
#[doc = "Field `CLSL` reader - CMD Line Signal Level"]
pub type ClslR = crate::BitReader;
#[doc = "DATA\\[7:0\\]
Line Signal Level\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dlsl {
    #[doc = "0: Data 0 line signal level"]
    Data0 = 0,
    #[doc = "1: Data 1 line signal level"]
    Data1 = 1,
    #[doc = "2: Data 2 line signal level"]
    Data2 = 2,
    #[doc = "3: Data 3 line signal level"]
    Data3 = 3,
    #[doc = "4: Data 4 line signal level"]
    Data4 = 4,
    #[doc = "5: Data 5 line signal level"]
    Data5 = 5,
    #[doc = "6: Data 6 line signal level"]
    Data6 = 6,
    #[doc = "7: Data 7 line signal level"]
    Data7 = 7,
}
impl From<Dlsl> for u8 {
    #[inline(always)]
    fn from(variant: Dlsl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dlsl {
    type Ux = u8;
}
impl crate::IsEnum for Dlsl {}
#[doc = "Field `DLSL` reader - DATA\\[7:0\\]
Line Signal Level"]
pub type DlslR = crate::FieldReader<Dlsl>;
impl DlslR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dlsl> {
        match self.bits {
            0 => Some(Dlsl::Data0),
            1 => Some(Dlsl::Data1),
            2 => Some(Dlsl::Data2),
            3 => Some(Dlsl::Data3),
            4 => Some(Dlsl::Data4),
            5 => Some(Dlsl::Data5),
            6 => Some(Dlsl::Data6),
            7 => Some(Dlsl::Data7),
            _ => None,
        }
    }
    #[doc = "Data 0 line signal level"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Dlsl::Data0
    }
    #[doc = "Data 1 line signal level"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Dlsl::Data1
    }
    #[doc = "Data 2 line signal level"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == Dlsl::Data2
    }
    #[doc = "Data 3 line signal level"]
    #[inline(always)]
    pub fn is_data3(&self) -> bool {
        *self == Dlsl::Data3
    }
    #[doc = "Data 4 line signal level"]
    #[inline(always)]
    pub fn is_data4(&self) -> bool {
        *self == Dlsl::Data4
    }
    #[doc = "Data 5 line signal level"]
    #[inline(always)]
    pub fn is_data5(&self) -> bool {
        *self == Dlsl::Data5
    }
    #[doc = "Data 6 line signal level"]
    #[inline(always)]
    pub fn is_data6(&self) -> bool {
        *self == Dlsl::Data6
    }
    #[doc = "Data 7 line signal level"]
    #[inline(always)]
    pub fn is_data7(&self) -> bool {
        *self == Dlsl::Data7
    }
}
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cihb(&self) -> CihbR {
        CihbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DATA)"]
    #[inline(always)]
    pub fn cdihb(&self) -> CdihbR {
        CdihbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Line Active"]
    #[inline(always)]
    pub fn dla(&self) -> DlaR {
        DlaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD Clock Stable"]
    #[inline(always)]
    pub fn sdstb(&self) -> SdstbR {
        SdstbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IPG_CLK Gated Off Internally"]
    #[inline(always)]
    pub fn ipgoff(&self) -> IpgoffR {
        IpgoffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HCLK Gated Off Internally"]
    #[inline(always)]
    pub fn hckoff(&self) -> HckoffR {
        HckoffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IPG_PERCLK Gated Off Internally"]
    #[inline(always)]
    pub fn peroff(&self) -> PeroffR {
        PeroffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD Clock Gated Off Internally"]
    #[inline(always)]
    pub fn sdoff(&self) -> SdoffR {
        SdoffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wta(&self) -> WtaR {
        WtaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rta(&self) -> RtaR {
        RtaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bwen(&self) -> BwenR {
        BwenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bren(&self) -> BrenR {
        BrenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Tape Select Change Done"]
    #[inline(always)]
    pub fn tscd(&self) -> TscdR {
        TscdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cinst(&self) -> CinstR {
        CinstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn cdpl(&self) -> CdplR {
        CdplR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn wpspl(&self) -> WpsplR {
        WpsplR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - CMD Line Signal Level"]
    #[inline(always)]
    pub fn clsl(&self) -> ClslR {
        ClslR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - DATA\\[7:0\\]
Line Signal Level"]
    #[inline(always)]
    pub fn dlsl(&self) -> DlslR {
        DlslR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRES_STATE")
            .field("cihb", &self.cihb())
            .field("cdihb", &self.cdihb())
            .field("dla", &self.dla())
            .field("sdstb", &self.sdstb())
            .field("ipgoff", &self.ipgoff())
            .field("hckoff", &self.hckoff())
            .field("peroff", &self.peroff())
            .field("sdoff", &self.sdoff())
            .field("wta", &self.wta())
            .field("rta", &self.rta())
            .field("bwen", &self.bwen())
            .field("bren", &self.bren())
            .field("rtr", &self.rtr())
            .field("tscd", &self.tscd())
            .field("cinst", &self.cinst())
            .field("cdpl", &self.cdpl())
            .field("wpspl", &self.wpspl())
            .field("clsl", &self.clsl())
            .field("dlsl", &self.dlsl())
            .finish()
    }
}
#[doc = "Present State\n\nYou can [`read`](crate::Reg::read) this register and get [`pres_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresStateSpec;
impl crate::RegisterSpec for PresStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pres_state::R`](R) reader structure"]
impl crate::Readable for PresStateSpec {}
#[doc = "`reset()` method sets PRES_STATE to value 0x8080"]
impl crate::Resettable for PresStateSpec {
    const RESET_VALUE: u32 = 0x8080;
}
