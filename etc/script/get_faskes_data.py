# 
# Script untuk mengunduh data fasilitas kesehatan (Faskes) dari cekdiri.id
# dan memasukkan ke data map_markers
#
#

import sys
import os
import json
import requests
from datetime import datetime
import psycopg2
from psycopg2 import sql

BASE_URI = "https://rest.cekdiri.id"

def resp_err_check(resp):
  if resp.status_code != 200:
    print("Error: %s", resp)
    sys.exit(resp.status_code)


def get_provinces():
  json_path = "provinces-data.json"

  if os.path.isfile(json_path):

    with open(json_path, 'r') as f:
      print("getting provinces data from local")
      return json.load(f)

  print("getting provinces data from remote")
  resp = requests.get(BASE_URI + "/province")
  resp_err_check(resp)

  data = resp.json()

  # save data for next call, to reduce bandwith usage
  with open(json_path, "w") as f:
    json.dump(data, f)

  return data


def get_occupation(province_id):
  resp = requests.get(BASE_URI + "/occupation/%d" % province_id)
  resp_err_check(resp)
  return resp.json()


def add_ts(d):
    d['ts'] = datetime.strptime(d['last_update'], '%a, %d %b %Y %H:%M:%S %Z')
    return d

def main():
    provinces = get_provinces()
    total =  0
    for prov in provinces['rows']:
        print("Processing province `%s`" % prov['nama_prov'])
        occs = get_occupation(int(prov['id']))
        conn = psycopg2.connect(os.environ['DATABASE_URL'])
        
        with conn.cursor() as cur:
            agg = {}
            occs = occs['rows']
            # sortir dari waktu yang paling baru
            
            occs = list(map(add_ts, occs))
            occs.sort(key=lambda a: a['ts'], reverse=False)

            for i, occ in enumerate(occs):
                # print(occ)
                h = occ['rumahsakit']
                hospital_name = h['nama_unit']
                address = h['alamat']
                lat = h['lat']
                lng = h['lon']

                # aggregate-kan
                if h['kode_rs'] not in agg:
                    agg[h['kode_rs']] = {
                        'used_lk': occ['used_lk'],
                        'used_ttl': occ['used_ttl'],
                        'uses_pr': occ['uses_pr'],
                        'vac_lk': occ['vac_lk'],
                        'vac_pr': occ['vac_pr'],
                        'vac_ttl': occ['vac_ttl'],
                        'waiting': occ['waiting'],
                    }

                    meta = [
                        sql.Literal("cekdiri.used_lk:%d" % occ['used_lk']),
                        sql.Literal("cekdiri.used_ttl:%d" % occ['used_ttl']),
                        sql.Literal("cekdiri.uses_pr:%d" % occ['uses_pr']),
                        sql.Literal("cekdiri.vac_lk:%d" % occ['vac_lk']),
                        sql.Literal("cekdiri.vac_pr:%d" % occ['vac_pr']),
                        sql.Literal("cekdiri.vac_ttl:%d" % occ['vac_ttl']),
                        sql.Literal("cekdiri.waiting:%d" % occ['waiting']),
                        sql.Literal("cekdiri.last_updated:%s" % occ['last_update'])
                    ]

                else:
                    p_occ = agg[h['kode_rs']]

                    if 'kelas' not in agg[h['kode_rs']]:
                        agg[h['kode_rs']]['kelas'] = []

                    if occ['kelas_ruang']['id'] in p_occ['kelas']:
                        # kelas sudah ada data terbaru, abaikan data lama
                        continue
                    
                    p_occ['kelas'].append(occ['kelas_ruang']['id'])

                    agg[h['kode_rs']]['used_lk'] += occ['used_lk']
                    agg[h['kode_rs']]['used_ttl'] += occ['used_ttl']
                    agg[h['kode_rs']]['uses_pr'] += occ['uses_pr']
                    agg[h['kode_rs']]['vac_lk'] += occ['vac_lk']
                    agg[h['kode_rs']]['vac_pr'] += occ['vac_pr']
                    agg[h['kode_rs']]['vac_ttl'] += occ['vac_ttl']
                    agg[h['kode_rs']]['waiting'] += occ['waiting']

                    meta = [
                        sql.Literal("cekdiri.used_lk:%d" % agg[h['kode_rs']]['used_lk'] ),
                        sql.Literal("cekdiri.used_ttl:%d" % agg[h['kode_rs']]['used_ttl'] ),
                        sql.Literal("cekdiri.uses_pr:%d" % agg[h['kode_rs']]['uses_pr'] ),
                        sql.Literal("cekdiri.vac_lk:%d" % agg[h['kode_rs']]['vac_lk'] ),
                        sql.Literal("cekdiri.vac_pr:%d" % agg[h['kode_rs']]['vac_pr'] ),
                        sql.Literal("cekdiri.vac_ttl:%d" % agg[h['kode_rs']]['vac_ttl'] ),
                        sql.Literal("cekdiri.waiting:%d" % agg[h['kode_rs']]['waiting'] ),
                        sql.Literal("cekdiri.last_updated:%s" % occ['last_update'])
                    ]
                    # end of aggregate

                stmt = sql.SQL("""INSERT INTO map_markers ("name", info, latitude, longitude, kind, meta)
                VALUES(
                {name}, {info}, {lat}, {lng}, {kind}, {meta}
                )
                ON CONFLICT (latitude, longitude)
                DO UPDATE 
                SET "name"={name}, info={info}, latitude={lat},  longitude={lng}, kind={kind}, meta={meta}
                ;""").format(name=sql.Literal(hospital_name), 
                info=sql.Literal(address),
                lat=sql.Literal(lat),
                lng=sql.Literal(lng),
                kind=sql.Literal(3),
                meta=sql.SQL('ARRAY[') + sql.SQL(', ').join(meta) + sql.SQL(']')
                )
                cur.execute(stmt)

                total += 1

                if i % 10 == 0:
                    conn.commit()

        print("  %d total data processed" % total)

    conn.close()

if __name__ == "__main__":
    main()
