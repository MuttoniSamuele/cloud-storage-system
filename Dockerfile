FROM postgres

ARG POSTGRES_USER
ARG POSTGRES_DB

RUN psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/users.sql
RUN psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/folders.sql
RUN psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/files.sql
