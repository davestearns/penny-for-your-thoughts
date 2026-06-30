# Scripts

This directory contains helpful scripts that are intended to be run manually as-needed.

## gen-currencies.py

This script downloads the latest [ISO 4217 Currency definitions](https://www.six-group.com/en/products-services/financial-information/market-reference-data/data-standards.html) and generates all the code for `src/iso_currencies.rs`. This list doesn't change very often, but when it does, run this script like so:

```bash
python3 scripts/gen-currencies.py > src/iso_currencies.rs
```
