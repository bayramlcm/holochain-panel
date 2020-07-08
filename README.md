# Holochain Panel

## Uygulama Hakkında
* Uygulama holochain - rust ilişkisi kullanılarak servisi hazırlandı.
* Uygulama içinde ortama uygun Kullanıcı Listele / Oluştur / Düzenle / Sil servisleri yazıldı.
* Servislerden dönen veriler Web ortamında gösterilmek üzere hazırlandı.


## Gerekli Hazırlıklar

* [Nix](https://nixos.org/) kurulumu yapılmalıdır.

## Nasıl çalıştırılır

### Nix ortamını hazırlama

``` console
$ nix-shell https://holochain.love
```

### Paketleri derleme

``` console
$ hc package
```

### Uygulamayı başlatma

``` console
$ hc run
```

### [Uygulama Arayüzüne Git](https://github.com/bayramlcm/holochain-panel-vue)