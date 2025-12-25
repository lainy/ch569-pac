#[doc = "Register `R8_HSPI_AUX` reader"]
pub type R = crate::R<R8HspiAuxSpec>;
#[doc = "Register `R8_HSPI_AUX` writer"]
pub type W = crate::W<R8HspiAuxSpec>;
#[doc = "Field `RB_HSPI_TCK_MOD` reader - parallel if tx clk polar control"]
pub type RbHspiTckModR = crate::BitReader;
#[doc = "Field `RB_HSPI_TCK_MOD` writer - parallel if tx clk polar control"]
pub type RbHspiTckModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_RCK_MOD` reader - parallel if rx clk polar control"]
pub type RbHspiRckModR = crate::BitReader;
#[doc = "Field `RB_HSPI_RCK_MOD` writer - parallel if rx clk polar control"]
pub type RbHspiRckModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_ACK_TX_MOD` reader - parallel if tx ack mode cfg"]
pub type RbHspiAckTxModR = crate::BitReader;
#[doc = "Field `RB_HSPI_ACK_TX_MOD` writer - parallel if tx ack mode cfg"]
pub type RbHspiAckTxModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_ACK_CNT_SEL` reader - delay time of parallel if send ack when receive done"]
pub type RbHspiAckCntSelR = crate::FieldReader;
#[doc = "Field `RB_HSPI_ACK_CNT_SEL` writer - delay time of parallel if send ack when receive done"]
pub type RbHspiAckCntSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - parallel if tx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_tck_mod(&self) -> RbHspiTckModR {
        RbHspiTckModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if rx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_rck_mod(&self) -> RbHspiRckModR {
        RbHspiRckModR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - parallel if tx ack mode cfg"]
    #[inline(always)]
    pub fn rb_hspi_ack_tx_mod(&self) -> RbHspiAckTxModR {
        RbHspiAckTxModR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - delay time of parallel if send ack when receive done"]
    #[inline(always)]
    pub fn rb_hspi_ack_cnt_sel(&self) -> RbHspiAckCntSelR {
        RbHspiAckCntSelR::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if tx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_tck_mod(&mut self) -> RbHspiTckModW<'_, R8HspiAuxSpec> {
        RbHspiTckModW::new(self, 0)
    }
    #[doc = "Bit 1 - parallel if rx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_rck_mod(&mut self) -> RbHspiRckModW<'_, R8HspiAuxSpec> {
        RbHspiRckModW::new(self, 1)
    }
    #[doc = "Bit 2 - parallel if tx ack mode cfg"]
    #[inline(always)]
    pub fn rb_hspi_ack_tx_mod(&mut self) -> RbHspiAckTxModW<'_, R8HspiAuxSpec> {
        RbHspiAckTxModW::new(self, 2)
    }
    #[doc = "Bits 3:4 - delay time of parallel if send ack when receive done"]
    #[inline(always)]
    pub fn rb_hspi_ack_cnt_sel(&mut self) -> RbHspiAckCntSelW<'_, R8HspiAuxSpec> {
        RbHspiAckCntSelW::new(self, 3)
    }
}
#[doc = "parallel if aux\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_aux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_aux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiAuxSpec;
impl crate::RegisterSpec for R8HspiAuxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_aux::R`](R) reader structure"]
impl crate::Readable for R8HspiAuxSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_aux::W`](W) writer structure"]
impl crate::Writable for R8HspiAuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_AUX to value 0"]
impl crate::Resettable for R8HspiAuxSpec {}
