-- -------------------------------------------------------------
-- TablePlus 3.5.0(309)
--
-- https://tableplus.com/
--
-- Database: ruspk
-- Generation Time: 2020-05-17 06:38:50.4460
-- -------------------------------------------------------------


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

INSERT INTO architecture (id, code)
VALUES
  ('16', 'x64'),
  ('17', 'apollolake');

INSERT INTO package (id, author_user_id, name, insert_date)
VALUES
  ('1', NULL, 'git', '2020-04-03 21:04:06'),
  (
    '2',
    NULL,
    'dnscrypt-proxy',
    '2020-04-06 18:11:52'
  );

INSERT INTO firmware (id, version, build)
VALUES
  ('14', '6.0', '5004'),
  ('15', '6.1', '5004'),
  ('16', '6.2', '24922');

INSERT INTO build (id, package_id, firmware_id, publisher_user_id, checksum, extract_size, path, md5, insert_date, active) VALUES
('1', '2', '16', NULL, '1f8675f9c7307666a8d2e18e6770d1b2', '200', 'dnscrypt-proxy.v5.f15047%5Brtd1296-armada37xx-aarch64%5D.spk', '1f8675f9c7307666a8d2e18e6770d1b2', '2020-04-06 18:18:46', '1'),
('2', '1', '16', NULL, 'asdhash', '300', 'x86_64.spk', 'sdhash', '2020-05-09 17:04:40', '1');

INSERT INTO build_architecture (build_id, architecture_id) VALUES
('1', '6'),
('1', '12'),
('1', '16'),
('1', '17'),
('2', '16');

INSERT INTO version (
    id,
    package_id,
    version,
    upstream_version,
    changelog,
    report_url,
    distributor,
    distributor_url,
    maintainer,
    maintainer_url,
    dependencies,
    conf_dependencies,
    conflicts,
    conf_conflicts,
    install_wizard,
    upgrade_wizard,
    startable,
    license,
    insert_date
  )
VALUES
  (
    '1',
    '1',
    '3',
    '1.8.4',
    'first version',
    'https://github.com/SynoCommunity/spksrc/issues',
    'SynoCommunity',
    'https://synocommunity.com/',
    'Safihre',
    '',
    'None',
    '{bla: {"dsm_min_ver": "5.0-4300"}}',
    'bra',
    '{bla: {"dsm_min_ver": "5.0-4300"}}',
    '0',
    '0',
    '0',
    'GNU',
    '2020-04-03 21:04:22'
  ),
  (
    '2',
    '1',
    '4',
    '2.1.2',
    'Update Git to 2.24.1',
    'hiugjh',
    'SynoCommunity',
    'https://synocommunity.com/',
    'Safihre',
    '',
    'None',
    '{blaa: {"dsm_min_ver": "5.0-4300"}}',
    'bou',
    '{bla: {"dsm_min_ver": "5.0-4300"}}',
    '0',
    '0',
    '0',
    'GNU',
    '2020-04-03 21:07:19'
  ),
  (
    '3',
    '2',
    '1',
    '2.0.33',
    'Update to 2.0.33',
    '',
    'SynoCommunity',
    'https://synocommunity.com/',
    'SynoCommunity',
    'seby.io',
    NULL,
    '',
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    '2020-04-06 18:14:27'
  ),
  (
    '4',
    '2',
    '2',
    '2.0.33',
    'fixes',
    '',
    'SynoCommunity',
    'https://synocommunity.com/',
    'SynoCommunity',
    'seby.io',
    NULL,
    '',
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    '2020-04-06 18:14:27'
  );

INSERT INTO description (version_id, language_id, description) VALUES
('1', '1', 'Git is a free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.'),
('2', '1', 'Git is a free and open source distributed version control system.'),
('2', '2', 'ytdrijnkml,;.kj'),
('3', '1', 'A flexible DNS proxy, with support for modern encrypted DNS protocols such as DNSCrypt v2 and DNS-over-HTTPS. https://github.com/jedisct1/dnscrypt-proxy'),
('3', '3', 'ljuiytrfghvbjknlm'),
('4', '1', 'A flexible DNS proxy, with support for modern encrypted DNS protocols such as DNSCrypt v2 and DNS-over-HTTPS. https://github.com/jedisct1/dnscrypt-proxy'),
('4', '5', 'germ');

INSERT INTO displayname (version_id, language_id, displayname) VALUES
('1', '1', 'git'),
('2', '1', 'git'),
('3', '1', 'dnscrypt-proxy'),
('4', '1', 'dnscrypt-proxy'),
('4', '5', 'dns-germ');

INSERT INTO icon (id, version_id, size, path) VALUES
('1', '3', '72', 'default.png'),
('2', '3', '256', 'default.png');


/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
