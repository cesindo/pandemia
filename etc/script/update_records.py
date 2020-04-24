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

    calculated_at = datetime.now().strftime("%d-%m-%Y")

    with conn.cursor() as cur:

        for district_id, district_name, city_id in districts:
            city = get_city(city_id, cur)

            loc_path = "/%s/%s/%s" % (city[2], city[1], city[0])

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

                total += 1

                conn.commit()


    print("  %d data processed" % total)

    conn.close()

if __name__ == "__main__":
    main()
