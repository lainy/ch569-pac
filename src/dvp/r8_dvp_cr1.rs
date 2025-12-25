#[doc = "Register `R8_DVP_CR1` reader"]
pub type R = crate::R<R8DvpCr1Spec>;
#[doc = "Register `R8_DVP_CR1` writer"]
pub type W = crate::W<R8DvpCr1Spec>;
#[doc = "Field `RB_DVP_DMA_ENABLE` reader - DVP dma enable"]
pub type RbDvpDmaEnableR = crate::BitReader;
#[doc = "Field `RB_DVP_DMA_ENABLE` writer - DVP dma enable"]
pub type RbDvpDmaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_ALL_CLR` reader - DVP all clear, high action"]
pub type RbDvpAllClrR = crate::BitReader;
#[doc = "Field `RB_DVP_ALL_CLR` writer - DVP all clear, high action"]
pub type RbDvpAllClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_RCV_CLR` reader - DVP receive logic clear, high action"]
pub type RbDvpRcvClrR = crate::BitReader;
#[doc = "Field `RB_DVP_RCV_CLR` writer - DVP receive logic clear, high action"]
pub type RbDvpRcvClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_BUF_TOG` reader - DVP bug toggle by software"]
pub type RbDvpBufTogR = crate::BitReader;
#[doc = "Field `RB_DVP_BUF_TOG` writer - DVP bug toggle by software"]
pub type RbDvpBufTogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_enable(&self) -> RbDvpDmaEnableR {
        RbDvpDmaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP all clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&self) -> RbDvpAllClrR {
        RbDvpAllClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive logic clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&self) -> RbDvpRcvClrR {
        RbDvpRcvClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&self) -> RbDvpBufTogR {
        RbDvpBufTogR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_enable(&mut self) -> RbDvpDmaEnableW<'_, R8DvpCr1Spec> {
        RbDvpDmaEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - DVP all clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&mut self) -> RbDvpAllClrW<'_, R8DvpCr1Spec> {
        RbDvpAllClrW::new(self, 1)
    }
    #[doc = "Bit 2 - DVP receive logic clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&mut self) -> RbDvpRcvClrW<'_, R8DvpCr1Spec> {
        RbDvpRcvClrW::new(self, 2)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&mut self) -> RbDvpBufTogW<'_, R8DvpCr1Spec> {
        RbDvpBufTogW::new(self, 3)
    }
}
#[doc = "DVP control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8DvpCr1Spec;
impl crate::RegisterSpec for R8DvpCr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_dvp_cr1::R`](R) reader structure"]
impl crate::Readable for R8DvpCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r8_dvp_cr1::W`](W) writer structure"]
impl crate::Writable for R8DvpCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_DVP_CR1 to value 0x06"]
impl crate::Resettable for R8DvpCr1Spec {
    const RESET_VALUE: u8 = 0x06;
}
