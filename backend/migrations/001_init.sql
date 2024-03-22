CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS Users;

CREATE TABLE Users (
  user_id UUID DEFAULT uuid_generate_v4 (),
  name VARCHAR NOT NULL,
  private_key TEXT NOT NULL,

  PRIMARY KEY (user_id)
);


DROP TABLE IF EXISTS Redirects;

CREATE TABLE Redirects (
  redirect_id UUID DEFAULT uuid_generate_v4 (),
  url_from VARCHAR NOT NULL,
  url_to VARCHAR NOT NULL,

  PRIMARY KEY (redirect_id)
);

DROP TABLE IF EXISTS Redirects_Users;

CREATE TABLE Redirects_Users (
  relation_id UUID DEFAULT uuid_generate_v4 (),
  redirect_id UUID NOT NULL,
  user_id UUID NOT NULL,

  PRIMARY KEY (relation_id),

  CONSTRAINT fk_redirects_users
      FOREIGN KEY (user_id)
        REFERENCES Users (user_id),
        
  CONSTRAINT fk_redirects_redirect
      FOREIGN KEY (redirect_id) 
        REFERENCES Redirects (redirect_id)
);
