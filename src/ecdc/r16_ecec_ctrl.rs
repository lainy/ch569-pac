#[doc = "Register `R16_ECEC_CTRL` reader"]
pub type R = crate::R<R16EcecCtrlSpec>;
#[doc = "Register `R16_ECEC_CTRL` writer"]
pub type W = crate::W<R16EcecCtrlSpec>;
#[doc = "Field `RB_ECDC_KEYEX_EN` reader - enable key expansion"]
pub type RbEcdcKeyexEnR = crate::BitReader;
#[doc = "Field `RB_ECDC_KEYEX_EN` writer - enable key expansion"]
pub type RbEcdcKeyexEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_RDPERI_EN` reader - when write data to dma"]
pub type RbEcdcRdperiEnR = crate::BitReader;
#[doc = "Field `RB_ECDC_RDPERI_EN` writer - when write data to dma"]
pub type RbEcdcRdperiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_WRPERI_EN` reader - when read data from dma"]
pub type RbEcdcWrperiEnR = crate::BitReader;
#[doc = "Field `RB_ECDC_WRPERI_EN` writer - when read data from dma"]
pub type RbEcdcWrperiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_MODE_SEL` reader - ECDC mode select"]
pub type RbEcdcModeSelR = crate::BitReader;
#[doc = "Field `RB_ECDC_MODE_SEL` writer - ECDC mode select"]
pub type RbEcdcModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_CLKDIV_MASK` reader - Clock divide factor"]
pub type RbEcdcClkdivMaskR = crate::FieldReader;
#[doc = "Field `RB_ECDC_CLKDIV_MASK` writer - Clock divide factor"]
pub type RbEcdcClkdivMaskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RB_ECDC_WRSRAM_EN` reader - module dma enable"]
pub type RbEcdcWrsramEnR = crate::BitReader;
#[doc = "Field `RB_ECDC_WRSRAM_EN` writer - module dma enable"]
pub type RbEcdcWrsramEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_ALGRM_MOD` reader - Encryption and decryption algorithm mode selection"]
pub type RbEcdcAlgrmModR = crate::BitReader;
#[doc = "Field `RB_ECDC_ALGRM_MOD` writer - Encryption and decryption algorithm mode selection"]
pub type RbEcdcAlgrmModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_CIPHER_MOD` reader - Block cipher mode selection"]
pub type RbEcdcCipherModR = crate::BitReader;
#[doc = "Field `RB_ECDC_CIPHER_MOD` writer - Block cipher mode selection"]
pub type RbEcdcCipherModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_KLEN_MASK` reader - Key length setting"]
pub type RbEcdcKlenMaskR = crate::FieldReader;
#[doc = "Field `RB_ECDC_KLEN_MASK` writer - Key length setting"]
pub type RbEcdcKlenMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_ECDC_DAT_MOD` writer - source data and result data is bit endian"]
pub type RbEcdcDatModW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable key expansion"]
    #[inline(always)]
    pub fn rb_ecdc_keyex_en(&self) -> RbEcdcKeyexEnR {
        RbEcdcKeyexEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when write data to dma"]
    #[inline(always)]
    pub fn rb_ecdc_rdperi_en(&self) -> RbEcdcRdperiEnR {
        RbEcdcRdperiEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when read data from dma"]
    #[inline(always)]
    pub fn rb_ecdc_wrperi_en(&self) -> RbEcdcWrperiEnR {
        RbEcdcWrperiEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECDC mode select"]
    #[inline(always)]
    pub fn rb_ecdc_mode_sel(&self) -> RbEcdcModeSelR {
        RbEcdcModeSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Clock divide factor"]
    #[inline(always)]
    pub fn rb_ecdc_clkdiv_mask(&self) -> RbEcdcClkdivMaskR {
        RbEcdcClkdivMaskR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - module dma enable"]
    #[inline(always)]
    pub fn rb_ecdc_wrsram_en(&self) -> RbEcdcWrsramEnR {
        RbEcdcWrsramEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Encryption and decryption algorithm mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_algrm_mod(&self) -> RbEcdcAlgrmModR {
        RbEcdcAlgrmModR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Block cipher mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_cipher_mod(&self) -> RbEcdcCipherModR {
        RbEcdcCipherModR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Key length setting"]
    #[inline(always)]
    pub fn rb_ecdc_klen_mask(&self) -> RbEcdcKlenMaskR {
        RbEcdcKlenMaskR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable key expansion"]
    #[inline(always)]
    pub fn rb_ecdc_keyex_en(&mut self) -> RbEcdcKeyexEnW<'_, R16EcecCtrlSpec> {
        RbEcdcKeyexEnW::new(self, 0)
    }
    #[doc = "Bit 1 - when write data to dma"]
    #[inline(always)]
    pub fn rb_ecdc_rdperi_en(&mut self) -> RbEcdcRdperiEnW<'_, R16EcecCtrlSpec> {
        RbEcdcRdperiEnW::new(self, 1)
    }
    #[doc = "Bit 2 - when read data from dma"]
    #[inline(always)]
    pub fn rb_ecdc_wrperi_en(&mut self) -> RbEcdcWrperiEnW<'_, R16EcecCtrlSpec> {
        RbEcdcWrperiEnW::new(self, 2)
    }
    #[doc = "Bit 3 - ECDC mode select"]
    #[inline(always)]
    pub fn rb_ecdc_mode_sel(&mut self) -> RbEcdcModeSelW<'_, R16EcecCtrlSpec> {
        RbEcdcModeSelW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Clock divide factor"]
    #[inline(always)]
    pub fn rb_ecdc_clkdiv_mask(&mut self) -> RbEcdcClkdivMaskW<'_, R16EcecCtrlSpec> {
        RbEcdcClkdivMaskW::new(self, 4)
    }
    #[doc = "Bit 7 - module dma enable"]
    #[inline(always)]
    pub fn rb_ecdc_wrsram_en(&mut self) -> RbEcdcWrsramEnW<'_, R16EcecCtrlSpec> {
        RbEcdcWrsramEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Encryption and decryption algorithm mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_algrm_mod(&mut self) -> RbEcdcAlgrmModW<'_, R16EcecCtrlSpec> {
        RbEcdcAlgrmModW::new(self, 8)
    }
    #[doc = "Bit 9 - Block cipher mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_cipher_mod(&mut self) -> RbEcdcCipherModW<'_, R16EcecCtrlSpec> {
        RbEcdcCipherModW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Key length setting"]
    #[inline(always)]
    pub fn rb_ecdc_klen_mask(&mut self) -> RbEcdcKlenMaskW<'_, R16EcecCtrlSpec> {
        RbEcdcKlenMaskW::new(self, 10)
    }
    #[doc = "Bit 13 - source data and result data is bit endian"]
    #[inline(always)]
    pub fn rb_ecdc_dat_mod(&mut self) -> RbEcdcDatModW<'_, R16EcecCtrlSpec> {
        RbEcdcDatModW::new(self, 13)
    }
}
#[doc = "ECED AES/SM4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_ecec_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_ecec_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16EcecCtrlSpec;
impl crate::RegisterSpec for R16EcecCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_ecec_ctrl::R`](R) reader structure"]
impl crate::Readable for R16EcecCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_ecec_ctrl::W`](W) writer structure"]
impl crate::Writable for R16EcecCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_ECEC_CTRL to value 0x20"]
impl crate::Resettable for R16EcecCtrlSpec {
    const RESET_VALUE: u16 = 0x20;
}
