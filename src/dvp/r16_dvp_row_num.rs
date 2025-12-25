#[doc = "Register `R16_DVP_ROW_NUM` reader"]
pub type R = crate::R<R16DvpRowNumSpec>;
#[doc = "Register `R16_DVP_ROW_NUM` writer"]
pub type W = crate::W<R16DvpRowNumSpec>;
#[doc = "Field `RB_DVP_ROW_NUM` reader - the number of rows contained in a frame of image data"]
pub type RbDvpRowNumR = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_ROW_NUM` writer - the number of rows contained in a frame of image data"]
pub type RbDvpRowNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of rows contained in a frame of image data"]
    #[inline(always)]
    pub fn rb_dvp_row_num(&self) -> RbDvpRowNumR {
        RbDvpRowNumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of rows contained in a frame of image data"]
    #[inline(always)]
    pub fn rb_dvp_row_num(&mut self) -> RbDvpRowNumW<'_, R16DvpRowNumSpec> {
        RbDvpRowNumW::new(self, 0)
    }
}
#[doc = "DVP row number of a frame indicator register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_row_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_dvp_row_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16DvpRowNumSpec;
impl crate::RegisterSpec for R16DvpRowNumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_dvp_row_num::R`](R) reader structure"]
impl crate::Readable for R16DvpRowNumSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_dvp_row_num::W`](W) writer structure"]
impl crate::Writable for R16DvpRowNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_DVP_ROW_NUM to value 0"]
impl crate::Resettable for R16DvpRowNumSpec {}
