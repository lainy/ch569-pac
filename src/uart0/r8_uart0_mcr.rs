#[doc = "Register `R8_UART0_MCR` reader"]
pub type R = crate::R<R8Uart0McrSpec>;
#[doc = "Register `R8_UART0_MCR` writer"]
pub type W = crate::W<R8Uart0McrSpec>;
#[doc = "Field `RB_MCR_DTR` reader - UART0 control DTR"]
pub type RbMcrDtrR = crate::BitReader;
#[doc = "Field `RB_MCR_DTR` writer - UART0 control DTR"]
pub type RbMcrDtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_RTS` reader - UART0 control RTS"]
pub type RbMcrRtsR = crate::BitReader;
#[doc = "Field `RB_MCR_RTS` writer - UART0 control RTS"]
pub type RbMcrRtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_OUT1` reader - UART0 control OUT1"]
pub type RbMcrOut1R = crate::BitReader;
#[doc = "Field `RB_MCR_OUT1` writer - UART0 control OUT1"]
pub type RbMcrOut1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_OUT2` reader - UART control OUT2"]
pub type RbMcrOut2R = crate::BitReader;
#[doc = "Field `RB_MCR_OUT2` writer - UART control OUT2"]
pub type RbMcrOut2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_LOOP` reader - UART0 enable local loop back"]
pub type RbMcrLoopR = crate::BitReader;
#[doc = "Field `RB_MCR_LOOP` writer - UART0 enable local loop back"]
pub type RbMcrLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_AU_FLOW_EN` reader - UART0 enable autoflow control"]
pub type RbMcrAuFlowEnR = crate::BitReader;
#[doc = "Field `RB_MCR_AU_FLOW_EN` writer - UART0 enable autoflow control"]
pub type RbMcrAuFlowEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_TNOW` reader - UART0 enable TNOW output on DTR pin"]
pub type RbMcrTnowR = crate::BitReader;
#[doc = "Field `RB_MCR_TNOW` writer - UART0 enable TNOW output on DTR pin"]
pub type RbMcrTnowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_HALF` reader - UART0 enable half-duplex"]
pub type RbMcrHalfR = crate::BitReader;
#[doc = "Field `RB_MCR_HALF` writer - UART0 enable half-duplex"]
pub type RbMcrHalfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART0 control DTR"]
    #[inline(always)]
    pub fn rb_mcr_dtr(&self) -> RbMcrDtrR {
        RbMcrDtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 control RTS"]
    #[inline(always)]
    pub fn rb_mcr_rts(&self) -> RbMcrRtsR {
        RbMcrRtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 control OUT1"]
    #[inline(always)]
    pub fn rb_mcr_out1(&self) -> RbMcrOut1R {
        RbMcrOut1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&self) -> RbMcrOut2R {
        RbMcrOut2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 enable local loop back"]
    #[inline(always)]
    pub fn rb_mcr_loop(&self) -> RbMcrLoopR {
        RbMcrLoopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn rb_mcr_au_flow_en(&self) -> RbMcrAuFlowEnR {
        RbMcrAuFlowEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 enable TNOW output on DTR pin"]
    #[inline(always)]
    pub fn rb_mcr_tnow(&self) -> RbMcrTnowR {
        RbMcrTnowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART0 enable half-duplex"]
    #[inline(always)]
    pub fn rb_mcr_half(&self) -> RbMcrHalfR {
        RbMcrHalfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 control DTR"]
    #[inline(always)]
    pub fn rb_mcr_dtr(&mut self) -> RbMcrDtrW<'_, R8Uart0McrSpec> {
        RbMcrDtrW::new(self, 0)
    }
    #[doc = "Bit 1 - UART0 control RTS"]
    #[inline(always)]
    pub fn rb_mcr_rts(&mut self) -> RbMcrRtsW<'_, R8Uart0McrSpec> {
        RbMcrRtsW::new(self, 1)
    }
    #[doc = "Bit 2 - UART0 control OUT1"]
    #[inline(always)]
    pub fn rb_mcr_out1(&mut self) -> RbMcrOut1W<'_, R8Uart0McrSpec> {
        RbMcrOut1W::new(self, 2)
    }
    #[doc = "Bit 3 - UART control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&mut self) -> RbMcrOut2W<'_, R8Uart0McrSpec> {
        RbMcrOut2W::new(self, 3)
    }
    #[doc = "Bit 4 - UART0 enable local loop back"]
    #[inline(always)]
    pub fn rb_mcr_loop(&mut self) -> RbMcrLoopW<'_, R8Uart0McrSpec> {
        RbMcrLoopW::new(self, 4)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn rb_mcr_au_flow_en(&mut self) -> RbMcrAuFlowEnW<'_, R8Uart0McrSpec> {
        RbMcrAuFlowEnW::new(self, 5)
    }
    #[doc = "Bit 6 - UART0 enable TNOW output on DTR pin"]
    #[inline(always)]
    pub fn rb_mcr_tnow(&mut self) -> RbMcrTnowW<'_, R8Uart0McrSpec> {
        RbMcrTnowW::new(self, 6)
    }
    #[doc = "Bit 7 - UART0 enable half-duplex"]
    #[inline(always)]
    pub fn rb_mcr_half(&mut self) -> RbMcrHalfW<'_, R8Uart0McrSpec> {
        RbMcrHalfW::new(self, 7)
    }
}
#[doc = "UART0 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0McrSpec;
impl crate::RegisterSpec for R8Uart0McrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_mcr::R`](R) reader structure"]
impl crate::Readable for R8Uart0McrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart0_mcr::W`](W) writer structure"]
impl crate::Writable for R8Uart0McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART0_MCR to value 0"]
impl crate::Resettable for R8Uart0McrSpec {}
