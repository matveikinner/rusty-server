CREATE DATABASE supertokens;

CREATE USER supertokens WITH ENCRYPTED PASSWORD 'supertokens';

GRANT ALL PRIVILEGES ON DATABASE supertokens TO supertokens;

GRANT ALL ON SCHEMA public TO supertokens;