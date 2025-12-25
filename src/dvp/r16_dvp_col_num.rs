#[doc = "Register `R16_DVP_COL_NUM` reader"]
pub type R = crate::R<R16DvpColNumSpec>;
#[doc = "Register `R16_DVP_COL_NUM` writer"]
pub type W = crate::W<R16DvpColNumSpec>;
#[doc = "Field `RB_DVP_COL_NUM` reader - the number of PCLK cyccles contained in a row of data in RGB mode"]
pub type RbDvpColNumR = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_COL_NUM` writer - the number of PCLK cyccles contained in a row of data in RGB mode"]
pub type RbDvpColNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of PCLK cyccles contained in a row of data in RGB mode"]
    #[inline(always)]
    pub fn rb_dvp_col_num(&self) -> RbDvpColNumR {
        RbDvpColNumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of PCLK cyccles contained in a row of data in RGB mode"]
    #[inline(always)]
    pub fn rb_dvp_col_num(&mut self) -> RbDvpColNumW<'_, R16DvpColNumSpec> {
        RbDvpColNumW::new(self, 0)
    }
}
#[doc = "DVP row number of a frame indicator register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_col_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_dvp_col_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16DvpColNumSpec;
impl crate::RegisterSpec for R16DvpColNumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_dvp_col_num::R`](R) reader structure"]
impl crate::Readable for R16DvpColNumSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_dvp_col_num::W`](W) writer structure"]
impl crate::Writable for R16DvpColNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_DVP_COL_NUM to value 0"]
impl crate::Resettable for R16DvpColNumSpec {}
