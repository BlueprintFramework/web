CREATE MATERIALIZED VIEW mv_extension_stats AS
SELECT 
	extensions.id,
	jsonb_build_object(
		'panels', (
			SELECT COUNT(*)
			FROM (
				SELECT jsonb_array_elements(data->'blueprint'->'extensions') as ext 
				FROM telemetry_data 
				WHERE id IN (
					SELECT latest_telemetry_data_id 
					FROM telemetry_panels_with_latest
				)
				AND created > NOW() - INTERVAL '2 days'
			) subq
			WHERE subq.ext->>'identifier' = extensions.identifier
		)
	) stats,
	(
		SELECT jsonb_agg(
			jsonb_build_object(
				'version', version,
				'percentage', percentage
			)
			ORDER BY percentage DESC
		)
		FROM (
			SELECT
				ext->>'version' AS version,
				(COUNT(*) * 100.0 / SUM(COUNT(*)) OVER ())::float8 AS percentage
			FROM
				(
					SELECT jsonb_array_elements(data->'blueprint'->'extensions') AS ext
					FROM telemetry_data
					WHERE
						id IN (
							SELECT telemetry_data.id
							FROM telemetry_panels_with_latest
							WHERE latest_telemetry_data_id = (
								SELECT latest_telemetry_data_id
								FROM telemetry_panels_with_latest
								ORDER BY created DESC
								LIMIT 1
							)
						)
						AND created > NOW() - INTERVAL '2 days'
				) AS subq
			WHERE subq.ext->>'identifier' = extensions.identifier
			GROUP BY subq.ext->>'version'
		) version_data
	) versions
FROM extensions;
CREATE UNIQUE INDEX mv_extension_stats_id_idx ON mv_extension_stats (id);
