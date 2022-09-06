-- Your SQL goes here

CREATE TABLE partner (
  id            VARCHAR (36) PRIMARY KEY,
  trading_name  VARCHAR (256) NOT NULL,
  owner_name    VARCHAR (256) NOT NULL,
  document      VARCHAR (256) NOT NULL
);
