CREATE TABLE language (
  id SERIAL PRIMARY KEY,
  code VARCHAR(3) NOT NULL UNIQUE,
  name VARCHAR(50) NOT NULL
);

INSERT INTO language (code, name) VALUES
    ('enu', 'English'),
    ('cht', 'Traditional Chinese'),
    ('chs', 'Simplified Chinese'),
    ('krn', 'Korean'),
    ('ger', 'German'),
    ('fre', 'French'),
    ('ita', 'Italian'),
    ('spn', 'Spanish'),
    ('jpn', 'Japanese'),
    ('dan', 'Danish'),
    ('nor', 'Norwegian'),
    ('sve', 'Swedish'),
    ('nld', 'Dutch'),
    ('rus', 'Russian'),
    ('plk', 'Polish'),
    ('ptb', 'Brazilian Portuguese'),
    ('ptg', 'European Portuguese'),
    ('hun', 'Hungarian'),
    ('trk', 'Turkish'),
    ('csy', 'Czech');
