FORMAT: 1A

# Pandemia rest API documentation

Dokumentasi REST API untuk proyek [Pandemia](https://pandemia.cesindo.top/)

Pandemia adalah program sumber terbuka (open source) yang dikembangkan oleh komunitas
untuk memudahkan dalam memantau persebaran wabah, sehingga dapat mengambil keputusan yang
lebih bijak dan terukur dalam melakukan kegiatan kesehariannya.
Dokumentasi API ini merupakan _auto-generated_ dari kode sumber yang ada di [Github](https://github.com/cesindo/pandemia).
Basis _endpoint_ : [https://pandemia.cesindo.top/api](https://pandemia.cesindo.top/api)
## Group Feed

### Query Feed [GET /feed/v1/query]

Mendapatkan daftar feed terbaru.

+ Parameters

    + loc: "Jakarta"
    + offset: 0 (number)
    + limit: 10 (number)

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "description": "",
            "result": [
                {
                    "count": 1,
                    "entries": [
                        {
                            "id": 101,
                            "creator_id": 0,
                            "creator_name": "",
                            "loc": "Jakarta",
                            "kind": 2,
                            "text": "3 kasus baru, total 310",
                            "hashtags": [],
                            "meta": [],
                            "ts": "2020-04-07T18:47:30.376384625Z"
                        }
                    ]
                }
            ]
        }

## Group MapArea

### Search Map Markers [GET /map_area/v1/search]

Mencari data pada radius 5km pada suatu wilayah menggunakan titik longlat.

+ Parameters

    + longitude: 110.408333 (number)
    + latitude: -7.840243 (number)
    + query: (optional) "Banguntapan"

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "description": "",
            "result": [
                {
                    "longitude": 110.408333,
                    "latitude": -7.840243,
                    "kind": 1,
                    "caption": "Bantul",
                    "desc": "Info wilayah Bantul Yogyakarta",
                    "detail": {
                        "total_cases": 1,
                        "total_deaths": 0,
                        "total_recovered": 1
                    }
                }
            ]
        }

## Group Pandemia

### Add Record [POST /pandemia/v1/add_record]

Add record.

+ Response 200 (application/json)

        {}

### Delete Record [POST /pandemia/v1/delete_record]

Delete record by id

+ Response 200 (application/json)

        {}

### Get Info Location [GET /pandemia/v1/info_location]

Get location stats data (single mode).

+ Parameters

    + loc: "Yogyakarta"
    + with_history: false (boolean, optional) - whether to return historical data or not.

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "description": "",
            "result": {
                "id": 1,
                "loc": "Yogyakarta",
                "loc_kind": 4,
                "total_cases": 10,
                "total_deaths": 1,
                "total_recovered": 5,
                "active_cases": 10,
                "critical_cases": 0,
                "latest": true,
                "meta": [],
                "last_updated": "2020-04-07T18:47:30.376384625Z"
            }
        }

### Get Info Locations [GET /pandemia/v1/info_locations]

Get per location stats data, use comma for multiple locations.

+ Parameters

    + loc: "Yogyakarta"
    + with_history: false (boolean, optional) - whether to return historical data or not.

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "description": "",
            "result": [
                {
                    "name": "Yogyakarta",
                    "latest_record": {
                        "id": 1,
                        "loc": "Yogyakarta",
                        "loc_kind": 4,
                        "total_cases": 10,
                        "total_deaths": 1,
                        "total_recovered": 5,
                        "active_cases": 10,
                        "critical_cases": 0,
                        "latest": true,
                        "meta": [],
                        "last_updated": "2020-04-07T18:47:30.376384625Z"
                    },
                    "history": []
                }
            ]
        }

### Latest Records [GET /pandemia/v1/latest_records]

Search for records

+ Response 200 (application/json)

        {}

### Search Records [GET /pandemia/v1/search_records]

Get latest data record search/query by location.

+ Parameters

    + query: "Yogyakarta"
    + offset: 0 (number)
    + limit: 10 (number)

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

## Group System

### Info [GET /system/v1/info]

Get build information.

+ Response 200 (application/json)

        {
            "build": "release build x86_64-unknown-linux-gnu @ 2020-04-07 01:11:19.718638708 +08:00",
            "git": "359e2c28f7ef7cc5c1b02f1d0c5ef75e4584b3d1",
            "version": "0.1.5"
        }

