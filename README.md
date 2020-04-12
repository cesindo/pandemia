Pandemia
============================

[![Build Status](https://travis-ci.org/cesindo/pandemia.svg?branch=master)](https://travis-ci.org/cesindo/pandemia)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
![Platform](https://img.shields.io/badge/platform-ios%20%7C%20android-%23989898)
[![Gitter](https://badges.gitter.im/pandemia_/community.svg)](https://gitter.im/pandemia_/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)


Pandemia adalah program sumber terbuka (_open source_) yang dikembangkan oleh komunitas
untuk memudahkan dalam memantau persebaran wabah, sehingga dapat mengambil keputusan yang 
lebih bijak dan terukur dalam melakukan kegiatan kesehariannya.

**Motivasi**

Melihat situasi pandemi covid-19 terutama di Indonesia yang semakin hari semakin luas, serta masih sangat terbatasnya peran teknologi dalam pemanfaatannya untuk mitigasi,
ditambah sumber data dari pemerintah yang masih tersebar di sana-sini, maka kami bermaksud untuk membuat satu fondasi aplikasi lengkap yang mana bisa dimanfaatkan
oleh penerbit data yang bisa merupakan pemerintah maupun organisasi independen serta pengkonsumsi data dalam hal ini masyarakat luas. Penerbit data memasukkan data melalui
_control center_ berbasis web, sementara masyarakat memonitor secara _real-time_ melalui _App_ pada _smartphone_ mereka, Mengapa App? Karena beberapa fungsi tidak maksimal atau bahkan tidak jalan apabila dalam bentuk web seperti yang sudah ada sekarang, kedua App tidak hanya berfungsi sebagai alat monitor, tetapi juga bisa digunakan sebagai alat bantu pelengkap data secara _crowd sourcing_.

**Target:**

1. [x] Realtime feed berupa data update kasus terbaru pada suatu daerah.
2. [x] Push notif untuk setiap update kasus terbaru di daerah kita.
3. [x] Data statistik daerah.
4. [x] Data Fakta dan Hoax berkaitan dengan pandemi.
5. [x] Peta Pandemi.
6. [ ] Push notif berupa peringatan ketika memasuki daerah dengan riwayat korban positif.
7. [x] Peta Keluhan, menandai diri kita sedang mengalami keluhan kesehatan secara anonim.
8. [x] Peta data fasilitas kesehatan seperti ketersediaan tempat tidur pasien pada rumah sakit.

Tangkapan Layar
-----------------

![Pandemia Feed](/img/pandemia-feed.jpg)
![Pandemia Push Notif](/img/pandemia-push-notif.jpg)
![Pandemia Push Notif Android](/img/pandemia-push-notif-android.jpg)
![Pandemia Stats](/img/pandemia-stats.png)
![Pandemia Hoax/Fact](/img/pandemia-hoaxfact.png)
![Pandemia Map](/img/pandemia-map.png)
![Pandemia Settings](/img/pandemia-settings.png)

**Tersedia juga control center (dashboard) dalam bentuk web untuk mempermudah komunitas dalam memperbaharui data.**

![Pandemia Control Center Login](/img/pandemia-cc.png)
![Pandemia Control Center](/img/pandemia-cc-records.png)

## Info Pengguna

Anda bisa mengunduh aplikasi Pandemia terbaru dari halaman [rilis](https://github.com/cesindo/pandemia/releases), halaman tersebut akan selalu diperbaharui apabila ada rilis terbaru.

## Info Pengembang

Pandemia adalah proyek sumber terbuka (_open source_), bagi para pengembang (_programmer_) yang ingin ikut membantu silahkan _fork_ kode sumber Pandemia di [https://github.com/cesindo/pandemia](https://github.com/cesindo/pandemia).

Pertanyaan dan diskusi teknis bisa langsung di [Gitter Pandemia](https://gitter.im/pandemia_/community), kami akan sangat senang melayani.

Untuk yang ingin langsung mengkonsumsi API bisa baca dokumentasi API di [Pandemia REST API](https://pandemia.cesindo.top/dev/).

Basis API endpoint ada di [https://pandemia.cesindo.top/api](https://pandemia.cesindo.top/api)

Contoh untuk mendapatkan informasi build [https://pandemia.cesindo.top/api/system/v1/info](https://pandemia.cesindo.top/api/system/v1/info).

Bagi pengguna Postman bisa download postman file definisi-nya: [pandemia-api.postman](https://pandemia.cesindo.top/downloads/pandemia-api.postman).

### Kebutuhan

Daftar kebutuhan berikut dibutuhkan apabila kita ingin melakukan build di mesin _environment_ lokal, kamu bisa juga melakukan build menggunakan Docker sehingga tidak perlu menginstall satu-per-satu kebutuhan ini. Untuk build menggunakan Docker lihat bagian *Build menngunakan Docker*.
Berikut kebutuhan pokok untuk bisa melakukan build di mesin lokal:

**Backend Server:**

1. [Rust](https://www.rust-lang.org/)
2. PostgreSQL >= 9.x
3. [diesel](http://diesel.rs)
4. [Aglio](https://www.npmjs.com/package/aglio) (optional, untuk dokumentasi)
5. [Rustfmt](https://github.com/rust-lang/rustfmt)
5. [Cargo clippy](https://github.com/rust-lang/rust-clippy)
6. [Cargo audit](https://github.com/RustSec/cargo-audit)
7. [Protocol Buffer](https://developers.google.com/protocol-buffers/)

**Web Frontend:**

1. [Node JS](https://nodejs.org)
2. [NPM](https://www.npmjs.com/)
3. [Yarn](https://yarnpkg.com/)

**Mobile Frontend:**

1. [Flutter](https://flutter.dev/)
2. [Xcode](https://developer.apple.com/xcode/) <-- untuk iOS
3. [Android SDK](https://developer.android.com/studio/releases/sdk-tools) <-- untuk Android

Build
-----------

Sebelum melakukan build pastikan dulu Libpq (Library-nya PostgreSQL) telah tersedia, di Ubuntu bisa menggunakan perintah `apt install libpq-dev` atau di Debian `apt install libpq-devel`, di OSX bisa menggunakan perintah: `brew install libpq`.

Buat konfigurasi environment variable melalui file `.env` (dot env):


```
export DATABASE_URL=postgresql://localhost/pandemia?sslmode=disable
export DATABASE_TEST_URL=postgresql://localhost/pandemia_test?sslmode=disable

export FCM_SERVER_KEY=xxxxxxxx
export GEOLOCATOR_API_KEY=xxxxxxxx
```

Untuk contoh bisa lihat file `.env.example`.

Setelah semua siap, ketikkan:

    $ cargo build


Build menggunakan Docker
----------------------------

Cara paling mudah untuk melakukan build adalah menggunakan Docker:

    $ docker run -it --rm -v $(pwd):/workdir \
        -v /tmp:/root/.cargo/git \
        -v /tmp:/root/.cargo/registry \
        anvie/rust-musl-build:latest \
        cargo build --release --target=x86_64-unknown-linux-musl

Docker image `anvie/rust-musl-build` adalah container berbasis Linux dan sudah berisi semua kebutuhan development untuk build project ini, setelah build selesai
output bisa didapatkan di `target/x86_64-unknown-linux-musl`.

Kamu bisa juga menjalankan perintah tersebut menggunakan make:

    $ make release-linux

Testing
----------

Testing kebanyakan ditulis terintegrasi (integration testing), untuk itu perlu menjalankan database
dan mempersiapkan environment-nya, ini hanya perlu dijalankan sekali, ketikkan:

    $ make test-env

**CATATAN**: Perintah `test-env` akan membuat database baru dengan nama `pandemia_test` dimana database ini akan digunakan
sebagai storage ketika proses testing terjadi.
Perintah `make test-env` ini juga perlu dijalankan ulang apabila ada perubahan schema untuk memastikan schema
dalam database selalu up-to-date.

Untuk melakukan test ketikkan:

    $ make test

Menjalankan
-------------

Untuk menjalankan service pandemia perlu dipastikan service PostgreSQL sudah jalan terlebih dahulu, dan telah disetup database-nya.

Untuk men-setup database bisa menggunakan perintah:

    $ make reset-db

Selanjutkan jalankan Pandemia servernya:

    $ cargo run --bin pandemia_server

Atau untuk development gunakan script:

    $ ./etc/script/run-dev.sh

Frontend
------------

Untuk frontend web menggunakan Vue.js, base ada di direktori `/frontends`.

Apabila ingin mencoba menjalankannya bisa check frontend web dengan langkah-langkah berikut:

    $ cd frontends/pandemia_web
    $ npm install
    $ npm start

Apabila menggunakan Yarn bisa:

    $ cd frontends/pandemia_web
    $ yarn install
    $ yarn serve

Buka http://localhost:8080/ atau apabila server juga jalan di lokal bisa port-nya berubah menjadi http://localhost:8081/

Untuk Vue.js ada di `/frontends/pandemia_web`:

    $ cd frontends/pandemia_web
    $ yarn install
    $ yarn serve

**CATATAN**: Kamu bisa menggunakan npm maupun yarn, tapi direkomendasikan menggunakan yarn.

Untuk frontend mobile menggunakan Flutter, bisa ditemukan di direktori `/frontends/pandemia_mobile`.
Contoh cara menjalankan:

    $ cd frontends/pandemia_mobile
    $ flutter pub get
    $ flutter run

Dokumentasi
-------------

Dokumentasi dibagikan menjadi beberapa bagian:

1. Dokumentasi pustaka (library).
2. Dokumentasi Rest API.

Untuk menggenerasikan dokumentasi pustaka cukup ketikkan:

    $ make lib-docs

Untuk menggenerasikan dokumentasi rest API:

    $ make api-docs

**CATATAN**: Penggenerasian dokumentasi untuk rest API membutuhkan tool [Aglio](https://www.npmjs.com/package/aglio).


Konvensi
------------

Setiap perubahan pada project ini harus mengikuti konvensi ini.

* Setiap menambahkan parameter melalui environment variable yang wajib (yang menyebabkan aplikasi error apabila tidak diset) contohnya seperti yang diakses menggunakan macro `env!` maka perlu mengupdate juga file `.env.example` karena sebagai contoh konfigurasi untuk team.

Sebelum melakukan commit harus:

* Memastikan kodenya telah diformat menggunakan perintah: `make fmt`.
* Memastikan kodenya telah layak sesuai standar dengan cara menjalankan perintah: `make lint`.
* Memastikan kodenya telah lolos unittest dengan cara menjalankan perintah: `make test`.
* Memastikan kodenya telah aman dari dependensi yang bermasalah dengan menjalankan perintah: `make audit`.
* Menggunakan tata bahasa yang mudah dipahami dan menjelaskan perubahan mendasar pada commit message-nya.


Troubleshooting
-----------------

*Case*

    $ docker-compose up
    ERROR: Version in "./docker-compose.yml" is unsupported.

Itu artinya versi `docker-compose` yang ada di sistem mu tidak support, maka perlu dilakukan install manual:

*Fix (ubuntu 16.04)* 

    $ sudo curl -L sudo curl -L "https://github.com/docker/compose/releases/download/1.22.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    $ sudo chmod +x /usr/local/bin/docker-compose

*Case*
 
Ketika install diesel muncul error seperti berikut:

    $ cargo install diesel_cli --no-default-features --features postgres 
     Error : cannot find lpg

Itu artinya library postgres-devel belum diinstall, maka perlu intsall dulu libpq:

*Fix (ubuntu 16.04)* 
    sudo apt install libpq-dev   

*Case*

Ketika sedang compile/test gagal dengan error kurang lebih seperti ini:
    
    ERROR pandemia::api::error] error: "relation \"transactions\" does not exist"

Itu artinya table schema mu yang digunakan untuk test belum up-to-date dengan schema terbaru, maka perlu dilakukan migration untuk apply patch-nya:

    $ diesel migration run --database-url postgresql://localhost/pandemia_test?sslmode=disable

Atau reset database untuk test-nya agar di-rebuild schema-nya dari pertama:

    $ make test-env


Kontributor
-------------

Terimakasih kepada para kontributor yang telah membantu proyek ini:

* Delameta (donasi server)
* Detax.org (data hoax/fakta)
* Robin - @anvie
* Cak Nasrul - @luffynas
* Fatkhurohman
* Samsul
* Rifai
* Muiz

Kami masih menerima donasi berupa server dan tenaga data, apabila ada yang bersedia menjadi kontributor/relawan kirim email ke proyekpandemia@gmail.com


