FORMAT: 1A

# Pandemia rest API documentation

Dokumentasi rest API public untuk Pandemia.

## Group User

### Register User [POST /user/v1/user/register]

Rest API endpoint untuk mendaftarkan akun baru.
Setelah register akun tidak langsung aktif, perlu melakukan
aktifasi menggunakan endpoint `/user/activate`.

+ Request JSON (application/json)

        {
          "full_name": "Robin",
          "email": "robin@example.com",
          "phone_num": "123"
        }

+ Response 200 (application/json)

        {}

