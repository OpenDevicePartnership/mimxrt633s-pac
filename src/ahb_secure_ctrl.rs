#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    rom_mem_rule: [RomMemRule; 4],
    _reserved1: [u8; 0x10],
    flexspi0_region0_rule: [Flexspi0Region0Rule; 4],
    flexspi0_region1_rule0: Flexspi0Region1Rule0,
    _reserved3: [u8; 0x0c],
    flexspi0_region2_rule0: Flexspi0Region2Rule0,
    _reserved4: [u8; 0x0c],
    flexspi0_region3_rule0: Flexspi0Region3Rule0,
    _reserved5: [u8; 0x0c],
    flexspi0_region4_rule0: Flexspi0Region4Rule0,
    _reserved6: [u8; 0x1c],
    ram00_rule: [Ram00Rule; 4],
    ram01_rule: [Ram01Rule; 4],
    _reserved8: [u8; 0x10],
    ram02_rule: [Ram02Rule; 4],
    ram03_rule: [Ram03Rule; 4],
    _reserved10: [u8; 0x10],
    ram04_rule: [Ram04Rule; 4],
    ram05_rule: [Ram05Rule; 4],
    ram06_rule: [Ram06Rule; 4],
    ram07_rule: [Ram07Rule; 4],
    _reserved14: [u8; 0x10],
    ram08_rule: [Ram08Rule; 4],
    ram09_rule: [Ram09Rule; 4],
    ram10_rule: [Ram10Rule; 4],
    ram11_rule: [Ram11Rule; 4],
    _reserved18: [u8; 0x10],
    ram12_rule: [Ram12Rule; 4],
    ram13_rule: [Ram13Rule; 4],
    ram14_rule: [Ram14Rule; 4],
    ram15_rule: [Ram15Rule; 4],
    _reserved22: [u8; 0x10],
    ram16_rule: [Ram16Rule; 4],
    ram17_rule: [Ram17Rule; 4],
    ram18_rule: [Ram18Rule; 4],
    ram19_rule: [Ram19Rule; 4],
    _reserved26: [u8; 0x10],
    ram20_rule: [Ram20Rule; 4],
    ram21_rule: [Ram21Rule; 4],
    ram22_rule: [Ram22Rule; 4],
    ram23_rule: [Ram23Rule; 4],
    _reserved30: [u8; 0x10],
    ram24_rule: [Ram24Rule; 4],
    ram25_rule: [Ram25Rule; 4],
    ram26_rule: [Ram26Rule; 4],
    ram27_rule: [Ram27Rule; 4],
    _reserved34: [u8; 0x10],
    ram28_rule: [Ram28Rule; 4],
    ram29_rule: [Ram29Rule; 4],
    _reserved36: [u8; 0x30],
    pif_hifi4_x_mem_rule0: PifHifi4XMemRule0,
    _reserved37: [u8; 0x1c],
    apb_grp0_mem_rule0: ApbGrp0MemRule0,
    apb_grp0_mem_rule1: ApbGrp0MemRule1,
    _reserved39: [u8; 0x08],
    apb_grp1_mem_rule0: ApbGrp1MemRule0,
    apb_grp1_mem_rule1: ApbGrp1MemRule1,
    apb_grp1_mem_rule2: ApbGrp1MemRule2,
    _reserved42: [u8; 0x04],
    ahb_periph0_slave_rule0: AhbPeriph0SlaveRule0,
    _reserved43: [u8; 0x0c],
    aips_bridge0_mem_rule0: AipsBridge0MemRule0,
    _reserved44: [u8; 0x0c],
    ahb_periph1_slave_rule0: AhbPeriph1SlaveRule0,
    _reserved45: [u8; 0x1c],
    aips_bridge1_mem_rule0: AipsBridge1MemRule0,
    aips_bridge1_mem_rule1: AipsBridge1MemRule1,
    _reserved47: [u8; 0x08],
    ahb_periph2_slave_rule0: AhbPeriph2SlaveRule0,
    _reserved48: [u8; 0x0c],
    security_ctrl_mem_rule0: SecurityCtrlMemRule0,
    _reserved49: [u8; 0x0c],
    ahb_periph3_slave_rule0: AhbPeriph3SlaveRule0,
    _reserved50: [u8; 0x0a2c],
    sec_vio_addr: [SecVioAddr; 18],
    _reserved51: [u8; 0x38],
    sec_vio_misc_info: [SecVioMiscInfo; 18],
    _reserved52: [u8; 0x38],
    sec_vio_info_valid: SecVioInfoValid,
    _reserved53: [u8; 0x7c],
    sec_gpio_mask0: SecGpioMask0,
    sec_gpio_mask1: SecGpioMask1,
    sec_gpio_mask2: SecGpioMask2,
    sec_gpio_mask3: SecGpioMask3,
    sec_gpio_mask4: SecGpioMask4,
    sec_gpio_mask5: SecGpioMask5,
    sec_gpio_mask6: SecGpioMask6,
    sec_gpio_mask7: SecGpioMask7,
    sec_dsp_int_mask: SecDspIntMask,
    _reserved62: [u8; 0x18],
    sec_mask_lock: SecMaskLock,
    _reserved63: [u8; 0x10],
    master_sec_level: MasterSecLevel,
    master_sec_level_anti_pol: MasterSecLevelAntiPol,
    _reserved65: [u8; 0x14],
    cm33_lock_reg: Cm33LockReg,
    _reserved66: [u8; 0x08],
    misc_ctrl_dp_reg: MiscCtrlDpReg,
    misc_ctrl_reg: MiscCtrlReg,
}
impl RegisterBlock {
    #[doc = "0x10..0x20 - Memory ROM Rule(n) Register"]
    #[inline(always)]
    pub const fn rom_mem_rule(&self, n: usize) -> &RomMemRule {
        &self.rom_mem_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Memory ROM Rule(n) Register"]
    #[inline(always)]
    pub fn rom_mem_rule_iter(&self) -> impl Iterator<Item = &RomMemRule> {
        self.rom_mem_rule.iter()
    }
    #[doc = "0x30..0x40 - FLEXSPI0 Region 0 Rule(n) Register"]
    #[inline(always)]
    pub const fn flexspi0_region0_rule(&self, n: usize) -> &Flexspi0Region0Rule {
        &self.flexspi0_region0_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - FLEXSPI0 Region 0 Rule(n) Register"]
    #[inline(always)]
    pub fn flexspi0_region0_rule_iter(&self) -> impl Iterator<Item = &Flexspi0Region0Rule> {
        self.flexspi0_region0_rule.iter()
    }
    #[doc = "0x40 - FLEXSPI0 Region 1 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region1_rule0(&self) -> &Flexspi0Region1Rule0 {
        &self.flexspi0_region1_rule0
    }
    #[doc = "0x50 - FLEXSPI0 Region 2 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region2_rule0(&self) -> &Flexspi0Region2Rule0 {
        &self.flexspi0_region2_rule0
    }
    #[doc = "0x60 - FLEXSPI0 Region 3 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region3_rule0(&self) -> &Flexspi0Region3Rule0 {
        &self.flexspi0_region3_rule0
    }
    #[doc = "0x70 - FLEXSPI0 Region 4 Rule 0 Register"]
    #[inline(always)]
    pub const fn flexspi0_region4_rule0(&self) -> &Flexspi0Region4Rule0 {
        &self.flexspi0_region4_rule0
    }
    #[doc = "0x90..0xa0 - SRAM Partition 00 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram00_rule(&self, n: usize) -> &Ram00Rule {
        &self.ram00_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xa0 - SRAM Partition 00 Rule(n) Register"]
    #[inline(always)]
    pub fn ram00_rule_iter(&self) -> impl Iterator<Item = &Ram00Rule> {
        self.ram00_rule.iter()
    }
    #[doc = "0xa0..0xb0 - SRAM Partition 01 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram01_rule(&self, n: usize) -> &Ram01Rule {
        &self.ram01_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xb0 - SRAM Partition 01 Rule(n) Register"]
    #[inline(always)]
    pub fn ram01_rule_iter(&self) -> impl Iterator<Item = &Ram01Rule> {
        self.ram01_rule.iter()
    }
    #[doc = "0xc0..0xd0 - SRAM Partition 02 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram02_rule(&self, n: usize) -> &Ram02Rule {
        &self.ram02_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xd0 - SRAM Partition 02 Rule(n) Register"]
    #[inline(always)]
    pub fn ram02_rule_iter(&self) -> impl Iterator<Item = &Ram02Rule> {
        self.ram02_rule.iter()
    }
    #[doc = "0xd0..0xe0 - SRAM Partition 03 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram03_rule(&self, n: usize) -> &Ram03Rule {
        &self.ram03_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd0..0xe0 - SRAM Partition 03 Rule(n) Register"]
    #[inline(always)]
    pub fn ram03_rule_iter(&self) -> impl Iterator<Item = &Ram03Rule> {
        self.ram03_rule.iter()
    }
    #[doc = "0xf0..0x100 - SRAM Partition 04 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram04_rule(&self, n: usize) -> &Ram04Rule {
        &self.ram04_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0x100 - SRAM Partition 04 Rule(n) Register"]
    #[inline(always)]
    pub fn ram04_rule_iter(&self) -> impl Iterator<Item = &Ram04Rule> {
        self.ram04_rule.iter()
    }
    #[doc = "0x100..0x110 - SRAM Partition 05 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram05_rule(&self, n: usize) -> &Ram05Rule {
        &self.ram05_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - SRAM Partition 05 Rule(n) Register"]
    #[inline(always)]
    pub fn ram05_rule_iter(&self) -> impl Iterator<Item = &Ram05Rule> {
        self.ram05_rule.iter()
    }
    #[doc = "0x110..0x120 - SRAM Partition 06 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram06_rule(&self, n: usize) -> &Ram06Rule {
        &self.ram06_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - SRAM Partition 06 Rule(n) Register"]
    #[inline(always)]
    pub fn ram06_rule_iter(&self) -> impl Iterator<Item = &Ram06Rule> {
        self.ram06_rule.iter()
    }
    #[doc = "0x120..0x130 - SRAM Partition 07 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram07_rule(&self, n: usize) -> &Ram07Rule {
        &self.ram07_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - SRAM Partition 07 Rule(n) Register"]
    #[inline(always)]
    pub fn ram07_rule_iter(&self) -> impl Iterator<Item = &Ram07Rule> {
        self.ram07_rule.iter()
    }
    #[doc = "0x140..0x150 - SRAM Partition 08 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram08_rule(&self, n: usize) -> &Ram08Rule {
        &self.ram08_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - SRAM Partition 08 Rule(n) Register"]
    #[inline(always)]
    pub fn ram08_rule_iter(&self) -> impl Iterator<Item = &Ram08Rule> {
        self.ram08_rule.iter()
    }
    #[doc = "0x150..0x160 - SRAM Partition 09 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram09_rule(&self, n: usize) -> &Ram09Rule {
        &self.ram09_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - SRAM Partition 09 Rule(n) Register"]
    #[inline(always)]
    pub fn ram09_rule_iter(&self) -> impl Iterator<Item = &Ram09Rule> {
        self.ram09_rule.iter()
    }
    #[doc = "0x160..0x170 - SRAM Partition 10 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram10_rule(&self, n: usize) -> &Ram10Rule {
        &self.ram10_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x170 - SRAM Partition 10 Rule(n) Register"]
    #[inline(always)]
    pub fn ram10_rule_iter(&self) -> impl Iterator<Item = &Ram10Rule> {
        self.ram10_rule.iter()
    }
    #[doc = "0x170..0x180 - SRAM Partition 11 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram11_rule(&self, n: usize) -> &Ram11Rule {
        &self.ram11_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x170..0x180 - SRAM Partition 11 Rule(n) Register"]
    #[inline(always)]
    pub fn ram11_rule_iter(&self) -> impl Iterator<Item = &Ram11Rule> {
        self.ram11_rule.iter()
    }
    #[doc = "0x190..0x1a0 - SRAM Partition 12 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram12_rule(&self, n: usize) -> &Ram12Rule {
        &self.ram12_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1a0 - SRAM Partition 12 Rule(n) Register"]
    #[inline(always)]
    pub fn ram12_rule_iter(&self) -> impl Iterator<Item = &Ram12Rule> {
        self.ram12_rule.iter()
    }
    #[doc = "0x1a0..0x1b0 - SRAM Partition 13 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram13_rule(&self, n: usize) -> &Ram13Rule {
        &self.ram13_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1b0 - SRAM Partition 13 Rule(n) Register"]
    #[inline(always)]
    pub fn ram13_rule_iter(&self) -> impl Iterator<Item = &Ram13Rule> {
        self.ram13_rule.iter()
    }
    #[doc = "0x1b0..0x1c0 - SRAM Partition 14 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram14_rule(&self, n: usize) -> &Ram14Rule {
        &self.ram14_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1c0 - SRAM Partition 14 Rule(n) Register"]
    #[inline(always)]
    pub fn ram14_rule_iter(&self) -> impl Iterator<Item = &Ram14Rule> {
        self.ram14_rule.iter()
    }
    #[doc = "0x1c0..0x1d0 - SRAM Partition 15 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram15_rule(&self, n: usize) -> &Ram15Rule {
        &self.ram15_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1d0 - SRAM Partition 15 Rule(n) Register"]
    #[inline(always)]
    pub fn ram15_rule_iter(&self) -> impl Iterator<Item = &Ram15Rule> {
        self.ram15_rule.iter()
    }
    #[doc = "0x1e0..0x1f0 - SRAM Partition 16 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram16_rule(&self, n: usize) -> &Ram16Rule {
        &self.ram16_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1e0..0x1f0 - SRAM Partition 16 Rule(n) Register"]
    #[inline(always)]
    pub fn ram16_rule_iter(&self) -> impl Iterator<Item = &Ram16Rule> {
        self.ram16_rule.iter()
    }
    #[doc = "0x1f0..0x200 - SRAM Partition 17 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram17_rule(&self, n: usize) -> &Ram17Rule {
        &self.ram17_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x200 - SRAM Partition 17 Rule(n) Register"]
    #[inline(always)]
    pub fn ram17_rule_iter(&self) -> impl Iterator<Item = &Ram17Rule> {
        self.ram17_rule.iter()
    }
    #[doc = "0x200..0x210 - SRAM Partition 18 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram18_rule(&self, n: usize) -> &Ram18Rule {
        &self.ram18_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x210 - SRAM Partition 18 Rule(n) Register"]
    #[inline(always)]
    pub fn ram18_rule_iter(&self) -> impl Iterator<Item = &Ram18Rule> {
        self.ram18_rule.iter()
    }
    #[doc = "0x210..0x220 - SRAM Partition 19 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram19_rule(&self, n: usize) -> &Ram19Rule {
        &self.ram19_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x210..0x220 - SRAM Partition 19 Rule(n) Register"]
    #[inline(always)]
    pub fn ram19_rule_iter(&self) -> impl Iterator<Item = &Ram19Rule> {
        self.ram19_rule.iter()
    }
    #[doc = "0x230..0x240 - SRAM Partition 20 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram20_rule(&self, n: usize) -> &Ram20Rule {
        &self.ram20_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x230..0x240 - SRAM Partition 20 Rule(n) Register"]
    #[inline(always)]
    pub fn ram20_rule_iter(&self) -> impl Iterator<Item = &Ram20Rule> {
        self.ram20_rule.iter()
    }
    #[doc = "0x240..0x250 - SRAM Partition 21 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram21_rule(&self, n: usize) -> &Ram21Rule {
        &self.ram21_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x250 - SRAM Partition 21 Rule(n) Register"]
    #[inline(always)]
    pub fn ram21_rule_iter(&self) -> impl Iterator<Item = &Ram21Rule> {
        self.ram21_rule.iter()
    }
    #[doc = "0x250..0x260 - SRAM Partition 22 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram22_rule(&self, n: usize) -> &Ram22Rule {
        &self.ram22_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x250..0x260 - SRAM Partition 22 Rule(n) Register"]
    #[inline(always)]
    pub fn ram22_rule_iter(&self) -> impl Iterator<Item = &Ram22Rule> {
        self.ram22_rule.iter()
    }
    #[doc = "0x260..0x270 - SRAM Partition 23 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram23_rule(&self, n: usize) -> &Ram23Rule {
        &self.ram23_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x260..0x270 - SRAM Partition 23 Rule(n) Register"]
    #[inline(always)]
    pub fn ram23_rule_iter(&self) -> impl Iterator<Item = &Ram23Rule> {
        self.ram23_rule.iter()
    }
    #[doc = "0x280..0x290 - SRAM Partition 24 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram24_rule(&self, n: usize) -> &Ram24Rule {
        &self.ram24_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x290 - SRAM Partition 24 Rule(n) Register"]
    #[inline(always)]
    pub fn ram24_rule_iter(&self) -> impl Iterator<Item = &Ram24Rule> {
        self.ram24_rule.iter()
    }
    #[doc = "0x290..0x2a0 - SRAM Partition 25 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram25_rule(&self, n: usize) -> &Ram25Rule {
        &self.ram25_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x290..0x2a0 - SRAM Partition 25 Rule(n) Register"]
    #[inline(always)]
    pub fn ram25_rule_iter(&self) -> impl Iterator<Item = &Ram25Rule> {
        self.ram25_rule.iter()
    }
    #[doc = "0x2a0..0x2b0 - SRAM Partition 26 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram26_rule(&self, n: usize) -> &Ram26Rule {
        &self.ram26_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2a0..0x2b0 - SRAM Partition 26 Rule(n) Register"]
    #[inline(always)]
    pub fn ram26_rule_iter(&self) -> impl Iterator<Item = &Ram26Rule> {
        self.ram26_rule.iter()
    }
    #[doc = "0x2b0..0x2c0 - SRAM Partition 27 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram27_rule(&self, n: usize) -> &Ram27Rule {
        &self.ram27_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2b0..0x2c0 - SRAM Partition 27 Rule(n) Register"]
    #[inline(always)]
    pub fn ram27_rule_iter(&self) -> impl Iterator<Item = &Ram27Rule> {
        self.ram27_rule.iter()
    }
    #[doc = "0x2d0..0x2e0 - SRAM Partition 28 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram28_rule(&self, n: usize) -> &Ram28Rule {
        &self.ram28_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d0..0x2e0 - SRAM Partition 28 Rule(n) Register"]
    #[inline(always)]
    pub fn ram28_rule_iter(&self) -> impl Iterator<Item = &Ram28Rule> {
        self.ram28_rule.iter()
    }
    #[doc = "0x2e0..0x2f0 - SRAM Partition 29 Rule(n) Register"]
    #[inline(always)]
    pub const fn ram29_rule(&self, n: usize) -> &Ram29Rule {
        &self.ram29_rule[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2e0..0x2f0 - SRAM Partition 29 Rule(n) Register"]
    #[inline(always)]
    pub fn ram29_rule_iter(&self) -> impl Iterator<Item = &Ram29Rule> {
        self.ram29_rule.iter()
    }
    #[doc = "0x320 - Security access rules for HiFi 4 memory sectors (0x24000000--0x240FFFFF). Each sector is 32 Kbytes, there're 4 sectors in total."]
    #[inline(always)]
    pub const fn pif_hifi4_x_mem_rule0(&self) -> &PifHifi4XMemRule0 {
        &self.pif_hifi4_x_mem_rule0
    }
    #[doc = "0x340 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp0_mem_rule0(&self) -> &ApbGrp0MemRule0 {
        &self.apb_grp0_mem_rule0
    }
    #[doc = "0x344 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp0_mem_rule1(&self) -> &ApbGrp0MemRule1 {
        &self.apb_grp0_mem_rule1
    }
    #[doc = "0x350 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp1_mem_rule0(&self) -> &ApbGrp1MemRule0 {
        &self.apb_grp1_mem_rule0
    }
    #[doc = "0x354 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp1_mem_rule1(&self) -> &ApbGrp1MemRule1 {
        &self.apb_grp1_mem_rule1
    }
    #[doc = "0x358 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn apb_grp1_mem_rule2(&self) -> &ApbGrp1MemRule2 {
        &self.apb_grp1_mem_rule2
    }
    #[doc = "0x360 - Security access rules for AHB peripheral slaves area 0x40100000--0x4010FFFF"]
    #[inline(always)]
    pub const fn ahb_periph0_slave_rule0(&self) -> &AhbPeriph0SlaveRule0 {
        &self.ahb_periph0_slave_rule0
    }
    #[doc = "0x370 - 0x40110000--0x4011FFFF"]
    #[inline(always)]
    pub const fn aips_bridge0_mem_rule0(&self) -> &AipsBridge0MemRule0 {
        &self.aips_bridge0_mem_rule0
    }
    #[doc = "0x380 - the memory map is 0x40120000--0x40127FFF"]
    #[inline(always)]
    pub const fn ahb_periph1_slave_rule0(&self) -> &AhbPeriph1SlaveRule0 {
        &self.ahb_periph1_slave_rule0
    }
    #[doc = "0x3a0 - Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn aips_bridge1_mem_rule0(&self) -> &AipsBridge1MemRule0 {
        &self.aips_bridge1_mem_rule0
    }
    #[doc = "0x3a4 - Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
    #[inline(always)]
    pub const fn aips_bridge1_mem_rule1(&self) -> &AipsBridge1MemRule1 {
        &self.aips_bridge1_mem_rule1
    }
    #[doc = "0x3b0 - Security access rules for AHB peripheral slaves area 0x40140000--0x4014BFFF"]
    #[inline(always)]
    pub const fn ahb_periph2_slave_rule0(&self) -> &AhbPeriph2SlaveRule0 {
        &self.ahb_periph2_slave_rule0
    }
    #[doc = "0x3c0 - 0x40148000--0x4014BFFF"]
    #[inline(always)]
    pub const fn security_ctrl_mem_rule0(&self) -> &SecurityCtrlMemRule0 {
        &self.security_ctrl_mem_rule0
    }
    #[doc = "0x3d0 - Security access rules for AHB peripheral slaves area 0x40150000--0x40158FFF"]
    #[inline(always)]
    pub const fn ahb_periph3_slave_rule0(&self) -> &AhbPeriph3SlaveRule0 {
        &self.ahb_periph3_slave_rule0
    }
    #[doc = "0xe00..0xe48 - most recent security violation address for AHB layer n"]
    #[inline(always)]
    pub const fn sec_vio_addr(&self, n: usize) -> &SecVioAddr {
        &self.sec_vio_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe00..0xe48 - most recent security violation address for AHB layer n"]
    #[inline(always)]
    pub fn sec_vio_addr_iter(&self) -> impl Iterator<Item = &SecVioAddr> {
        self.sec_vio_addr.iter()
    }
    #[doc = "0xe80..0xec8 - most recent security violation miscellaneous information for AHB layer n"]
    #[inline(always)]
    pub const fn sec_vio_misc_info(&self, n: usize) -> &SecVioMiscInfo {
        &self.sec_vio_misc_info[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe80..0xec8 - most recent security violation miscellaneous information for AHB layer n"]
    #[inline(always)]
    pub fn sec_vio_misc_info_iter(&self) -> impl Iterator<Item = &SecVioMiscInfo> {
        self.sec_vio_misc_info.iter()
    }
    #[doc = "0xf00 - security violation address/information registers valid flags"]
    #[inline(always)]
    pub const fn sec_vio_info_valid(&self) -> &SecVioInfoValid {
        &self.sec_vio_info_valid
    }
    #[doc = "0xf80 - Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
    #[inline(always)]
    pub const fn sec_gpio_mask0(&self) -> &SecGpioMask0 {
        &self.sec_gpio_mask0
    }
    #[doc = "0xf84 - Secure GPIO mask for port 1 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask1(&self) -> &SecGpioMask1 {
        &self.sec_gpio_mask1
    }
    #[doc = "0xf88 - Secure GPIO mask for port 2 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask2(&self) -> &SecGpioMask2 {
        &self.sec_gpio_mask2
    }
    #[doc = "0xf8c - Secure GPIO mask for port 3 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask3(&self) -> &SecGpioMask3 {
        &self.sec_gpio_mask3
    }
    #[doc = "0xf90 - Secure GPIO mask for port 4 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask4(&self) -> &SecGpioMask4 {
        &self.sec_gpio_mask4
    }
    #[doc = "0xf94 - Secure GPIO mask for port 5 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask5(&self) -> &SecGpioMask5 {
        &self.sec_gpio_mask5
    }
    #[doc = "0xf98 - Secure GPIO mask for port 6 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask6(&self) -> &SecGpioMask6 {
        &self.sec_gpio_mask6
    }
    #[doc = "0xf9c - Secure GPIO mask for port 7 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask7(&self) -> &SecGpioMask7 {
        &self.sec_gpio_mask7
    }
    #[doc = "0xfa0 - secure general purpose register 8 used to mask interrupts to DSP for security purpose"]
    #[inline(always)]
    pub const fn sec_dsp_int_mask(&self) -> &SecDspIntMask {
        &self.sec_dsp_int_mask
    }
    #[doc = "0xfbc - sec_gp_reg write-lock bits"]
    #[inline(always)]
    pub const fn sec_mask_lock(&self) -> &SecMaskLock {
        &self.sec_mask_lock
    }
    #[doc = "0xfd0 - master secure level register"]
    #[inline(always)]
    pub const fn master_sec_level(&self) -> &MasterSecLevel {
        &self.master_sec_level
    }
    #[doc = "0xfd4 - master secure level anti-pole register"]
    #[inline(always)]
    pub const fn master_sec_level_anti_pol(&self) -> &MasterSecLevelAntiPol {
        &self.master_sec_level_anti_pol
    }
    #[doc = "0xfec - m33 lock control register"]
    #[inline(always)]
    pub const fn cm33_lock_reg(&self) -> &Cm33LockReg {
        &self.cm33_lock_reg
    }
    #[doc = "0xff8 - secure control duplicate register"]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(&self) -> &MiscCtrlDpReg {
        &self.misc_ctrl_dp_reg
    }
    #[doc = "0xffc - secure control register"]
    #[inline(always)]
    pub const fn misc_ctrl_reg(&self) -> &MiscCtrlReg {
        &self.misc_ctrl_reg
    }
}
#[doc = "ROM_MEM_RULE (rw) register accessor: Memory ROM Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_mem_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_mem_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mem_rule`]
module"]
#[doc(alias = "ROM_MEM_RULE")]
pub type RomMemRule = crate::Reg<rom_mem_rule::RomMemRuleSpec>;
#[doc = "Memory ROM Rule(n) Register"]
pub mod rom_mem_rule;
#[doc = "FLEXSPI0_REGION0_RULE (rw) register accessor: FLEXSPI0 Region 0 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi0_region0_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi0_region0_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspi0_region0_rule`]
module"]
#[doc(alias = "FLEXSPI0_REGION0_RULE")]
pub type Flexspi0Region0Rule = crate::Reg<flexspi0_region0_rule::Flexspi0Region0RuleSpec>;
#[doc = "FLEXSPI0 Region 0 Rule(n) Register"]
pub mod flexspi0_region0_rule;
#[doc = "FLEXSPI0_REGION1_RULE0 (rw) register accessor: FLEXSPI0 Region 1 Rule 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi0_region1_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi0_region1_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspi0_region1_rule0`]
module"]
#[doc(alias = "FLEXSPI0_REGION1_RULE0")]
pub type Flexspi0Region1Rule0 = crate::Reg<flexspi0_region1_rule0::Flexspi0Region1Rule0Spec>;
#[doc = "FLEXSPI0 Region 1 Rule 0 Register"]
pub mod flexspi0_region1_rule0;
#[doc = "FLEXSPI0_REGION2_RULE0 (rw) register accessor: FLEXSPI0 Region 2 Rule 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi0_region2_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi0_region2_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspi0_region2_rule0`]
module"]
#[doc(alias = "FLEXSPI0_REGION2_RULE0")]
pub type Flexspi0Region2Rule0 = crate::Reg<flexspi0_region2_rule0::Flexspi0Region2Rule0Spec>;
#[doc = "FLEXSPI0 Region 2 Rule 0 Register"]
pub mod flexspi0_region2_rule0;
#[doc = "FLEXSPI0_REGION3_RULE0 (rw) register accessor: FLEXSPI0 Region 3 Rule 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi0_region3_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi0_region3_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspi0_region3_rule0`]
module"]
#[doc(alias = "FLEXSPI0_REGION3_RULE0")]
pub type Flexspi0Region3Rule0 = crate::Reg<flexspi0_region3_rule0::Flexspi0Region3Rule0Spec>;
#[doc = "FLEXSPI0 Region 3 Rule 0 Register"]
pub mod flexspi0_region3_rule0;
#[doc = "FLEXSPI0_REGION4_RULE0 (rw) register accessor: FLEXSPI0 Region 4 Rule 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi0_region4_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi0_region4_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspi0_region4_rule0`]
module"]
#[doc(alias = "FLEXSPI0_REGION4_RULE0")]
pub type Flexspi0Region4Rule0 = crate::Reg<flexspi0_region4_rule0::Flexspi0Region4Rule0Spec>;
#[doc = "FLEXSPI0 Region 4 Rule 0 Register"]
pub mod flexspi0_region4_rule0;
#[doc = "RAM00_RULE (rw) register accessor: SRAM Partition 00 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram00_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram00_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram00_rule`]
module"]
#[doc(alias = "RAM00_RULE")]
pub type Ram00Rule = crate::Reg<ram00_rule::Ram00RuleSpec>;
#[doc = "SRAM Partition 00 Rule(n) Register"]
pub mod ram00_rule;
#[doc = "RAM01_RULE (rw) register accessor: SRAM Partition 01 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram01_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram01_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram01_rule`]
module"]
#[doc(alias = "RAM01_RULE")]
pub type Ram01Rule = crate::Reg<ram01_rule::Ram01RuleSpec>;
#[doc = "SRAM Partition 01 Rule(n) Register"]
pub mod ram01_rule;
#[doc = "RAM02_RULE (rw) register accessor: SRAM Partition 02 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram02_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram02_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram02_rule`]
module"]
#[doc(alias = "RAM02_RULE")]
pub type Ram02Rule = crate::Reg<ram02_rule::Ram02RuleSpec>;
#[doc = "SRAM Partition 02 Rule(n) Register"]
pub mod ram02_rule;
#[doc = "RAM03_RULE (rw) register accessor: SRAM Partition 03 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram03_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram03_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram03_rule`]
module"]
#[doc(alias = "RAM03_RULE")]
pub type Ram03Rule = crate::Reg<ram03_rule::Ram03RuleSpec>;
#[doc = "SRAM Partition 03 Rule(n) Register"]
pub mod ram03_rule;
#[doc = "RAM04_RULE (rw) register accessor: SRAM Partition 04 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram04_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram04_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram04_rule`]
module"]
#[doc(alias = "RAM04_RULE")]
pub type Ram04Rule = crate::Reg<ram04_rule::Ram04RuleSpec>;
#[doc = "SRAM Partition 04 Rule(n) Register"]
pub mod ram04_rule;
#[doc = "RAM05_RULE (rw) register accessor: SRAM Partition 05 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram05_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram05_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram05_rule`]
module"]
#[doc(alias = "RAM05_RULE")]
pub type Ram05Rule = crate::Reg<ram05_rule::Ram05RuleSpec>;
#[doc = "SRAM Partition 05 Rule(n) Register"]
pub mod ram05_rule;
#[doc = "RAM06_RULE (rw) register accessor: SRAM Partition 06 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram06_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram06_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram06_rule`]
module"]
#[doc(alias = "RAM06_RULE")]
pub type Ram06Rule = crate::Reg<ram06_rule::Ram06RuleSpec>;
#[doc = "SRAM Partition 06 Rule(n) Register"]
pub mod ram06_rule;
#[doc = "RAM07_RULE (rw) register accessor: SRAM Partition 07 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram07_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram07_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram07_rule`]
module"]
#[doc(alias = "RAM07_RULE")]
pub type Ram07Rule = crate::Reg<ram07_rule::Ram07RuleSpec>;
#[doc = "SRAM Partition 07 Rule(n) Register"]
pub mod ram07_rule;
#[doc = "RAM08_RULE (rw) register accessor: SRAM Partition 08 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram08_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram08_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram08_rule`]
module"]
#[doc(alias = "RAM08_RULE")]
pub type Ram08Rule = crate::Reg<ram08_rule::Ram08RuleSpec>;
#[doc = "SRAM Partition 08 Rule(n) Register"]
pub mod ram08_rule;
#[doc = "RAM09_RULE (rw) register accessor: SRAM Partition 09 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram09_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram09_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram09_rule`]
module"]
#[doc(alias = "RAM09_RULE")]
pub type Ram09Rule = crate::Reg<ram09_rule::Ram09RuleSpec>;
#[doc = "SRAM Partition 09 Rule(n) Register"]
pub mod ram09_rule;
#[doc = "RAM10_RULE (rw) register accessor: SRAM Partition 10 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram10_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram10_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram10_rule`]
module"]
#[doc(alias = "RAM10_RULE")]
pub type Ram10Rule = crate::Reg<ram10_rule::Ram10RuleSpec>;
#[doc = "SRAM Partition 10 Rule(n) Register"]
pub mod ram10_rule;
#[doc = "RAM11_RULE (rw) register accessor: SRAM Partition 11 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram11_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram11_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram11_rule`]
module"]
#[doc(alias = "RAM11_RULE")]
pub type Ram11Rule = crate::Reg<ram11_rule::Ram11RuleSpec>;
#[doc = "SRAM Partition 11 Rule(n) Register"]
pub mod ram11_rule;
#[doc = "RAM12_RULE (rw) register accessor: SRAM Partition 12 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram12_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram12_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram12_rule`]
module"]
#[doc(alias = "RAM12_RULE")]
pub type Ram12Rule = crate::Reg<ram12_rule::Ram12RuleSpec>;
#[doc = "SRAM Partition 12 Rule(n) Register"]
pub mod ram12_rule;
#[doc = "RAM13_RULE (rw) register accessor: SRAM Partition 13 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram13_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram13_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram13_rule`]
module"]
#[doc(alias = "RAM13_RULE")]
pub type Ram13Rule = crate::Reg<ram13_rule::Ram13RuleSpec>;
#[doc = "SRAM Partition 13 Rule(n) Register"]
pub mod ram13_rule;
#[doc = "RAM14_RULE (rw) register accessor: SRAM Partition 14 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram14_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram14_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram14_rule`]
module"]
#[doc(alias = "RAM14_RULE")]
pub type Ram14Rule = crate::Reg<ram14_rule::Ram14RuleSpec>;
#[doc = "SRAM Partition 14 Rule(n) Register"]
pub mod ram14_rule;
#[doc = "RAM15_RULE (rw) register accessor: SRAM Partition 15 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram15_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram15_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram15_rule`]
module"]
#[doc(alias = "RAM15_RULE")]
pub type Ram15Rule = crate::Reg<ram15_rule::Ram15RuleSpec>;
#[doc = "SRAM Partition 15 Rule(n) Register"]
pub mod ram15_rule;
#[doc = "RAM16_RULE (rw) register accessor: SRAM Partition 16 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram16_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram16_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram16_rule`]
module"]
#[doc(alias = "RAM16_RULE")]
pub type Ram16Rule = crate::Reg<ram16_rule::Ram16RuleSpec>;
#[doc = "SRAM Partition 16 Rule(n) Register"]
pub mod ram16_rule;
#[doc = "RAM17_RULE (rw) register accessor: SRAM Partition 17 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram17_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram17_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram17_rule`]
module"]
#[doc(alias = "RAM17_RULE")]
pub type Ram17Rule = crate::Reg<ram17_rule::Ram17RuleSpec>;
#[doc = "SRAM Partition 17 Rule(n) Register"]
pub mod ram17_rule;
#[doc = "RAM18_RULE (rw) register accessor: SRAM Partition 18 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram18_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram18_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram18_rule`]
module"]
#[doc(alias = "RAM18_RULE")]
pub type Ram18Rule = crate::Reg<ram18_rule::Ram18RuleSpec>;
#[doc = "SRAM Partition 18 Rule(n) Register"]
pub mod ram18_rule;
#[doc = "RAM19_RULE (rw) register accessor: SRAM Partition 19 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram19_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram19_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram19_rule`]
module"]
#[doc(alias = "RAM19_RULE")]
pub type Ram19Rule = crate::Reg<ram19_rule::Ram19RuleSpec>;
#[doc = "SRAM Partition 19 Rule(n) Register"]
pub mod ram19_rule;
#[doc = "RAM20_RULE (rw) register accessor: SRAM Partition 20 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram20_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram20_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram20_rule`]
module"]
#[doc(alias = "RAM20_RULE")]
pub type Ram20Rule = crate::Reg<ram20_rule::Ram20RuleSpec>;
#[doc = "SRAM Partition 20 Rule(n) Register"]
pub mod ram20_rule;
#[doc = "RAM21_RULE (rw) register accessor: SRAM Partition 21 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram21_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram21_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram21_rule`]
module"]
#[doc(alias = "RAM21_RULE")]
pub type Ram21Rule = crate::Reg<ram21_rule::Ram21RuleSpec>;
#[doc = "SRAM Partition 21 Rule(n) Register"]
pub mod ram21_rule;
#[doc = "RAM22_RULE (rw) register accessor: SRAM Partition 22 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram22_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram22_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram22_rule`]
module"]
#[doc(alias = "RAM22_RULE")]
pub type Ram22Rule = crate::Reg<ram22_rule::Ram22RuleSpec>;
#[doc = "SRAM Partition 22 Rule(n) Register"]
pub mod ram22_rule;
#[doc = "RAM23_RULE (rw) register accessor: SRAM Partition 23 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram23_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram23_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram23_rule`]
module"]
#[doc(alias = "RAM23_RULE")]
pub type Ram23Rule = crate::Reg<ram23_rule::Ram23RuleSpec>;
#[doc = "SRAM Partition 23 Rule(n) Register"]
pub mod ram23_rule;
#[doc = "RAM24_RULE (rw) register accessor: SRAM Partition 24 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram24_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram24_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram24_rule`]
module"]
#[doc(alias = "RAM24_RULE")]
pub type Ram24Rule = crate::Reg<ram24_rule::Ram24RuleSpec>;
#[doc = "SRAM Partition 24 Rule(n) Register"]
pub mod ram24_rule;
#[doc = "RAM25_RULE (rw) register accessor: SRAM Partition 25 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram25_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram25_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram25_rule`]
module"]
#[doc(alias = "RAM25_RULE")]
pub type Ram25Rule = crate::Reg<ram25_rule::Ram25RuleSpec>;
#[doc = "SRAM Partition 25 Rule(n) Register"]
pub mod ram25_rule;
#[doc = "RAM26_RULE (rw) register accessor: SRAM Partition 26 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram26_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram26_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram26_rule`]
module"]
#[doc(alias = "RAM26_RULE")]
pub type Ram26Rule = crate::Reg<ram26_rule::Ram26RuleSpec>;
#[doc = "SRAM Partition 26 Rule(n) Register"]
pub mod ram26_rule;
#[doc = "RAM27_RULE (rw) register accessor: SRAM Partition 27 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram27_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram27_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram27_rule`]
module"]
#[doc(alias = "RAM27_RULE")]
pub type Ram27Rule = crate::Reg<ram27_rule::Ram27RuleSpec>;
#[doc = "SRAM Partition 27 Rule(n) Register"]
pub mod ram27_rule;
#[doc = "RAM28_RULE (rw) register accessor: SRAM Partition 28 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram28_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram28_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram28_rule`]
module"]
#[doc(alias = "RAM28_RULE")]
pub type Ram28Rule = crate::Reg<ram28_rule::Ram28RuleSpec>;
#[doc = "SRAM Partition 28 Rule(n) Register"]
pub mod ram28_rule;
#[doc = "RAM29_RULE (rw) register accessor: SRAM Partition 29 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram29_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram29_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram29_rule`]
module"]
#[doc(alias = "RAM29_RULE")]
pub type Ram29Rule = crate::Reg<ram29_rule::Ram29RuleSpec>;
#[doc = "SRAM Partition 29 Rule(n) Register"]
pub mod ram29_rule;
#[doc = "PIF_HIFI4_X_MEM_RULE0 (rw) register accessor: Security access rules for HiFi 4 memory sectors (0x24000000--0x240FFFFF). Each sector is 32 Kbytes, there're 4 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_hifi4_x_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pif_hifi4_x_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pif_hifi4_x_mem_rule0`]
module"]
#[doc(alias = "PIF_HIFI4_X_MEM_RULE0")]
pub type PifHifi4XMemRule0 = crate::Reg<pif_hifi4_x_mem_rule0::PifHifi4XMemRule0Spec>;
#[doc = "Security access rules for HiFi 4 memory sectors (0x24000000--0x240FFFFF). Each sector is 32 Kbytes, there're 4 sectors in total."]
pub mod pif_hifi4_x_mem_rule0;
#[doc = "APB_GRP0_MEM_RULE0 (rw) register accessor: Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp0_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp0_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_grp0_mem_rule0`]
module"]
#[doc(alias = "APB_GRP0_MEM_RULE0")]
pub type ApbGrp0MemRule0 = crate::Reg<apb_grp0_mem_rule0::ApbGrp0MemRule0Spec>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod apb_grp0_mem_rule0;
#[doc = "APB_GRP0_MEM_RULE1 (rw) register accessor: Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp0_mem_rule1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp0_mem_rule1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_grp0_mem_rule1`]
module"]
#[doc(alias = "APB_GRP0_MEM_RULE1")]
pub type ApbGrp0MemRule1 = crate::Reg<apb_grp0_mem_rule1::ApbGrp0MemRule1Spec>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod apb_grp0_mem_rule1;
#[doc = "APB_GRP1_MEM_RULE0 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp1_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp1_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_grp1_mem_rule0`]
module"]
#[doc(alias = "APB_GRP1_MEM_RULE0")]
pub type ApbGrp1MemRule0 = crate::Reg<apb_grp1_mem_rule0::ApbGrp1MemRule0Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod apb_grp1_mem_rule0;
#[doc = "APB_GRP1_MEM_RULE1 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp1_mem_rule1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp1_mem_rule1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_grp1_mem_rule1`]
module"]
#[doc(alias = "APB_GRP1_MEM_RULE1")]
pub type ApbGrp1MemRule1 = crate::Reg<apb_grp1_mem_rule1::ApbGrp1MemRule1Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod apb_grp1_mem_rule1;
#[doc = "APB_GRP1_MEM_RULE2 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp1_mem_rule2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp1_mem_rule2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_grp1_mem_rule2`]
module"]
#[doc(alias = "APB_GRP1_MEM_RULE2")]
pub type ApbGrp1MemRule2 = crate::Reg<apb_grp1_mem_rule2::ApbGrp1MemRule2Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod apb_grp1_mem_rule2;
#[doc = "AHB_PERIPH0_SLAVE_RULE0 (rw) register accessor: Security access rules for AHB peripheral slaves area 0x40100000--0x4010FFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph0_slave_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph0_slave_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_periph0_slave_rule0`]
module"]
#[doc(alias = "AHB_PERIPH0_SLAVE_RULE0")]
pub type AhbPeriph0SlaveRule0 = crate::Reg<ahb_periph0_slave_rule0::AhbPeriph0SlaveRule0Spec>;
#[doc = "Security access rules for AHB peripheral slaves area 0x40100000--0x4010FFFF"]
pub mod ahb_periph0_slave_rule0;
#[doc = "AIPS_BRIDGE0_MEM_RULE0 (rw) register accessor: 0x40110000--0x4011FFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`aips_bridge0_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aips_bridge0_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aips_bridge0_mem_rule0`]
module"]
#[doc(alias = "AIPS_BRIDGE0_MEM_RULE0")]
pub type AipsBridge0MemRule0 = crate::Reg<aips_bridge0_mem_rule0::AipsBridge0MemRule0Spec>;
#[doc = "0x40110000--0x4011FFFF"]
pub mod aips_bridge0_mem_rule0;
#[doc = "AHB_PERIPH1_SLAVE_RULE0 (rw) register accessor: the memory map is 0x40120000--0x40127FFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph1_slave_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph1_slave_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_periph1_slave_rule0`]
module"]
#[doc(alias = "AHB_PERIPH1_SLAVE_RULE0")]
pub type AhbPeriph1SlaveRule0 = crate::Reg<ahb_periph1_slave_rule0::AhbPeriph1SlaveRule0Spec>;
#[doc = "the memory map is 0x40120000--0x40127FFF"]
pub mod ahb_periph1_slave_rule0;
#[doc = "AIPS_BRIDGE1_MEM_RULE0 (rw) register accessor: Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`aips_bridge1_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aips_bridge1_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aips_bridge1_mem_rule0`]
module"]
#[doc(alias = "AIPS_BRIDGE1_MEM_RULE0")]
pub type AipsBridge1MemRule0 = crate::Reg<aips_bridge1_mem_rule0::AipsBridge1MemRule0Spec>;
#[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod aips_bridge1_mem_rule0;
#[doc = "AIPS_BRIDGE1_MEM_RULE1 (rw) register accessor: Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`aips_bridge1_mem_rule1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aips_bridge1_mem_rule1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aips_bridge1_mem_rule1`]
module"]
#[doc(alias = "AIPS_BRIDGE1_MEM_RULE1")]
pub type AipsBridge1MemRule1 = crate::Reg<aips_bridge1_mem_rule1::AipsBridge1MemRule1Spec>;
#[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total."]
pub mod aips_bridge1_mem_rule1;
#[doc = "AHB_PERIPH2_SLAVE_RULE0 (rw) register accessor: Security access rules for AHB peripheral slaves area 0x40140000--0x4014BFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph2_slave_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph2_slave_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_periph2_slave_rule0`]
module"]
#[doc(alias = "AHB_PERIPH2_SLAVE_RULE0")]
pub type AhbPeriph2SlaveRule0 = crate::Reg<ahb_periph2_slave_rule0::AhbPeriph2SlaveRule0Spec>;
#[doc = "Security access rules for AHB peripheral slaves area 0x40140000--0x4014BFFF"]
pub mod ahb_periph2_slave_rule0;
#[doc = "SECURITY_CTRL_MEM_RULE0 (rw) register accessor: 0x40148000--0x4014BFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`security_ctrl_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_ctrl_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@security_ctrl_mem_rule0`]
module"]
#[doc(alias = "SECURITY_CTRL_MEM_RULE0")]
pub type SecurityCtrlMemRule0 = crate::Reg<security_ctrl_mem_rule0::SecurityCtrlMemRule0Spec>;
#[doc = "0x40148000--0x4014BFFF"]
pub mod security_ctrl_mem_rule0;
#[doc = "AHB_PERIPH3_SLAVE_RULE0 (rw) register accessor: Security access rules for AHB peripheral slaves area 0x40150000--0x40158FFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph3_slave_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph3_slave_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_periph3_slave_rule0`]
module"]
#[doc(alias = "AHB_PERIPH3_SLAVE_RULE0")]
pub type AhbPeriph3SlaveRule0 = crate::Reg<ahb_periph3_slave_rule0::AhbPeriph3SlaveRule0Spec>;
#[doc = "Security access rules for AHB peripheral slaves area 0x40150000--0x40158FFF"]
pub mod ahb_periph3_slave_rule0;
#[doc = "SEC_VIO_ADDR (r) register accessor: most recent security violation address for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_vio_addr`]
module"]
#[doc(alias = "SEC_VIO_ADDR")]
pub type SecVioAddr = crate::Reg<sec_vio_addr::SecVioAddrSpec>;
#[doc = "most recent security violation address for AHB layer n"]
pub mod sec_vio_addr;
#[doc = "SEC_VIO_MISC_INFO (r) register accessor: most recent security violation miscellaneous information for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_misc_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_vio_misc_info`]
module"]
#[doc(alias = "SEC_VIO_MISC_INFO")]
pub type SecVioMiscInfo = crate::Reg<sec_vio_misc_info::SecVioMiscInfoSpec>;
#[doc = "most recent security violation miscellaneous information for AHB layer n"]
pub mod sec_vio_misc_info;
#[doc = "SEC_VIO_INFO_VALID (rw) register accessor: security violation address/information registers valid flags\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_info_valid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_vio_info_valid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_vio_info_valid`]
module"]
#[doc(alias = "SEC_VIO_INFO_VALID")]
pub type SecVioInfoValid = crate::Reg<sec_vio_info_valid::SecVioInfoValidSpec>;
#[doc = "security violation address/information registers valid flags"]
pub mod sec_vio_info_valid;
#[doc = "SEC_GPIO_MASK0 (rw) register accessor: Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask0`]
module"]
#[doc(alias = "SEC_GPIO_MASK0")]
pub type SecGpioMask0 = crate::Reg<sec_gpio_mask0::SecGpioMask0Spec>;
#[doc = "Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
pub mod sec_gpio_mask0;
#[doc = "SEC_GPIO_MASK1 (rw) register accessor: Secure GPIO mask for port 1 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask1`]
module"]
#[doc(alias = "SEC_GPIO_MASK1")]
pub type SecGpioMask1 = crate::Reg<sec_gpio_mask1::SecGpioMask1Spec>;
#[doc = "Secure GPIO mask for port 1 pins."]
pub mod sec_gpio_mask1;
#[doc = "SEC_GPIO_MASK2 (rw) register accessor: Secure GPIO mask for port 2 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask2`]
module"]
#[doc(alias = "SEC_GPIO_MASK2")]
pub type SecGpioMask2 = crate::Reg<sec_gpio_mask2::SecGpioMask2Spec>;
#[doc = "Secure GPIO mask for port 2 pins."]
pub mod sec_gpio_mask2;
#[doc = "SEC_GPIO_MASK3 (rw) register accessor: Secure GPIO mask for port 3 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask3`]
module"]
#[doc(alias = "SEC_GPIO_MASK3")]
pub type SecGpioMask3 = crate::Reg<sec_gpio_mask3::SecGpioMask3Spec>;
#[doc = "Secure GPIO mask for port 3 pins."]
pub mod sec_gpio_mask3;
#[doc = "SEC_GPIO_MASK4 (rw) register accessor: Secure GPIO mask for port 4 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask4`]
module"]
#[doc(alias = "SEC_GPIO_MASK4")]
pub type SecGpioMask4 = crate::Reg<sec_gpio_mask4::SecGpioMask4Spec>;
#[doc = "Secure GPIO mask for port 4 pins."]
pub mod sec_gpio_mask4;
#[doc = "SEC_GPIO_MASK5 (rw) register accessor: Secure GPIO mask for port 5 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask5`]
module"]
#[doc(alias = "SEC_GPIO_MASK5")]
pub type SecGpioMask5 = crate::Reg<sec_gpio_mask5::SecGpioMask5Spec>;
#[doc = "Secure GPIO mask for port 5 pins."]
pub mod sec_gpio_mask5;
#[doc = "SEC_GPIO_MASK6 (rw) register accessor: Secure GPIO mask for port 6 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask6`]
module"]
#[doc(alias = "SEC_GPIO_MASK6")]
pub type SecGpioMask6 = crate::Reg<sec_gpio_mask6::SecGpioMask6Spec>;
#[doc = "Secure GPIO mask for port 6 pins."]
pub mod sec_gpio_mask6;
#[doc = "SEC_GPIO_MASK7 (rw) register accessor: Secure GPIO mask for port 7 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask7`]
module"]
#[doc(alias = "SEC_GPIO_MASK7")]
pub type SecGpioMask7 = crate::Reg<sec_gpio_mask7::SecGpioMask7Spec>;
#[doc = "Secure GPIO mask for port 7 pins."]
pub mod sec_gpio_mask7;
#[doc = "SEC_DSP_INT_MASK (rw) register accessor: secure general purpose register 8 used to mask interrupts to DSP for security purpose\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_dsp_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_dsp_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_dsp_int_mask`]
module"]
#[doc(alias = "SEC_DSP_INT_MASK")]
pub type SecDspIntMask = crate::Reg<sec_dsp_int_mask::SecDspIntMaskSpec>;
#[doc = "secure general purpose register 8 used to mask interrupts to DSP for security purpose"]
pub mod sec_dsp_int_mask;
#[doc = "SEC_MASK_LOCK (rw) register accessor: sec_gp_reg write-lock bits\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_mask_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_mask_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_mask_lock`]
module"]
#[doc(alias = "SEC_MASK_LOCK")]
pub type SecMaskLock = crate::Reg<sec_mask_lock::SecMaskLockSpec>;
#[doc = "sec_gp_reg write-lock bits"]
pub mod sec_mask_lock;
#[doc = "MASTER_SEC_LEVEL (rw) register accessor: master secure level register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_sec_level`]
module"]
#[doc(alias = "MASTER_SEC_LEVEL")]
pub type MasterSecLevel = crate::Reg<master_sec_level::MasterSecLevelSpec>;
#[doc = "master secure level register"]
pub mod master_sec_level;
#[doc = "MASTER_SEC_LEVEL_ANTI_POL (rw) register accessor: master secure level anti-pole register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_level_anti_pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_level_anti_pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_sec_level_anti_pol`]
module"]
#[doc(alias = "MASTER_SEC_LEVEL_ANTI_POL")]
pub type MasterSecLevelAntiPol = crate::Reg<master_sec_level_anti_pol::MasterSecLevelAntiPolSpec>;
#[doc = "master secure level anti-pole register"]
pub mod master_sec_level_anti_pol;
#[doc = "CM33_LOCK_REG (rw) register accessor: m33 lock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cm33_lock_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm33_lock_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm33_lock_reg`]
module"]
#[doc(alias = "CM33_LOCK_REG")]
pub type Cm33LockReg = crate::Reg<cm33_lock_reg::Cm33LockRegSpec>;
#[doc = "m33 lock control register"]
pub mod cm33_lock_reg;
#[doc = "MISC_CTRL_DP_REG (rw) register accessor: secure control duplicate register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl_dp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl_dp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl_dp_reg`]
module"]
#[doc(alias = "MISC_CTRL_DP_REG")]
pub type MiscCtrlDpReg = crate::Reg<misc_ctrl_dp_reg::MiscCtrlDpRegSpec>;
#[doc = "secure control duplicate register"]
pub mod misc_ctrl_dp_reg;
#[doc = "MISC_CTRL_REG (rw) register accessor: secure control register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl_reg`]
module"]
#[doc(alias = "MISC_CTRL_REG")]
pub type MiscCtrlReg = crate::Reg<misc_ctrl_reg::MiscCtrlRegSpec>;
#[doc = "secure control register"]
pub mod misc_ctrl_reg;
