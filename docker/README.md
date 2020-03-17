
Untuk melakukan build docker ini gunakan script `build.sh`.

Secara bawaan script akan men-download latest binary dari precompiled nightly build untuk dimasukkan ke docker image, kalau ingin melakukan kompilasi sendiri (tidak download) maka tambahkan parameter `compile`, contoh:

    $ ./build.sh compile

Maka binary akan dihasilkan dari hasil kompilasi menggunakan docker image.

