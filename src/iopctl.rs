#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pio0_0: Pio0_0,
    pio0_1: Pio0_1,
    pio0_2: Pio0_2,
    pio0_3: Pio0_3,
    pio0_4: Pio0_4,
    pio0_5: Pio0_5,
    pio0_6: Pio0_6,
    pio0_7: Pio0_7,
    pio0_8: Pio0_8,
    pio0_9: Pio0_9,
    pio0_10: Pio0_10,
    pio0_11: Pio0_11,
    pio0_12: Pio0_12,
    pio0_13: Pio0_13,
    pio0_14: Pio0_14,
    pio0_15: Pio0_15,
    pio0_16: Pio0_16,
    pio0_17: Pio0_17,
    pio0_18: Pio0_18,
    pio0_19: Pio0_19,
    pio0_20: Pio0_20,
    pio0_21: Pio0_21,
    pio0_22: Pio0_22,
    pio0_23: Pio0_23,
    pio0_24: Pio0_24,
    pio0_25: Pio0_25,
    pio0_26: Pio0_26,
    pio0_27: Pio0_27,
    pio0_28: Pio0_28,
    pio0_29: Pio0_29,
    pio0_30: Pio0_30,
    pio0_31: Pio0_31,
    pio1_0: Pio1_0,
    pio1_1: Pio1_1,
    pio1_2: Pio1_2,
    pio1_3: Pio1_3,
    pio1_4: Pio1_4,
    pio1_5: Pio1_5,
    pio1_6: Pio1_6,
    pio1_7: Pio1_7,
    pio1_8: Pio1_8,
    pio1_9: Pio1_9,
    pio1_10: Pio1_10,
    pio1_11: Pio1_11,
    pio1_12: Pio1_12,
    pio1_13: Pio1_13,
    pio1_14: Pio1_14,
    pio1_15: Pio1_15,
    pio1_16: Pio1_16,
    pio1_17: Pio1_17,
    pio1_18: Pio1_18,
    pio1_19: Pio1_19,
    pio1_20: Pio1_20,
    pio1_21: Pio1_21,
    pio1_22: Pio1_22,
    pio1_23: Pio1_23,
    pio1_24: Pio1_24,
    pio1_25: Pio1_25,
    pio1_26: Pio1_26,
    pio1_27: Pio1_27,
    pio1_28: Pio1_28,
    pio1_29: Pio1_29,
    pio1_30: Pio1_30,
    pio1_31: Pio1_31,
    pio2_0: Pio2_0,
    pio2_1: Pio2_1,
    pio2_2: Pio2_2,
    pio2_3: Pio2_3,
    pio2_4: Pio2_4,
    pio2_5: Pio2_5,
    pio2_6: Pio2_6,
    pio2_7: Pio2_7,
    pio2_8: Pio2_8,
    pio2_9: Pio2_9,
    pio2_10: Pio2_10,
    pio2_11: Pio2_11,
    pio2_12: Pio2_12,
    pio2_13: Pio2_13,
    pio2_14: Pio2_14,
    pio2_15: Pio2_15,
    pio2_16: Pio2_16,
    pio2_17: Pio2_17,
    pio2_18: Pio2_18,
    pio2_19: Pio2_19,
    pio2_20: Pio2_20,
    pio2_21: Pio2_21,
    pio2_22: Pio2_22,
    pio2_23: Pio2_23,
    pio2_24: Pio2_24,
    pio2_25: Pio2_25,
    pio2_26: Pio2_26,
    pio2_27: Pio2_27,
    pio2_28: Pio2_28,
    pio2_29: Pio2_29,
    pio2_30: Pio2_30,
    pio2_31: Pio2_31,
    pio3_0: Pio3_0,
    pio3_1: Pio3_1,
    pio3_2: Pio3_2,
    pio3_3: Pio3_3,
    pio3_4: Pio3_4,
    pio3_5: Pio3_5,
    pio3_6: Pio3_6,
    pio3_7: Pio3_7,
    pio3_8: Pio3_8,
    pio3_9: Pio3_9,
    pio3_10: Pio3_10,
    pio3_11: Pio3_11,
    pio3_12: Pio3_12,
    pio3_13: Pio3_13,
    pio3_14: Pio3_14,
    pio3_15: Pio3_15,
    pio3_16: Pio3_16,
    pio3_17: Pio3_17,
    pio3_18: Pio3_18,
    pio3_19: Pio3_19,
    pio3_20: Pio3_20,
    pio3_21: Pio3_21,
    pio3_22: Pio3_22,
    pio3_23: Pio3_23,
    pio3_24: Pio3_24,
    pio3_25: Pio3_25,
    pio3_26: Pio3_26,
    pio3_27: Pio3_27,
    pio3_28: Pio3_28,
    pio3_29: Pio3_29,
    pio3_30: Pio3_30,
    pio3_31: Pio3_31,
    pio4_0: Pio4_0,
    pio4_1: Pio4_1,
    pio4_2: Pio4_2,
    pio4_3: Pio4_3,
    pio4_4: Pio4_4,
    pio4_5: Pio4_5,
    pio4_6: Pio4_6,
    pio4_7: Pio4_7,
    pio4_8: Pio4_8,
    pio4_9: Pio4_9,
    pio4_10: Pio4_10,
    pio4_11: Pio4_11,
    pio4_12: Pio4_12,
    pio4_13: Pio4_13,
    pio4_14: Pio4_14,
    pio4_15: Pio4_15,
    pio4_16: Pio4_16,
    pio4_17: Pio4_17,
    pio4_18: Pio4_18,
    pio4_19: Pio4_19,
    pio4_20: Pio4_20,
    pio4_21: Pio4_21,
    pio4_22: Pio4_22,
    pio4_23: Pio4_23,
    pio4_24: Pio4_24,
    pio4_25: Pio4_25,
    pio4_26: Pio4_26,
    pio4_27: Pio4_27,
    pio4_28: Pio4_28,
    pio4_29: Pio4_29,
    pio4_30: Pio4_30,
    pio4_31: Pio4_31,
    pio5_0: Pio5_0,
    pio5_1: Pio5_1,
    pio5_2: Pio5_2,
    pio5_3: Pio5_3,
    pio5_4: Pio5_4,
    pio5_5: Pio5_5,
    pio5_6: Pio5_6,
    pio5_7: Pio5_7,
    pio5_8: Pio5_8,
    pio5_9: Pio5_9,
    pio5_10: Pio5_10,
    pio5_11: Pio5_11,
    pio5_12: Pio5_12,
    pio5_13: Pio5_13,
    pio5_14: Pio5_14,
    pio5_15: Pio5_15,
    pio5_16: Pio5_16,
    pio5_17: Pio5_17,
    pio5_18: Pio5_18,
    pio5_19: Pio5_19,
    pio5_20: Pio5_20,
    pio5_21: Pio5_21,
    pio5_22: Pio5_22,
    pio5_23: Pio5_23,
    pio5_24: Pio5_24,
    pio5_25: Pio5_25,
    pio5_26: Pio5_26,
    pio5_27: Pio5_27,
    pio5_28: Pio5_28,
    pio5_29: Pio5_29,
    pio5_30: Pio5_30,
    pio5_31: Pio5_31,
    pio6_0: Pio6_0,
    pio6_1: Pio6_1,
    pio6_2: Pio6_2,
    pio6_3: Pio6_3,
    pio6_4: Pio6_4,
    pio6_5: Pio6_5,
    pio6_6: Pio6_6,
    pio6_7: Pio6_7,
    pio6_8: Pio6_8,
    pio6_9: Pio6_9,
    pio6_10: Pio6_10,
    pio6_11: Pio6_11,
    pio6_12: Pio6_12,
    pio6_13: Pio6_13,
    pio6_14: Pio6_14,
    pio6_15: Pio6_15,
    pio6_16: Pio6_16,
    pio6_17: Pio6_17,
    pio6_18: Pio6_18,
    pio6_19: Pio6_19,
    pio6_20: Pio6_20,
    pio6_21: Pio6_21,
    pio6_22: Pio6_22,
    pio6_23: Pio6_23,
    pio6_24: Pio6_24,
    pio6_25: Pio6_25,
    pio6_26: Pio6_26,
    pio6_27: Pio6_27,
    pio6_28: Pio6_28,
    pio6_29: Pio6_29,
    pio6_30: Pio6_30,
    pio6_31: Pio6_31,
    pio7_0: Pio7_0,
    pio7_1: Pio7_1,
    pio7_2: Pio7_2,
    pio7_3: Pio7_3,
    pio7_4: Pio7_4,
    pio7_5: Pio7_5,
    pio7_6: Pio7_6,
    pio7_7: Pio7_7,
    pio7_8: Pio7_8,
    pio7_9: Pio7_9,
    pio7_10: Pio7_10,
    pio7_11: Pio7_11,
    pio7_12: Pio7_12,
    pio7_13: Pio7_13,
    pio7_14: Pio7_14,
    pio7_15: Pio7_15,
    pio7_16: Pio7_16,
    pio7_17: Pio7_17,
    pio7_18: Pio7_18,
    pio7_19: Pio7_19,
    pio7_20: Pio7_20,
    pio7_21: Pio7_21,
    pio7_22: Pio7_22,
    pio7_23: Pio7_23,
    pio7_24: Pio7_24,
    pio7_25: Pio7_25,
    pio7_26: Pio7_26,
    pio7_27: Pio7_27,
    pio7_28: Pio7_28,
    pio7_29: Pio7_29,
    pio7_30: Pio7_30,
    pio7_31: Pio7_31,
    fc15_i2c_scl: Fc15I2cScl,
    fc15_i2c_sda: Fc15I2cSda,
}
impl RegisterBlock {
    #[doc = "0x00 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_0(&self) -> &Pio0_0 {
        &self.pio0_0
    }
    #[doc = "0x04 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_1(&self) -> &Pio0_1 {
        &self.pio0_1
    }
    #[doc = "0x08 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_2(&self) -> &Pio0_2 {
        &self.pio0_2
    }
    #[doc = "0x0c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_3(&self) -> &Pio0_3 {
        &self.pio0_3
    }
    #[doc = "0x10 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_4(&self) -> &Pio0_4 {
        &self.pio0_4
    }
    #[doc = "0x14 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_5(&self) -> &Pio0_5 {
        &self.pio0_5
    }
    #[doc = "0x18 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_6(&self) -> &Pio0_6 {
        &self.pio0_6
    }
    #[doc = "0x1c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_7(&self) -> &Pio0_7 {
        &self.pio0_7
    }
    #[doc = "0x20 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_8(&self) -> &Pio0_8 {
        &self.pio0_8
    }
    #[doc = "0x24 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_9(&self) -> &Pio0_9 {
        &self.pio0_9
    }
    #[doc = "0x28 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_10(&self) -> &Pio0_10 {
        &self.pio0_10
    }
    #[doc = "0x2c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_11(&self) -> &Pio0_11 {
        &self.pio0_11
    }
    #[doc = "0x30 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_12(&self) -> &Pio0_12 {
        &self.pio0_12
    }
    #[doc = "0x34 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_13(&self) -> &Pio0_13 {
        &self.pio0_13
    }
    #[doc = "0x38 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_14(&self) -> &Pio0_14 {
        &self.pio0_14
    }
    #[doc = "0x3c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_15(&self) -> &Pio0_15 {
        &self.pio0_15
    }
    #[doc = "0x40 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_16(&self) -> &Pio0_16 {
        &self.pio0_16
    }
    #[doc = "0x44 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_17(&self) -> &Pio0_17 {
        &self.pio0_17
    }
    #[doc = "0x48 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_18(&self) -> &Pio0_18 {
        &self.pio0_18
    }
    #[doc = "0x4c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_19(&self) -> &Pio0_19 {
        &self.pio0_19
    }
    #[doc = "0x50 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_20(&self) -> &Pio0_20 {
        &self.pio0_20
    }
    #[doc = "0x54 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_21(&self) -> &Pio0_21 {
        &self.pio0_21
    }
    #[doc = "0x58 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_22(&self) -> &Pio0_22 {
        &self.pio0_22
    }
    #[doc = "0x5c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_23(&self) -> &Pio0_23 {
        &self.pio0_23
    }
    #[doc = "0x60 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_24(&self) -> &Pio0_24 {
        &self.pio0_24
    }
    #[doc = "0x64 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_25(&self) -> &Pio0_25 {
        &self.pio0_25
    }
    #[doc = "0x68 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_26(&self) -> &Pio0_26 {
        &self.pio0_26
    }
    #[doc = "0x6c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_27(&self) -> &Pio0_27 {
        &self.pio0_27
    }
    #[doc = "0x70 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_28(&self) -> &Pio0_28 {
        &self.pio0_28
    }
    #[doc = "0x74 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_29(&self) -> &Pio0_29 {
        &self.pio0_29
    }
    #[doc = "0x78 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_30(&self) -> &Pio0_30 {
        &self.pio0_30
    }
    #[doc = "0x7c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio0_31(&self) -> &Pio0_31 {
        &self.pio0_31
    }
    #[doc = "0x80 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_0(&self) -> &Pio1_0 {
        &self.pio1_0
    }
    #[doc = "0x84 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_1(&self) -> &Pio1_1 {
        &self.pio1_1
    }
    #[doc = "0x88 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_2(&self) -> &Pio1_2 {
        &self.pio1_2
    }
    #[doc = "0x8c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_3(&self) -> &Pio1_3 {
        &self.pio1_3
    }
    #[doc = "0x90 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_4(&self) -> &Pio1_4 {
        &self.pio1_4
    }
    #[doc = "0x94 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_5(&self) -> &Pio1_5 {
        &self.pio1_5
    }
    #[doc = "0x98 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_6(&self) -> &Pio1_6 {
        &self.pio1_6
    }
    #[doc = "0x9c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_7(&self) -> &Pio1_7 {
        &self.pio1_7
    }
    #[doc = "0xa0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_8(&self) -> &Pio1_8 {
        &self.pio1_8
    }
    #[doc = "0xa4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_9(&self) -> &Pio1_9 {
        &self.pio1_9
    }
    #[doc = "0xa8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_10(&self) -> &Pio1_10 {
        &self.pio1_10
    }
    #[doc = "0xac - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_11(&self) -> &Pio1_11 {
        &self.pio1_11
    }
    #[doc = "0xb0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_12(&self) -> &Pio1_12 {
        &self.pio1_12
    }
    #[doc = "0xb4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_13(&self) -> &Pio1_13 {
        &self.pio1_13
    }
    #[doc = "0xb8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_14(&self) -> &Pio1_14 {
        &self.pio1_14
    }
    #[doc = "0xbc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_15(&self) -> &Pio1_15 {
        &self.pio1_15
    }
    #[doc = "0xc0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_16(&self) -> &Pio1_16 {
        &self.pio1_16
    }
    #[doc = "0xc4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_17(&self) -> &Pio1_17 {
        &self.pio1_17
    }
    #[doc = "0xc8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_18(&self) -> &Pio1_18 {
        &self.pio1_18
    }
    #[doc = "0xcc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_19(&self) -> &Pio1_19 {
        &self.pio1_19
    }
    #[doc = "0xd0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_20(&self) -> &Pio1_20 {
        &self.pio1_20
    }
    #[doc = "0xd4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_21(&self) -> &Pio1_21 {
        &self.pio1_21
    }
    #[doc = "0xd8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_22(&self) -> &Pio1_22 {
        &self.pio1_22
    }
    #[doc = "0xdc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_23(&self) -> &Pio1_23 {
        &self.pio1_23
    }
    #[doc = "0xe0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_24(&self) -> &Pio1_24 {
        &self.pio1_24
    }
    #[doc = "0xe4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_25(&self) -> &Pio1_25 {
        &self.pio1_25
    }
    #[doc = "0xe8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_26(&self) -> &Pio1_26 {
        &self.pio1_26
    }
    #[doc = "0xec - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_27(&self) -> &Pio1_27 {
        &self.pio1_27
    }
    #[doc = "0xf0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_28(&self) -> &Pio1_28 {
        &self.pio1_28
    }
    #[doc = "0xf4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_29(&self) -> &Pio1_29 {
        &self.pio1_29
    }
    #[doc = "0xf8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_30(&self) -> &Pio1_30 {
        &self.pio1_30
    }
    #[doc = "0xfc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio1_31(&self) -> &Pio1_31 {
        &self.pio1_31
    }
    #[doc = "0x100 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_0(&self) -> &Pio2_0 {
        &self.pio2_0
    }
    #[doc = "0x104 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_1(&self) -> &Pio2_1 {
        &self.pio2_1
    }
    #[doc = "0x108 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_2(&self) -> &Pio2_2 {
        &self.pio2_2
    }
    #[doc = "0x10c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_3(&self) -> &Pio2_3 {
        &self.pio2_3
    }
    #[doc = "0x110 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_4(&self) -> &Pio2_4 {
        &self.pio2_4
    }
    #[doc = "0x114 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_5(&self) -> &Pio2_5 {
        &self.pio2_5
    }
    #[doc = "0x118 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_6(&self) -> &Pio2_6 {
        &self.pio2_6
    }
    #[doc = "0x11c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_7(&self) -> &Pio2_7 {
        &self.pio2_7
    }
    #[doc = "0x120 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_8(&self) -> &Pio2_8 {
        &self.pio2_8
    }
    #[doc = "0x124 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_9(&self) -> &Pio2_9 {
        &self.pio2_9
    }
    #[doc = "0x128 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_10(&self) -> &Pio2_10 {
        &self.pio2_10
    }
    #[doc = "0x12c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_11(&self) -> &Pio2_11 {
        &self.pio2_11
    }
    #[doc = "0x130 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_12(&self) -> &Pio2_12 {
        &self.pio2_12
    }
    #[doc = "0x134 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_13(&self) -> &Pio2_13 {
        &self.pio2_13
    }
    #[doc = "0x138 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_14(&self) -> &Pio2_14 {
        &self.pio2_14
    }
    #[doc = "0x13c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_15(&self) -> &Pio2_15 {
        &self.pio2_15
    }
    #[doc = "0x140 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_16(&self) -> &Pio2_16 {
        &self.pio2_16
    }
    #[doc = "0x144 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_17(&self) -> &Pio2_17 {
        &self.pio2_17
    }
    #[doc = "0x148 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_18(&self) -> &Pio2_18 {
        &self.pio2_18
    }
    #[doc = "0x14c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_19(&self) -> &Pio2_19 {
        &self.pio2_19
    }
    #[doc = "0x150 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_20(&self) -> &Pio2_20 {
        &self.pio2_20
    }
    #[doc = "0x154 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_21(&self) -> &Pio2_21 {
        &self.pio2_21
    }
    #[doc = "0x158 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_22(&self) -> &Pio2_22 {
        &self.pio2_22
    }
    #[doc = "0x15c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_23(&self) -> &Pio2_23 {
        &self.pio2_23
    }
    #[doc = "0x160 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_24(&self) -> &Pio2_24 {
        &self.pio2_24
    }
    #[doc = "0x164 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_25(&self) -> &Pio2_25 {
        &self.pio2_25
    }
    #[doc = "0x168 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_26(&self) -> &Pio2_26 {
        &self.pio2_26
    }
    #[doc = "0x16c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_27(&self) -> &Pio2_27 {
        &self.pio2_27
    }
    #[doc = "0x170 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_28(&self) -> &Pio2_28 {
        &self.pio2_28
    }
    #[doc = "0x174 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_29(&self) -> &Pio2_29 {
        &self.pio2_29
    }
    #[doc = "0x178 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_30(&self) -> &Pio2_30 {
        &self.pio2_30
    }
    #[doc = "0x17c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio2_31(&self) -> &Pio2_31 {
        &self.pio2_31
    }
    #[doc = "0x180 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_0(&self) -> &Pio3_0 {
        &self.pio3_0
    }
    #[doc = "0x184 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_1(&self) -> &Pio3_1 {
        &self.pio3_1
    }
    #[doc = "0x188 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_2(&self) -> &Pio3_2 {
        &self.pio3_2
    }
    #[doc = "0x18c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_3(&self) -> &Pio3_3 {
        &self.pio3_3
    }
    #[doc = "0x190 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_4(&self) -> &Pio3_4 {
        &self.pio3_4
    }
    #[doc = "0x194 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_5(&self) -> &Pio3_5 {
        &self.pio3_5
    }
    #[doc = "0x198 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_6(&self) -> &Pio3_6 {
        &self.pio3_6
    }
    #[doc = "0x19c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_7(&self) -> &Pio3_7 {
        &self.pio3_7
    }
    #[doc = "0x1a0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_8(&self) -> &Pio3_8 {
        &self.pio3_8
    }
    #[doc = "0x1a4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_9(&self) -> &Pio3_9 {
        &self.pio3_9
    }
    #[doc = "0x1a8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_10(&self) -> &Pio3_10 {
        &self.pio3_10
    }
    #[doc = "0x1ac - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_11(&self) -> &Pio3_11 {
        &self.pio3_11
    }
    #[doc = "0x1b0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_12(&self) -> &Pio3_12 {
        &self.pio3_12
    }
    #[doc = "0x1b4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_13(&self) -> &Pio3_13 {
        &self.pio3_13
    }
    #[doc = "0x1b8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_14(&self) -> &Pio3_14 {
        &self.pio3_14
    }
    #[doc = "0x1bc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_15(&self) -> &Pio3_15 {
        &self.pio3_15
    }
    #[doc = "0x1c0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_16(&self) -> &Pio3_16 {
        &self.pio3_16
    }
    #[doc = "0x1c4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_17(&self) -> &Pio3_17 {
        &self.pio3_17
    }
    #[doc = "0x1c8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_18(&self) -> &Pio3_18 {
        &self.pio3_18
    }
    #[doc = "0x1cc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_19(&self) -> &Pio3_19 {
        &self.pio3_19
    }
    #[doc = "0x1d0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_20(&self) -> &Pio3_20 {
        &self.pio3_20
    }
    #[doc = "0x1d4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_21(&self) -> &Pio3_21 {
        &self.pio3_21
    }
    #[doc = "0x1d8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_22(&self) -> &Pio3_22 {
        &self.pio3_22
    }
    #[doc = "0x1dc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_23(&self) -> &Pio3_23 {
        &self.pio3_23
    }
    #[doc = "0x1e0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_24(&self) -> &Pio3_24 {
        &self.pio3_24
    }
    #[doc = "0x1e4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_25(&self) -> &Pio3_25 {
        &self.pio3_25
    }
    #[doc = "0x1e8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_26(&self) -> &Pio3_26 {
        &self.pio3_26
    }
    #[doc = "0x1ec - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_27(&self) -> &Pio3_27 {
        &self.pio3_27
    }
    #[doc = "0x1f0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_28(&self) -> &Pio3_28 {
        &self.pio3_28
    }
    #[doc = "0x1f4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_29(&self) -> &Pio3_29 {
        &self.pio3_29
    }
    #[doc = "0x1f8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_30(&self) -> &Pio3_30 {
        &self.pio3_30
    }
    #[doc = "0x1fc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio3_31(&self) -> &Pio3_31 {
        &self.pio3_31
    }
    #[doc = "0x200 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_0(&self) -> &Pio4_0 {
        &self.pio4_0
    }
    #[doc = "0x204 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_1(&self) -> &Pio4_1 {
        &self.pio4_1
    }
    #[doc = "0x208 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_2(&self) -> &Pio4_2 {
        &self.pio4_2
    }
    #[doc = "0x20c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_3(&self) -> &Pio4_3 {
        &self.pio4_3
    }
    #[doc = "0x210 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_4(&self) -> &Pio4_4 {
        &self.pio4_4
    }
    #[doc = "0x214 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_5(&self) -> &Pio4_5 {
        &self.pio4_5
    }
    #[doc = "0x218 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_6(&self) -> &Pio4_6 {
        &self.pio4_6
    }
    #[doc = "0x21c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_7(&self) -> &Pio4_7 {
        &self.pio4_7
    }
    #[doc = "0x220 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_8(&self) -> &Pio4_8 {
        &self.pio4_8
    }
    #[doc = "0x224 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_9(&self) -> &Pio4_9 {
        &self.pio4_9
    }
    #[doc = "0x228 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_10(&self) -> &Pio4_10 {
        &self.pio4_10
    }
    #[doc = "0x22c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_11(&self) -> &Pio4_11 {
        &self.pio4_11
    }
    #[doc = "0x230 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_12(&self) -> &Pio4_12 {
        &self.pio4_12
    }
    #[doc = "0x234 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_13(&self) -> &Pio4_13 {
        &self.pio4_13
    }
    #[doc = "0x238 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_14(&self) -> &Pio4_14 {
        &self.pio4_14
    }
    #[doc = "0x23c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_15(&self) -> &Pio4_15 {
        &self.pio4_15
    }
    #[doc = "0x240 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_16(&self) -> &Pio4_16 {
        &self.pio4_16
    }
    #[doc = "0x244 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_17(&self) -> &Pio4_17 {
        &self.pio4_17
    }
    #[doc = "0x248 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_18(&self) -> &Pio4_18 {
        &self.pio4_18
    }
    #[doc = "0x24c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_19(&self) -> &Pio4_19 {
        &self.pio4_19
    }
    #[doc = "0x250 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_20(&self) -> &Pio4_20 {
        &self.pio4_20
    }
    #[doc = "0x254 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_21(&self) -> &Pio4_21 {
        &self.pio4_21
    }
    #[doc = "0x258 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_22(&self) -> &Pio4_22 {
        &self.pio4_22
    }
    #[doc = "0x25c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_23(&self) -> &Pio4_23 {
        &self.pio4_23
    }
    #[doc = "0x260 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_24(&self) -> &Pio4_24 {
        &self.pio4_24
    }
    #[doc = "0x264 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_25(&self) -> &Pio4_25 {
        &self.pio4_25
    }
    #[doc = "0x268 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_26(&self) -> &Pio4_26 {
        &self.pio4_26
    }
    #[doc = "0x26c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_27(&self) -> &Pio4_27 {
        &self.pio4_27
    }
    #[doc = "0x270 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_28(&self) -> &Pio4_28 {
        &self.pio4_28
    }
    #[doc = "0x274 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_29(&self) -> &Pio4_29 {
        &self.pio4_29
    }
    #[doc = "0x278 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_30(&self) -> &Pio4_30 {
        &self.pio4_30
    }
    #[doc = "0x27c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio4_31(&self) -> &Pio4_31 {
        &self.pio4_31
    }
    #[doc = "0x280 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_0(&self) -> &Pio5_0 {
        &self.pio5_0
    }
    #[doc = "0x284 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_1(&self) -> &Pio5_1 {
        &self.pio5_1
    }
    #[doc = "0x288 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_2(&self) -> &Pio5_2 {
        &self.pio5_2
    }
    #[doc = "0x28c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_3(&self) -> &Pio5_3 {
        &self.pio5_3
    }
    #[doc = "0x290 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_4(&self) -> &Pio5_4 {
        &self.pio5_4
    }
    #[doc = "0x294 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_5(&self) -> &Pio5_5 {
        &self.pio5_5
    }
    #[doc = "0x298 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_6(&self) -> &Pio5_6 {
        &self.pio5_6
    }
    #[doc = "0x29c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_7(&self) -> &Pio5_7 {
        &self.pio5_7
    }
    #[doc = "0x2a0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_8(&self) -> &Pio5_8 {
        &self.pio5_8
    }
    #[doc = "0x2a4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_9(&self) -> &Pio5_9 {
        &self.pio5_9
    }
    #[doc = "0x2a8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_10(&self) -> &Pio5_10 {
        &self.pio5_10
    }
    #[doc = "0x2ac - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_11(&self) -> &Pio5_11 {
        &self.pio5_11
    }
    #[doc = "0x2b0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_12(&self) -> &Pio5_12 {
        &self.pio5_12
    }
    #[doc = "0x2b4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_13(&self) -> &Pio5_13 {
        &self.pio5_13
    }
    #[doc = "0x2b8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_14(&self) -> &Pio5_14 {
        &self.pio5_14
    }
    #[doc = "0x2bc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_15(&self) -> &Pio5_15 {
        &self.pio5_15
    }
    #[doc = "0x2c0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_16(&self) -> &Pio5_16 {
        &self.pio5_16
    }
    #[doc = "0x2c4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_17(&self) -> &Pio5_17 {
        &self.pio5_17
    }
    #[doc = "0x2c8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_18(&self) -> &Pio5_18 {
        &self.pio5_18
    }
    #[doc = "0x2cc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_19(&self) -> &Pio5_19 {
        &self.pio5_19
    }
    #[doc = "0x2d0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_20(&self) -> &Pio5_20 {
        &self.pio5_20
    }
    #[doc = "0x2d4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_21(&self) -> &Pio5_21 {
        &self.pio5_21
    }
    #[doc = "0x2d8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_22(&self) -> &Pio5_22 {
        &self.pio5_22
    }
    #[doc = "0x2dc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_23(&self) -> &Pio5_23 {
        &self.pio5_23
    }
    #[doc = "0x2e0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_24(&self) -> &Pio5_24 {
        &self.pio5_24
    }
    #[doc = "0x2e4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_25(&self) -> &Pio5_25 {
        &self.pio5_25
    }
    #[doc = "0x2e8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_26(&self) -> &Pio5_26 {
        &self.pio5_26
    }
    #[doc = "0x2ec - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_27(&self) -> &Pio5_27 {
        &self.pio5_27
    }
    #[doc = "0x2f0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_28(&self) -> &Pio5_28 {
        &self.pio5_28
    }
    #[doc = "0x2f4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_29(&self) -> &Pio5_29 {
        &self.pio5_29
    }
    #[doc = "0x2f8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_30(&self) -> &Pio5_30 {
        &self.pio5_30
    }
    #[doc = "0x2fc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio5_31(&self) -> &Pio5_31 {
        &self.pio5_31
    }
    #[doc = "0x300 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_0(&self) -> &Pio6_0 {
        &self.pio6_0
    }
    #[doc = "0x304 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_1(&self) -> &Pio6_1 {
        &self.pio6_1
    }
    #[doc = "0x308 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_2(&self) -> &Pio6_2 {
        &self.pio6_2
    }
    #[doc = "0x30c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_3(&self) -> &Pio6_3 {
        &self.pio6_3
    }
    #[doc = "0x310 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_4(&self) -> &Pio6_4 {
        &self.pio6_4
    }
    #[doc = "0x314 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_5(&self) -> &Pio6_5 {
        &self.pio6_5
    }
    #[doc = "0x318 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_6(&self) -> &Pio6_6 {
        &self.pio6_6
    }
    #[doc = "0x31c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_7(&self) -> &Pio6_7 {
        &self.pio6_7
    }
    #[doc = "0x320 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_8(&self) -> &Pio6_8 {
        &self.pio6_8
    }
    #[doc = "0x324 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_9(&self) -> &Pio6_9 {
        &self.pio6_9
    }
    #[doc = "0x328 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_10(&self) -> &Pio6_10 {
        &self.pio6_10
    }
    #[doc = "0x32c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_11(&self) -> &Pio6_11 {
        &self.pio6_11
    }
    #[doc = "0x330 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_12(&self) -> &Pio6_12 {
        &self.pio6_12
    }
    #[doc = "0x334 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_13(&self) -> &Pio6_13 {
        &self.pio6_13
    }
    #[doc = "0x338 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_14(&self) -> &Pio6_14 {
        &self.pio6_14
    }
    #[doc = "0x33c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_15(&self) -> &Pio6_15 {
        &self.pio6_15
    }
    #[doc = "0x340 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_16(&self) -> &Pio6_16 {
        &self.pio6_16
    }
    #[doc = "0x344 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_17(&self) -> &Pio6_17 {
        &self.pio6_17
    }
    #[doc = "0x348 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_18(&self) -> &Pio6_18 {
        &self.pio6_18
    }
    #[doc = "0x34c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_19(&self) -> &Pio6_19 {
        &self.pio6_19
    }
    #[doc = "0x350 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_20(&self) -> &Pio6_20 {
        &self.pio6_20
    }
    #[doc = "0x354 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_21(&self) -> &Pio6_21 {
        &self.pio6_21
    }
    #[doc = "0x358 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_22(&self) -> &Pio6_22 {
        &self.pio6_22
    }
    #[doc = "0x35c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_23(&self) -> &Pio6_23 {
        &self.pio6_23
    }
    #[doc = "0x360 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_24(&self) -> &Pio6_24 {
        &self.pio6_24
    }
    #[doc = "0x364 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_25(&self) -> &Pio6_25 {
        &self.pio6_25
    }
    #[doc = "0x368 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_26(&self) -> &Pio6_26 {
        &self.pio6_26
    }
    #[doc = "0x36c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_27(&self) -> &Pio6_27 {
        &self.pio6_27
    }
    #[doc = "0x370 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_28(&self) -> &Pio6_28 {
        &self.pio6_28
    }
    #[doc = "0x374 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_29(&self) -> &Pio6_29 {
        &self.pio6_29
    }
    #[doc = "0x378 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_30(&self) -> &Pio6_30 {
        &self.pio6_30
    }
    #[doc = "0x37c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio6_31(&self) -> &Pio6_31 {
        &self.pio6_31
    }
    #[doc = "0x380 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_0(&self) -> &Pio7_0 {
        &self.pio7_0
    }
    #[doc = "0x384 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_1(&self) -> &Pio7_1 {
        &self.pio7_1
    }
    #[doc = "0x388 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_2(&self) -> &Pio7_2 {
        &self.pio7_2
    }
    #[doc = "0x38c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_3(&self) -> &Pio7_3 {
        &self.pio7_3
    }
    #[doc = "0x390 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_4(&self) -> &Pio7_4 {
        &self.pio7_4
    }
    #[doc = "0x394 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_5(&self) -> &Pio7_5 {
        &self.pio7_5
    }
    #[doc = "0x398 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_6(&self) -> &Pio7_6 {
        &self.pio7_6
    }
    #[doc = "0x39c - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_7(&self) -> &Pio7_7 {
        &self.pio7_7
    }
    #[doc = "0x3a0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_8(&self) -> &Pio7_8 {
        &self.pio7_8
    }
    #[doc = "0x3a4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_9(&self) -> &Pio7_9 {
        &self.pio7_9
    }
    #[doc = "0x3a8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_10(&self) -> &Pio7_10 {
        &self.pio7_10
    }
    #[doc = "0x3ac - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_11(&self) -> &Pio7_11 {
        &self.pio7_11
    }
    #[doc = "0x3b0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_12(&self) -> &Pio7_12 {
        &self.pio7_12
    }
    #[doc = "0x3b4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_13(&self) -> &Pio7_13 {
        &self.pio7_13
    }
    #[doc = "0x3b8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_14(&self) -> &Pio7_14 {
        &self.pio7_14
    }
    #[doc = "0x3bc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_15(&self) -> &Pio7_15 {
        &self.pio7_15
    }
    #[doc = "0x3c0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_16(&self) -> &Pio7_16 {
        &self.pio7_16
    }
    #[doc = "0x3c4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_17(&self) -> &Pio7_17 {
        &self.pio7_17
    }
    #[doc = "0x3c8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_18(&self) -> &Pio7_18 {
        &self.pio7_18
    }
    #[doc = "0x3cc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_19(&self) -> &Pio7_19 {
        &self.pio7_19
    }
    #[doc = "0x3d0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_20(&self) -> &Pio7_20 {
        &self.pio7_20
    }
    #[doc = "0x3d4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_21(&self) -> &Pio7_21 {
        &self.pio7_21
    }
    #[doc = "0x3d8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_22(&self) -> &Pio7_22 {
        &self.pio7_22
    }
    #[doc = "0x3dc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_23(&self) -> &Pio7_23 {
        &self.pio7_23
    }
    #[doc = "0x3e0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_24(&self) -> &Pio7_24 {
        &self.pio7_24
    }
    #[doc = "0x3e4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_25(&self) -> &Pio7_25 {
        &self.pio7_25
    }
    #[doc = "0x3e8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_26(&self) -> &Pio7_26 {
        &self.pio7_26
    }
    #[doc = "0x3ec - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_27(&self) -> &Pio7_27 {
        &self.pio7_27
    }
    #[doc = "0x3f0 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_28(&self) -> &Pio7_28 {
        &self.pio7_28
    }
    #[doc = "0x3f4 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_29(&self) -> &Pio7_29 {
        &self.pio7_29
    }
    #[doc = "0x3f8 - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_30(&self) -> &Pio7_30 {
        &self.pio7_30
    }
    #[doc = "0x3fc - iop pad control register for port0 to port5"]
    #[inline(always)]
    pub const fn pio7_31(&self) -> &Pio7_31 {
        &self.pio7_31
    }
    #[doc = "0x400 - Special Registers (No GPIO Function)"]
    #[inline(always)]
    pub const fn fc15_i2c_scl(&self) -> &Fc15I2cScl {
        &self.fc15_i2c_scl
    }
    #[doc = "0x404 - Special Registers (No GPIO Function)"]
    #[inline(always)]
    pub const fn fc15_i2c_sda(&self) -> &Fc15I2cSda {
        &self.fc15_i2c_sda
    }
}
#[doc = "PIO0_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_0`]
module"]
#[doc(alias = "PIO0_0")]
pub type Pio0_0 = crate::Reg<pio0_0::Pio0_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_0;
#[doc = "PIO0_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_1`]
module"]
#[doc(alias = "PIO0_1")]
pub type Pio0_1 = crate::Reg<pio0_1::Pio0_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_1;
#[doc = "PIO0_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_2`]
module"]
#[doc(alias = "PIO0_2")]
pub type Pio0_2 = crate::Reg<pio0_2::Pio0_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_2;
#[doc = "PIO0_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_3`]
module"]
#[doc(alias = "PIO0_3")]
pub type Pio0_3 = crate::Reg<pio0_3::Pio0_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_3;
#[doc = "PIO0_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_4`]
module"]
#[doc(alias = "PIO0_4")]
pub type Pio0_4 = crate::Reg<pio0_4::Pio0_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_4;
#[doc = "PIO0_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_5`]
module"]
#[doc(alias = "PIO0_5")]
pub type Pio0_5 = crate::Reg<pio0_5::Pio0_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_5;
#[doc = "PIO0_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_6`]
module"]
#[doc(alias = "PIO0_6")]
pub type Pio0_6 = crate::Reg<pio0_6::Pio0_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_6;
#[doc = "PIO0_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_7`]
module"]
#[doc(alias = "PIO0_7")]
pub type Pio0_7 = crate::Reg<pio0_7::Pio0_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_7;
#[doc = "PIO0_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_8`]
module"]
#[doc(alias = "PIO0_8")]
pub type Pio0_8 = crate::Reg<pio0_8::Pio0_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_8;
#[doc = "PIO0_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_9`]
module"]
#[doc(alias = "PIO0_9")]
pub type Pio0_9 = crate::Reg<pio0_9::Pio0_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_9;
#[doc = "PIO0_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_10`]
module"]
#[doc(alias = "PIO0_10")]
pub type Pio0_10 = crate::Reg<pio0_10::Pio0_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_10;
#[doc = "PIO0_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_11`]
module"]
#[doc(alias = "PIO0_11")]
pub type Pio0_11 = crate::Reg<pio0_11::Pio0_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_11;
#[doc = "PIO0_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_12`]
module"]
#[doc(alias = "PIO0_12")]
pub type Pio0_12 = crate::Reg<pio0_12::Pio0_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_12;
#[doc = "PIO0_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_13`]
module"]
#[doc(alias = "PIO0_13")]
pub type Pio0_13 = crate::Reg<pio0_13::Pio0_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_13;
#[doc = "PIO0_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_14`]
module"]
#[doc(alias = "PIO0_14")]
pub type Pio0_14 = crate::Reg<pio0_14::Pio0_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_14;
#[doc = "PIO0_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_15`]
module"]
#[doc(alias = "PIO0_15")]
pub type Pio0_15 = crate::Reg<pio0_15::Pio0_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_15;
#[doc = "PIO0_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_16`]
module"]
#[doc(alias = "PIO0_16")]
pub type Pio0_16 = crate::Reg<pio0_16::Pio0_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_16;
#[doc = "PIO0_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_17`]
module"]
#[doc(alias = "PIO0_17")]
pub type Pio0_17 = crate::Reg<pio0_17::Pio0_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_17;
#[doc = "PIO0_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_18`]
module"]
#[doc(alias = "PIO0_18")]
pub type Pio0_18 = crate::Reg<pio0_18::Pio0_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_18;
#[doc = "PIO0_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_19`]
module"]
#[doc(alias = "PIO0_19")]
pub type Pio0_19 = crate::Reg<pio0_19::Pio0_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_19;
#[doc = "PIO0_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_20`]
module"]
#[doc(alias = "PIO0_20")]
pub type Pio0_20 = crate::Reg<pio0_20::Pio0_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_20;
#[doc = "PIO0_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_21`]
module"]
#[doc(alias = "PIO0_21")]
pub type Pio0_21 = crate::Reg<pio0_21::Pio0_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_21;
#[doc = "PIO0_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_22`]
module"]
#[doc(alias = "PIO0_22")]
pub type Pio0_22 = crate::Reg<pio0_22::Pio0_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_22;
#[doc = "PIO0_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_23`]
module"]
#[doc(alias = "PIO0_23")]
pub type Pio0_23 = crate::Reg<pio0_23::Pio0_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_23;
#[doc = "PIO0_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_24`]
module"]
#[doc(alias = "PIO0_24")]
pub type Pio0_24 = crate::Reg<pio0_24::Pio0_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_24;
#[doc = "PIO0_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_25`]
module"]
#[doc(alias = "PIO0_25")]
pub type Pio0_25 = crate::Reg<pio0_25::Pio0_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_25;
#[doc = "PIO0_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_26`]
module"]
#[doc(alias = "PIO0_26")]
pub type Pio0_26 = crate::Reg<pio0_26::Pio0_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_26;
#[doc = "PIO0_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_27`]
module"]
#[doc(alias = "PIO0_27")]
pub type Pio0_27 = crate::Reg<pio0_27::Pio0_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_27;
#[doc = "PIO0_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_28`]
module"]
#[doc(alias = "PIO0_28")]
pub type Pio0_28 = crate::Reg<pio0_28::Pio0_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_28;
#[doc = "PIO0_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_29`]
module"]
#[doc(alias = "PIO0_29")]
pub type Pio0_29 = crate::Reg<pio0_29::Pio0_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_29;
#[doc = "PIO0_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_30`]
module"]
#[doc(alias = "PIO0_30")]
pub type Pio0_30 = crate::Reg<pio0_30::Pio0_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_30;
#[doc = "PIO0_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_31`]
module"]
#[doc(alias = "PIO0_31")]
pub type Pio0_31 = crate::Reg<pio0_31::Pio0_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio0_31;
#[doc = "PIO1_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_0`]
module"]
#[doc(alias = "PIO1_0")]
pub type Pio1_0 = crate::Reg<pio1_0::Pio1_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_0;
#[doc = "PIO1_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_1`]
module"]
#[doc(alias = "PIO1_1")]
pub type Pio1_1 = crate::Reg<pio1_1::Pio1_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_1;
#[doc = "PIO1_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_2`]
module"]
#[doc(alias = "PIO1_2")]
pub type Pio1_2 = crate::Reg<pio1_2::Pio1_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_2;
#[doc = "PIO1_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_3`]
module"]
#[doc(alias = "PIO1_3")]
pub type Pio1_3 = crate::Reg<pio1_3::Pio1_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_3;
#[doc = "PIO1_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_4`]
module"]
#[doc(alias = "PIO1_4")]
pub type Pio1_4 = crate::Reg<pio1_4::Pio1_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_4;
#[doc = "PIO1_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_5`]
module"]
#[doc(alias = "PIO1_5")]
pub type Pio1_5 = crate::Reg<pio1_5::Pio1_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_5;
#[doc = "PIO1_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_6`]
module"]
#[doc(alias = "PIO1_6")]
pub type Pio1_6 = crate::Reg<pio1_6::Pio1_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_6;
#[doc = "PIO1_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_7`]
module"]
#[doc(alias = "PIO1_7")]
pub type Pio1_7 = crate::Reg<pio1_7::Pio1_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_7;
#[doc = "PIO1_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_8`]
module"]
#[doc(alias = "PIO1_8")]
pub type Pio1_8 = crate::Reg<pio1_8::Pio1_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_8;
#[doc = "PIO1_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_9`]
module"]
#[doc(alias = "PIO1_9")]
pub type Pio1_9 = crate::Reg<pio1_9::Pio1_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_9;
#[doc = "PIO1_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_10`]
module"]
#[doc(alias = "PIO1_10")]
pub type Pio1_10 = crate::Reg<pio1_10::Pio1_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_10;
#[doc = "PIO1_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_11`]
module"]
#[doc(alias = "PIO1_11")]
pub type Pio1_11 = crate::Reg<pio1_11::Pio1_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_11;
#[doc = "PIO1_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_12`]
module"]
#[doc(alias = "PIO1_12")]
pub type Pio1_12 = crate::Reg<pio1_12::Pio1_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_12;
#[doc = "PIO1_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_13`]
module"]
#[doc(alias = "PIO1_13")]
pub type Pio1_13 = crate::Reg<pio1_13::Pio1_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_13;
#[doc = "PIO1_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_14`]
module"]
#[doc(alias = "PIO1_14")]
pub type Pio1_14 = crate::Reg<pio1_14::Pio1_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_14;
#[doc = "PIO1_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_15`]
module"]
#[doc(alias = "PIO1_15")]
pub type Pio1_15 = crate::Reg<pio1_15::Pio1_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_15;
#[doc = "PIO1_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_16`]
module"]
#[doc(alias = "PIO1_16")]
pub type Pio1_16 = crate::Reg<pio1_16::Pio1_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_16;
#[doc = "PIO1_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_17`]
module"]
#[doc(alias = "PIO1_17")]
pub type Pio1_17 = crate::Reg<pio1_17::Pio1_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_17;
#[doc = "PIO1_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_18`]
module"]
#[doc(alias = "PIO1_18")]
pub type Pio1_18 = crate::Reg<pio1_18::Pio1_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_18;
#[doc = "PIO1_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_19`]
module"]
#[doc(alias = "PIO1_19")]
pub type Pio1_19 = crate::Reg<pio1_19::Pio1_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_19;
#[doc = "PIO1_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_20`]
module"]
#[doc(alias = "PIO1_20")]
pub type Pio1_20 = crate::Reg<pio1_20::Pio1_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_20;
#[doc = "PIO1_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_21`]
module"]
#[doc(alias = "PIO1_21")]
pub type Pio1_21 = crate::Reg<pio1_21::Pio1_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_21;
#[doc = "PIO1_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_22`]
module"]
#[doc(alias = "PIO1_22")]
pub type Pio1_22 = crate::Reg<pio1_22::Pio1_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_22;
#[doc = "PIO1_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_23`]
module"]
#[doc(alias = "PIO1_23")]
pub type Pio1_23 = crate::Reg<pio1_23::Pio1_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_23;
#[doc = "PIO1_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_24`]
module"]
#[doc(alias = "PIO1_24")]
pub type Pio1_24 = crate::Reg<pio1_24::Pio1_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_24;
#[doc = "PIO1_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_25`]
module"]
#[doc(alias = "PIO1_25")]
pub type Pio1_25 = crate::Reg<pio1_25::Pio1_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_25;
#[doc = "PIO1_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_26`]
module"]
#[doc(alias = "PIO1_26")]
pub type Pio1_26 = crate::Reg<pio1_26::Pio1_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_26;
#[doc = "PIO1_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_27`]
module"]
#[doc(alias = "PIO1_27")]
pub type Pio1_27 = crate::Reg<pio1_27::Pio1_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_27;
#[doc = "PIO1_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_28`]
module"]
#[doc(alias = "PIO1_28")]
pub type Pio1_28 = crate::Reg<pio1_28::Pio1_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_28;
#[doc = "PIO1_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_29`]
module"]
#[doc(alias = "PIO1_29")]
pub type Pio1_29 = crate::Reg<pio1_29::Pio1_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_29;
#[doc = "PIO1_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_30`]
module"]
#[doc(alias = "PIO1_30")]
pub type Pio1_30 = crate::Reg<pio1_30::Pio1_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_30;
#[doc = "PIO1_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_31`]
module"]
#[doc(alias = "PIO1_31")]
pub type Pio1_31 = crate::Reg<pio1_31::Pio1_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio1_31;
#[doc = "PIO2_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_0`]
module"]
#[doc(alias = "PIO2_0")]
pub type Pio2_0 = crate::Reg<pio2_0::Pio2_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_0;
#[doc = "PIO2_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_1`]
module"]
#[doc(alias = "PIO2_1")]
pub type Pio2_1 = crate::Reg<pio2_1::Pio2_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_1;
#[doc = "PIO2_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_2`]
module"]
#[doc(alias = "PIO2_2")]
pub type Pio2_2 = crate::Reg<pio2_2::Pio2_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_2;
#[doc = "PIO2_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_3`]
module"]
#[doc(alias = "PIO2_3")]
pub type Pio2_3 = crate::Reg<pio2_3::Pio2_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_3;
#[doc = "PIO2_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_4`]
module"]
#[doc(alias = "PIO2_4")]
pub type Pio2_4 = crate::Reg<pio2_4::Pio2_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_4;
#[doc = "PIO2_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_5`]
module"]
#[doc(alias = "PIO2_5")]
pub type Pio2_5 = crate::Reg<pio2_5::Pio2_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_5;
#[doc = "PIO2_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_6`]
module"]
#[doc(alias = "PIO2_6")]
pub type Pio2_6 = crate::Reg<pio2_6::Pio2_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_6;
#[doc = "PIO2_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_7`]
module"]
#[doc(alias = "PIO2_7")]
pub type Pio2_7 = crate::Reg<pio2_7::Pio2_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_7;
#[doc = "PIO2_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_8`]
module"]
#[doc(alias = "PIO2_8")]
pub type Pio2_8 = crate::Reg<pio2_8::Pio2_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_8;
#[doc = "PIO2_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_9`]
module"]
#[doc(alias = "PIO2_9")]
pub type Pio2_9 = crate::Reg<pio2_9::Pio2_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_9;
#[doc = "PIO2_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_10`]
module"]
#[doc(alias = "PIO2_10")]
pub type Pio2_10 = crate::Reg<pio2_10::Pio2_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_10;
#[doc = "PIO2_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_11`]
module"]
#[doc(alias = "PIO2_11")]
pub type Pio2_11 = crate::Reg<pio2_11::Pio2_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_11;
#[doc = "PIO2_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_12`]
module"]
#[doc(alias = "PIO2_12")]
pub type Pio2_12 = crate::Reg<pio2_12::Pio2_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_12;
#[doc = "PIO2_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_13`]
module"]
#[doc(alias = "PIO2_13")]
pub type Pio2_13 = crate::Reg<pio2_13::Pio2_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_13;
#[doc = "PIO2_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_14`]
module"]
#[doc(alias = "PIO2_14")]
pub type Pio2_14 = crate::Reg<pio2_14::Pio2_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_14;
#[doc = "PIO2_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_15`]
module"]
#[doc(alias = "PIO2_15")]
pub type Pio2_15 = crate::Reg<pio2_15::Pio2_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_15;
#[doc = "PIO2_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_16`]
module"]
#[doc(alias = "PIO2_16")]
pub type Pio2_16 = crate::Reg<pio2_16::Pio2_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_16;
#[doc = "PIO2_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_17`]
module"]
#[doc(alias = "PIO2_17")]
pub type Pio2_17 = crate::Reg<pio2_17::Pio2_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_17;
#[doc = "PIO2_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_18`]
module"]
#[doc(alias = "PIO2_18")]
pub type Pio2_18 = crate::Reg<pio2_18::Pio2_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_18;
#[doc = "PIO2_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_19`]
module"]
#[doc(alias = "PIO2_19")]
pub type Pio2_19 = crate::Reg<pio2_19::Pio2_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_19;
#[doc = "PIO2_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_20`]
module"]
#[doc(alias = "PIO2_20")]
pub type Pio2_20 = crate::Reg<pio2_20::Pio2_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_20;
#[doc = "PIO2_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_21`]
module"]
#[doc(alias = "PIO2_21")]
pub type Pio2_21 = crate::Reg<pio2_21::Pio2_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_21;
#[doc = "PIO2_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_22`]
module"]
#[doc(alias = "PIO2_22")]
pub type Pio2_22 = crate::Reg<pio2_22::Pio2_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_22;
#[doc = "PIO2_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_23`]
module"]
#[doc(alias = "PIO2_23")]
pub type Pio2_23 = crate::Reg<pio2_23::Pio2_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_23;
#[doc = "PIO2_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_24`]
module"]
#[doc(alias = "PIO2_24")]
pub type Pio2_24 = crate::Reg<pio2_24::Pio2_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_24;
#[doc = "PIO2_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_25`]
module"]
#[doc(alias = "PIO2_25")]
pub type Pio2_25 = crate::Reg<pio2_25::Pio2_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_25;
#[doc = "PIO2_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_26`]
module"]
#[doc(alias = "PIO2_26")]
pub type Pio2_26 = crate::Reg<pio2_26::Pio2_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_26;
#[doc = "PIO2_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_27`]
module"]
#[doc(alias = "PIO2_27")]
pub type Pio2_27 = crate::Reg<pio2_27::Pio2_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_27;
#[doc = "PIO2_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_28`]
module"]
#[doc(alias = "PIO2_28")]
pub type Pio2_28 = crate::Reg<pio2_28::Pio2_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_28;
#[doc = "PIO2_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_29`]
module"]
#[doc(alias = "PIO2_29")]
pub type Pio2_29 = crate::Reg<pio2_29::Pio2_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_29;
#[doc = "PIO2_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_30`]
module"]
#[doc(alias = "PIO2_30")]
pub type Pio2_30 = crate::Reg<pio2_30::Pio2_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_30;
#[doc = "PIO2_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio2_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio2_31`]
module"]
#[doc(alias = "PIO2_31")]
pub type Pio2_31 = crate::Reg<pio2_31::Pio2_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio2_31;
#[doc = "PIO3_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_0`]
module"]
#[doc(alias = "PIO3_0")]
pub type Pio3_0 = crate::Reg<pio3_0::Pio3_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_0;
#[doc = "PIO3_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_1`]
module"]
#[doc(alias = "PIO3_1")]
pub type Pio3_1 = crate::Reg<pio3_1::Pio3_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_1;
#[doc = "PIO3_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_2`]
module"]
#[doc(alias = "PIO3_2")]
pub type Pio3_2 = crate::Reg<pio3_2::Pio3_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_2;
#[doc = "PIO3_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_3`]
module"]
#[doc(alias = "PIO3_3")]
pub type Pio3_3 = crate::Reg<pio3_3::Pio3_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_3;
#[doc = "PIO3_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_4`]
module"]
#[doc(alias = "PIO3_4")]
pub type Pio3_4 = crate::Reg<pio3_4::Pio3_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_4;
#[doc = "PIO3_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_5`]
module"]
#[doc(alias = "PIO3_5")]
pub type Pio3_5 = crate::Reg<pio3_5::Pio3_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_5;
#[doc = "PIO3_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_6`]
module"]
#[doc(alias = "PIO3_6")]
pub type Pio3_6 = crate::Reg<pio3_6::Pio3_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_6;
#[doc = "PIO3_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_7`]
module"]
#[doc(alias = "PIO3_7")]
pub type Pio3_7 = crate::Reg<pio3_7::Pio3_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_7;
#[doc = "PIO3_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_8`]
module"]
#[doc(alias = "PIO3_8")]
pub type Pio3_8 = crate::Reg<pio3_8::Pio3_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_8;
#[doc = "PIO3_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_9`]
module"]
#[doc(alias = "PIO3_9")]
pub type Pio3_9 = crate::Reg<pio3_9::Pio3_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_9;
#[doc = "PIO3_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_10`]
module"]
#[doc(alias = "PIO3_10")]
pub type Pio3_10 = crate::Reg<pio3_10::Pio3_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_10;
#[doc = "PIO3_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_11`]
module"]
#[doc(alias = "PIO3_11")]
pub type Pio3_11 = crate::Reg<pio3_11::Pio3_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_11;
#[doc = "PIO3_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_12`]
module"]
#[doc(alias = "PIO3_12")]
pub type Pio3_12 = crate::Reg<pio3_12::Pio3_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_12;
#[doc = "PIO3_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_13`]
module"]
#[doc(alias = "PIO3_13")]
pub type Pio3_13 = crate::Reg<pio3_13::Pio3_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_13;
#[doc = "PIO3_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_14`]
module"]
#[doc(alias = "PIO3_14")]
pub type Pio3_14 = crate::Reg<pio3_14::Pio3_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_14;
#[doc = "PIO3_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_15`]
module"]
#[doc(alias = "PIO3_15")]
pub type Pio3_15 = crate::Reg<pio3_15::Pio3_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_15;
#[doc = "PIO3_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_16`]
module"]
#[doc(alias = "PIO3_16")]
pub type Pio3_16 = crate::Reg<pio3_16::Pio3_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_16;
#[doc = "PIO3_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_17`]
module"]
#[doc(alias = "PIO3_17")]
pub type Pio3_17 = crate::Reg<pio3_17::Pio3_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_17;
#[doc = "PIO3_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_18`]
module"]
#[doc(alias = "PIO3_18")]
pub type Pio3_18 = crate::Reg<pio3_18::Pio3_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_18;
#[doc = "PIO3_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_19`]
module"]
#[doc(alias = "PIO3_19")]
pub type Pio3_19 = crate::Reg<pio3_19::Pio3_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_19;
#[doc = "PIO3_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_20`]
module"]
#[doc(alias = "PIO3_20")]
pub type Pio3_20 = crate::Reg<pio3_20::Pio3_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_20;
#[doc = "PIO3_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_21`]
module"]
#[doc(alias = "PIO3_21")]
pub type Pio3_21 = crate::Reg<pio3_21::Pio3_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_21;
#[doc = "PIO3_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_22`]
module"]
#[doc(alias = "PIO3_22")]
pub type Pio3_22 = crate::Reg<pio3_22::Pio3_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_22;
#[doc = "PIO3_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_23`]
module"]
#[doc(alias = "PIO3_23")]
pub type Pio3_23 = crate::Reg<pio3_23::Pio3_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_23;
#[doc = "PIO3_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_24`]
module"]
#[doc(alias = "PIO3_24")]
pub type Pio3_24 = crate::Reg<pio3_24::Pio3_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_24;
#[doc = "PIO3_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_25`]
module"]
#[doc(alias = "PIO3_25")]
pub type Pio3_25 = crate::Reg<pio3_25::Pio3_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_25;
#[doc = "PIO3_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_26`]
module"]
#[doc(alias = "PIO3_26")]
pub type Pio3_26 = crate::Reg<pio3_26::Pio3_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_26;
#[doc = "PIO3_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_27`]
module"]
#[doc(alias = "PIO3_27")]
pub type Pio3_27 = crate::Reg<pio3_27::Pio3_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_27;
#[doc = "PIO3_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_28`]
module"]
#[doc(alias = "PIO3_28")]
pub type Pio3_28 = crate::Reg<pio3_28::Pio3_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_28;
#[doc = "PIO3_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_29`]
module"]
#[doc(alias = "PIO3_29")]
pub type Pio3_29 = crate::Reg<pio3_29::Pio3_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_29;
#[doc = "PIO3_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_30`]
module"]
#[doc(alias = "PIO3_30")]
pub type Pio3_30 = crate::Reg<pio3_30::Pio3_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_30;
#[doc = "PIO3_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio3_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio3_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio3_31`]
module"]
#[doc(alias = "PIO3_31")]
pub type Pio3_31 = crate::Reg<pio3_31::Pio3_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio3_31;
#[doc = "PIO4_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_0`]
module"]
#[doc(alias = "PIO4_0")]
pub type Pio4_0 = crate::Reg<pio4_0::Pio4_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_0;
#[doc = "PIO4_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_1`]
module"]
#[doc(alias = "PIO4_1")]
pub type Pio4_1 = crate::Reg<pio4_1::Pio4_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_1;
#[doc = "PIO4_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_2`]
module"]
#[doc(alias = "PIO4_2")]
pub type Pio4_2 = crate::Reg<pio4_2::Pio4_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_2;
#[doc = "PIO4_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_3`]
module"]
#[doc(alias = "PIO4_3")]
pub type Pio4_3 = crate::Reg<pio4_3::Pio4_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_3;
#[doc = "PIO4_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_4`]
module"]
#[doc(alias = "PIO4_4")]
pub type Pio4_4 = crate::Reg<pio4_4::Pio4_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_4;
#[doc = "PIO4_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_5`]
module"]
#[doc(alias = "PIO4_5")]
pub type Pio4_5 = crate::Reg<pio4_5::Pio4_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_5;
#[doc = "PIO4_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_6`]
module"]
#[doc(alias = "PIO4_6")]
pub type Pio4_6 = crate::Reg<pio4_6::Pio4_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_6;
#[doc = "PIO4_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_7`]
module"]
#[doc(alias = "PIO4_7")]
pub type Pio4_7 = crate::Reg<pio4_7::Pio4_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_7;
#[doc = "PIO4_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_8`]
module"]
#[doc(alias = "PIO4_8")]
pub type Pio4_8 = crate::Reg<pio4_8::Pio4_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_8;
#[doc = "PIO4_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_9`]
module"]
#[doc(alias = "PIO4_9")]
pub type Pio4_9 = crate::Reg<pio4_9::Pio4_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_9;
#[doc = "PIO4_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_10`]
module"]
#[doc(alias = "PIO4_10")]
pub type Pio4_10 = crate::Reg<pio4_10::Pio4_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_10;
#[doc = "PIO4_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_11`]
module"]
#[doc(alias = "PIO4_11")]
pub type Pio4_11 = crate::Reg<pio4_11::Pio4_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_11;
#[doc = "PIO4_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_12`]
module"]
#[doc(alias = "PIO4_12")]
pub type Pio4_12 = crate::Reg<pio4_12::Pio4_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_12;
#[doc = "PIO4_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_13`]
module"]
#[doc(alias = "PIO4_13")]
pub type Pio4_13 = crate::Reg<pio4_13::Pio4_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_13;
#[doc = "PIO4_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_14`]
module"]
#[doc(alias = "PIO4_14")]
pub type Pio4_14 = crate::Reg<pio4_14::Pio4_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_14;
#[doc = "PIO4_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_15`]
module"]
#[doc(alias = "PIO4_15")]
pub type Pio4_15 = crate::Reg<pio4_15::Pio4_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_15;
#[doc = "PIO4_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_16`]
module"]
#[doc(alias = "PIO4_16")]
pub type Pio4_16 = crate::Reg<pio4_16::Pio4_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_16;
#[doc = "PIO4_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_17`]
module"]
#[doc(alias = "PIO4_17")]
pub type Pio4_17 = crate::Reg<pio4_17::Pio4_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_17;
#[doc = "PIO4_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_18`]
module"]
#[doc(alias = "PIO4_18")]
pub type Pio4_18 = crate::Reg<pio4_18::Pio4_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_18;
#[doc = "PIO4_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_19`]
module"]
#[doc(alias = "PIO4_19")]
pub type Pio4_19 = crate::Reg<pio4_19::Pio4_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_19;
#[doc = "PIO4_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_20`]
module"]
#[doc(alias = "PIO4_20")]
pub type Pio4_20 = crate::Reg<pio4_20::Pio4_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_20;
#[doc = "PIO4_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_21`]
module"]
#[doc(alias = "PIO4_21")]
pub type Pio4_21 = crate::Reg<pio4_21::Pio4_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_21;
#[doc = "PIO4_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_22`]
module"]
#[doc(alias = "PIO4_22")]
pub type Pio4_22 = crate::Reg<pio4_22::Pio4_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_22;
#[doc = "PIO4_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_23`]
module"]
#[doc(alias = "PIO4_23")]
pub type Pio4_23 = crate::Reg<pio4_23::Pio4_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_23;
#[doc = "PIO4_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_24`]
module"]
#[doc(alias = "PIO4_24")]
pub type Pio4_24 = crate::Reg<pio4_24::Pio4_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_24;
#[doc = "PIO4_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_25`]
module"]
#[doc(alias = "PIO4_25")]
pub type Pio4_25 = crate::Reg<pio4_25::Pio4_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_25;
#[doc = "PIO4_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_26`]
module"]
#[doc(alias = "PIO4_26")]
pub type Pio4_26 = crate::Reg<pio4_26::Pio4_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_26;
#[doc = "PIO4_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_27`]
module"]
#[doc(alias = "PIO4_27")]
pub type Pio4_27 = crate::Reg<pio4_27::Pio4_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_27;
#[doc = "PIO4_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_28`]
module"]
#[doc(alias = "PIO4_28")]
pub type Pio4_28 = crate::Reg<pio4_28::Pio4_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_28;
#[doc = "PIO4_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_29`]
module"]
#[doc(alias = "PIO4_29")]
pub type Pio4_29 = crate::Reg<pio4_29::Pio4_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_29;
#[doc = "PIO4_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_30`]
module"]
#[doc(alias = "PIO4_30")]
pub type Pio4_30 = crate::Reg<pio4_30::Pio4_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_30;
#[doc = "PIO4_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio4_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4_31`]
module"]
#[doc(alias = "PIO4_31")]
pub type Pio4_31 = crate::Reg<pio4_31::Pio4_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio4_31;
#[doc = "PIO5_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_0`]
module"]
#[doc(alias = "PIO5_0")]
pub type Pio5_0 = crate::Reg<pio5_0::Pio5_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_0;
#[doc = "PIO5_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_1`]
module"]
#[doc(alias = "PIO5_1")]
pub type Pio5_1 = crate::Reg<pio5_1::Pio5_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_1;
#[doc = "PIO5_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_2`]
module"]
#[doc(alias = "PIO5_2")]
pub type Pio5_2 = crate::Reg<pio5_2::Pio5_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_2;
#[doc = "PIO5_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_3`]
module"]
#[doc(alias = "PIO5_3")]
pub type Pio5_3 = crate::Reg<pio5_3::Pio5_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_3;
#[doc = "PIO5_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_4`]
module"]
#[doc(alias = "PIO5_4")]
pub type Pio5_4 = crate::Reg<pio5_4::Pio5_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_4;
#[doc = "PIO5_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_5`]
module"]
#[doc(alias = "PIO5_5")]
pub type Pio5_5 = crate::Reg<pio5_5::Pio5_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_5;
#[doc = "PIO5_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_6`]
module"]
#[doc(alias = "PIO5_6")]
pub type Pio5_6 = crate::Reg<pio5_6::Pio5_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_6;
#[doc = "PIO5_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_7`]
module"]
#[doc(alias = "PIO5_7")]
pub type Pio5_7 = crate::Reg<pio5_7::Pio5_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_7;
#[doc = "PIO5_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_8`]
module"]
#[doc(alias = "PIO5_8")]
pub type Pio5_8 = crate::Reg<pio5_8::Pio5_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_8;
#[doc = "PIO5_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_9`]
module"]
#[doc(alias = "PIO5_9")]
pub type Pio5_9 = crate::Reg<pio5_9::Pio5_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_9;
#[doc = "PIO5_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_10`]
module"]
#[doc(alias = "PIO5_10")]
pub type Pio5_10 = crate::Reg<pio5_10::Pio5_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_10;
#[doc = "PIO5_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_11`]
module"]
#[doc(alias = "PIO5_11")]
pub type Pio5_11 = crate::Reg<pio5_11::Pio5_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_11;
#[doc = "PIO5_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_12`]
module"]
#[doc(alias = "PIO5_12")]
pub type Pio5_12 = crate::Reg<pio5_12::Pio5_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_12;
#[doc = "PIO5_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_13`]
module"]
#[doc(alias = "PIO5_13")]
pub type Pio5_13 = crate::Reg<pio5_13::Pio5_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_13;
#[doc = "PIO5_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_14`]
module"]
#[doc(alias = "PIO5_14")]
pub type Pio5_14 = crate::Reg<pio5_14::Pio5_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_14;
#[doc = "PIO5_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_15`]
module"]
#[doc(alias = "PIO5_15")]
pub type Pio5_15 = crate::Reg<pio5_15::Pio5_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_15;
#[doc = "PIO5_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_16`]
module"]
#[doc(alias = "PIO5_16")]
pub type Pio5_16 = crate::Reg<pio5_16::Pio5_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_16;
#[doc = "PIO5_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_17`]
module"]
#[doc(alias = "PIO5_17")]
pub type Pio5_17 = crate::Reg<pio5_17::Pio5_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_17;
#[doc = "PIO5_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_18`]
module"]
#[doc(alias = "PIO5_18")]
pub type Pio5_18 = crate::Reg<pio5_18::Pio5_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_18;
#[doc = "PIO5_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_19`]
module"]
#[doc(alias = "PIO5_19")]
pub type Pio5_19 = crate::Reg<pio5_19::Pio5_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_19;
#[doc = "PIO5_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_20`]
module"]
#[doc(alias = "PIO5_20")]
pub type Pio5_20 = crate::Reg<pio5_20::Pio5_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_20;
#[doc = "PIO5_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_21`]
module"]
#[doc(alias = "PIO5_21")]
pub type Pio5_21 = crate::Reg<pio5_21::Pio5_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_21;
#[doc = "PIO5_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_22`]
module"]
#[doc(alias = "PIO5_22")]
pub type Pio5_22 = crate::Reg<pio5_22::Pio5_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_22;
#[doc = "PIO5_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_23`]
module"]
#[doc(alias = "PIO5_23")]
pub type Pio5_23 = crate::Reg<pio5_23::Pio5_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_23;
#[doc = "PIO5_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_24`]
module"]
#[doc(alias = "PIO5_24")]
pub type Pio5_24 = crate::Reg<pio5_24::Pio5_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_24;
#[doc = "PIO5_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_25`]
module"]
#[doc(alias = "PIO5_25")]
pub type Pio5_25 = crate::Reg<pio5_25::Pio5_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_25;
#[doc = "PIO5_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_26`]
module"]
#[doc(alias = "PIO5_26")]
pub type Pio5_26 = crate::Reg<pio5_26::Pio5_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_26;
#[doc = "PIO5_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_27`]
module"]
#[doc(alias = "PIO5_27")]
pub type Pio5_27 = crate::Reg<pio5_27::Pio5_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_27;
#[doc = "PIO5_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_28`]
module"]
#[doc(alias = "PIO5_28")]
pub type Pio5_28 = crate::Reg<pio5_28::Pio5_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_28;
#[doc = "PIO5_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_29`]
module"]
#[doc(alias = "PIO5_29")]
pub type Pio5_29 = crate::Reg<pio5_29::Pio5_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_29;
#[doc = "PIO5_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_30`]
module"]
#[doc(alias = "PIO5_30")]
pub type Pio5_30 = crate::Reg<pio5_30::Pio5_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_30;
#[doc = "PIO5_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio5_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio5_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio5_31`]
module"]
#[doc(alias = "PIO5_31")]
pub type Pio5_31 = crate::Reg<pio5_31::Pio5_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio5_31;
#[doc = "PIO6_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_0`]
module"]
#[doc(alias = "PIO6_0")]
pub type Pio6_0 = crate::Reg<pio6_0::Pio6_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_0;
#[doc = "PIO6_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_1`]
module"]
#[doc(alias = "PIO6_1")]
pub type Pio6_1 = crate::Reg<pio6_1::Pio6_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_1;
#[doc = "PIO6_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_2`]
module"]
#[doc(alias = "PIO6_2")]
pub type Pio6_2 = crate::Reg<pio6_2::Pio6_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_2;
#[doc = "PIO6_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_3`]
module"]
#[doc(alias = "PIO6_3")]
pub type Pio6_3 = crate::Reg<pio6_3::Pio6_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_3;
#[doc = "PIO6_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_4`]
module"]
#[doc(alias = "PIO6_4")]
pub type Pio6_4 = crate::Reg<pio6_4::Pio6_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_4;
#[doc = "PIO6_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_5`]
module"]
#[doc(alias = "PIO6_5")]
pub type Pio6_5 = crate::Reg<pio6_5::Pio6_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_5;
#[doc = "PIO6_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_6`]
module"]
#[doc(alias = "PIO6_6")]
pub type Pio6_6 = crate::Reg<pio6_6::Pio6_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_6;
#[doc = "PIO6_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_7`]
module"]
#[doc(alias = "PIO6_7")]
pub type Pio6_7 = crate::Reg<pio6_7::Pio6_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_7;
#[doc = "PIO6_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_8`]
module"]
#[doc(alias = "PIO6_8")]
pub type Pio6_8 = crate::Reg<pio6_8::Pio6_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_8;
#[doc = "PIO6_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_9`]
module"]
#[doc(alias = "PIO6_9")]
pub type Pio6_9 = crate::Reg<pio6_9::Pio6_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_9;
#[doc = "PIO6_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_10`]
module"]
#[doc(alias = "PIO6_10")]
pub type Pio6_10 = crate::Reg<pio6_10::Pio6_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_10;
#[doc = "PIO6_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_11`]
module"]
#[doc(alias = "PIO6_11")]
pub type Pio6_11 = crate::Reg<pio6_11::Pio6_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_11;
#[doc = "PIO6_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_12`]
module"]
#[doc(alias = "PIO6_12")]
pub type Pio6_12 = crate::Reg<pio6_12::Pio6_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_12;
#[doc = "PIO6_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_13`]
module"]
#[doc(alias = "PIO6_13")]
pub type Pio6_13 = crate::Reg<pio6_13::Pio6_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_13;
#[doc = "PIO6_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_14`]
module"]
#[doc(alias = "PIO6_14")]
pub type Pio6_14 = crate::Reg<pio6_14::Pio6_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_14;
#[doc = "PIO6_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_15`]
module"]
#[doc(alias = "PIO6_15")]
pub type Pio6_15 = crate::Reg<pio6_15::Pio6_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_15;
#[doc = "PIO6_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_16`]
module"]
#[doc(alias = "PIO6_16")]
pub type Pio6_16 = crate::Reg<pio6_16::Pio6_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_16;
#[doc = "PIO6_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_17`]
module"]
#[doc(alias = "PIO6_17")]
pub type Pio6_17 = crate::Reg<pio6_17::Pio6_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_17;
#[doc = "PIO6_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_18`]
module"]
#[doc(alias = "PIO6_18")]
pub type Pio6_18 = crate::Reg<pio6_18::Pio6_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_18;
#[doc = "PIO6_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_19`]
module"]
#[doc(alias = "PIO6_19")]
pub type Pio6_19 = crate::Reg<pio6_19::Pio6_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_19;
#[doc = "PIO6_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_20`]
module"]
#[doc(alias = "PIO6_20")]
pub type Pio6_20 = crate::Reg<pio6_20::Pio6_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_20;
#[doc = "PIO6_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_21`]
module"]
#[doc(alias = "PIO6_21")]
pub type Pio6_21 = crate::Reg<pio6_21::Pio6_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_21;
#[doc = "PIO6_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_22`]
module"]
#[doc(alias = "PIO6_22")]
pub type Pio6_22 = crate::Reg<pio6_22::Pio6_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_22;
#[doc = "PIO6_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_23`]
module"]
#[doc(alias = "PIO6_23")]
pub type Pio6_23 = crate::Reg<pio6_23::Pio6_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_23;
#[doc = "PIO6_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_24`]
module"]
#[doc(alias = "PIO6_24")]
pub type Pio6_24 = crate::Reg<pio6_24::Pio6_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_24;
#[doc = "PIO6_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_25`]
module"]
#[doc(alias = "PIO6_25")]
pub type Pio6_25 = crate::Reg<pio6_25::Pio6_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_25;
#[doc = "PIO6_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_26`]
module"]
#[doc(alias = "PIO6_26")]
pub type Pio6_26 = crate::Reg<pio6_26::Pio6_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_26;
#[doc = "PIO6_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_27`]
module"]
#[doc(alias = "PIO6_27")]
pub type Pio6_27 = crate::Reg<pio6_27::Pio6_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_27;
#[doc = "PIO6_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_28`]
module"]
#[doc(alias = "PIO6_28")]
pub type Pio6_28 = crate::Reg<pio6_28::Pio6_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_28;
#[doc = "PIO6_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_29`]
module"]
#[doc(alias = "PIO6_29")]
pub type Pio6_29 = crate::Reg<pio6_29::Pio6_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_29;
#[doc = "PIO6_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_30`]
module"]
#[doc(alias = "PIO6_30")]
pub type Pio6_30 = crate::Reg<pio6_30::Pio6_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_30;
#[doc = "PIO6_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio6_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio6_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio6_31`]
module"]
#[doc(alias = "PIO6_31")]
pub type Pio6_31 = crate::Reg<pio6_31::Pio6_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio6_31;
#[doc = "PIO7_0 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_0`]
module"]
#[doc(alias = "PIO7_0")]
pub type Pio7_0 = crate::Reg<pio7_0::Pio7_0Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_0;
#[doc = "PIO7_1 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_1`]
module"]
#[doc(alias = "PIO7_1")]
pub type Pio7_1 = crate::Reg<pio7_1::Pio7_1Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_1;
#[doc = "PIO7_2 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_2`]
module"]
#[doc(alias = "PIO7_2")]
pub type Pio7_2 = crate::Reg<pio7_2::Pio7_2Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_2;
#[doc = "PIO7_3 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_3`]
module"]
#[doc(alias = "PIO7_3")]
pub type Pio7_3 = crate::Reg<pio7_3::Pio7_3Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_3;
#[doc = "PIO7_4 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_4`]
module"]
#[doc(alias = "PIO7_4")]
pub type Pio7_4 = crate::Reg<pio7_4::Pio7_4Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_4;
#[doc = "PIO7_5 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_5`]
module"]
#[doc(alias = "PIO7_5")]
pub type Pio7_5 = crate::Reg<pio7_5::Pio7_5Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_5;
#[doc = "PIO7_6 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_6`]
module"]
#[doc(alias = "PIO7_6")]
pub type Pio7_6 = crate::Reg<pio7_6::Pio7_6Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_6;
#[doc = "PIO7_7 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_7`]
module"]
#[doc(alias = "PIO7_7")]
pub type Pio7_7 = crate::Reg<pio7_7::Pio7_7Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_7;
#[doc = "PIO7_8 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_8`]
module"]
#[doc(alias = "PIO7_8")]
pub type Pio7_8 = crate::Reg<pio7_8::Pio7_8Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_8;
#[doc = "PIO7_9 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_9`]
module"]
#[doc(alias = "PIO7_9")]
pub type Pio7_9 = crate::Reg<pio7_9::Pio7_9Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_9;
#[doc = "PIO7_10 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_10`]
module"]
#[doc(alias = "PIO7_10")]
pub type Pio7_10 = crate::Reg<pio7_10::Pio7_10Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_10;
#[doc = "PIO7_11 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_11`]
module"]
#[doc(alias = "PIO7_11")]
pub type Pio7_11 = crate::Reg<pio7_11::Pio7_11Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_11;
#[doc = "PIO7_12 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_12`]
module"]
#[doc(alias = "PIO7_12")]
pub type Pio7_12 = crate::Reg<pio7_12::Pio7_12Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_12;
#[doc = "PIO7_13 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_13`]
module"]
#[doc(alias = "PIO7_13")]
pub type Pio7_13 = crate::Reg<pio7_13::Pio7_13Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_13;
#[doc = "PIO7_14 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_14`]
module"]
#[doc(alias = "PIO7_14")]
pub type Pio7_14 = crate::Reg<pio7_14::Pio7_14Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_14;
#[doc = "PIO7_15 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_15`]
module"]
#[doc(alias = "PIO7_15")]
pub type Pio7_15 = crate::Reg<pio7_15::Pio7_15Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_15;
#[doc = "PIO7_16 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_16`]
module"]
#[doc(alias = "PIO7_16")]
pub type Pio7_16 = crate::Reg<pio7_16::Pio7_16Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_16;
#[doc = "PIO7_17 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_17`]
module"]
#[doc(alias = "PIO7_17")]
pub type Pio7_17 = crate::Reg<pio7_17::Pio7_17Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_17;
#[doc = "PIO7_18 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_18`]
module"]
#[doc(alias = "PIO7_18")]
pub type Pio7_18 = crate::Reg<pio7_18::Pio7_18Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_18;
#[doc = "PIO7_19 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_19`]
module"]
#[doc(alias = "PIO7_19")]
pub type Pio7_19 = crate::Reg<pio7_19::Pio7_19Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_19;
#[doc = "PIO7_20 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_20`]
module"]
#[doc(alias = "PIO7_20")]
pub type Pio7_20 = crate::Reg<pio7_20::Pio7_20Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_20;
#[doc = "PIO7_21 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_21`]
module"]
#[doc(alias = "PIO7_21")]
pub type Pio7_21 = crate::Reg<pio7_21::Pio7_21Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_21;
#[doc = "PIO7_22 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_22`]
module"]
#[doc(alias = "PIO7_22")]
pub type Pio7_22 = crate::Reg<pio7_22::Pio7_22Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_22;
#[doc = "PIO7_23 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_23`]
module"]
#[doc(alias = "PIO7_23")]
pub type Pio7_23 = crate::Reg<pio7_23::Pio7_23Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_23;
#[doc = "PIO7_24 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_24`]
module"]
#[doc(alias = "PIO7_24")]
pub type Pio7_24 = crate::Reg<pio7_24::Pio7_24Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_24;
#[doc = "PIO7_25 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_25`]
module"]
#[doc(alias = "PIO7_25")]
pub type Pio7_25 = crate::Reg<pio7_25::Pio7_25Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_25;
#[doc = "PIO7_26 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_26`]
module"]
#[doc(alias = "PIO7_26")]
pub type Pio7_26 = crate::Reg<pio7_26::Pio7_26Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_26;
#[doc = "PIO7_27 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_27`]
module"]
#[doc(alias = "PIO7_27")]
pub type Pio7_27 = crate::Reg<pio7_27::Pio7_27Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_27;
#[doc = "PIO7_28 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_28`]
module"]
#[doc(alias = "PIO7_28")]
pub type Pio7_28 = crate::Reg<pio7_28::Pio7_28Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_28;
#[doc = "PIO7_29 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_29`]
module"]
#[doc(alias = "PIO7_29")]
pub type Pio7_29 = crate::Reg<pio7_29::Pio7_29Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_29;
#[doc = "PIO7_30 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_30`]
module"]
#[doc(alias = "PIO7_30")]
pub type Pio7_30 = crate::Reg<pio7_30::Pio7_30Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_30;
#[doc = "PIO7_31 (rw) register accessor: iop pad control register for port0 to port5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio7_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio7_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio7_31`]
module"]
#[doc(alias = "PIO7_31")]
pub type Pio7_31 = crate::Reg<pio7_31::Pio7_31Spec>;
#[doc = "iop pad control register for port0 to port5"]
pub mod pio7_31;
#[doc = "FC15_I2C_SCL (rw) register accessor: Special Registers (No GPIO Function)\n\nYou can [`read`](crate::Reg::read) this register and get [`fc15_i2c_scl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc15_i2c_scl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc15_i2c_scl`]
module"]
#[doc(alias = "FC15_I2C_SCL")]
pub type Fc15I2cScl = crate::Reg<fc15_i2c_scl::Fc15I2cSclSpec>;
#[doc = "Special Registers (No GPIO Function)"]
pub mod fc15_i2c_scl;
#[doc = "FC15_I2C_SDA (rw) register accessor: Special Registers (No GPIO Function)\n\nYou can [`read`](crate::Reg::read) this register and get [`fc15_i2c_sda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc15_i2c_sda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc15_i2c_sda`]
module"]
#[doc(alias = "FC15_I2C_SDA")]
pub type Fc15I2cSda = crate::Reg<fc15_i2c_sda::Fc15I2cSdaSpec>;
#[doc = "Special Registers (No GPIO Function)"]
pub mod fc15_i2c_sda;
