CREATE TABLE architecture (
  id SERIAL PRIMARY KEY,
  code VARCHAR(20) NOT NULL UNIQUE
);

INSERT INTO architecture (code) VALUES
    ('noarch'),
    ('ppc824x'),
    ('ppc854x'),
    ('ppc853x'),
    ('88f628x'),
    ('x86'),
    ('bromolow'),
    ('cedarview'),
    ('qoriq'),
    ('armada370'),
    ('armadaxp'),
    ('evansport'),
    ('comcerto2k'),
    ('avoton'),
    ('armada375');
