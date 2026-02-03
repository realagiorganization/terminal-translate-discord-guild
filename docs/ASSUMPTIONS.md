# Assumptions

- `format validate` treats the input as valid if it is JSON and a top-level object.
- The optional top-level `format` field, when present, must be a string with value `dump` or `upload`.
- If `--format` is provided but the input omits `format`, validation still passes and reports `format=missing`.
- The optional top-level `version` field, when present, must be an unsigned integer.
