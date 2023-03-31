SELECT
    d.name          AS Department
    , e.Employee
    , e.Salary
FROM Department AS d
JOIN (
    SELECT
        id, Employee, Salary
    FROM (
        SELECT
            d.id        AS id
            , e.name    AS Employee
            , e.salary  AS Salary,
            RANK() OVER (PARTITION BY d.id ORDER BY e.salary DESC) AS R
        FROM Department AS d
        JOIN Employee AS e ON d.id = e.departmentId
    ) s
    WHERE R <= 3
) AS e USING(id);
