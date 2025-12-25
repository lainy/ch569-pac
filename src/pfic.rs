#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr1: Isr1,
    isr2: Isr2,
    _reserved2: [u8; 0x18],
    ipr1: Ipr1,
    ipr2: Ipr2,
    _reserved4: [u8; 0x18],
    ithresdr: Ithresdr,
    fibaddrr: Fibaddrr,
    cfgr: Cfgr,
    gisr: Gisr,
    _reserved8: [u8; 0x10],
    fifoaddrr0: Fifoaddrr0,
    fifoaddrr1: Fifoaddrr1,
    fifoaddrr2: Fifoaddrr2,
    fifoaddrr3: Fifoaddrr3,
    _reserved12: [u8; 0x90],
    ienr1: Ienr1,
    ienr2: Ienr2,
    _reserved14: [u8; 0x78],
    irer1: Irer1,
    irer2: Irer2,
    _reserved16: [u8; 0x78],
    ipsr1: Ipsr1,
    ipsr2: Ipsr2,
    _reserved18: [u8; 0x78],
    iprr1: Iprr1,
    iprr2: Iprr2,
    _reserved20: [u8; 0x78],
    iactr1: Iactr1,
    iactr2: Iactr2,
    _reserved22: [u8; 0xf8],
    iprior0: Iprior0,
    _reserved23: [u8; 0x1c],
    iprior1: Iprior1,
    _reserved24: [u8; 0x1c],
    iprior2: Iprior2,
    _reserved25: [u8; 0x1c],
    iprior3: Iprior3,
    _reserved26: [u8; 0x1c],
    iprior4: Iprior4,
    _reserved27: [u8; 0x1c],
    iprior5: Iprior5,
    _reserved28: [u8; 0x1c],
    iprior6: Iprior6,
    _reserved29: [u8; 0x1c],
    iprior7: Iprior7,
    _reserved30: [u8; 0x1c],
    iprior8: Iprior8,
    _reserved31: [u8; 0x1c],
    iprior9: Iprior9,
    _reserved32: [u8; 0x1c],
    iprior10: Iprior10,
    _reserved33: [u8; 0x1c],
    iprior11: Iprior11,
    _reserved34: [u8; 0x1c],
    iprior12: Iprior12,
    _reserved35: [u8; 0x1c],
    iprior13: Iprior13,
    _reserved36: [u8; 0x1c],
    iprior14: Iprior14,
    _reserved37: [u8; 0x1c],
    iprior15: Iprior15,
    _reserved38: [u8; 0x1c],
    iprior16: Iprior16,
    _reserved39: [u8; 0x1c],
    iprior17: Iprior17,
    _reserved40: [u8; 0x1c],
    iprior18: Iprior18,
    _reserved41: [u8; 0x1c],
    iprior19: Iprior19,
    _reserved42: [u8; 0x1c],
    iprior20: Iprior20,
    _reserved43: [u8; 0x1c],
    iprior21: Iprior21,
    _reserved44: [u8; 0x1c],
    iprior22: Iprior22,
    _reserved45: [u8; 0x1c],
    iprior23: Iprior23,
    _reserved46: [u8; 0x1c],
    iprior24: Iprior24,
    _reserved47: [u8; 0x1c],
    iprior25: Iprior25,
    _reserved48: [u8; 0x1c],
    iprior26: Iprior26,
    _reserved49: [u8; 0x1c],
    iprior27: Iprior27,
    _reserved50: [u8; 0x1c],
    iprior28: Iprior28,
    _reserved51: [u8; 0x1c],
    iprior29: Iprior29,
    _reserved52: [u8; 0x1c],
    iprior30: Iprior30,
    _reserved53: [u8; 0x1c],
    iprior31: Iprior31,
    _reserved54: [u8; 0x1c],
    iprior32: Iprior32,
    _reserved55: [u8; 0x1c],
    iprior33: Iprior33,
    _reserved56: [u8; 0x1c],
    iprior34: Iprior34,
    _reserved57: [u8; 0x1c],
    iprior35: Iprior35,
    _reserved58: [u8; 0x1c],
    iprior36: Iprior36,
    _reserved59: [u8; 0x1c],
    iprior37: Iprior37,
    _reserved60: [u8; 0x1c],
    iprior38: Iprior38,
    _reserved61: [u8; 0x1c],
    iprior39: Iprior39,
    _reserved62: [u8; 0x1c],
    iprior40: Iprior40,
    _reserved63: [u8; 0x1c],
    iprior41: Iprior41,
    _reserved64: [u8; 0x1c],
    iprior42: Iprior42,
    _reserved65: [u8; 0x1c],
    iprior43: Iprior43,
    _reserved66: [u8; 0x1c],
    iprior44: Iprior44,
    _reserved67: [u8; 0x1c],
    iprior45: Iprior45,
    _reserved68: [u8; 0x1c],
    iprior46: Iprior46,
    _reserved69: [u8; 0x1c],
    iprior47: Iprior47,
    _reserved70: [u8; 0x1c],
    iprior48: Iprior48,
    _reserved71: [u8; 0x1c],
    iprior49: Iprior49,
    _reserved72: [u8; 0x1c],
    iprior50: Iprior50,
    _reserved73: [u8; 0x1c],
    iprior51: Iprior51,
    _reserved74: [u8; 0x1c],
    iprior52: Iprior52,
    _reserved75: [u8; 0x1c],
    iprior53: Iprior53,
    _reserved76: [u8; 0x1c],
    iprior54: Iprior54,
    _reserved77: [u8; 0x1c],
    iprior55: Iprior55,
    _reserved78: [u8; 0x1c],
    iprior56: Iprior56,
    _reserved79: [u8; 0x1c],
    iprior57: Iprior57,
    _reserved80: [u8; 0x1c],
    iprior58: Iprior58,
    _reserved81: [u8; 0x1c],
    iprior59: Iprior59,
    _reserved82: [u8; 0x1c],
    iprior60: Iprior60,
    _reserved83: [u8; 0x1c],
    iprior61: Iprior61,
    _reserved84: [u8; 0x1c],
    iprior62: Iprior62,
    _reserved85: [u8; 0x1c],
    iprior63: Iprior63,
    _reserved86: [u8; 0x012c],
    sctlr: Sctlr,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr1(&self) -> &Isr1 {
        &self.isr1
    }
    #[doc = "0x04 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr2(&self) -> &Isr2 {
        &self.isr2
    }
    #[doc = "0x20 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ipr1(&self) -> &Ipr1 {
        &self.ipr1
    }
    #[doc = "0x24 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ipr2(&self) -> &Ipr2 {
        &self.ipr2
    }
    #[doc = "0x40 - Interrupt Priority Register"]
    #[inline(always)]
    pub const fn ithresdr(&self) -> &Ithresdr {
        &self.ithresdr
    }
    #[doc = "0x44 - Interrupt Fast Address Register"]
    #[inline(always)]
    pub const fn fibaddrr(&self) -> &Fibaddrr {
        &self.fibaddrr
    }
    #[doc = "0x48 - Interrupt Config Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x4c - Interrupt Global Register"]
    #[inline(always)]
    pub const fn gisr(&self) -> &Gisr {
        &self.gisr
    }
    #[doc = "0x60 - Interrupt 0 address Register"]
    #[inline(always)]
    pub const fn fifoaddrr0(&self) -> &Fifoaddrr0 {
        &self.fifoaddrr0
    }
    #[doc = "0x64 - Interrupt 1 address Register"]
    #[inline(always)]
    pub const fn fifoaddrr1(&self) -> &Fifoaddrr1 {
        &self.fifoaddrr1
    }
    #[doc = "0x68 - Interrupt 2 address Register"]
    #[inline(always)]
    pub const fn fifoaddrr2(&self) -> &Fifoaddrr2 {
        &self.fifoaddrr2
    }
    #[doc = "0x6c - Interrupt 3 address Register"]
    #[inline(always)]
    pub const fn fifoaddrr3(&self) -> &Fifoaddrr3 {
        &self.fifoaddrr3
    }
    #[doc = "0x100 - Interrupt Setting Register"]
    #[inline(always)]
    pub const fn ienr1(&self) -> &Ienr1 {
        &self.ienr1
    }
    #[doc = "0x104 - Interrupt Setting Register"]
    #[inline(always)]
    pub const fn ienr2(&self) -> &Ienr2 {
        &self.ienr2
    }
    #[doc = "0x180 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn irer1(&self) -> &Irer1 {
        &self.irer1
    }
    #[doc = "0x184 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn irer2(&self) -> &Irer2 {
        &self.irer2
    }
    #[doc = "0x200 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ipsr1(&self) -> &Ipsr1 {
        &self.ipsr1
    }
    #[doc = "0x204 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ipsr2(&self) -> &Ipsr2 {
        &self.ipsr2
    }
    #[doc = "0x280 - Interrupt Pending Clear Register"]
    #[inline(always)]
    pub const fn iprr1(&self) -> &Iprr1 {
        &self.iprr1
    }
    #[doc = "0x284 - Interrupt Pending Clear Register"]
    #[inline(always)]
    pub const fn iprr2(&self) -> &Iprr2 {
        &self.iprr2
    }
    #[doc = "0x300 - Interrupt ACTIVE Register"]
    #[inline(always)]
    pub const fn iactr1(&self) -> &Iactr1 {
        &self.iactr1
    }
    #[doc = "0x304 - Interrupt ACTIVE Register"]
    #[inline(always)]
    pub const fn iactr2(&self) -> &Iactr2 {
        &self.iactr2
    }
    #[doc = "0x400 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior0(&self) -> &Iprior0 {
        &self.iprior0
    }
    #[doc = "0x420 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior1(&self) -> &Iprior1 {
        &self.iprior1
    }
    #[doc = "0x440 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior2(&self) -> &Iprior2 {
        &self.iprior2
    }
    #[doc = "0x460 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior3(&self) -> &Iprior3 {
        &self.iprior3
    }
    #[doc = "0x480 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior4(&self) -> &Iprior4 {
        &self.iprior4
    }
    #[doc = "0x4a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior5(&self) -> &Iprior5 {
        &self.iprior5
    }
    #[doc = "0x4c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior6(&self) -> &Iprior6 {
        &self.iprior6
    }
    #[doc = "0x4e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior7(&self) -> &Iprior7 {
        &self.iprior7
    }
    #[doc = "0x500 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior8(&self) -> &Iprior8 {
        &self.iprior8
    }
    #[doc = "0x520 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior9(&self) -> &Iprior9 {
        &self.iprior9
    }
    #[doc = "0x540 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior10(&self) -> &Iprior10 {
        &self.iprior10
    }
    #[doc = "0x560 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior11(&self) -> &Iprior11 {
        &self.iprior11
    }
    #[doc = "0x580 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior12(&self) -> &Iprior12 {
        &self.iprior12
    }
    #[doc = "0x5a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior13(&self) -> &Iprior13 {
        &self.iprior13
    }
    #[doc = "0x5c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior14(&self) -> &Iprior14 {
        &self.iprior14
    }
    #[doc = "0x5e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior15(&self) -> &Iprior15 {
        &self.iprior15
    }
    #[doc = "0x600 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior16(&self) -> &Iprior16 {
        &self.iprior16
    }
    #[doc = "0x620 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior17(&self) -> &Iprior17 {
        &self.iprior17
    }
    #[doc = "0x640 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior18(&self) -> &Iprior18 {
        &self.iprior18
    }
    #[doc = "0x660 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior19(&self) -> &Iprior19 {
        &self.iprior19
    }
    #[doc = "0x680 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior20(&self) -> &Iprior20 {
        &self.iprior20
    }
    #[doc = "0x6a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior21(&self) -> &Iprior21 {
        &self.iprior21
    }
    #[doc = "0x6c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior22(&self) -> &Iprior22 {
        &self.iprior22
    }
    #[doc = "0x6e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior23(&self) -> &Iprior23 {
        &self.iprior23
    }
    #[doc = "0x700 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior24(&self) -> &Iprior24 {
        &self.iprior24
    }
    #[doc = "0x720 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior25(&self) -> &Iprior25 {
        &self.iprior25
    }
    #[doc = "0x740 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior26(&self) -> &Iprior26 {
        &self.iprior26
    }
    #[doc = "0x760 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior27(&self) -> &Iprior27 {
        &self.iprior27
    }
    #[doc = "0x780 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior28(&self) -> &Iprior28 {
        &self.iprior28
    }
    #[doc = "0x7a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior29(&self) -> &Iprior29 {
        &self.iprior29
    }
    #[doc = "0x7c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior30(&self) -> &Iprior30 {
        &self.iprior30
    }
    #[doc = "0x7e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior31(&self) -> &Iprior31 {
        &self.iprior31
    }
    #[doc = "0x800 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior32(&self) -> &Iprior32 {
        &self.iprior32
    }
    #[doc = "0x820 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior33(&self) -> &Iprior33 {
        &self.iprior33
    }
    #[doc = "0x840 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior34(&self) -> &Iprior34 {
        &self.iprior34
    }
    #[doc = "0x860 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior35(&self) -> &Iprior35 {
        &self.iprior35
    }
    #[doc = "0x880 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior36(&self) -> &Iprior36 {
        &self.iprior36
    }
    #[doc = "0x8a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior37(&self) -> &Iprior37 {
        &self.iprior37
    }
    #[doc = "0x8c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior38(&self) -> &Iprior38 {
        &self.iprior38
    }
    #[doc = "0x8e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior39(&self) -> &Iprior39 {
        &self.iprior39
    }
    #[doc = "0x900 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior40(&self) -> &Iprior40 {
        &self.iprior40
    }
    #[doc = "0x920 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior41(&self) -> &Iprior41 {
        &self.iprior41
    }
    #[doc = "0x940 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior42(&self) -> &Iprior42 {
        &self.iprior42
    }
    #[doc = "0x960 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior43(&self) -> &Iprior43 {
        &self.iprior43
    }
    #[doc = "0x980 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior44(&self) -> &Iprior44 {
        &self.iprior44
    }
    #[doc = "0x9a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior45(&self) -> &Iprior45 {
        &self.iprior45
    }
    #[doc = "0x9c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior46(&self) -> &Iprior46 {
        &self.iprior46
    }
    #[doc = "0x9e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior47(&self) -> &Iprior47 {
        &self.iprior47
    }
    #[doc = "0xa00 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior48(&self) -> &Iprior48 {
        &self.iprior48
    }
    #[doc = "0xa20 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior49(&self) -> &Iprior49 {
        &self.iprior49
    }
    #[doc = "0xa40 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior50(&self) -> &Iprior50 {
        &self.iprior50
    }
    #[doc = "0xa60 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior51(&self) -> &Iprior51 {
        &self.iprior51
    }
    #[doc = "0xa80 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior52(&self) -> &Iprior52 {
        &self.iprior52
    }
    #[doc = "0xaa0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior53(&self) -> &Iprior53 {
        &self.iprior53
    }
    #[doc = "0xac0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior54(&self) -> &Iprior54 {
        &self.iprior54
    }
    #[doc = "0xae0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior55(&self) -> &Iprior55 {
        &self.iprior55
    }
    #[doc = "0xb00 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior56(&self) -> &Iprior56 {
        &self.iprior56
    }
    #[doc = "0xb20 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior57(&self) -> &Iprior57 {
        &self.iprior57
    }
    #[doc = "0xb40 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior58(&self) -> &Iprior58 {
        &self.iprior58
    }
    #[doc = "0xb60 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior59(&self) -> &Iprior59 {
        &self.iprior59
    }
    #[doc = "0xb80 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior60(&self) -> &Iprior60 {
        &self.iprior60
    }
    #[doc = "0xba0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior61(&self) -> &Iprior61 {
        &self.iprior61
    }
    #[doc = "0xbc0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior62(&self) -> &Iprior62 {
        &self.iprior62
    }
    #[doc = "0xbe0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn iprior63(&self) -> &Iprior63 {
        &self.iprior63
    }
    #[doc = "0xd10 - System Control Register"]
    #[inline(always)]
    pub const fn sctlr(&self) -> &Sctlr {
        &self.sctlr
    }
}
#[doc = "ISR1 (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`] module"]
#[doc(alias = "ISR1")]
pub type Isr1 = crate::Reg<isr1::Isr1Spec>;
#[doc = "Interrupt Status Register"]
pub mod isr1;
#[doc = "ISR2 (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr2`] module"]
#[doc(alias = "ISR2")]
pub type Isr2 = crate::Reg<isr2::Isr2Spec>;
#[doc = "Interrupt Status Register"]
pub mod isr2;
#[doc = "IPR1 (r) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr1`] module"]
#[doc(alias = "IPR1")]
pub type Ipr1 = crate::Reg<ipr1::Ipr1Spec>;
#[doc = "Interrupt Pending Register"]
pub mod ipr1;
#[doc = "IPR2 (r) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr2`] module"]
#[doc(alias = "IPR2")]
pub type Ipr2 = crate::Reg<ipr2::Ipr2Spec>;
#[doc = "Interrupt Pending Register"]
pub mod ipr2;
#[doc = "ITHRESDR (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ithresdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ithresdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ithresdr`] module"]
#[doc(alias = "ITHRESDR")]
pub type Ithresdr = crate::Reg<ithresdr::IthresdrSpec>;
#[doc = "Interrupt Priority Register"]
pub mod ithresdr;
#[doc = "FIBADDRR (rw) register accessor: Interrupt Fast Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fibaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fibaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fibaddrr`] module"]
#[doc(alias = "FIBADDRR")]
pub type Fibaddrr = crate::Reg<fibaddrr::FibaddrrSpec>;
#[doc = "Interrupt Fast Address Register"]
pub mod fibaddrr;
#[doc = "CFGR (rw) register accessor: Interrupt Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Interrupt Config Register"]
pub mod cfgr;
#[doc = "GISR (r) register accessor: Interrupt Global Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gisr`] module"]
#[doc(alias = "GISR")]
pub type Gisr = crate::Reg<gisr::GisrSpec>;
#[doc = "Interrupt Global Register"]
pub mod gisr;
#[doc = "FIFOADDRR0 (rw) register accessor: Interrupt 0 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoaddrr0`] module"]
#[doc(alias = "FIFOADDRR0")]
pub type Fifoaddrr0 = crate::Reg<fifoaddrr0::Fifoaddrr0Spec>;
#[doc = "Interrupt 0 address Register"]
pub mod fifoaddrr0;
#[doc = "FIFOADDRR1 (rw) register accessor: Interrupt 1 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoaddrr1`] module"]
#[doc(alias = "FIFOADDRR1")]
pub type Fifoaddrr1 = crate::Reg<fifoaddrr1::Fifoaddrr1Spec>;
#[doc = "Interrupt 1 address Register"]
pub mod fifoaddrr1;
#[doc = "FIFOADDRR2 (rw) register accessor: Interrupt 2 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoaddrr2`] module"]
#[doc(alias = "FIFOADDRR2")]
pub type Fifoaddrr2 = crate::Reg<fifoaddrr2::Fifoaddrr2Spec>;
#[doc = "Interrupt 2 address Register"]
pub mod fifoaddrr2;
#[doc = "FIFOADDRR3 (rw) register accessor: Interrupt 3 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoaddrr3`] module"]
#[doc(alias = "FIFOADDRR3")]
pub type Fifoaddrr3 = crate::Reg<fifoaddrr3::Fifoaddrr3Spec>;
#[doc = "Interrupt 3 address Register"]
pub mod fifoaddrr3;
#[doc = "IENR1 (rw) register accessor: Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr1`] module"]
#[doc(alias = "IENR1")]
pub type Ienr1 = crate::Reg<ienr1::Ienr1Spec>;
#[doc = "Interrupt Setting Register"]
pub mod ienr1;
#[doc = "IENR2 (rw) register accessor: Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr2`] module"]
#[doc(alias = "IENR2")]
pub type Ienr2 = crate::Reg<ienr2::Ienr2Spec>;
#[doc = "Interrupt Setting Register"]
pub mod ienr2;
#[doc = "IRER1 (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer1`] module"]
#[doc(alias = "IRER1")]
pub type Irer1 = crate::Reg<irer1::Irer1Spec>;
#[doc = "Interrupt Clear Register"]
pub mod irer1;
#[doc = "IRER2 (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer2`] module"]
#[doc(alias = "IRER2")]
pub type Irer2 = crate::Reg<irer2::Irer2Spec>;
#[doc = "Interrupt Clear Register"]
pub mod irer2;
#[doc = "IPSR1 (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipsr1`] module"]
#[doc(alias = "IPSR1")]
pub type Ipsr1 = crate::Reg<ipsr1::Ipsr1Spec>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr1;
#[doc = "IPSR2 (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipsr2`] module"]
#[doc(alias = "IPSR2")]
pub type Ipsr2 = crate::Reg<ipsr2::Ipsr2Spec>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr2;
#[doc = "IPRR1 (rw) register accessor: Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprr1`] module"]
#[doc(alias = "IPRR1")]
pub type Iprr1 = crate::Reg<iprr1::Iprr1Spec>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr1;
#[doc = "IPRR2 (rw) register accessor: Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprr2`] module"]
#[doc(alias = "IPRR2")]
pub type Iprr2 = crate::Reg<iprr2::Iprr2Spec>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr2;
#[doc = "IACTR1 (rw) register accessor: Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iactr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iactr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iactr1`] module"]
#[doc(alias = "IACTR1")]
pub type Iactr1 = crate::Reg<iactr1::Iactr1Spec>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr1;
#[doc = "IACTR2 (rw) register accessor: Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iactr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iactr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iactr2`] module"]
#[doc(alias = "IACTR2")]
pub type Iactr2 = crate::Reg<iactr2::Iactr2Spec>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr2;
#[doc = "IPRIOR0 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior0`] module"]
#[doc(alias = "IPRIOR0")]
pub type Iprior0 = crate::Reg<iprior0::Iprior0Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior0;
#[doc = "IPRIOR1 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior1`] module"]
#[doc(alias = "IPRIOR1")]
pub type Iprior1 = crate::Reg<iprior1::Iprior1Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior1;
#[doc = "IPRIOR2 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior2`] module"]
#[doc(alias = "IPRIOR2")]
pub type Iprior2 = crate::Reg<iprior2::Iprior2Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior2;
#[doc = "IPRIOR3 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior3`] module"]
#[doc(alias = "IPRIOR3")]
pub type Iprior3 = crate::Reg<iprior3::Iprior3Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior3;
#[doc = "IPRIOR4 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior4`] module"]
#[doc(alias = "IPRIOR4")]
pub type Iprior4 = crate::Reg<iprior4::Iprior4Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior4;
#[doc = "IPRIOR5 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior5`] module"]
#[doc(alias = "IPRIOR5")]
pub type Iprior5 = crate::Reg<iprior5::Iprior5Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior5;
#[doc = "IPRIOR6 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior6`] module"]
#[doc(alias = "IPRIOR6")]
pub type Iprior6 = crate::Reg<iprior6::Iprior6Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior6;
#[doc = "IPRIOR7 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior7`] module"]
#[doc(alias = "IPRIOR7")]
pub type Iprior7 = crate::Reg<iprior7::Iprior7Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior7;
#[doc = "IPRIOR8 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior8`] module"]
#[doc(alias = "IPRIOR8")]
pub type Iprior8 = crate::Reg<iprior8::Iprior8Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior8;
#[doc = "IPRIOR9 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior9`] module"]
#[doc(alias = "IPRIOR9")]
pub type Iprior9 = crate::Reg<iprior9::Iprior9Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior9;
#[doc = "IPRIOR10 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior10`] module"]
#[doc(alias = "IPRIOR10")]
pub type Iprior10 = crate::Reg<iprior10::Iprior10Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior10;
#[doc = "IPRIOR11 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior11`] module"]
#[doc(alias = "IPRIOR11")]
pub type Iprior11 = crate::Reg<iprior11::Iprior11Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior11;
#[doc = "IPRIOR12 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior12`] module"]
#[doc(alias = "IPRIOR12")]
pub type Iprior12 = crate::Reg<iprior12::Iprior12Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior12;
#[doc = "IPRIOR13 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior13`] module"]
#[doc(alias = "IPRIOR13")]
pub type Iprior13 = crate::Reg<iprior13::Iprior13Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior13;
#[doc = "IPRIOR14 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior14`] module"]
#[doc(alias = "IPRIOR14")]
pub type Iprior14 = crate::Reg<iprior14::Iprior14Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior14;
#[doc = "IPRIOR15 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior15`] module"]
#[doc(alias = "IPRIOR15")]
pub type Iprior15 = crate::Reg<iprior15::Iprior15Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior15;
#[doc = "IPRIOR16 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior16`] module"]
#[doc(alias = "IPRIOR16")]
pub type Iprior16 = crate::Reg<iprior16::Iprior16Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior16;
#[doc = "IPRIOR17 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior17`] module"]
#[doc(alias = "IPRIOR17")]
pub type Iprior17 = crate::Reg<iprior17::Iprior17Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior17;
#[doc = "IPRIOR18 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior18`] module"]
#[doc(alias = "IPRIOR18")]
pub type Iprior18 = crate::Reg<iprior18::Iprior18Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior18;
#[doc = "IPRIOR19 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior19`] module"]
#[doc(alias = "IPRIOR19")]
pub type Iprior19 = crate::Reg<iprior19::Iprior19Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior19;
#[doc = "IPRIOR20 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior20`] module"]
#[doc(alias = "IPRIOR20")]
pub type Iprior20 = crate::Reg<iprior20::Iprior20Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior20;
#[doc = "IPRIOR21 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior21`] module"]
#[doc(alias = "IPRIOR21")]
pub type Iprior21 = crate::Reg<iprior21::Iprior21Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior21;
#[doc = "IPRIOR22 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior22`] module"]
#[doc(alias = "IPRIOR22")]
pub type Iprior22 = crate::Reg<iprior22::Iprior22Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior22;
#[doc = "IPRIOR23 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior23`] module"]
#[doc(alias = "IPRIOR23")]
pub type Iprior23 = crate::Reg<iprior23::Iprior23Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior23;
#[doc = "IPRIOR24 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior24`] module"]
#[doc(alias = "IPRIOR24")]
pub type Iprior24 = crate::Reg<iprior24::Iprior24Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior24;
#[doc = "IPRIOR25 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior25`] module"]
#[doc(alias = "IPRIOR25")]
pub type Iprior25 = crate::Reg<iprior25::Iprior25Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior25;
#[doc = "IPRIOR26 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior26`] module"]
#[doc(alias = "IPRIOR26")]
pub type Iprior26 = crate::Reg<iprior26::Iprior26Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior26;
#[doc = "IPRIOR27 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior27`] module"]
#[doc(alias = "IPRIOR27")]
pub type Iprior27 = crate::Reg<iprior27::Iprior27Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior27;
#[doc = "IPRIOR28 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior28`] module"]
#[doc(alias = "IPRIOR28")]
pub type Iprior28 = crate::Reg<iprior28::Iprior28Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior28;
#[doc = "IPRIOR29 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior29`] module"]
#[doc(alias = "IPRIOR29")]
pub type Iprior29 = crate::Reg<iprior29::Iprior29Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior29;
#[doc = "IPRIOR30 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior30`] module"]
#[doc(alias = "IPRIOR30")]
pub type Iprior30 = crate::Reg<iprior30::Iprior30Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior30;
#[doc = "IPRIOR31 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior31`] module"]
#[doc(alias = "IPRIOR31")]
pub type Iprior31 = crate::Reg<iprior31::Iprior31Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior31;
#[doc = "IPRIOR32 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior32`] module"]
#[doc(alias = "IPRIOR32")]
pub type Iprior32 = crate::Reg<iprior32::Iprior32Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior32;
#[doc = "IPRIOR33 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior33`] module"]
#[doc(alias = "IPRIOR33")]
pub type Iprior33 = crate::Reg<iprior33::Iprior33Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior33;
#[doc = "IPRIOR34 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior34`] module"]
#[doc(alias = "IPRIOR34")]
pub type Iprior34 = crate::Reg<iprior34::Iprior34Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior34;
#[doc = "IPRIOR35 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior35`] module"]
#[doc(alias = "IPRIOR35")]
pub type Iprior35 = crate::Reg<iprior35::Iprior35Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior35;
#[doc = "IPRIOR36 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior36`] module"]
#[doc(alias = "IPRIOR36")]
pub type Iprior36 = crate::Reg<iprior36::Iprior36Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior36;
#[doc = "IPRIOR37 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior37`] module"]
#[doc(alias = "IPRIOR37")]
pub type Iprior37 = crate::Reg<iprior37::Iprior37Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior37;
#[doc = "IPRIOR38 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior38`] module"]
#[doc(alias = "IPRIOR38")]
pub type Iprior38 = crate::Reg<iprior38::Iprior38Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior38;
#[doc = "IPRIOR39 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior39`] module"]
#[doc(alias = "IPRIOR39")]
pub type Iprior39 = crate::Reg<iprior39::Iprior39Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior39;
#[doc = "IPRIOR40 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior40`] module"]
#[doc(alias = "IPRIOR40")]
pub type Iprior40 = crate::Reg<iprior40::Iprior40Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior40;
#[doc = "IPRIOR41 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior41`] module"]
#[doc(alias = "IPRIOR41")]
pub type Iprior41 = crate::Reg<iprior41::Iprior41Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior41;
#[doc = "IPRIOR42 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior42`] module"]
#[doc(alias = "IPRIOR42")]
pub type Iprior42 = crate::Reg<iprior42::Iprior42Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior42;
#[doc = "IPRIOR43 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior43`] module"]
#[doc(alias = "IPRIOR43")]
pub type Iprior43 = crate::Reg<iprior43::Iprior43Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior43;
#[doc = "IPRIOR44 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior44`] module"]
#[doc(alias = "IPRIOR44")]
pub type Iprior44 = crate::Reg<iprior44::Iprior44Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior44;
#[doc = "IPRIOR45 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior45`] module"]
#[doc(alias = "IPRIOR45")]
pub type Iprior45 = crate::Reg<iprior45::Iprior45Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior45;
#[doc = "IPRIOR46 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior46`] module"]
#[doc(alias = "IPRIOR46")]
pub type Iprior46 = crate::Reg<iprior46::Iprior46Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior46;
#[doc = "IPRIOR47 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior47`] module"]
#[doc(alias = "IPRIOR47")]
pub type Iprior47 = crate::Reg<iprior47::Iprior47Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior47;
#[doc = "IPRIOR48 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior48`] module"]
#[doc(alias = "IPRIOR48")]
pub type Iprior48 = crate::Reg<iprior48::Iprior48Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior48;
#[doc = "IPRIOR49 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior49`] module"]
#[doc(alias = "IPRIOR49")]
pub type Iprior49 = crate::Reg<iprior49::Iprior49Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior49;
#[doc = "IPRIOR50 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior50`] module"]
#[doc(alias = "IPRIOR50")]
pub type Iprior50 = crate::Reg<iprior50::Iprior50Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior50;
#[doc = "IPRIOR51 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior51`] module"]
#[doc(alias = "IPRIOR51")]
pub type Iprior51 = crate::Reg<iprior51::Iprior51Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior51;
#[doc = "IPRIOR52 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior52`] module"]
#[doc(alias = "IPRIOR52")]
pub type Iprior52 = crate::Reg<iprior52::Iprior52Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior52;
#[doc = "IPRIOR53 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior53`] module"]
#[doc(alias = "IPRIOR53")]
pub type Iprior53 = crate::Reg<iprior53::Iprior53Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior53;
#[doc = "IPRIOR54 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior54`] module"]
#[doc(alias = "IPRIOR54")]
pub type Iprior54 = crate::Reg<iprior54::Iprior54Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior54;
#[doc = "IPRIOR55 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior55`] module"]
#[doc(alias = "IPRIOR55")]
pub type Iprior55 = crate::Reg<iprior55::Iprior55Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior55;
#[doc = "IPRIOR56 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior56`] module"]
#[doc(alias = "IPRIOR56")]
pub type Iprior56 = crate::Reg<iprior56::Iprior56Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior56;
#[doc = "IPRIOR57 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior57`] module"]
#[doc(alias = "IPRIOR57")]
pub type Iprior57 = crate::Reg<iprior57::Iprior57Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior57;
#[doc = "IPRIOR58 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior58`] module"]
#[doc(alias = "IPRIOR58")]
pub type Iprior58 = crate::Reg<iprior58::Iprior58Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior58;
#[doc = "IPRIOR59 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior59`] module"]
#[doc(alias = "IPRIOR59")]
pub type Iprior59 = crate::Reg<iprior59::Iprior59Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior59;
#[doc = "IPRIOR60 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior60`] module"]
#[doc(alias = "IPRIOR60")]
pub type Iprior60 = crate::Reg<iprior60::Iprior60Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior60;
#[doc = "IPRIOR61 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior61`] module"]
#[doc(alias = "IPRIOR61")]
pub type Iprior61 = crate::Reg<iprior61::Iprior61Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior61;
#[doc = "IPRIOR62 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior62`] module"]
#[doc(alias = "IPRIOR62")]
pub type Iprior62 = crate::Reg<iprior62::Iprior62Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior62;
#[doc = "IPRIOR63 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprior63`] module"]
#[doc(alias = "IPRIOR63")]
pub type Iprior63 = crate::Reg<iprior63::Iprior63Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior63;
#[doc = "SCTLR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctlr`] module"]
#[doc(alias = "SCTLR")]
pub type Sctlr = crate::Reg<sctlr::SctlrSpec>;
#[doc = "System Control Register"]
pub mod sctlr;
