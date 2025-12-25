#[doc = "Register `R32_DVP_DMA_BUF1` reader"]
pub type R = crate::R<R32DvpDmaBuf1Spec>;
#[doc = "Register `R32_DVP_DMA_BUF1` writer"]
pub type W = crate::W<R32DvpDmaBuf1Spec>;
#[doc = "Field `RB_DVP_DMA_BUF1` reader - the receiving address1 of DMA"]
pub type RbDvpDmaBuf1R = crate::FieldReader<u32>;
#[doc = "Field `RB_DVP_DMA_BUF1` writer - the receiving address1 of DMA"]
pub type RbDvpDmaBuf1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - the receiving address1 of DMA"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf1(&self) -> RbDvpDmaBuf1R {
        RbDvpDmaBuf1R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - the receiving address1 of DMA"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf1(&mut self) -> RbDvpDmaBuf1W<'_, R32DvpDmaBuf1Spec> {
        RbDvpDmaBuf1W::new(self, 0)
    }
}
#[doc = "DVP dma buffer1 addr\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_dvp_dma_buf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_dvp_dma_buf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32DvpDmaBuf1Spec;
impl crate::RegisterSpec for R32DvpDmaBuf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_dvp_dma_buf1::R`](R) reader structure"]
impl crate::Readable for R32DvpDmaBuf1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_dvp_dma_buf1::W`](W) writer structure"]
impl crate::Writable for R32DvpDmaBuf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_DVP_DMA_BUF1 to value 0"]
impl crate::Resettable for R32DvpDmaBuf1Spec {}
