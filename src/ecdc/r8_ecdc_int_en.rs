#[doc = "Register `R8_ECDC_INT_EN` reader"]
pub type R = crate::R<R8EcdcIntEnSpec>;
#[doc = "Register `R8_ECDC_INT_EN` writer"]
pub type W = crate::W<R8EcdcIntEnSpec>;
#[doc = "Field `RB_ECDC_IE_EKDONE` reader - Key extension completion interrupt enable"]
pub type RbEcdcIeEkdoneR = crate::BitReader;
#[doc = "Field `RB_ECDC_IE_EKDONE` writer - Key extension completion interrupt enable"]
pub type RbEcdcIeEkdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_IE_SINGLE` reader - Single encryption and decryption completion interrupt enable"]
pub type RbEcdcIeSingleR = crate::BitReader;
#[doc = "Field `RB_ECDC_IE_SINGLE` writer - Single encryption and decryption completion interrupt enable"]
pub type RbEcdcIeSingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_IE_WRSRAM` writer - Memory to memory encryption and decryption completion interrupt enable"]
pub type RbEcdcIeWrsramW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Key extension completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_ekdone(&self) -> RbEcdcIeEkdoneR {
        RbEcdcIeEkdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_single(&self) -> RbEcdcIeSingleR {
        RbEcdcIeSingleR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key extension completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_ekdone(&mut self) -> RbEcdcIeEkdoneW<'_, R8EcdcIntEnSpec> {
        RbEcdcIeEkdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_single(&mut self) -> RbEcdcIeSingleW<'_, R8EcdcIntEnSpec> {
        RbEcdcIeSingleW::new(self, 1)
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_wrsram(&mut self) -> RbEcdcIeWrsramW<'_, R8EcdcIntEnSpec> {
        RbEcdcIeWrsramW::new(self, 2)
    }
}
#[doc = "Interupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_ecdc_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_ecdc_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8EcdcIntEnSpec;
impl crate::RegisterSpec for R8EcdcIntEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_ecdc_int_en::R`](R) reader structure"]
impl crate::Readable for R8EcdcIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_ecdc_int_en::W`](W) writer structure"]
impl crate::Writable for R8EcdcIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ECDC_INT_EN to value 0"]
impl crate::Resettable for R8EcdcIntEnSpec {}
