CREATE TABLE votes (
  link_id INTEGER REFERENCES links(id),
  category_id INTEGER REFERENCES categories(id),
  uuid VARCHAR NOT NULL,
  ip VARCHAR NOT NULL,
  PRIMARY KEY (link_id, category_id, uuid)
)
