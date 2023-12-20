---- Base app schema

-- User
CREATE TABLE "user" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    username varchar(128) NOT NULL UNIQUE
);

-- Vendor
CREATE TABLE vendor (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    name varchar(128) NOT NULL UNIQUE
);