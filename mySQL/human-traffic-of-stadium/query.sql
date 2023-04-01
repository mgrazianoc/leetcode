WITH cte AS (
    SELECT
        *
        , id - ROW_NUMBER() OVER() AS id_diff
    FROM Stadium
    WHERE people > 99
)
SELECT
    id
    , visit_date
    , people
FROM cte
WHERE id_diff IN (SELECT id_diff FROM cte GROUP BY id_diff HAVING count(*) > 2)
ORDER BY visit_date;
