CREATE TABLE votes (
  link_id INTEGER REFERENCES links(id),
  category_id INTEGER REFERENCES categories(id),
  uuid VARCHAR NOT NULL,
  ip VARCHAR NOT NULL,
  PRIMARY KEY (link_id, category_id, uuid)
);

INSERT INTO votes(link_id, category_id, uuid, ip) VALUES (1, 5, '123', '0.0.0.0');
