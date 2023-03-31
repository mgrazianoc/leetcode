SELECT 
    *
FROM (
    SELECT DISTINCT
        salary AS SecondHighestSalary
    FROM Employee
    ORDER BY salary DESC
    LIMIT 1,1
) s
UNION
SELECT
    NULL AS SecondHighestSalary
LIMIT 1;