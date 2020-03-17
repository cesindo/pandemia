#!/usr/bin/env python

import os
from glob import glob

dir_path = os.path.dirname(os.path.realpath(__file__))

sql_files = [y for x in os.walk(dir_path + '/../migrations') for y in glob(os.path.join(x[0], '*.sql'))]

sql_files.sort()

fout = open("postgre_server/init.sql", "w")

for sql in sql_files:
  if 'diesel_initial' in sql or sql.endswith("down.sql"):
    continue
  print("processing: %s" % sql)
  f = open(sql, "r")
  for line in f.readlines():
    fout.write(line)
  f.close()

fout.close()



