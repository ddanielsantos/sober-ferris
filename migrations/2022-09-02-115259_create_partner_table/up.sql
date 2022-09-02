-- Your SQL goes here

CREATE TABLE partner (
  id            VARCHAR (36) PRIMARY KEY,
  tradingName   VARCHAR (256) NOT NULL,
  ownerName     VARCHAR (256) NOT NULL,
  document      VARCHAR (256) NOT NULL,
  coverageArea  GEOMETRY NOT NULL,
  address       GEOMETRY NOT NULL
);
