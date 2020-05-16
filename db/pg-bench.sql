BEGIN;
SELECT
  package.id AS package_id,
  version.id AS version_id,
  (
    CASE
      WHEN version.report_url <> '' THEN true
      ELSE false
    END
  ) AS beta,
  version.conflicts AS conflictpkgs,
  version.dependencies AS deppkgs,
  version.changelog,
  description.description AS "desc",
  version.distributor,
  version.distributor_url,
  displayname.displayname AS dname,
  build.path AS link,
  version.maintainer,
  version.maintainer_url,
  package.name AS package,
  version.install_wizard AS qinst,
  version.startable AS qstart,
  version.upgrade_wizard AS qupgrade,
  version.upstream_version,
  version.version AS revision,
  build.md5,
  build.extract_size AS size
FROM (
    (
      package
      INNER JOIN (
          (
            (
              version
              LEFT OUTER JOIN description ON description.version_id = version.id
                AND description.language_id = CASE
                  WHEN EXISTS (
                    SELECT
                      1
                    FROM description
                    WHERE
                      description.language_id = 1
                  ) THEN 1
                  ELSE 1
                END
            )
            LEFT OUTER JOIN displayname ON displayname.version_id = version.id
              AND displayname.language_id = CASE
                WHEN EXISTS (
                  SELECT
                    1
                  FROM displayname
                  WHERE
                    displayname.language_id = 1
                ) THEN 1
                ELSE 1
              END
          )
          INNER JOIN (
              SELECT
                version.id,
                MAX(version.version) AS version,
                package_id
              FROM version
              GROUP BY
                version.id,
                version.package_id
            ) ver ON version.package_id = ver.package_id
            AND version.version = ver.version
        ) ON version.package_id = package.id
    )
    INNER JOIN (
        (
          build
          INNER JOIN firmware ON firmware.id = build.firmware_id
            AND firmware.version = '6.2'
        )
        INNER JOIN build_architecture ON build_architecture.build_id = build.id
          AND build_architecture.architecture_id = 6
      ) ON build.package_id = package.id
  )
WHERE
  build.active = true
  AND firmware.build >= 6000
  AND (
    true
    OR version.report_url = ''
  );
END;
