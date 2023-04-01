WITH cte AS (
    SELECT
        t.*
    FROM Trips AS t
    LEFT JOIN Users AS c ON t.client_id = c.users_id
    LEFT JOIN Users AS d ON t.driver_id = d.users_id
    WHERE c.banned != 'Yes' AND d.banned != 'Yes'
)
SELECT DISTINCT
    t.request_at                                             AS 'Day'
    , ROUND(IFNULL(canceled.count / requested.count, 0), 2)  AS 'Cancellation Rate'
FROM Trips AS t
LEFT JOIN (
    SELECT
        cte.request_at
        , COUNT(id) AS 'count'
    FROM cte
    WHERE cte.status != "completed"
    GROUP BY cte.request_at
) AS canceled ON t.request_at = canceled.request_at
JOIN (
    SELECT
        cte.request_at
        , COUNT(id) AS 'count'
    FROM cte
    GROUP BY cte.request_at
) AS requested ON t.request_at = requested.request_at
WHERE
    t.request_at BETWEEN '2013-10-01' AND '2013-10-03'
;
