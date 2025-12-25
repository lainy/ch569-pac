#[doc = "Register `R8_UEP4_1_MOD` reader"]
pub type R = crate::R<R8Uep4_1ModSpec>;
#[doc = "Register `R8_UEP4_1_MOD` writer"]
pub type W = crate::W<R8Uep4_1ModSpec>;
#[doc = "Field `RB_UEP4_BUF_MOD` reader - buffer mode of USB endpoint 4(8/12)"]
pub type RbUep4BufModR = crate::BitReader;
#[doc = "Field `RB_UEP4_BUF_MOD` writer - buffer mode of USB endpoint 4(8/12)"]
pub type RbUep4BufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP4_TX_EN` reader - enable USB endpoint 4(8/12) transmittal (IN)"]
pub type RbUep4TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP4_TX_EN` writer - enable USB endpoint 4(8/12) transmittal (IN)"]
pub type RbUep4TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP4_RX_EN` reader - enable USB endpoint 4(8/12) receiving (OUT)"]
pub type RbUep4RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP4_RX_EN` writer - enable USB endpoint 4(8/12) receiving (OUT)"]
pub type RbUep4RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP1_BUF_MOD` reader - buffer mode of USB endpoint 1(9)"]
pub type RbUep1BufModR = crate::BitReader;
#[doc = "Field `RB_UEP1_BUF_MOD` writer - buffer mode of USB endpoint 1(9)"]
pub type RbUep1BufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP1_TX_EN` reader - enable USB endpoint 1(9) transmittal (IN)"]
pub type RbUep1TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP1_TX_EN` writer - enable USB endpoint 1(9) transmittal (IN)"]
pub type RbUep1TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP1_RX_EN` reader - enable USB endpoint 1(9) receiving (OUT)"]
pub type RbUep1RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP1_RX_EN` writer - enable USB endpoint 1(9) receiving (OUT)"]
pub type RbUep1RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 4(8/12)"]
    #[inline(always)]
    pub fn rb_uep4_buf_mod(&self) -> RbUep4BufModR {
        RbUep4BufModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 4(8/12) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep4_tx_en(&self) -> RbUep4TxEnR {
        RbUep4TxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 4(8/12) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep4_rx_en(&self) -> RbUep4RxEnR {
        RbUep4RxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 1(9)"]
    #[inline(always)]
    pub fn rb_uep1_buf_mod(&self) -> RbUep1BufModR {
        RbUep1BufModR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 1(9) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep1_tx_en(&self) -> RbUep1TxEnR {
        RbUep1TxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 1(9) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep1_rx_en(&self) -> RbUep1RxEnR {
        RbUep1RxEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 4(8/12)"]
    #[inline(always)]
    pub fn rb_uep4_buf_mod(&mut self) -> RbUep4BufModW<'_, R8Uep4_1ModSpec> {
        RbUep4BufModW::new(self, 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 4(8/12) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep4_tx_en(&mut self) -> RbUep4TxEnW<'_, R8Uep4_1ModSpec> {
        RbUep4TxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - enable USB endpoint 4(8/12) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep4_rx_en(&mut self) -> RbUep4RxEnW<'_, R8Uep4_1ModSpec> {
        RbUep4RxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 1(9)"]
    #[inline(always)]
    pub fn rb_uep1_buf_mod(&mut self) -> RbUep1BufModW<'_, R8Uep4_1ModSpec> {
        RbUep1BufModW::new(self, 4)
    }
    #[doc = "Bit 6 - enable USB endpoint 1(9) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep1_tx_en(&mut self) -> RbUep1TxEnW<'_, R8Uep4_1ModSpec> {
        RbUep1TxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - enable USB endpoint 1(9) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep1_rx_en(&mut self) -> RbUep1RxEnW<'_, R8Uep4_1ModSpec> {
        RbUep1RxEnW::new(self, 7)
    }
}
#[doc = "endpoint 1(9)/4(8/12) mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_1_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_1_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep4_1ModSpec;
impl crate::RegisterSpec for R8Uep4_1ModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep4_1_mod::R`](R) reader structure"]
impl crate::Readable for R8Uep4_1ModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep4_1_mod::W`](W) writer structure"]
impl crate::Writable for R8Uep4_1ModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP4_1_MOD to value 0"]
impl crate::Resettable for R8Uep4_1ModSpec {}
