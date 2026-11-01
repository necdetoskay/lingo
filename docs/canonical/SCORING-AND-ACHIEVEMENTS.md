# Scoring and Achievements v1.0

## Round scoring

A solved round receives a fixed base score of 500 plus an attempt bonus.

| Solved on attempt | Bonus | Total |
| --- | ---: | ---: |
| 1 | 500 | 1000 |
| 2 | 400 | 900 |
| 3 | 300 | 800 |
| 4 | 200 | 700 |
| 5 | 100 | 600 |

A failed round awards zero points. Scores do not depend on word length. Statistics must retain length-separated success measures.

## Streaks and badges

A solved round increments the current streak. A failed round resets it; total score remains intact. Initial badges are: first solved word, 5/10/25-round streaks, first-attempt solve, and 100 total solved words. Badge awarding is idempotent and survives app restarts.
