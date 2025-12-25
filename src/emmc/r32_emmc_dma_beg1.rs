#[doc = "Register `R32_EMMC_DMA_BEG1` reader"]
pub type R = crate::R<R32EmmcDmaBeg1Spec>;
#[doc = "Register `R32_EMMC_DMA_BEG1` writer"]
pub type W = crate::W<R32EmmcDmaBeg1Spec>;
#[doc = "Field `RB_EMMC_DMAAD1_MASK` reader - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
pub type RbEmmcDmaad1MaskR = crate::FieldReader<u32>;
#[doc = "Field `RB_EMMC_DMAAD1_MASK` writer - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
pub type RbEmmcDmaad1MaskW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
    #[inline(always)]
    pub fn rb_emmc_dmaad1_mask(&self) -> RbEmmcDmaad1MaskR {
        RbEmmcDmaad1MaskR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
    #[inline(always)]
    pub fn rb_emmc_dmaad1_mask(&mut self) -> RbEmmcDmaad1MaskW<'_, R32EmmcDmaBeg1Spec> {
        RbEmmcDmaad1MaskW::new(self, 0)
    }
}
#[doc = "SD 16bits DMA start address register when to operate\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_dma_beg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_dma_beg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcDmaBeg1Spec;
impl crate::RegisterSpec for R32EmmcDmaBeg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_dma_beg1::R`](R) reader structure"]
impl crate::Readable for R32EmmcDmaBeg1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_emmc_dma_beg1::W`](W) writer structure"]
impl crate::Writable for R32EmmcDmaBeg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_EMMC_DMA_BEG1 to value 0"]
impl crate::Resettable for R32EmmcDmaBeg1Spec {}
