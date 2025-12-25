#[doc = "Register `STK_CMPHR` reader"]
pub type R = crate::R<StkCmphrSpec>;
#[doc = "Register `STK_CMPHR` writer"]
pub type W = crate::W<StkCmphrSpec>;
#[doc = "Field `CMPH` reader - CMPH"]
pub type CmphR = crate::FieldReader<u32>;
#[doc = "Field `CMPH` writer - CMPH"]
pub type CmphW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CMPH"]
    #[inline(always)]
    pub fn cmph(&self) -> CmphR {
        CmphR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMPH"]
    #[inline(always)]
    pub fn cmph(&mut self) -> CmphW<'_, StkCmphrSpec> {
        CmphW::new(self, 0)
    }
}
#[doc = "Systick compare high register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cmphr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cmphr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StkCmphrSpec;
impl crate::RegisterSpec for StkCmphrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cmphr::R`](R) reader structure"]
impl crate::Readable for StkCmphrSpec {}
#[doc = "`write(|w| ..)` method takes [`stk_cmphr::W`](W) writer structure"]
impl crate::Writable for StkCmphrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STK_CMPHR to value 0"]
impl crate::Resettable for StkCmphrSpec {}
