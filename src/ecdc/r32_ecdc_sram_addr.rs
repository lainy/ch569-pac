#[doc = "Register `R32_ECDC_SRAM_ADDR` reader"]
pub type R = crate::R<R32EcdcSramAddrSpec>;
#[doc = "Register `R32_ECDC_SRAM_ADDR` writer"]
pub type W = crate::W<R32EcdcSramAddrSpec>;
#[doc = "Field `RB_ECDC_SRAM_ADDR` reader - encryption and decryption sram start address register"]
pub type RbEcdcSramAddrR = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_SRAM_ADDR` writer - encryption and decryption sram start address register"]
pub type RbEcdcSramAddrW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - encryption and decryption sram start address register"]
    #[inline(always)]
    pub fn rb_ecdc_sram_addr(&self) -> RbEcdcSramAddrR {
        RbEcdcSramAddrR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - encryption and decryption sram start address register"]
    #[inline(always)]
    pub fn rb_ecdc_sram_addr(&mut self) -> RbEcdcSramAddrW<'_, R32EcdcSramAddrSpec> {
        RbEcdcSramAddrW::new(self, 0)
    }
}
#[doc = "encryption and decryption sram start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sram_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sram_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcSramAddrSpec;
impl crate::RegisterSpec for R32EcdcSramAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_sram_addr::R`](R) reader structure"]
impl crate::Readable for R32EcdcSramAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_sram_addr::W`](W) writer structure"]
impl crate::Writable for R32EcdcSramAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_SRAM_ADDR to value 0"]
impl crate::Resettable for R32EcdcSramAddrSpec {}
