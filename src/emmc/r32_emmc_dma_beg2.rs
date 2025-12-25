#[doc = "Register `R32_EMMC_DMA_BEG2` reader"]
pub type R = crate::R<R32EmmcDmaBeg2Spec>;
#[doc = "Register `R32_EMMC_DMA_BEG2` writer"]
pub type W = crate::W<R32EmmcDmaBeg2Spec>;
#[doc = "Field `RB_EMMC_DMAAD2_MASK` reader - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
pub type RbEmmcDmaad2MaskR = crate::FieldReader<u32>;
#[doc = "Field `RB_EMMC_DMAAD2_MASK` writer - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
pub type RbEmmcDmaad2MaskW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
    #[inline(always)]
    pub fn rb_emmc_dmaad2_mask(&self) -> RbEmmcDmaad2MaskR {
        RbEmmcDmaad2MaskR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
    #[inline(always)]
    pub fn rb_emmc_dmaad2_mask(&mut self) -> RbEmmcDmaad2MaskW<'_, R32EmmcDmaBeg2Spec> {
        RbEmmcDmaad2MaskW::new(self, 0)
    }
}
#[doc = "SD 16bits DMA start address register when to operate\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_dma_beg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_dma_beg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcDmaBeg2Spec;
impl crate::RegisterSpec for R32EmmcDmaBeg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_dma_beg2::R`](R) reader structure"]
impl crate::Readable for R32EmmcDmaBeg2Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_emmc_dma_beg2::W`](W) writer structure"]
impl crate::Writable for R32EmmcDmaBeg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_EMMC_DMA_BEG2 to value 0"]
impl crate::Resettable for R32EmmcDmaBeg2Spec {}
