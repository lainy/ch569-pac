#[doc = "Register `R8_UEP2_3_MOD_R8_UH_EP_MOD` reader"]
pub type R = crate::R<R8Uep2_3ModR8UhEpModSpec>;
#[doc = "Register `R8_UEP2_3_MOD_R8_UH_EP_MOD` writer"]
pub type W = crate::W<R8Uep2_3ModR8UhEpModSpec>;
#[doc = "Field `RB_UEP2_BUF_MOD_RB_UH_RX_EN` reader - buffer mode of USB endpoint 2(10) / USB host receive endpoint (IN) enable"]
pub type RbUep2BufModRbUhRxEnR = crate::BitReader;
#[doc = "Field `RB_UEP2_BUF_MOD_RB_UH_RX_EN` writer - buffer mode of USB endpoint 2(10) / USB host receive endpoint (IN) enable"]
pub type RbUep2BufModRbUhRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP2_TX_EN` reader - enable USB endpoint 2(10) transmittal (IN)"]
pub type RbUep2TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP2_TX_EN` writer - enable USB endpoint 2(10) transmittal (IN)"]
pub type RbUep2TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP2_RX_EN` reader - enable USB endpoint 2(10) receiving (OUT)"]
pub type RbUep2RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP2_RX_EN` writer - enable USB endpoint 2(10) receiving (OUT)"]
pub type RbUep2RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP3_BUF_MOD` reader - buffer mode of USB endpoint 3(11)"]
pub type RbUep3BufModR = crate::BitReader;
#[doc = "Field `RB_UEP3_BUF_MOD` writer - buffer mode of USB endpoint 3(11)"]
pub type RbUep3BufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP3_TX_EN_RB_UH_TX_EN` reader - enable USB endpoint 3(11) transmittal (IN) / USB host send endpoint (SETUP/OUT) enable"]
pub type RbUep3TxEnRbUhTxEnR = crate::BitReader;
#[doc = "Field `RB_UEP3_TX_EN_RB_UH_TX_EN` writer - enable USB endpoint 3(11) transmittal (IN) / USB host send endpoint (SETUP/OUT) enable"]
pub type RbUep3TxEnRbUhTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP3_RX_EN` reader - enable USB endpoint 3(11) receiving (OUT)"]
pub type RbUep3RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP3_RX_EN` writer - enable USB endpoint 3(11) receiving (OUT)"]
pub type RbUep3RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 2(10) / USB host receive endpoint (IN) enable"]
    #[inline(always)]
    pub fn rb_uep2_buf_mod_rb_uh_rx_en(&self) -> RbUep2BufModRbUhRxEnR {
        RbUep2BufModRbUhRxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 2(10) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep2_tx_en(&self) -> RbUep2TxEnR {
        RbUep2TxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 2(10) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep2_rx_en(&self) -> RbUep2RxEnR {
        RbUep2RxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 3(11)"]
    #[inline(always)]
    pub fn rb_uep3_buf_mod(&self) -> RbUep3BufModR {
        RbUep3BufModR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 3(11) transmittal (IN) / USB host send endpoint (SETUP/OUT) enable"]
    #[inline(always)]
    pub fn rb_uep3_tx_en_rb_uh_tx_en(&self) -> RbUep3TxEnRbUhTxEnR {
        RbUep3TxEnRbUhTxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 3(11) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&self) -> RbUep3RxEnR {
        RbUep3RxEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 2(10) / USB host receive endpoint (IN) enable"]
    #[inline(always)]
    pub fn rb_uep2_buf_mod_rb_uh_rx_en(
        &mut self,
    ) -> RbUep2BufModRbUhRxEnW<'_, R8Uep2_3ModR8UhEpModSpec> {
        RbUep2BufModRbUhRxEnW::new(self, 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 2(10) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep2_tx_en(&mut self) -> RbUep2TxEnW<'_, R8Uep2_3ModR8UhEpModSpec> {
        RbUep2TxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - enable USB endpoint 2(10) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep2_rx_en(&mut self) -> RbUep2RxEnW<'_, R8Uep2_3ModR8UhEpModSpec> {
        RbUep2RxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 3(11)"]
    #[inline(always)]
    pub fn rb_uep3_buf_mod(&mut self) -> RbUep3BufModW<'_, R8Uep2_3ModR8UhEpModSpec> {
        RbUep3BufModW::new(self, 4)
    }
    #[doc = "Bit 6 - enable USB endpoint 3(11) transmittal (IN) / USB host send endpoint (SETUP/OUT) enable"]
    #[inline(always)]
    pub fn rb_uep3_tx_en_rb_uh_tx_en(
        &mut self,
    ) -> RbUep3TxEnRbUhTxEnW<'_, R8Uep2_3ModR8UhEpModSpec> {
        RbUep3TxEnRbUhTxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - enable USB endpoint 3(11) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&mut self) -> RbUep3RxEnW<'_, R8Uep2_3ModR8UhEpModSpec> {
        RbUep3RxEnW::new(self, 7)
    }
}
#[doc = "endpoint 2(10)/3(11) mode / USB host endpoint mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_3_mod_r8_uh_ep_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_3_mod_r8_uh_ep_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep2_3ModR8UhEpModSpec;
impl crate::RegisterSpec for R8Uep2_3ModR8UhEpModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep2_3_mod_r8_uh_ep_mod::R`](R) reader structure"]
impl crate::Readable for R8Uep2_3ModR8UhEpModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep2_3_mod_r8_uh_ep_mod::W`](W) writer structure"]
impl crate::Writable for R8Uep2_3ModR8UhEpModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP2_3_MOD_R8_UH_EP_MOD to value 0"]
impl crate::Resettable for R8Uep2_3ModR8UhEpModSpec {}
