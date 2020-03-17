FROM alpine:3.7
ARG database_url=postgresql://postgres@postgre_server/pandemia?sslmode=disable

#ADD http://178.128.219.222/linux-x86_64-musl/pandemia_server-nightly-latest /usr/bin/pandemia_server
#ADD ./pandemia_server-nightly-latest /usr/bin/pandemia_server
ADD ./pandemia_server /usr/bin/pandemia_server
ADD ./start.sh /usr/bin/start_server.sh
#ADD http://178.128.219.222/linux-x86_64-musl/pandemia_server-nightly /usr/bin/pandemia_server
RUN chmod +x /usr/bin/pandemia_server

ENV DATABASE_URL=$database_url

EXPOSE 8080 9090

CMD ["ash", "/usr/bin/start_server.sh"]

