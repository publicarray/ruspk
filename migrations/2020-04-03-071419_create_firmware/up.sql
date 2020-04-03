CREATE TABLE firmware (
  id SERIAL PRIMARY KEY,
  version VARCHAR(3) NOT NULL UNIQUE,
  build INT NOT NULL
);

INSERT INTO firmware (version, build) VALUES
        ('2.0', 731),
        ('2.1', 844),
        ('2.2', 942),
        ('2.3', 1139),
        ('3.0', 1337),
        ('3.1', 1594),
        ('3.2', 1922),
        ('4.0', 2198),
        ('4.1', 2636),
        ('4.2', 3202),
        ('4.3', 3776),
        ('5.0', 4458),
        ('5.1', 5004);
