#[doc = "Register `R32_ECDC_SRAM_LEN` reader"]
pub type R = crate::R<R32EcdcSramLenSpec>;
#[doc = "Register `R32_ECDC_SRAM_LEN` writer"]
pub type W = crate::W<R32EcdcSramLenSpec>;
#[doc = "Field `RB_ECDC_SRAM_LEN` reader - encryption and decryption sram size register"]
pub type RbEcdcSramLenR = crate::FieldReader<u16>;
#[doc = "Field `RB_ECDC_SRAM_LEN` writer - encryption and decryption sram size register"]
pub type RbEcdcSramLenW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - encryption and decryption sram size register"]
    #[inline(always)]
    pub fn rb_ecdc_sram_len(&self) -> RbEcdcSramLenR {
        RbEcdcSramLenR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - encryption and decryption sram size register"]
    #[inline(always)]
    pub fn rb_ecdc_sram_len(&mut self) -> RbEcdcSramLenW<'_, R32EcdcSramLenSpec> {
        RbEcdcSramLenW::new(self, 0)
    }
}
#[doc = "encryption and decryption sram size register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sram_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sram_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcSramLenSpec;
impl crate::RegisterSpec for R32EcdcSramLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_sram_len::R`](R) reader structure"]
impl crate::Readable for R32EcdcSramLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_sram_len::W`](W) writer structure"]
impl crate::Writable for R32EcdcSramLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_SRAM_LEN to value 0"]
impl crate::Resettable for R32EcdcSramLenSpec {}
