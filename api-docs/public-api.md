FORMAT: 1A

# Pandemia rest API documentation

Dokumentasi rest API public untuk Pandemia.

## Group Admin

### Add Admin [POST /admin/v1/add]

Rest API endpoint untuk menambahkan admin baru.

+ Response 200 (application/json)

        {}

### Admin Count [GET /admin/v1/count]

Mendapatkan jumlah admin secara keseluruhan.

+ Response 200 (application/json)

        {}

### Admin Detail [GET /admin/v1/detail]

Mendapatkan data admin berdasarkan ID.

+ Response 200 (application/json)

        {}

### Delete Admin [POST /admin/v1/delete]

Delete admin.

+ Response 200 (application/json)

        {}

### List Admin [GET /admin/v1/list]

Mendapatkan daftar admin

+ Response 200 (application/json)

        {}

### Me Info [GET /admin/v1/me/info]

Mendapatkan informasi current admin.

+ Response 200 (application/json)

        {}

### Reset Password [POST /admin/v1/reset_password/request]

Request code untuk reset password.

+ Response 200 (application/json)

        {}

### Reset Password Verify [POST /admin/v1/reset_password/verify]

Verifikasi token untuk reset password.

+ Response 200 (application/json)

        {}

### Set New Password [POST /admin/v1/reset_password]

Mengubah password dengan password yang baru berdasarkan reset password code.

+ Response 200 (application/json)

        {}

### Update Password [POST /admin/v1/update_password]

Update password.

+ Response 200 (application/json)

        {}

## Group Authorization

API endpoint untuk keperluan otorisasi.

### Admin Authorize [POST /auth/v1/admin/authorize]

Meng-otorisasi akun admin
Admin bisa melakukan otorisasi menggunakan email / nomor telp.

+ Response 200 (application/json)

        {}

### Admin Unauthorize [POST /auth/v1/admin/unauthorize]

Unauthorize current user session, this will invalidate all valid access tokens.

+ Response 200 (application/json)

        {}

### Authorize [POST /auth/v1/authorize]

Meng-otorisasi akun yang telah teregister
User bisa melakukan otorisasi menggunakan email / nomor telp.

+ Response 200 (application/json)

        {}

### Authorize Device [POST /auth/v1/device/authorize]

Authorize user\'s device.
Ini akan otomatis membuat user baru dan menghubungkan token push notif provider
ke user tersebut.

+ Response 200 (application/json)

        {}

### Unauthorize [POST /auth/v1/unauthorize]

Unauthorize current user session, this will invalidate all valid access tokens.

+ Response 200 (application/json)

        {}

### User Get Key [GET /auth/v1/get_key]

Mendapatkan keypair dari user.

+ Response 200 (application/json)

        {}

## Group Feed

### Feed Count [GET /feed/v1/count]

Mendapatkan jumlah feed secara keseluruhan.

+ Response 200 (application/json)

        {}

### Feed Detail [GET /feed/v1/detail]

Mendapatkan data feed berdasarkan ID.

+ Response 200 (application/json)

        {}

### List Feed [GET /feed/v1/list]

Mendapatkan daftar feed

+ Response 200 (application/json)

        {}

### Query Feed [GET /feed/v1/query]

Mendapatkan daftar feed

+ Response 200 (application/json)

        {}

### Search Feed [GET /feed/v1/search]

Mendapatkan daftar feed

+ Response 200 (application/json)

        {}

## Group MapArea

### Search Map Markers [GET /map_area/v1/search]

Search for map_markers

+ Response 200 (application/json)

        {}

## Group Pandemia

### Add Record [GET /pandemia/v1/record/add]

Add record.

+ Response 200 (application/json)

        {}

### Add Record [POST /pandemia/v1/add_record]

Add record.

+ Response 200 (application/json)

        {}

### Delete Record [POST /pandemia/v1/delete_record]

Delete record by id

+ Response 200 (application/json)

        {}

### Get Info Location [GET /pandemia/v1/info_location]

Get location info (single mode)

+ Response 200 (application/json)

        {}

### Get Info Location [GET /v1/info_location]

Get location info.

+ Response 200 (application/json)

        {}

### Get Info Location [GET v1/info_location]

Get location info.

+ Response 200 (application/json)

        {}

### Get Info Location [GET /record/v1/info_location]

Get location info.

+ Response 200 (application/json)

        {}

### Get Info Location [GET /records/v1/info_location]

Get location info.

+ Response 200 (application/json)

        {}

### Get Info Locations [GET /pandemia/v1/info_locations]

Get location info (multiple mode)

+ Response 200 (application/json)

        {}

### Latest Records [GET /pandemia/v1/latest_records]

Search for records

+ Response 200 (application/json)

        {}

### Search Map Markers [GET /pandemia/v1/map/search_area]

Search for map_markers

+ Response 200 (application/json)

        {}

### Search Records [GET /pandemia/v1/search]

Search for records

+ Response 200 (application/json)

        {}

### Search Records [GET /pandemia/v1/search_records]

Search for records

+ Response 200 (application/json)

        {}

### Update Complaint [POST /pandemia/v1/set_complaint]

Update complaint.

+ Response 200 (application/json)

        {}

### Update Records [POST /pandemia/v1/update_records]

Update multiple records at once.

+ Response 200 (application/json)

        {}

### Update Settings [POST /pandemia/v1/update_settings]

Update user settings.

+ Response 200 (application/json)

        {}

## Group System

### Info [GET /system/v1/info]



+ Response 200 (application/json)

        {}

## Group User

### Activate User [POST /user/v1/user/activate]

Mengaktifkan user yang telah teregister.
Ini nantinya dijadikan link yang akan dikirimkan ke email pendaftar.

+ Response 200 (application/json)

        {}

### Add User [GET /user/v1/add]

docs.

+ Response 200 (application/json)

        {}

### Connect Create [POST /user/v1/me/connect/create]

Register and connect current account to event push notif (FCM).
Parameter `app_id` adalah app id dari client app.

+ Response 200 (application/json)

        {}

### Connect Remove [POST /user/v1/me/connect/remove]

Revoke or disconnect current account to event push notif (FCM).
Parameter `app_id` adalah app id dari client app.

+ Response 200 (application/json)

        {}

### Get Settings [GET /user/v1/settings]

Get user settings.

+ Response 200 (application/json)

        {}

### List User [GET /user/v1/users]

Listing user

+ Response 200 (application/json)

        {}

### Me Info [GET /user/v1/me/info]

Mendapatkan informasi current user.

+ Response 200 (application/json)

        {}

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

### Update Location [POST /user/v1/me/update_loc]

Update latest location

+ Response 200 (application/json)

        {}

### Update Password [POST /user/v1/update_password]

Update password.

+ Response 200 (application/json)

        {}

### Update Push Notif Settings [POST /user/v1/push_notif_settings]

Update user\'s push notif settings.

+ Response 200 (application/json)

        {}

### Update Setting [POST /user/v1/update_setting]

Update user settings.

+ Response 200 (application/json)

        {}

### Update Settings [POST /user/v1/update_settings]

Update user settings.

+ Response 200 (application/json)

        {}

### Update Users [POST /user/v1/update]

Update users.

+ Response 200 (application/json)

        {}

### User Info [GET /user/v1/user/info]

Mendapatkan data akun.

+ Response 200 (application/json)

        {}

