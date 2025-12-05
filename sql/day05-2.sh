#!/bin/bash
sqlite3 :memory: <<'EOF'
CREATE TABLE ranges (start INTEGER, end INTEGER);
.separator -
.import ../d/ranges.txt ranges
WITH prev AS (
    SELECT start, end,
           MAX(end) OVER (ORDER BY start, end ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING) AS prev_max
    FROM ranges
),
grouped AS (
    SELECT start, end,
           SUM(CASE WHEN prev_max IS NULL OR start > prev_max THEN 1 ELSE 0 END)
               OVER (ORDER BY start, end) AS grp
    FROM prev
),
merged AS (
    SELECT MIN(start) AS start, MAX(end) AS end FROM grouped GROUP BY grp
)
SELECT SUM(end - start + 1) FROM merged;
EOF