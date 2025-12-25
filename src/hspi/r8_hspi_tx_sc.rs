#[doc = "Register `R8_HSPI_TX_SC` reader"]
pub type R = crate::R<R8HspiTxScSpec>;
#[doc = "Register `R8_HSPI_TX_SC` writer"]
pub type W = crate::W<R8HspiTxScSpec>;
#[doc = "Field `RB_HSPI_TX_NUM` reader - parallel if tx sequence num"]
pub type RbHspiTxNumR = crate::FieldReader;
#[doc = "Field `RB_HSPI_TX_NUM` writer - parallel if tx sequence num"]
pub type RbHspiTxNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RB_HSPI_TX_TOG` reader - parallel if tx addr toggle flag"]
pub type RbHspiTxTogR = crate::BitReader;
#[doc = "Field `RB_HSPI_TX_TOG` writer - parallel if tx addr toggle flag"]
pub type RbHspiTxTogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - parallel if tx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_tx_num(&self) -> RbHspiTxNumR {
        RbHspiTxNumR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - parallel if tx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog(&self) -> RbHspiTxTogR {
        RbHspiTxTogR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - parallel if tx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_tx_num(&mut self) -> RbHspiTxNumW<'_, R8HspiTxScSpec> {
        RbHspiTxNumW::new(self, 0)
    }
    #[doc = "Bit 4 - parallel if tx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog(&mut self) -> RbHspiTxTogW<'_, R8HspiTxScSpec> {
        RbHspiTxTogW::new(self, 4)
    }
}
#[doc = "parallel TX sequence ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_tx_sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_tx_sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiTxScSpec;
impl crate::RegisterSpec for R8HspiTxScSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_tx_sc::R`](R) reader structure"]
impl crate::Readable for R8HspiTxScSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_tx_sc::W`](W) writer structure"]
impl crate::Writable for R8HspiTxScSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_TX_SC to value 0"]
impl crate::Resettable for R8HspiTxScSpec {}
