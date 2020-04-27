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
    cur.execute("SELECT id, name, province, country_code FROM cities WHERE id=%d" % city_id)
    return cur.fetchone()

def main():
    conn = psycopg2.connect(os.environ['DATABASE_URL'])

    districts = []
    with conn.cursor() as cur:
        cur.execute("""SELECT id,name,city_id FROM districts ORDER BY name ASC""")
        districts = cur.fetchall()
        # for district_id, name in districts:
        #     print(name)

    total_city = 0
    total_district = 0

    calculated_at = datetime.now().strftime("%d-%m-%Y")

    with conn.cursor() as cur:

        cities = []

        for district_id, district_name, city_id in districts:
            city = get_city(city_id, cur)

            if city[0] not in list(map(lambda a: a[0], cities)):
                cities.append(city)

            loc_path = "/%s/%s/%s" % (city[3], city[2], city[1])

            # hapus data yang telah dikalkulasikan sebelumnya pada hari yang sama apabila ada
            # memastikan ke-idempoten-an hasil kalkulasinya
            # cur.execute(sql.SQL("""SELECT COUNT(id) FROM records WHERE loc={loc} AND loc_path={loc_path} and meta @> '{{calculated_at=%s}}'""" % calculated_at)
            #     .format(loc=sql.Literal(district_name),loc_path=sql.Literal(loc_path)))
            # count = cur.fetchone()
            # if count[0] > 0:
            cur.execute(sql.SQL("""DELETE FROM records WHERE loc={loc} AND loc_path={loc_path} and meta @> '{{:daily_calculation:,calculated_at=%s}}'""" % calculated_at)
                .format(loc=sql.Literal(district_name),loc_path=sql.Literal(loc_path)))
                # continue
    

            sqlq = sql.SQL("""SELECT SUM(odp) AS odp, SUM(pdp) AS pdp, SUM(cases) AS positive,
            SUM(recovered) AS recovered, SUM(deaths) AS deaths, SUM(ppdwt) AS ppwdt,
            SUM(pptb) AS pptb, SUM(odpsp) AS odpsp, SUM(pdps) AS pdps,
            SUM(pdpm) AS pdpm, SUM(otg) AS otg
            FROM district_data WHERE district_id={district_id}
            """).format(district_id=sql.Literal(district_id))

            cur.execute(sqlq)

            for odp, pdp, positive, recovered, deaths, ppdwt, pptb, odpsp, pdps, pdpm, otg in cur.fetchall():

                print("processing %s ..." % loc_path)

                sqlq = sql.SQL("""INSERT INTO records 
                (loc, loc_kind, total_cases, total_deaths, 
                total_recovered, latest, 
                ppdwt, pptb, odp, odpsp, pdp, pdps, pdpm, otg, loc_path, meta)VALUES
                ({loc}, 5, {total_cases}, {total_deaths}, 
                {total_recovered}, false,
                {ppdwt}, {pptb}, {odp}, {odpsp}, {pdp}, {pdps}, {pdpm}, {otg}, {loc_path}, {meta})
                """).format(loc=sql.Literal(district_name),
                total_cases = sql.Literal(positive),
                total_deaths = sql.Literal(deaths),
                total_recovered = sql.Literal(recovered),
                ppdwt = sql.Literal(ppdwt),
                pptb = sql.Literal(pptb),
                odp = sql.Literal(odp),
                odpsp = sql.Literal(odpsp),
                pdp = sql.Literal(pdp),
                pdps = sql.Literal(pdps),
                pdpm = sql.Literal(pdpm),
                otg = sql.Literal(otg),
                loc_path = sql.Literal(loc_path),
                meta=sql.SQL('ARRAY[\'loc_scope=Indonesia\',\':daily_calculation:\',\'calculated_at=%s\']' % calculated_at)
                 )

                cur.execute(sqlq)

                total_district += 1

                conn.commit()

        # Calculate latest records for city
        for city in cities:
            sqlq = sql.SQL("""SELECT
            COALESCE(SUM(cases)) AS positive, COALESCE(SUM(recovered)) AS recovered, COALESCE(SUM(deaths)) AS deaths FROM
            district_data WHERE city_id={city_id}""").format(city_id=sql.Literal(city[0]))
            cur.execute(sqlq)

            positive, recovered, deaths = cur.fetchone()

            sqlq = sql.SQL("""INSERT INTO records (loc, loc_kind, total_cases, total_recovered, total_deaths, latest, meta)VALUES
            ({loc}, 4,{positive}, {recovered}, {deaths}, true, {meta})
            """).format(
                loc = sql.Literal(city[1]),
                positive=sql.Literal(positive),
                recovered=sql.Literal(recovered),
                deaths=sql.Literal(deaths),
                meta=sql.SQL('ARRAY[\'loc_scope=Indonesia\',\':daily_calculation:\',\'calculated_at=%s\']' % calculated_at)
            )

            cur.execute(sqlq)
            conn.commit()

            total_city += 1

    print("  %d district(s) and %d citie(s) processed" % (total_district, total_city))

    conn.close()

if __name__ == "__main__":
    main()
