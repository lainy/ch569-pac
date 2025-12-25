#[doc = "Register `R8_UEP7_MOD` reader"]
pub type R = crate::R<R8Uep7ModSpec>;
#[doc = "Register `R8_UEP7_MOD` writer"]
pub type W = crate::W<R8Uep7ModSpec>;
#[doc = "Field `RB_UEP7_BUF_MOD` reader - buffer mode of USB endpoint 7(15)"]
pub type RbUep7BufModR = crate::BitReader;
#[doc = "Field `RB_UEP7_BUF_MOD` writer - buffer mode of USB endpoint 7(15)"]
pub type RbUep7BufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP7_TX_EN` reader - enable USB endpoint 7(15) transmittal (IN)"]
pub type RbUep7TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP7_TX_EN` writer - enable USB endpoint 7(15) transmittal (IN)"]
pub type RbUep7TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP7_RX_EN` reader - enable USB endpoint 7(15) receiving (OUT)"]
pub type RbUep7RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP7_RX_EN` writer - enable USB endpoint 7(15) receiving (OUT)"]
pub type RbUep7RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 7(15)"]
    #[inline(always)]
    pub fn rb_uep7_buf_mod(&self) -> RbUep7BufModR {
        RbUep7BufModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 7(15) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep7_tx_en(&self) -> RbUep7TxEnR {
        RbUep7TxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 7(15) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep7_rx_en(&self) -> RbUep7RxEnR {
        RbUep7RxEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 7(15)"]
    #[inline(always)]
    pub fn rb_uep7_buf_mod(&mut self) -> RbUep7BufModW<'_, R8Uep7ModSpec> {
        RbUep7BufModW::new(self, 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 7(15) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep7_tx_en(&mut self) -> RbUep7TxEnW<'_, R8Uep7ModSpec> {
        RbUep7TxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - enable USB endpoint 7(15) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep7_rx_en(&mut self) -> RbUep7RxEnW<'_, R8Uep7ModSpec> {
        RbUep7RxEnW::new(self, 3)
    }
}
#[doc = "endpoint 7(15) mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep7_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep7_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep7ModSpec;
impl crate::RegisterSpec for R8Uep7ModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep7_mod::R`](R) reader structure"]
impl crate::Readable for R8Uep7ModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep7_mod::W`](W) writer structure"]
impl crate::Writable for R8Uep7ModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP7_MOD to value 0"]
impl crate::Resettable for R8Uep7ModSpec {}
