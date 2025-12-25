#[doc = "Register `R8_UART3_LCR` reader"]
pub type R = crate::R<R8Uart3LcrSpec>;
#[doc = "Register `R8_UART3_LCR` writer"]
pub type W = crate::W<R8Uart3LcrSpec>;
#[doc = "Field `RB_LCR_WORD_SZ` reader - UART word bit length"]
pub type RbLcrWordSzR = crate::FieldReader;
#[doc = "Field `RB_LCR_WORD_SZ` writer - UART word bit length"]
pub type RbLcrWordSzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_LCR_STOP_BIT` reader - UART stop bit length"]
pub type RbLcrStopBitR = crate::BitReader;
#[doc = "Field `RB_LCR_STOP_BIT` writer - UART stop bit length"]
pub type RbLcrStopBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_LCR_PAR_EN` reader - UART parity enable"]
pub type RbLcrParEnR = crate::BitReader;
#[doc = "Field `RB_LCR_PAR_EN` writer - UART parity enable"]
pub type RbLcrParEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_LCR_PAR_MOD` reader - UART parity mode"]
pub type RbLcrParModR = crate::FieldReader;
#[doc = "Field `RB_LCR_PAR_MOD` writer - UART parity mode"]
pub type RbLcrParModW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_LCR_BREAK_EN` reader - UART break control enable"]
pub type RbLcrBreakEnR = crate::BitReader;
#[doc = "Field `RB_LCR_BREAK_EN` writer - UART break control enable"]
pub type RbLcrBreakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_LCR_DLAB_RB_LCR_GP_BIT` reader - UART reserved bit / UART general purpose bit"]
pub type RbLcrDlabRbLcrGpBitR = crate::BitReader;
#[doc = "Field `RB_LCR_DLAB_RB_LCR_GP_BIT` writer - UART reserved bit / UART general purpose bit"]
pub type RbLcrDlabRbLcrGpBitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - UART word bit length"]
    #[inline(always)]
    pub fn rb_lcr_word_sz(&self) -> RbLcrWordSzR {
        RbLcrWordSzR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - UART stop bit length"]
    #[inline(always)]
    pub fn rb_lcr_stop_bit(&self) -> RbLcrStopBitR {
        RbLcrStopBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART parity enable"]
    #[inline(always)]
    pub fn rb_lcr_par_en(&self) -> RbLcrParEnR {
        RbLcrParEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART parity mode"]
    #[inline(always)]
    pub fn rb_lcr_par_mod(&self) -> RbLcrParModR {
        RbLcrParModR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - UART break control enable"]
    #[inline(always)]
    pub fn rb_lcr_break_en(&self) -> RbLcrBreakEnR {
        RbLcrBreakEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART reserved bit / UART general purpose bit"]
    #[inline(always)]
    pub fn rb_lcr_dlab_rb_lcr_gp_bit(&self) -> RbLcrDlabRbLcrGpBitR {
        RbLcrDlabRbLcrGpBitR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART word bit length"]
    #[inline(always)]
    pub fn rb_lcr_word_sz(&mut self) -> RbLcrWordSzW<'_, R8Uart3LcrSpec> {
        RbLcrWordSzW::new(self, 0)
    }
    #[doc = "Bit 2 - UART stop bit length"]
    #[inline(always)]
    pub fn rb_lcr_stop_bit(&mut self) -> RbLcrStopBitW<'_, R8Uart3LcrSpec> {
        RbLcrStopBitW::new(self, 2)
    }
    #[doc = "Bit 3 - UART parity enable"]
    #[inline(always)]
    pub fn rb_lcr_par_en(&mut self) -> RbLcrParEnW<'_, R8Uart3LcrSpec> {
        RbLcrParEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - UART parity mode"]
    #[inline(always)]
    pub fn rb_lcr_par_mod(&mut self) -> RbLcrParModW<'_, R8Uart3LcrSpec> {
        RbLcrParModW::new(self, 4)
    }
    #[doc = "Bit 6 - UART break control enable"]
    #[inline(always)]
    pub fn rb_lcr_break_en(&mut self) -> RbLcrBreakEnW<'_, R8Uart3LcrSpec> {
        RbLcrBreakEnW::new(self, 6)
    }
    #[doc = "Bit 7 - UART reserved bit / UART general purpose bit"]
    #[inline(always)]
    pub fn rb_lcr_dlab_rb_lcr_gp_bit(&mut self) -> RbLcrDlabRbLcrGpBitW<'_, R8Uart3LcrSpec> {
        RbLcrDlabRbLcrGpBitW::new(self, 7)
    }
}
#[doc = "UART3 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart3LcrSpec;
impl crate::RegisterSpec for R8Uart3LcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart3_lcr::R`](R) reader structure"]
impl crate::Readable for R8Uart3LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart3_lcr::W`](W) writer structure"]
impl crate::Writable for R8Uart3LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART3_LCR to value 0"]
impl crate::Resettable for R8Uart3LcrSpec {}
