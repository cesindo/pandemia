CREATE TABLE cities (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  "name" TEXT NOT NULL,
  province TEXT NOT NULL,
  country_code TEXT NOT NULL,
  area_code VARCHAR(30) NOT NULL,
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_cities_area_code ON cities(area_code);
CREATE UNIQUE INDEX idx_cities_province_name ON cities(province,"name");

ALTER TABLE sub_reports ADD COLUMN city_id BIGINT NOT NULL REFERENCES cities(id);

-- Add pre-set city/regency data.
COPY public.cities (id, "name", province, country_code, area_code, ts) FROM stdin;
1	Wonosobo	Jawa Tengah	Indonesia	W318	2020-04-15 06:03:05.460201
2	Kota Binjai	Sumatera Utara	Indonesia	UU5V	2020-04-20 17:12:28.760031
3	Kabupaten Pulau Taliabu	Maluku Utara	Indonesia	AGES	2020-04-20 17:12:28.760031
4	Kabupaten Bengkulu Tengah	Bengkulu	Indonesia	JH1E	2020-04-20 17:12:28.760031
5	Kota Jakarta Pusat	Dki Jakarta	Indonesia	RBJG	2020-04-20 17:12:28.760031
6	Kabupaten Konawe	Sulawesi Tenggara	Indonesia	FVCL	2020-04-20 17:12:28.760031
7	Kabupaten Malinau	Kalimantan Utara	Indonesia	H5VD	2020-04-20 17:12:28.760031
8	Kabupaten Rokan Hulu	Riau	Indonesia	0AKG	2020-04-20 17:12:28.760031
9	Kabupaten Indramayu	Jawa Barat	Indonesia	GDBB	2020-04-20 17:12:28.760031
10	Kabupaten Barito Selatan	Kalimantan Tengah	Indonesia	YGXI	2020-04-20 17:12:28.760031
11	Kabupaten Lanny Jaya	Papua	Indonesia	NLBG	2020-04-20 17:12:28.760031
12	Kabupaten Parigi Moutong	Sulawesi Tengah	Indonesia	3CUV	2020-04-20 17:12:28.760031
13	Kabupaten Banjarnegara	Jawa Tengah	Indonesia	NEBT	2020-04-20 17:12:28.760031
14	Kabupaten Rote Ndao	Nusa Tenggara Timur	Indonesia	AOOR	2020-04-20 17:12:28.760031
15	Kabupaten Lamongan	Jawa Timur	Indonesia	TLVT	2020-04-20 17:12:28.760031
16	Kabupaten Aceh Tamiang	Aceh	Indonesia	UPAC	2020-04-20 17:12:28.760031
17	Kota Tanjung Balai	Sumatera Utara	Indonesia	TBM1	2020-04-20 17:12:28.760031
18	Kabupaten Ponorogo	Jawa Timur	Indonesia	RKJF	2020-04-20 17:12:28.760031
19	Kabupaten Demak	Jawa Tengah	Indonesia	6UV9	2020-04-20 17:12:28.760031
20	Kabupaten Indragiri Hulu	Riau	Indonesia	7Q5U	2020-04-20 17:12:28.760031
21	Kabupaten Malang	Jawa Timur	Indonesia	VGOF	2020-04-20 17:12:28.760031
22	Kabupaten Sabu Raijua	Nusa Tenggara Timur	Indonesia	4DXX	2020-04-20 17:12:28.760031
23	Kabupaten Belu	Nusa Tenggara Timur	Indonesia	Y3R3	2020-04-20 17:12:28.760031
24	Kabupaten Pacitan	Jawa Timur	Indonesia	RSFL	2020-04-20 17:12:28.760031
25	Kota Metro	Lampung	Indonesia	E2NW	2020-04-20 17:12:28.760031
26	Kota Prabumulih	Sumatera Selatan	Indonesia	M7LS	2020-04-20 17:12:28.760031
27	Kota Parepare	Sulawesi Selatan	Indonesia	2IB9	2020-04-20 17:12:28.760031
28	Kabupaten Hulu Sungai Selatan	Kalimantan Selatan	Indonesia	ZG44	2020-04-20 17:12:28.760031
29	Kabupaten Tolikara	Papua	Indonesia	VSJ1	2020-04-20 17:12:28.760031
30	Kabupaten Pidie	Aceh	Indonesia	FHYE	2020-04-20 17:12:28.760031
31	Kota Palu	Sulawesi Tengah	Indonesia	KQHC	2020-04-20 17:12:28.760031
32	Kota Kendari	Sulawesi Tenggara	Indonesia	JWXX	2020-04-20 17:12:28.760031
33	Kabupaten Rembang	Jawa Tengah	Indonesia	ZUUV	2020-04-20 17:12:28.760031
34	Kota Jakarta Barat	Dki Jakarta	Indonesia	2GO4	2020-04-20 17:12:28.760031
35	Kabupaten Sekadau	Kalimantan Barat	Indonesia	XZCX	2020-04-20 17:12:28.760031
36	Kabupaten Belitung	Kepulauan Bangka Belitung	Indonesia	FGCC	2020-04-20 17:12:28.760031
37	Kabupaten Trenggalek	Jawa Timur	Indonesia	IMDO	2020-04-20 17:12:28.760031
38	Kabupaten Maluku Tengah	Maluku	Indonesia	DCCW	2020-04-20 17:12:28.760031
39	Kabupaten Karo	Sumatera Utara	Indonesia	TT9M	2020-04-20 17:12:28.760031
40	Kabupaten Teluk Bintuni	Papua Barat	Indonesia	QHKZ	2020-04-20 17:12:28.760031
41	Kabupaten Bone	Sulawesi Selatan	Indonesia	BFVD	2020-04-20 17:12:28.760031
42	Kabupaten Kepulauan Meranti	Riau	Indonesia	VKGT	2020-04-20 17:12:28.760031
43	Kabupaten Mamuju Utara	Sulawesi Barat	Indonesia	JT1Z	2020-04-20 17:12:28.760031
44	Kabupaten Soppeng	Sulawesi Selatan	Indonesia	ADBS	2020-04-20 17:12:28.760031
45	Kabupaten Tojo Una-Una	Sulawesi Tengah	Indonesia	ZFHD	2020-04-20 17:12:28.760031
46	Kabupaten Kutai Barat	Kalimantan Timur	Indonesia	QSAK	2020-04-20 17:12:28.760031
47	Kabupaten Lamandau	Kalimantan Tengah	Indonesia	MJ7E	2020-04-20 17:12:28.760031
48	Kabupaten Tapanuli Selatan	Sumatera Utara	Indonesia	RSE3	2020-04-20 17:12:28.760031
49	Kabupaten Pohuwato	Gorontalo	Indonesia	7AGC	2020-04-20 17:12:28.760031
50	Kabupaten Magetan	Jawa Timur	Indonesia	TO6N	2020-04-20 17:12:28.760031
51	Kabupaten Malaka	Nusa Tenggara Timur	Indonesia	I8MS	2020-04-20 17:12:28.760031
52	Kabupaten Morowali Utara	Sulawesi Tengah	Indonesia	LPKL	2020-04-20 17:12:28.760031
53	Kabupaten Aceh Singkil	Aceh	Indonesia	EBOV	2020-04-20 17:12:28.760031
54	Kabupaten Mempawah	Kalimantan Barat	Indonesia	UYPI	2020-04-20 17:12:28.760031
55	Kabupaten Penajam Paser Utara	Kalimantan Timur	Indonesia	JXWB	2020-04-20 17:12:28.760031
56	Kabupaten Mandailing Natal	Sumatera Utara	Indonesia	PJEX	2020-04-20 17:12:28.760031
57	Kabupaten Bandung	Jawa Barat	Indonesia	TUAN	2020-04-20 17:12:28.760031
58	Kabupaten Nunukan	Kalimantan Utara	Indonesia	JGBB	2020-04-20 17:12:28.760031
59	Kota Balikpapan	Kalimantan Timur	Indonesia	OX3A	2020-04-20 17:12:28.760031
60	Kota Palopo	Sulawesi Selatan	Indonesia	NOM1	2020-04-20 17:12:28.760031
61	Kota Jakarta Utara	Dki Jakarta	Indonesia	ZKXU	2020-04-20 17:12:28.760031
62	Kabupaten Mamuju	Sulawesi Barat	Indonesia	JNMT	2020-04-20 17:12:28.760031
63	Kota Sibolga	Sumatera Utara	Indonesia	LIEB	2020-04-20 17:12:28.760031
64	Kabupaten Bolaang Mongondow Timur	Sulawesi Utara	Indonesia	3S9W	2020-04-20 17:12:28.760031
65	Kabupaten Banjar	Kalimantan Selatan	Indonesia	NJK7	2020-04-20 17:12:28.760031
66	Kota Depok	Jawa Barat	Indonesia	QVIG	2020-04-20 17:12:28.760031
67	Kabupaten Agam	Sumatera Barat	Indonesia	TLQQ	2020-04-20 17:12:28.760031
68	Kabupaten Sukabumi	Jawa Barat	Indonesia	ADRA	2020-04-20 17:12:28.760031
69	Kabupaten Tanjung Jabung Timur	Jambi	Indonesia	XPEH	2020-04-20 17:12:28.760031
70	Kabupaten Landak	Kalimantan Barat	Indonesia	MRB8	2020-04-20 17:12:28.760031
71	Kabupaten Minahasa	Sulawesi Utara	Indonesia	KPJM	2020-04-20 17:12:28.760031
72	Kabupaten Ngawi	Jawa Timur	Indonesia	ISJW	2020-04-20 17:12:28.760031
73	Kabupaten Wonogiri	Jawa Tengah	Indonesia	C4DC	2020-04-20 17:12:28.760031
74	Kabupaten Halmahera Barat	Maluku Utara	Indonesia	ZMS3	2020-04-20 17:12:28.760031
75	Kabupaten Bandung Barat	Jawa Barat	Indonesia	8ISI	2020-04-20 17:12:28.760031
76	Kota Bogor	Jawa Barat	Indonesia	VKST	2020-04-20 17:12:28.760031
77	Kabupaten Sikka	Nusa Tenggara Timur	Indonesia	A7BV	2020-04-20 17:12:28.760031
78	Kabupaten Poso	Sulawesi Tengah	Indonesia	ZM4J	2020-04-20 17:12:28.760031
79	Kota Bekasi	Jawa Barat	Indonesia	BN7K	2020-04-20 17:12:28.760031
80	Kabupaten Seram Bagian Timur	Maluku	Indonesia	OQQ1	2020-04-20 17:12:28.760031
81	Kabupaten Banggai Laut	Sulawesi Tengah	Indonesia	UTDU	2020-04-20 17:12:28.760031
82	Kabupaten Simeulue	Aceh	Indonesia	PWWX	2020-04-20 17:12:28.760031
83	Kabupaten Lombok Timur	Nusa Tenggara Barat	Indonesia	FPGB	2020-04-20 17:12:28.760031
84	Kabupaten Lahat	Sumatera Selatan	Indonesia	AI5K	2020-04-20 17:12:28.760031
85	Kabupaten Bone Bolango	Gorontalo	Indonesia	QFGP	2020-04-20 17:12:28.760031
86	Kabupaten Tabanan	Bali	Indonesia	SKJ4	2020-04-20 17:12:28.760031
87	Kabupaten Nias Selatan	Sumatera Utara	Indonesia	HPOW	2020-04-20 17:12:28.760031
88	Kabupaten Bengkayang	Kalimantan Barat	Indonesia	GFXB	2020-04-20 17:12:28.760031
89	Kabupaten Karawang	Jawa Barat	Indonesia	IB5Y	2020-04-20 17:12:28.760031
90	Kabupaten Wakatobi	Sulawesi Tenggara	Indonesia	GCWV	2020-04-20 17:12:28.760031
91	Kabupaten Donggala	Sulawesi Tengah	Indonesia	2M7C	2020-04-20 17:12:28.760031
92	Kabupaten Klungkung	Bali	Indonesia	VCWM	2020-04-20 17:12:28.760031
93	Kota Gunungsitoli	Sumatera Utara	Indonesia	RMLK	2020-04-20 17:12:28.760031
94	Kabupaten Buton Selatan	Sulawesi Tenggara	Indonesia	7VOO	2020-04-20 17:12:28.760031
95	Kota Cilegon	Banten	Indonesia	LML2	2020-04-20 17:12:28.760031
96	Kabupaten Tanggamus	Lampung	Indonesia	1SDJ	2020-04-20 17:12:28.760031
97	Kabupaten Pesisir Barat	Lampung	Indonesia	3IIK	2020-04-20 17:12:28.760031
98	Kota Surabaya	Jawa Timur	Indonesia	LOGN	2020-04-20 17:12:28.760031
99	Kabupaten Lembata	Nusa Tenggara Timur	Indonesia	BNZ7	2020-04-20 17:12:28.760031
100	Kabupaten Jeneponto	Sulawesi Selatan	Indonesia	Y6TQ	2020-04-20 17:12:28.760031
101	Kabupaten Muara Enim	Sumatera Selatan	Indonesia	SFB1	2020-04-20 17:12:28.760031
102	Kabupaten Tana Toraja	Sulawesi Selatan	Indonesia	KYO6	2020-04-20 17:12:28.760031
103	Kabupaten Halmahera Utara	Maluku Utara	Indonesia	LA8N	2020-04-20 17:12:28.760031
104	Kota Sorong	Papua Barat	Indonesia	3K76	2020-04-20 17:12:28.760031
105	Kabupaten Humbang Hasundutan	Sumatera Utara	Indonesia	TPQO	2020-04-20 17:12:28.760031
106	Kabupaten Tapanuli Utara	Sumatera Utara	Indonesia	EWCP	2020-04-20 17:12:28.760031
107	Kota Surakarta	Jawa Tengah	Indonesia	KLVJ	2020-04-20 17:12:28.760031
108	Kabupaten Aceh Tenggara	Aceh	Indonesia	RPAK	2020-04-20 17:12:28.760031
109	Kabupaten Nagekeo	Nusa Tenggara Timur	Indonesia	ELLO	2020-04-20 17:12:28.760031
110	Kabupaten Toli-Toli	Sulawesi Tengah	Indonesia	TAUF	2020-04-20 17:12:28.760031
111	Kabupaten Kutai Timur	Kalimantan Timur	Indonesia	KBCN	2020-04-20 17:12:28.760031
112	Kabupaten Dairi	Sumatera Utara	Indonesia	NJTQ	2020-04-20 17:12:28.760031
113	Kabupaten Puncak Jaya	Papua	Indonesia	9KFN	2020-04-20 17:12:28.760031
114	Kabupaten Nabire	Papua	Indonesia	HRCB	2020-04-20 17:12:28.760031
115	Kabupaten Puncak	Papua	Indonesia	C9UU	2020-04-20 17:12:28.760031
116	Kabupaten Pemalang	Jawa Tengah	Indonesia	X5FC	2020-04-20 17:12:28.760031
117	Kabupaten Serdang Bedagai	Sumatera Utara	Indonesia	Q1AK	2020-04-20 17:12:28.760031
118	Kabupaten Tegal	Jawa Tengah	Indonesia	B5ZM	2020-04-20 17:12:28.760031
119	Kabupaten Morowali	Sulawesi Tengah	Indonesia	GCA4	2020-04-20 17:12:28.760031
120	Kabupaten Merauke	Papua	Indonesia	V4U5	2020-04-20 17:12:28.760031
121	Kabupaten Aceh Utara	Aceh	Indonesia	OAS6	2020-04-20 17:12:28.760031
122	Kabupaten Kepulauan Sangihe	Sulawesi Utara	Indonesia	REH5	2020-04-20 17:12:28.760031
123	Kota Bukittinggi	Sumatera Barat	Indonesia	NCZL	2020-04-20 17:12:28.760031
124	Kabupaten Ogan Komering Ilir	Sumatera Selatan	Indonesia	HOX9	2020-04-20 17:12:28.760031
125	Kabupaten Kupang	Nusa Tenggara Timur	Indonesia	P8TQ	2020-04-20 17:12:28.760031
126	Kota Tomohon	Sulawesi Utara	Indonesia	D3MT	2020-04-20 17:12:28.760031
127	Kota Madiun	Jawa Timur	Indonesia	GWXC	2020-04-20 17:12:28.760031
128	Kabupaten Bangka Selatan	Kepulauan Bangka Belitung	Indonesia	1RGP	2020-04-20 17:12:28.760031
129	Kabupaten Cirebon	Jawa Barat	Indonesia	SIVT	2020-04-20 17:12:28.760031
130	Kabupaten Aceh Tengah	Aceh	Indonesia	NDYB	2020-04-20 17:12:28.760031
131	Kota Ambon	Maluku	Indonesia	GYWX	2020-04-20 17:12:28.760031
132	Kabupaten Pelalawan	Riau	Indonesia	N37D	2020-04-20 17:12:28.760031
133	Kabupaten Grobogan	Jawa Tengah	Indonesia	BADO	2020-04-20 17:12:28.760031
134	Kabupaten Sintang	Kalimantan Barat	Indonesia	DQIT	2020-04-20 17:12:28.760031
135	Kabupaten Kaur	Bengkulu	Indonesia	OG5P	2020-04-20 17:12:28.760031
136	Kota Bengkulu	Bengkulu	Indonesia	8MGA	2020-04-20 17:12:28.760031
137	Kabupaten Timor Tengah Selatan	Nusa Tenggara Timur	Indonesia	5CUS	2020-04-20 17:12:28.760031
138	Kabupaten Seruyan	Kalimantan Tengah	Indonesia	PTC6	2020-04-20 17:12:28.760031
139	Kabupaten Kolaka Utara	Sulawesi Tenggara	Indonesia	295P	2020-04-20 17:12:28.760031
140	Kabupaten Keerom	Papua	Indonesia	CCCN	2020-04-20 17:12:28.760031
141	Kabupaten Bulungan	Kalimantan Utara	Indonesia	LGLP	2020-04-20 17:12:28.760031
142	Kota Solok	Sumatera Barat	Indonesia	G4J5	2020-04-20 17:12:28.760031
143	Kabupaten Karang Asem	Bali	Indonesia	KOVS	2020-04-20 17:12:28.760031
144	Kabupaten Gunung Mas	Kalimantan Tengah	Indonesia	KK3O	2020-04-20 17:12:28.760031
145	Kota Sukabumi	Jawa Barat	Indonesia	WXHM	2020-04-20 17:12:28.760031
146	Kabupaten Luwu	Sulawesi Selatan	Indonesia	1TT3	2020-04-20 17:12:28.760031
147	Kota Mataram	Nusa Tenggara Barat	Indonesia	2XSE	2020-04-20 17:12:28.760031
148	Kabupaten Padang Lawas Utara	Sumatera Utara	Indonesia	A4CV	2020-04-20 17:12:28.760031
149	Kabupaten Sumba Barat	Nusa Tenggara Timur	Indonesia	KOL2	2020-04-20 17:12:28.760031
150	Kabupaten Bengkalis	Riau	Indonesia	SE7M	2020-04-20 17:12:28.760031
151	Kota Kotamobagu	Sulawesi Utara	Indonesia	3BFN	2020-04-20 17:12:28.760031
152	Kota Samarinda	Kalimantan Timur	Indonesia	NICK	2020-04-20 17:12:28.760031
153	Kabupaten Jayapura	Papua	Indonesia	OT7P	2020-04-20 17:12:28.760031
154	Kabupaten Pringsewu	Lampung	Indonesia	M1RP	2020-04-20 17:12:28.760031
155	Kabupaten Mamberamo Raya	Papua	Indonesia	YK49	2020-04-20 17:12:28.760031
156	Kabupaten Kapuas	Kalimantan Tengah	Indonesia	OG5J	2020-04-20 17:12:28.760031
157	Kabupaten Tambrauw	Papua Barat	Indonesia	5RKX	2020-04-20 17:12:28.760031
158	Kabupaten Minahasa Selatan	Sulawesi Utara	Indonesia	6RK9	2020-04-20 17:12:28.760031
159	Kabupaten Tebo	Jambi	Indonesia	TZWQ	2020-04-20 17:12:28.760031
160	Kabupaten Tangerang	Banten	Indonesia	RZBG	2020-04-20 17:12:28.760031
161	Kabupaten Kulon Progo	Di Yogyakarta	Indonesia	TIWG	2020-04-20 17:12:28.760031
162	Kota Bontang	Kalimantan Timur	Indonesia	JO6I	2020-04-20 17:12:28.760031
163	Kabupaten Buru	Maluku	Indonesia	IBR8	2020-04-20 17:12:28.760031
164	Kabupaten Bintan	Kepulauan Riau	Indonesia	RWQW	2020-04-20 17:12:28.760031
165	Kabupaten Kepulauan Aru	Maluku	Indonesia	XBU4	2020-04-20 17:12:28.760031
166	Kabupaten Yalimo	Papua	Indonesia	TODX	2020-04-20 17:12:28.760031
167	Kabupaten Mahakam Hulu	Kalimantan Timur	Indonesia	OBOG	2020-04-20 17:12:28.760031
168	Kabupaten Barito Kuala	Kalimantan Selatan	Indonesia	KZWE	2020-04-20 17:12:28.760031
169	Kabupaten Labuhan Batu Selatan	Sumatera Utara	Indonesia	ITUB	2020-04-20 17:12:28.760031
170	Kabupaten Musi Banyuasin	Sumatera Selatan	Indonesia	RBUA	2020-04-20 17:12:28.760031
171	Kota Salatiga	Jawa Tengah	Indonesia	LMID	2020-04-20 17:12:28.760031
172	Kota Sabang	Aceh	Indonesia	TZAR	2020-04-20 17:12:28.760031
173	Kabupaten Blitar	Jawa Timur	Indonesia	C5VF	2020-04-20 17:12:28.760031
174	Kabupaten Mesuji	Lampung	Indonesia	S9DH	2020-04-20 17:12:28.760031
175	Kabupaten Ogan Komering Ulu Selatan	Sumatera Selatan	Indonesia	KRX4	2020-04-20 17:12:28.760031
176	Kota Banjar	Jawa Barat	Indonesia	RUIJ	2020-04-20 17:12:28.760031
177	Kabupaten Kebumen	Jawa Tengah	Indonesia	XNMP	2020-04-20 17:12:28.760031
178	Kabupaten Kudus	Jawa Tengah	Indonesia	OGQB	2020-04-20 17:12:28.760031
179	Kabupaten Muna	Sulawesi Tenggara	Indonesia	C9OW	2020-04-20 17:12:28.760031
180	Kabupaten Pesawaran	Lampung	Indonesia	JPNV	2020-04-20 17:12:28.760031
181	Kabupaten Kutai Kartanegara	Kalimantan Timur	Indonesia	TJBN	2020-04-20 17:12:28.760031
182	Kabupaten Kepahiang	Bengkulu	Indonesia	SEEM	2020-04-20 17:12:28.760031
183	Kabupaten Halmahera Selatan	Maluku Utara	Indonesia	GCQ8	2020-04-20 17:12:28.760031
184	Kota Pekalongan	Jawa Tengah	Indonesia	H9RE	2020-04-20 17:12:28.760031
185	Kota Malang	Jawa Timur	Indonesia	WEUL	2020-04-20 17:12:28.760031
186	Kabupaten Pamekasan	Jawa Timur	Indonesia	UVW8	2020-04-20 17:12:28.760031
187	Kabupaten Bombana	Sulawesi Tenggara	Indonesia	5LEO	2020-04-20 17:12:28.760031
188	Kabupaten Halmahera Tengah	Maluku Utara	Indonesia	J2JE	2020-04-20 17:12:28.760031
189	Kota Palangka Raya	Kalimantan Tengah	Indonesia	LLAN	2020-04-20 17:12:28.760031
190	Kabupaten Katingan	Kalimantan Tengah	Indonesia	ZGAG	2020-04-20 17:12:28.760031
191	Kabupaten Solok Selatan	Sumatera Barat	Indonesia	SROA	2020-04-20 17:12:28.760031
192	Kota Probolinggo	Jawa Timur	Indonesia	AGOX	2020-04-20 17:12:28.760031
193	Kabupaten Sumba Barat Daya	Nusa Tenggara Timur	Indonesia	UJSZ	2020-04-20 17:12:28.760031
194	Kabupaten Flores Timur	Nusa Tenggara Timur	Indonesia	FP7K	2020-04-20 17:12:28.760031
195	Kabupaten Tanah Bumbu	Kalimantan Selatan	Indonesia	JL9T	2020-04-20 17:12:28.760031
196	Kabupaten Ngada	Nusa Tenggara Timur	Indonesia	NSH9	2020-04-20 17:12:28.760031
197	Kota Baubau	Sulawesi Tenggara	Indonesia	EIWO	2020-04-20 17:12:28.760031
198	Kabupaten Sumedang	Jawa Barat	Indonesia	YX51	2020-04-20 17:12:28.760031
199	Kabupaten Jepara	Jawa Tengah	Indonesia	YTBZ	2020-04-20 17:12:28.760031
200	Kota Pariaman	Sumatera Barat	Indonesia	JY7F	2020-04-20 17:12:28.760031
201	Kabupaten Batang Hari	Jambi	Indonesia	JYEX	2020-04-20 17:12:28.760031
202	Kabupaten Boalemo	Gorontalo	Indonesia	OMR8	2020-04-20 17:12:28.760031
203	Kabupaten Kotawaringin Barat	Kalimantan Tengah	Indonesia	71CU	2020-04-20 17:12:28.760031
204	Kabupaten Sigi	Sulawesi Tengah	Indonesia	TK48	2020-04-20 17:12:28.760031
205	Kabupaten Mamasa	Sulawesi Barat	Indonesia	C1WB	2020-04-20 17:12:28.760031
206	Kabupaten Seluma	Bengkulu	Indonesia	Y1CX	2020-04-20 17:12:28.760031
207	Kabupaten Timor Tengah Utara	Nusa Tenggara Timur	Indonesia	UM7E	2020-04-20 17:12:28.760031
208	Kota Padang Panjang	Sumatera Barat	Indonesia	MDTE	2020-04-20 17:12:28.760031
209	Kabupaten Madiun	Jawa Timur	Indonesia	CHDB	2020-04-20 17:12:28.760031
210	Kabupaten Buol	Sulawesi Tengah	Indonesia	45JB	2020-04-20 17:12:28.760031
211	Kabupaten Batang	Jawa Tengah	Indonesia	5L6Y	2020-04-20 17:12:28.760031
212	Kabupaten Indragiri Hilir	Riau	Indonesia	FB7I	2020-04-20 17:12:28.760031
213	Kabupaten Garut	Jawa Barat	Indonesia	C4TK	2020-04-20 17:12:28.760031
214	Kabupaten Rokan Hilir	Riau	Indonesia	5FIA	2020-04-20 17:12:28.760031
215	Kabupaten Lombok Utara	Nusa Tenggara Barat	Indonesia	2OOO	2020-04-20 17:12:28.760031
216	Kabupaten Manokwari Selatan	Papua Barat	Indonesia	CI4O	2020-04-20 17:12:28.760031
217	Kota Pekanbaru	Riau	Indonesia	1GP5	2020-04-20 17:12:28.760031
218	Kabupaten Padang Lawas	Sumatera Utara	Indonesia	L9GQ	2020-04-20 17:12:28.760031
219	Kabupaten Kubu Raya	Kalimantan Barat	Indonesia	VMQB	2020-04-20 17:12:28.760031
220	Kabupaten Majalengka	Jawa Barat	Indonesia	XXTJ	2020-04-20 17:12:28.760031
221	Kabupaten Maros	Sulawesi Selatan	Indonesia	BWUH	2020-04-20 17:12:28.760031
222	Kabupaten Manggarai	Nusa Tenggara Timur	Indonesia	DMHF	2020-04-20 17:12:28.760031
223	Kabupaten Tuban	Jawa Timur	Indonesia	B64N	2020-04-20 17:12:28.760031
224	Kabupaten Sidoarjo	Jawa Timur	Indonesia	P8CP	2020-04-20 17:12:28.760031
225	Kabupaten Sambas	Kalimantan Barat	Indonesia	O2UJ	2020-04-20 17:12:28.760031
226	Kabupaten Musi Rawas Utara	Sumatera Selatan	Indonesia	BKBG	2020-04-20 17:12:28.760031
227	Kota Tebing Tinggi	Sumatera Utara	Indonesia	7BRE	2020-04-20 17:12:28.760031
228	Kabupaten Subang	Jawa Barat	Indonesia	9LNK	2020-04-20 17:12:28.760031
229	Kabupaten Hulu Sungai Tengah	Kalimantan Selatan	Indonesia	JRSV	2020-04-20 17:12:28.760031
230	Kota Pasuruan	Jawa Timur	Indonesia	EJBP	2020-04-20 17:12:28.760031
231	Kota Tasikmalaya	Jawa Barat	Indonesia	QGCF	2020-04-20 17:12:28.760031
232	Kabupaten Gunung Kidul	Di Yogyakarta	Indonesia	OFFD	2020-04-20 17:12:28.760031
233	Kabupaten Asmat	Papua	Indonesia	H9XS	2020-04-20 17:12:28.760031
234	Kabupaten Manokwari	Papua Barat	Indonesia	T891	2020-04-20 17:12:28.760031
235	Kabupaten Enrekang	Sulawesi Selatan	Indonesia	J1ES	2020-04-20 17:12:28.760031
236	Kabupaten Mimika	Papua	Indonesia	N3DF	2020-04-20 17:12:28.760031
237	Kota Batu	Jawa Timur	Indonesia	U6BZ	2020-04-20 17:12:28.760031
238	Kabupaten Kolaka Timur	Sulawesi Tenggara	Indonesia	PEOG	2020-04-20 17:12:28.760031
239	Kabupaten Barito Utara	Kalimantan Tengah	Indonesia	U2VI	2020-04-20 17:12:28.760031
240	Kabupaten Gowa	Sulawesi Selatan	Indonesia	HBVY	2020-04-20 17:12:28.760031
241	Kabupaten Tulang Bawang Barat	Lampung	Indonesia	KT1E	2020-04-20 17:12:28.760031
242	Kabupaten Ende	Nusa Tenggara Timur	Indonesia	2BFL	2020-04-20 17:12:28.760031
243	Kabupaten Kediri	Jawa Timur	Indonesia	CKEZ	2020-04-20 17:12:28.760031
244	Kabupaten Situbondo	Jawa Timur	Indonesia	NIFI	2020-04-20 17:12:28.760031
245	Kabupaten Kepulauan Seribu	Dki Jakarta	Indonesia	OQRE	2020-04-20 17:12:28.760031
246	Kabupaten Sinjai	Sulawesi Selatan	Indonesia	5GUZ	2020-04-20 17:12:28.760031
247	Kabupaten Maluku Tenggara	Maluku	Indonesia	IQIZ	2020-04-20 17:12:28.760031
248	Kabupaten Paniai	Papua	Indonesia	SE7D	2020-04-20 17:12:28.760031
249	Kabupaten Ketapang	Kalimantan Barat	Indonesia	Y9RB	2020-04-20 17:12:28.760031
250	Kabupaten Purwakarta	Jawa Barat	Indonesia	JXNW	2020-04-20 17:12:28.760031
251	Kabupaten Kuningan	Jawa Barat	Indonesia	S25G	2020-04-20 17:12:28.760031
252	Kabupaten Bojonegoro	Jawa Timur	Indonesia	KKYJ	2020-04-20 17:12:28.760031
253	Kabupaten Muaro Jambi	Jambi	Indonesia	BQXH	2020-04-20 17:12:28.760031
254	Kabupaten Belitung Timur	Kepulauan Bangka Belitung	Indonesia	GSGX	2020-04-20 17:12:28.760031
255	Kabupaten Ogan Komering Ulu	Sumatera Selatan	Indonesia	JYXC	2020-04-20 17:12:28.760031
256	Kabupaten Badung	Bali	Indonesia	EFPD	2020-04-20 17:12:28.760031
257	Kota B A T A M	Kepulauan Riau	Indonesia	NHO8	2020-04-20 17:12:28.760031
258	Kabupaten Bolaang Mongondow	Sulawesi Utara	Indonesia	OCDG	2020-04-20 17:12:28.760031
259	Kabupaten Bengkulu Utara	Bengkulu	Indonesia	EIWZ	2020-04-20 17:12:28.760031
260	Kabupaten Pinrang	Sulawesi Selatan	Indonesia	3VIF	2020-04-20 17:12:28.760031
261	Kota Blitar	Jawa Timur	Indonesia	VGWB	2020-04-20 17:12:28.760031
262	Kabupaten Kerinci	Jambi	Indonesia	9MAS	2020-04-20 17:12:28.760031
263	Kabupaten Tulungagung	Jawa Timur	Indonesia	L84Q	2020-04-20 17:12:28.760031
264	Kabupaten Pakpak Bharat	Sumatera Utara	Indonesia	MTEA	2020-04-20 17:12:28.760031
265	Kabupaten Sarmi	Papua	Indonesia	KTIZ	2020-04-20 17:12:28.760031
266	Kabupaten Sleman	Di Yogyakarta	Indonesia	FMGT	2020-04-20 17:12:28.760031
267	Kota Jakarta Timur	Dki Jakarta	Indonesia	6DT9	2020-04-20 17:12:28.760031
268	Kabupaten Pangkajene Dan Kepulauan	Sulawesi Selatan	Indonesia	ILOE	2020-04-20 17:12:28.760031
269	Kabupaten Nganjuk	Jawa Timur	Indonesia	2UGB	2020-04-20 17:12:28.760031
270	Kabupaten Sijunjung	Sumatera Barat	Indonesia	HQ3T	2020-04-20 17:12:28.760031
271	Kabupaten Samosir	Sumatera Utara	Indonesia	Y8UL	2020-04-20 17:12:28.760031
272	Kabupaten Buru Selatan	Maluku	Indonesia	BYNM	2020-04-20 17:12:28.760031
273	Kabupaten Bima	Nusa Tenggara Barat	Indonesia	26UH	2020-04-20 17:12:28.760031
274	Kabupaten Lima Puluh Kota	Sumatera Barat	Indonesia	SBBY	2020-04-20 17:12:28.760031
275	Kota Cirebon	Jawa Barat	Indonesia	P5H8	2020-04-20 17:12:28.760031
276	Kota Semarang	Jawa Tengah	Indonesia	Q6MS	2020-04-20 17:12:28.760031
277	Kabupaten Sukoharjo	Jawa Tengah	Indonesia	044H	2020-04-20 17:12:28.760031
278	Kabupaten Barru	Sulawesi Selatan	Indonesia	U7BT	2020-04-20 17:12:28.760031
279	Kabupaten Intan Jaya	Papua	Indonesia	F5FQ	2020-04-20 17:12:28.760031
280	Kabupaten Rejang Lebong	Bengkulu	Indonesia	E2DG	2020-04-20 17:12:28.760031
281	Kota Subulussalam	Aceh	Indonesia	88OA	2020-04-20 17:12:28.760031
282	Kabupaten Lumajang	Jawa Timur	Indonesia	JY8I	2020-04-20 17:12:28.760031
283	Kabupaten Nias	Sumatera Utara	Indonesia	4PQU	2020-04-20 17:12:28.760031
284	Kabupaten Jombang	Jawa Timur	Indonesia	VENV	2020-04-20 17:12:28.760031
285	Kabupaten Kapuas Hulu	Kalimantan Barat	Indonesia	HQDC	2020-04-20 17:12:28.760031
286	Kabupaten Aceh Selatan	Aceh	Indonesia	XOWD	2020-04-20 17:12:28.760031
287	Kabupaten Majene	Sulawesi Barat	Indonesia	TB4Y	2020-04-20 17:12:28.760031
288	Kota Tarakan	Kalimantan Utara	Indonesia	DHEL	2020-04-20 17:12:28.760031
289	Kabupaten Kepulauan Talaud	Sulawesi Utara	Indonesia	PCVI	2020-04-20 17:12:28.760031
290	Kabupaten Nias Utara	Sumatera Utara	Indonesia	B4RF	2020-04-20 17:12:28.760031
291	Kota Bandar Lampung	Lampung	Indonesia	TJAP	2020-04-20 17:12:28.760031
292	Kota Cimahi	Jawa Barat	Indonesia	WOLE	2020-04-20 17:12:28.760031
293	Kabupaten Serang	Banten	Indonesia	FOHE	2020-04-20 17:12:28.760031
294	Kabupaten Lebak	Banten	Indonesia	DNR7	2020-04-20 17:12:28.760031
295	Kota Padang	Sumatera Barat	Indonesia	YVFC	2020-04-20 17:12:28.760031
296	Kabupaten Konawe Utara	Sulawesi Tenggara	Indonesia	MTXC	2020-04-20 17:12:28.760031
297	Kabupaten Kotawaringin Timur	Kalimantan Tengah	Indonesia	WTU8	2020-04-20 17:12:28.760031
298	Kabupaten Langkat	Sumatera Utara	Indonesia	XMNR	2020-04-20 17:12:28.760031
299	Kabupaten Bekasi	Jawa Barat	Indonesia	FPGC	2020-04-20 17:12:28.760031
300	Kabupaten Minahasa Utara	Sulawesi Utara	Indonesia	N2H3	2020-04-20 17:12:28.760031
301	Kabupaten Dogiyai	Papua	Indonesia	QZGD	2020-04-20 17:12:28.760031
302	Kabupaten Gorontalo Utara	Gorontalo	Indonesia	NHKM	2020-04-20 17:12:28.760031
303	Kabupaten Lampung Timur	Lampung	Indonesia	DPY0	2020-04-20 17:12:28.760031
304	Kabupaten Klaten	Jawa Tengah	Indonesia	J6CP	2020-04-20 17:12:28.760031
305	Kabupaten Luwu Utara	Sulawesi Selatan	Indonesia	Z6X7	2020-04-20 17:12:28.760031
306	Kabupaten Penukal Abab Lematang Ilir	Sumatera Selatan	Indonesia	SMX8	2020-04-20 17:12:28.760031
307	Kota Payakumbuh	Sumatera Barat	Indonesia	BNKY	2020-04-20 17:12:28.760031
308	Kabupaten Batu Bara	Sumatera Utara	Indonesia	P22G	2020-04-20 17:12:28.760031
309	Kabupaten Jayawijaya	Papua	Indonesia	BITX	2020-04-20 17:12:28.760031
310	Kabupaten Pasaman	Sumatera Barat	Indonesia	1EKE	2020-04-20 17:12:28.760031
311	Kabupaten Natuna	Kepulauan Riau	Indonesia	5SEO	2020-04-20 17:12:28.760031
312	Kabupaten Hulu Sungai Utara	Kalimantan Selatan	Indonesia	YQEY	2020-04-20 17:12:28.760031
313	Kabupaten Sumba Timur	Nusa Tenggara Timur	Indonesia	WCFZ	2020-04-20 17:12:28.760031
314	Kota Banda Aceh	Aceh	Indonesia	XD8A	2020-04-20 17:12:28.760031
315	Kabupaten Alor	Nusa Tenggara Timur	Indonesia	1S9R	2020-04-20 17:12:28.760031
316	Kabupaten Sumba Tengah	Nusa Tenggara Timur	Indonesia	UB8V	2020-04-20 17:12:28.760031
317	Kabupaten Kepulauan Anambas	Kepulauan Riau	Indonesia	T2UU	2020-04-20 17:12:28.760031
318	Kabupaten Wajo	Sulawesi Selatan	Indonesia	HOIL	2020-04-20 17:12:28.760031
319	Kabupaten Buton Tengah	Sulawesi Tenggara	Indonesia	HWAG	2020-04-20 17:12:28.760031
320	Kabupaten Deiyai	Papua	Indonesia	NOFT	2020-04-20 17:12:28.760031
321	Kabupaten Kota Baru	Kalimantan Selatan	Indonesia	AUTY	2020-04-20 17:12:28.760031
322	Kabupaten Probolinggo	Jawa Timur	Indonesia	82I9	2020-04-20 17:12:28.760031
323	Kabupaten Waropen	Papua	Indonesia	TR0O	2020-04-20 17:12:28.760031
324	Kabupaten Manggarai Timur	Nusa Tenggara Timur	Indonesia	28KV	2020-04-20 17:12:28.760031
325	Kabupaten Pegunungan Arfak	Papua Barat	Indonesia	BO0R	2020-04-20 17:12:28.760031
326	Kabupaten Pasuruan	Jawa Timur	Indonesia	DIDL	2020-04-20 17:12:28.760031
327	Kabupaten Bantul	Di Yogyakarta	Indonesia	FN32	2020-04-20 17:12:28.760031
328	Kabupaten Pesisir Selatan	Sumatera Barat	Indonesia	CIWM	2020-04-20 17:12:28.760031
329	Kabupaten Pangandaran	Jawa Barat	Indonesia	DPLL	2020-04-20 17:12:28.760031
330	Kabupaten Solok	Sumatera Barat	Indonesia	R4TL	2020-04-20 17:12:28.760031
331	Kabupaten Karanganyar	Jawa Tengah	Indonesia	VUAX	2020-04-20 17:12:28.760031
332	Kabupaten Kepulauan Yapen	Papua	Indonesia	CUTN	2020-04-20 17:12:28.760031
333	Kabupaten Pulau Morotai	Maluku Utara	Indonesia	JUFN	2020-04-20 17:12:28.760031
334	Kota Makassar	Sulawesi Selatan	Indonesia	CJJR	2020-04-20 17:12:28.760031
335	Kabupaten Wonosobo	Jawa Tengah	Indonesia	8LUK	2020-04-20 17:12:28.760031
336	Kabupaten Tana Tidung	Kalimantan Utara	Indonesia	5Q6I	2020-04-20 17:12:28.760031
337	Kabupaten Sorong	Papua Barat	Indonesia	HRCZ	2020-04-20 17:12:28.760031
338	Kota Pontianak	Kalimantan Barat	Indonesia	V7LQ	2020-04-20 17:12:28.760031
339	Kabupaten Buton Utara	Sulawesi Tenggara	Indonesia	BVOE	2020-04-20 17:12:28.760031
340	Kabupaten Musi Rawas	Sumatera Selatan	Indonesia	QS2A	2020-04-20 17:12:28.760031
341	Kabupaten Tanah Datar	Sumatera Barat	Indonesia	NHXZ	2020-04-20 17:12:28.760031
342	Kabupaten Sampang	Jawa Timur	Indonesia	1GR9	2020-04-20 17:12:28.760031
343	Kota Yogyakarta	Di Yogyakarta	Indonesia	TMSX	2020-04-20 17:12:28.760031
344	Kota Kupang	Nusa Tenggara Timur	Indonesia	DZPU	2020-04-20 17:12:28.760031
345	Kabupaten Tulangbawang	Lampung	Indonesia	1STW	2020-04-20 17:12:28.760031
346	Kabupaten Bantaeng	Sulawesi Selatan	Indonesia	ZOWA	2020-04-20 17:12:28.760031
347	Kabupaten Jember	Jawa Timur	Indonesia	KMOB	2020-04-20 17:12:28.760031
348	Kota Pagar Alam	Sumatera Selatan	Indonesia	FQBB	2020-04-20 17:12:28.760031
349	Kabupaten Bireuen	Aceh	Indonesia	79B8	2020-04-20 17:12:28.760031
350	Kabupaten Sumbawa	Nusa Tenggara Barat	Indonesia	QCHJ	2020-04-20 17:12:28.760031
351	Kota Mojokerto	Jawa Timur	Indonesia	YJRC	2020-04-20 17:12:28.760031
352	Kabupaten Bangkalan	Jawa Timur	Indonesia	JH7K	2020-04-20 17:12:28.760031
353	Kabupaten Kepulauan Mentawai	Sumatera Barat	Indonesia	AAQJ	2020-04-20 17:12:28.760031
354	Kabupaten Karimun	Kepulauan Riau	Indonesia	PNTB	2020-04-20 17:12:28.760031
355	Kabupaten Sanggau	Kalimantan Barat	Indonesia	AILP	2020-04-20 17:12:28.760031
356	Kabupaten Buton	Sulawesi Tenggara	Indonesia	IX1P	2020-04-20 17:12:28.760031
357	Kabupaten Lampung Tengah	Lampung	Indonesia	7CXX	2020-04-20 17:12:28.760031
358	Kota Pangkal Pinang	Kepulauan Bangka Belitung	Indonesia	OFHN	2020-04-20 17:12:28.760031
359	Kabupaten Balangan	Kalimantan Selatan	Indonesia	Z90J	2020-04-20 17:12:28.760031
360	Kabupaten Banyu Asin	Sumatera Selatan	Indonesia	Q7D1	2020-04-20 17:12:28.760031
361	Kabupaten Pasaman Barat	Sumatera Barat	Indonesia	HUJ7	2020-04-20 17:12:28.760031
362	Kabupaten Tapanuli Tengah	Sumatera Utara	Indonesia	IDIS	2020-04-20 17:12:28.760031
363	Kabupaten Toba Samosir	Sumatera Utara	Indonesia	V4RE	2020-04-20 17:12:28.760031
364	Kabupaten Deli Serdang	Sumatera Utara	Indonesia	2RT9	2020-04-20 17:12:28.760031
365	Kabupaten Banyumas	Jawa Tengah	Indonesia	4S7T	2020-04-20 17:12:28.760031
366	Kabupaten Bangka Barat	Kepulauan Bangka Belitung	Indonesia	HOHG	2020-04-20 17:12:28.760031
367	Kabupaten Nduga	Papua	Indonesia	XHZO	2020-04-20 17:12:28.760031
368	Kabupaten Sorong Selatan	Papua Barat	Indonesia	ODOW	2020-04-20 17:12:28.760031
369	Kabupaten Bondowoso	Jawa Timur	Indonesia	78DP	2020-04-20 17:12:28.760031
370	Kabupaten Magelang	Jawa Tengah	Indonesia	MVRI	2020-04-20 17:12:28.760031
371	Kota Medan	Sumatera Utara	Indonesia	ZJM2	2020-04-20 17:12:28.760031
372	Kabupaten Minahasa Tenggara	Sulawesi Utara	Indonesia	CQBG	2020-04-20 17:12:28.760031
373	Kabupaten Kepulauan Selayar	Sulawesi Selatan	Indonesia	IHAZ	2020-04-20 17:12:28.760031
374	Kabupaten Lombok Tengah	Nusa Tenggara Barat	Indonesia	FIGD	2020-04-20 17:12:28.760031
375	Kota Tidore Kepulauan	Maluku Utara	Indonesia	ZGBO	2020-04-20 17:12:28.760031
376	Kabupaten Way Kanan	Lampung	Indonesia	J1KR	2020-04-20 17:12:28.760031
377	Kabupaten Sragen	Jawa Tengah	Indonesia	9XQU	2020-04-20 17:12:28.760031
378	Kabupaten Kampar	Riau	Indonesia	TJDT	2020-04-20 17:12:28.760031
379	Kota Denpasar	Bali	Indonesia	T0V5	2020-04-20 17:12:28.760031
380	Kabupaten Luwu Timur	Sulawesi Selatan	Indonesia	Q7L9	2020-04-20 17:12:28.760031
381	Kabupaten Tasikmalaya	Jawa Barat	Indonesia	OM94	2020-04-20 17:12:28.760031
382	Kabupaten Aceh Jaya	Aceh	Indonesia	EPID	2020-04-20 17:12:28.760031
383	Kabupaten Pulang Pisau	Kalimantan Tengah	Indonesia	VTSP	2020-04-20 17:12:28.760031
384	Kabupaten Kepulauan Sula	Maluku Utara	Indonesia	UMG4	2020-04-20 17:12:28.760031
385	Kabupaten Temanggung	Jawa Tengah	Indonesia	L8YF	2020-04-20 17:12:28.760031
386	Kota Tanjung Pinang	Kepulauan Riau	Indonesia	QCZK	2020-04-20 17:12:28.760031
387	Kabupaten Boyolali	Jawa Tengah	Indonesia	CVPT	2020-04-20 17:12:28.760031
388	Kabupaten Bangli	Bali	Indonesia	BBC0	2020-04-20 17:12:28.760031
389	Kabupaten Dharmasraya	Sumatera Barat	Indonesia	XL5C	2020-04-20 17:12:28.760031
390	Kota Jakarta Selatan	Dki Jakarta	Indonesia	BNP8	2020-04-20 17:12:28.760031
391	Kabupaten Barito Timur	Kalimantan Tengah	Indonesia	HIWB	2020-04-20 17:12:28.760031
392	Kabupaten Supiori	Papua	Indonesia	5EFQ	2020-04-20 17:12:28.760031
393	Kabupaten Aceh Barat Daya	Aceh	Indonesia	LEWD	2020-04-20 17:12:28.760031
394	Kabupaten Tapin	Kalimantan Selatan	Indonesia	Q5WT	2020-04-20 17:12:28.760031
395	Kota Sawah Lunto	Sumatera Barat	Indonesia	AWWC	2020-04-20 17:12:28.760031
396	Kabupaten Jembrana	Bali	Indonesia	8ACF	2020-04-20 17:12:28.760031
397	Kabupaten Maybrat	Papua Barat	Indonesia	MHRX	2020-04-20 17:12:28.760031
398	Kabupaten Banggai Kepulauan	Sulawesi Tengah	Indonesia	EH6U	2020-04-20 17:12:28.760031
399	Kota Magelang	Jawa Tengah	Indonesia	0366	2020-04-20 17:12:28.760031
400	Kabupaten Siau Tagulandang Biaro	Sulawesi Utara	Indonesia	HLVC	2020-04-20 17:12:28.760031
401	Kabupaten S I A K	Riau	Indonesia	YSPP	2020-04-20 17:12:28.760031
402	Kabupaten Pidie Jaya	Aceh	Indonesia	XMS8	2020-04-20 17:12:28.760031
403	Kabupaten Mojokerto	Jawa Timur	Indonesia	JQKR	2020-04-20 17:12:28.760031
404	Kabupaten Bogor	Jawa Barat	Indonesia	RX7N	2020-04-20 17:12:28.760031
405	Kabupaten Raja Ampat	Papua Barat	Indonesia	OYLS	2020-04-20 17:12:28.760031
406	Kota Gorontalo	Gorontalo	Indonesia	HRNH	2020-04-20 17:12:28.760031
407	Kabupaten Maluku Barat Daya	Maluku	Indonesia	UTNC	2020-04-20 17:12:28.760031
408	Kota Langsa	Aceh	Indonesia	OJOO	2020-04-20 17:12:28.760031
409	Kabupaten Lampung Utara	Lampung	Indonesia	CENJ	2020-04-20 17:12:28.760031
410	Kabupaten Halmahera Timur	Maluku Utara	Indonesia	1GRK	2020-04-20 17:12:28.760031
411	Kabupaten Nagan Raya	Aceh	Indonesia	XDLY	2020-04-20 17:12:28.760031
412	Kota Ternate	Maluku Utara	Indonesia	JSMY	2020-04-20 17:12:28.760031
413	Kabupaten Boven Digoel	Papua	Indonesia	RY1Z	2020-04-20 17:12:28.760031
414	Kabupaten Konawe Kepulauan	Sulawesi Tenggara	Indonesia	QOHL	2020-04-20 17:12:28.760031
415	Kabupaten Empat Lawang	Sumatera Selatan	Indonesia	R4XG	2020-04-20 17:12:28.760031
416	Kabupaten Kolaka	Sulawesi Tenggara	Indonesia	NN50	2020-04-20 17:12:28.760031
417	Kota Padangsidimpuan	Sumatera Utara	Indonesia	2SK3	2020-04-20 17:12:28.760031
418	Kabupaten Labuhan Batu	Sumatera Utara	Indonesia	ZCO7	2020-04-20 17:12:28.760031
419	Kabupaten Pandeglang	Banten	Indonesia	OA6Y	2020-04-20 17:12:28.760031
420	Kabupaten Purworejo	Jawa Tengah	Indonesia	3R7U	2020-04-20 17:12:28.760031
421	Kabupaten Gayo Lues	Aceh	Indonesia	Q8UQ	2020-04-20 17:12:28.760031
422	Kabupaten Konawe Selatan	Sulawesi Tenggara	Indonesia	WKCO	2020-04-20 17:12:28.760031
423	Kabupaten Pati	Jawa Tengah	Indonesia	PJ5D	2020-04-20 17:12:28.760031
424	Kabupaten Asahan	Sumatera Utara	Indonesia	7BD9	2020-04-20 17:12:28.760031
425	Kota Bima	Nusa Tenggara Barat	Indonesia	4ODD	2020-04-20 17:12:28.760031
426	Kabupaten Lebong	Bengkulu	Indonesia	1AKP	2020-04-20 17:12:28.760031
427	Kabupaten Sumbawa Barat	Nusa Tenggara Barat	Indonesia	KPPN	2020-04-20 17:12:28.760031
428	Kabupaten Fakfak	Papua Barat	Indonesia	HWH8	2020-04-20 17:12:28.760031
429	Kabupaten Sidenreng Rappang	Sulawesi Selatan	Indonesia	ECYA	2020-04-20 17:12:28.760031
430	Kabupaten Bungo	Jambi	Indonesia	XBYM	2020-04-20 17:12:28.760031
431	Kabupaten Aceh Timur	Aceh	Indonesia	VEQ3	2020-04-20 17:12:28.760031
432	Kota Bandung	Jawa Barat	Indonesia	P5CS	2020-04-20 17:12:28.760031
433	Kabupaten Ciamis	Jawa Barat	Indonesia	TPWU	2020-04-20 17:12:28.760031
434	Kabupaten Gresik	Jawa Timur	Indonesia	ZGKK	2020-04-20 17:12:28.760031
435	Kabupaten Pekalongan	Jawa Tengah	Indonesia	7JXO	2020-04-20 17:12:28.760031
436	Kabupaten Tabalong	Kalimantan Selatan	Indonesia	GGWT	2020-04-20 17:12:28.760031
437	Kabupaten Bangka	Kepulauan Bangka Belitung	Indonesia	SV5P	2020-04-20 17:12:28.760031
438	Kota Serang	Banten	Indonesia	7DD3	2020-04-20 17:12:28.760031
439	Kota Tual	Maluku	Indonesia	ID68	2020-04-20 17:12:28.760031
440	Kabupaten Kaimana	Papua Barat	Indonesia	II1C	2020-04-20 17:12:28.760031
441	Kabupaten Tanah Laut	Kalimantan Selatan	Indonesia	HWFG	2020-04-20 17:12:28.760031
442	Kabupaten Takalar	Sulawesi Selatan	Indonesia	N0RT	2020-04-20 17:12:28.760031
443	Kabupaten Ogan Komering Ulu Timur	Sumatera Selatan	Indonesia	JYI0	2020-04-20 17:12:28.760031
444	Kabupaten Lampung Barat	Lampung	Indonesia	FFTH	2020-04-20 17:12:28.760031
445	Kota D U M A I	Riau	Indonesia	CY7J	2020-04-20 17:12:28.760031
446	Kabupaten Banyuwangi	Jawa Timur	Indonesia	CKMU	2020-04-20 17:12:28.760031
447	Kabupaten Lingga	Kepulauan Riau	Indonesia	XS3F	2020-04-20 17:12:28.760031
448	Kabupaten Banggai	Sulawesi Tengah	Indonesia	C3RS	2020-04-20 17:12:28.760031
449	Kabupaten Maluku Tenggara Barat	Maluku	Indonesia	AXAO	2020-04-20 17:12:28.760031
450	Kabupaten Bengkulu Selatan	Bengkulu	Indonesia	Y2II	2020-04-20 17:12:28.760031
451	Kabupaten Seram Bagian Barat	Maluku	Indonesia	12IG	2020-04-20 17:12:28.760031
452	Kabupaten Bolaang Mongondow Utara	Sulawesi Utara	Indonesia	HCXK	2020-04-20 17:12:28.760031
453	Kabupaten Merangin	Jambi	Indonesia	C54O	2020-04-20 17:12:28.760031
454	Kabupaten Blora	Jawa Tengah	Indonesia	PQJN	2020-04-20 17:12:28.760031
455	Kabupaten Manggarai Barat	Nusa Tenggara Timur	Indonesia	KM3V	2020-04-20 17:12:28.760031
456	Kabupaten Lampung Selatan	Lampung	Indonesia	QUOQ	2020-04-20 17:12:28.760031
457	Kabupaten Aceh Barat	Aceh	Indonesia	SPO1	2020-04-20 17:12:28.760031
458	Kabupaten Mamberamo Tengah	Papua	Indonesia	R7JS	2020-04-20 17:12:28.760031
459	Kabupaten Brebes	Jawa Tengah	Indonesia	9BIQ	2020-04-20 17:12:28.760031
460	Kabupaten Buleleng	Bali	Indonesia	NHB0	2020-04-20 17:12:28.760031
461	Kabupaten Pegunungan Bintang	Papua	Indonesia	MEOC	2020-04-20 17:12:28.760031
462	Kabupaten Paser	Kalimantan Timur	Indonesia	6YYP	2020-04-20 17:12:28.760031
463	Kabupaten Simalungun	Sumatera Utara	Indonesia	LBLC	2020-04-20 17:12:28.760031
464	Kota Sungai Penuh	Jambi	Indonesia	WBCY	2020-04-20 17:12:28.760031
465	Kota Manado	Sulawesi Utara	Indonesia	1R0S	2020-04-20 17:12:28.760031
466	Kabupaten Bolaang Mongondow Selatan	Sulawesi Utara	Indonesia	ZJUI	2020-04-20 17:12:28.760031
467	Kota Lhokseumawe	Aceh	Indonesia	UEYJ	2020-04-20 17:12:28.760031
468	Kabupaten Purbalingga	Jawa Tengah	Indonesia	LJJ8	2020-04-20 17:12:28.760031
469	Kabupaten Kayong Utara	Kalimantan Barat	Indonesia	XYK4	2020-04-20 17:12:28.760031
470	Kota Tangerang Selatan	Banten	Indonesia	7STT	2020-04-20 17:12:28.760031
471	Kabupaten Berau	Kalimantan Timur	Indonesia	UGE1	2020-04-20 17:12:28.760031
472	Kabupaten Labuhan Batu Utara	Sumatera Utara	Indonesia	HI1I	2020-04-20 17:12:28.760031
473	Kabupaten Polewali Mandar	Sulawesi Barat	Indonesia	J1KJ	2020-04-20 17:12:28.760031
474	Kabupaten Teluk Wondama	Papua Barat	Indonesia	LG1G	2020-04-20 17:12:28.760031
475	Kabupaten Cianjur	Jawa Barat	Indonesia	KA0F	2020-04-20 17:12:28.760031
476	Kota Banjar Baru	Kalimantan Selatan	Indonesia	KJNI	2020-04-20 17:12:28.760031
477	Kabupaten Murung Raya	Kalimantan Tengah	Indonesia	I8MP	2020-04-20 17:12:28.760031
478	Kota Jambi	Jambi	Indonesia	1GSV	2020-04-20 17:12:28.760031
479	Kota Tangerang	Banten	Indonesia	VXWE	2020-04-20 17:12:28.760031
480	Kabupaten Yahukimo	Papua	Indonesia	PXWZ	2020-04-20 17:12:28.760031
481	Kabupaten Kuantan Singingi	Riau	Indonesia	YHST	2020-04-20 17:12:28.760031
482	Kota Singkawang	Kalimantan Barat	Indonesia	XTAI	2020-04-20 17:12:28.760031
483	Kabupaten Muna Barat	Sulawesi Tenggara	Indonesia	UBXO	2020-04-20 17:12:28.760031
484	Kabupaten Dompu	Nusa Tenggara Barat	Indonesia	UM7D	2020-04-20 17:12:28.760031
485	Kabupaten Nias Barat	Sumatera Utara	Indonesia	TSCU	2020-04-20 17:12:28.760031
486	Kabupaten Padang Pariaman	Sumatera Barat	Indonesia	IVQE	2020-04-20 17:12:28.760031
487	Kabupaten Gianyar	Bali	Indonesia	4WST	2020-04-20 17:12:28.760031
488	Kabupaten Bener Meriah	Aceh	Indonesia	UPT4	2020-04-20 17:12:28.760031
489	Kota Bitung	Sulawesi Utara	Indonesia	HMXF	2020-04-20 17:12:28.760031
490	Kabupaten Cilacap	Jawa Tengah	Indonesia	G8XK	2020-04-20 17:12:28.760031
491	Kota Kediri	Jawa Timur	Indonesia	JVZD	2020-04-20 17:12:28.760031
492	Kabupaten Melawi	Kalimantan Barat	Indonesia	IGQC	2020-04-20 17:12:28.760031
493	Kota Banjarmasin	Kalimantan Selatan	Indonesia	94XR	2020-04-20 17:12:28.760031
494	Kota Jayapura	Papua	Indonesia	ZXWD	2020-04-20 17:12:28.760031
495	Kabupaten Lombok Barat	Nusa Tenggara Barat	Indonesia	UPXQ	2020-04-20 17:12:28.760031
496	Kabupaten Mukomuko	Bengkulu	Indonesia	P1TW	2020-04-20 17:12:28.760031
497	Kabupaten Semarang	Jawa Tengah	Indonesia	NRCE	2020-04-20 17:12:28.760031
498	Kabupaten Sarolangun	Jambi	Indonesia	ZAPS	2020-04-20 17:12:28.760031
499	Kabupaten Tanjung Jabung Barat	Jambi	Indonesia	6Y6O	2020-04-20 17:12:28.760031
500	Kabupaten Kendal	Jawa Tengah	Indonesia	FWSO	2020-04-20 17:12:28.760031
501	Kabupaten Biak Numfor	Papua	Indonesia	AZGA	2020-04-20 17:12:28.760031
502	Kabupaten Ogan Ilir	Sumatera Selatan	Indonesia	XDNT	2020-04-20 17:12:28.760031
503	Kabupaten Mamuju Tengah	Sulawesi Barat	Indonesia	CLJS	2020-04-20 17:12:28.760031
504	Kabupaten Bangka Tengah	Kepulauan Bangka Belitung	Indonesia	MMYK	2020-04-20 17:12:28.760031
505	Kabupaten Sumenep	Jawa Timur	Indonesia	EBPN	2020-04-20 17:12:28.760031
506	Kabupaten Gorontalo	Gorontalo	Indonesia	LOGR	2020-04-20 17:12:28.760031
507	Kota Tegal	Jawa Tengah	Indonesia	NMGT	2020-04-20 17:12:28.760031
508	Kota Lubuklinggau	Sumatera Selatan	Indonesia	JIIT	2020-04-20 17:12:28.760031
509	Kota Palembang	Sumatera Selatan	Indonesia	IY4G	2020-04-20 17:12:28.760031
510	Kabupaten Toraja Utara	Sulawesi Selatan	Indonesia	CRAO	2020-04-20 17:12:28.760031
511	Kota Pematang Siantar	Sumatera Utara	Indonesia	DTHZ	2020-04-20 17:12:28.760031
512	Kabupaten Mappi	Papua	Indonesia	GGKU	2020-04-20 17:12:28.760031
513	Kabupaten Bulukumba	Sulawesi Selatan	Indonesia	RJJM	2020-04-20 17:12:28.760031
514	Kabupaten Sukamara	Kalimantan Tengah	Indonesia	YPEM	2020-04-20 17:12:28.760031
515	Kabupaten Aceh Besar	Aceh	Indonesia	CUPV	2020-04-20 17:12:28.760031
\.

