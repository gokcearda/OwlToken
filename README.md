<div align="center">
  <img src="assets/image.png" width="120" alt="OwlToken logo" />
  <h1>OwlToken 🦉</h1>
  <h4>Modern Blockchain İçin Güvenli ve Gelişmiş Token Sözleşmesi</h4>
</div>

---

## Proje Hakkında

**OwlToken**, Soroban blockchain üzerinde çalışan, gelişmiş güvenlik ve yönetim özellikleri sunan üretime hazır bir akıllı token kontratıdır.  
Projede; klasik token transferi ve izinler (allowance) dışında, admin bazlı mint/burn, dondurma (freeze), esnek yönetim ve merkeziyetsiz altyapı öne çıkmaktadır.

- **Modüler mimari** (her bir işlev kendi dosyasında)
- **Freeze (Dondurma):** Kötü niyetli faaliyetleri önlemek için admin istediği hesabı dondurabilir/açabilir.
- **Admin Kontrolü:** Arz artırma, yakma, yönetici değişikliği gibi fonksiyonlar tek elde güvenli.
- **Güvenli transfer ve izinler** (ERC20 benzeri, transfer_from, approve vb.).
- **Testlerle doğrulanmış ve extensible yapı.**

---

## Kurulum ve Derleme

1. **Depoyu klonla:**
    ```sh
    git clone https://github.com/gokcearda/OwlToken.git
    cd owltoken
    ```

2. **Bağımlılıkları yükle ve derle:**
    ```sh
    cargo build --release --target wasm32-unknown-unknown
    ```
    derleme sonunda:  
    `target/wasm32-unknown-unknown/release/owltoken.wasm`  
    dosyasını elde edersin.

3. **Testler (isteğe bağlı):**
    ```sh
    cargo test
    ```

---

## Kullanım (Kısaca)

- **Freighter veya başka bir cüzdanla testnet adresi edin.**
- .wasm sözleşmeni Soroban CLI veya başka bir araçla ağa yükle.
- Admin adresini ayarla, mint/transfer/freeze gibi işlevleri kullanmaya başla.

Temel fonksiyonlar:
- **Mint:** Yeni token üret ve kullanıcıya ata
- **Transfer:** Kullanıcılar arasında transfer
- **Burn:** Token yak
- **Freeze/Unfreeze:** Kullanıcı hesabını dondur/aç
- **Approve/Transfer_from:** Harcama yetkilendirmesi (allowance)

---

## Dosya Yapısı