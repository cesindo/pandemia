# 
# Script untuk memperbaharui records data dari latest data
#
#

import sys
import os
import json
from datetime import datetime
import psycopg2
from psycopg2 import sql

def get_city(city_id, cur):
    cur.execute("SELECT name, province, country_code FROM cities WHERE id=%d" % city_id)
    return cur.fetchone()

def main():
    conn = psycopg2.connect(os.environ['DATABASE_URL'])

    districts = []
    with conn.cursor() as cur:
        cur.execute("""SELECT id,name,city_id FROM districts ORDER BY name ASC""")
        districts = cur.fetchall()
        # for district_id, name in districts:
        #     print(name)

    total = 0

    with conn.cursor() as cur:

        for district_id, district_name, city_id in districts:
            (city_name, province, country_code) = get_city(city_id, cur)

            loc_path = "/%s/%s/%s" % (country_code, province, city_name)

            sql_query = sql.SQL("""UPDATE districts SET meta={meta} WHERE id={district_id}""").format(
                district_id=sql.Literal(district_id),
                meta= sql.Literal('{city_id=%d, city=%s, province=%s}' % (city_id, city_name, province))
            )

            cur.execute(sql_query)

            total = total + 1

        conn.commit()

    print("  %d data processed" % total)

    conn.close()

if __name__ == "__main__":
    main()

