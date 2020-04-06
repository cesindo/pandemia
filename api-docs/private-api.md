FORMAT: 1A

# Pandemia rest API documentation

Dokumentasi rest API privat untuk Pandemia.

## Group Admin

## Group Authorization

### Admin Unauthorize [POST /auth/v1/admin/unauthorize]

Unauthorize user, this will invalidate all valid access tokens.

+ Response 200 (application/json)

        {}

### Authorize Device [GET /auth/v1/device/authorize]

Authorize user\'s device.

+ Response 200 (application/json)

        {}

### Remove Access Token [POST /auth/v1/remove_access_token]

Menghapus akses token

+ Response 200 (application/json)

        {}

### Unauthorize [POST /auth/v1/unauthorize]

Unauthorize user, this will invalidate all valid access tokens.

+ Response 200 (application/json)

        {}

## Group Feed

### Add Feed [POST /feed/v1/add]

Rest API endpoint untuk menambahkan feed baru.

+ Response 200 (application/json)

        {}

### Delete Feed [POST /feed/v1/delete]

Delete feed.

+ Response 200 (application/json)

        {}

## Group MapArea

## Group Pandemia

### Test Push Notif [POST /pandemia/v1/test/push_notif]

Test push notif functionality, only for internal testing purposes.

+ Response 200 (application/json)

        {}

## Group User

### List User [GET /user/v1/users]

Listing user

+ Response 200 (application/json)

        {}

### Search Users [GET /user/v1/search]

Mencari akun berdasarkan kata kunci.

+ Response 200 (application/json)

        {}

### User Count [GET /user/v1/user/count]

Mendapatkan jumlah akun secara keseluruhan.

+ Response 200 (application/json)

        {}

### User Info [GET /user/v1/user/info]

Mendapatkan data akun.

+ Response 200 (application/json)

        {}

