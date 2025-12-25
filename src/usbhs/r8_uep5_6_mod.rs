#[doc = "Register `R8_UEP5_6_MOD` reader"]
pub type R = crate::R<R8Uep5_6ModSpec>;
#[doc = "Register `R8_UEP5_6_MOD` writer"]
pub type W = crate::W<R8Uep5_6ModSpec>;
#[doc = "Field `RB_UEP5_BUF_MOD` reader - buffer mode of USB endpoint 5(13)"]
pub type RbUep5BufModR = crate::BitReader;
#[doc = "Field `RB_UEP5_BUF_MOD` writer - buffer mode of USB endpoint 5(13)"]
pub type RbUep5BufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP5_TX_EN` reader - enable USB endpoint 5(13) transmittal (IN)"]
pub type RbUep5TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP5_TX_EN` writer - enable USB endpoint 5(13) transmittal (IN)"]
pub type RbUep5TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP5_RX_EN` reader - enable USB endpoint 5(13) receiving (OUT)"]
pub type RbUep5RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP5_RX_EN` writer - enable USB endpoint 5(13) receiving (OUT)"]
pub type RbUep5RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP6_BUF_MOD` reader - buffer mode of USB endpoint 6(14)"]
pub type RbUep6BufModR = crate::BitReader;
#[doc = "Field `RB_UEP6_BUF_MOD` writer - buffer mode of USB endpoint 6(14)"]
pub type RbUep6BufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP6_TX_EN` reader - enable USB endpoint 6(14) transmittal (IN)"]
pub type RbUep6TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP6_TX_EN` writer - enable USB endpoint 6(14) transmittal (IN)"]
pub type RbUep6TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP6_RX_EN` reader - enable USB endpoint 6(14) receiving (OUT)"]
pub type RbUep6RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP6_RX_EN` writer - enable USB endpoint 6(14) receiving (OUT)"]
pub type RbUep6RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 5(13)"]
    #[inline(always)]
    pub fn rb_uep5_buf_mod(&self) -> RbUep5BufModR {
        RbUep5BufModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 5(13) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep5_tx_en(&self) -> RbUep5TxEnR {
        RbUep5TxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 5(13) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep5_rx_en(&self) -> RbUep5RxEnR {
        RbUep5RxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 6(14)"]
    #[inline(always)]
    pub fn rb_uep6_buf_mod(&self) -> RbUep6BufModR {
        RbUep6BufModR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 6(14) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep6_tx_en(&self) -> RbUep6TxEnR {
        RbUep6TxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 6(14) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep6_rx_en(&self) -> RbUep6RxEnR {
        RbUep6RxEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 5(13)"]
    #[inline(always)]
    pub fn rb_uep5_buf_mod(&mut self) -> RbUep5BufModW<'_, R8Uep5_6ModSpec> {
        RbUep5BufModW::new(self, 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 5(13) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep5_tx_en(&mut self) -> RbUep5TxEnW<'_, R8Uep5_6ModSpec> {
        RbUep5TxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - enable USB endpoint 5(13) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep5_rx_en(&mut self) -> RbUep5RxEnW<'_, R8Uep5_6ModSpec> {
        RbUep5RxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 6(14)"]
    #[inline(always)]
    pub fn rb_uep6_buf_mod(&mut self) -> RbUep6BufModW<'_, R8Uep5_6ModSpec> {
        RbUep6BufModW::new(self, 4)
    }
    #[doc = "Bit 6 - enable USB endpoint 6(14) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep6_tx_en(&mut self) -> RbUep6TxEnW<'_, R8Uep5_6ModSpec> {
        RbUep6TxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - enable USB endpoint 6(14) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep6_rx_en(&mut self) -> RbUep6RxEnW<'_, R8Uep5_6ModSpec> {
        RbUep6RxEnW::new(self, 7)
    }
}
#[doc = "endpoint 5(13)/6(14) mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep5_6_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep5_6_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep5_6ModSpec;
impl crate::RegisterSpec for R8Uep5_6ModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep5_6_mod::R`](R) reader structure"]
impl crate::Readable for R8Uep5_6ModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep5_6_mod::W`](W) writer structure"]
impl crate::Writable for R8Uep5_6ModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP5_6_MOD to value 0"]
impl crate::Resettable for R8Uep5_6ModSpec {}
