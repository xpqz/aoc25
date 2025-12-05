#!/bin/bash
sqlite3 :memory: <<'EOF'
CREATE TABLE ranges (start INTEGER, end INTEGER);
CREATE TABLE candidates (value INTEGER);
.separator -
.import ../d/ranges.txt ranges
.separator " "
.import ../d/values.txt candidates
SELECT COUNT(*) FROM candidates WHERE EXISTS(SELECT 1 FROM ranges WHERE value >= start AND value <= end);
EOF