Description
All escrows have a single beneficiary. Add support for splitting the escrowed amount between multiple beneficiaries on release.

Tasks

Add beneficiaries: Vec<Address> and shares: Vec<u32> (basis points summing to 10000) to Escrow struct

Update create_escrow to accept multiple beneficiaries and shares

On release, distribute funds proportionally to each beneficiary

Validate shares sum to 10000 on creation

Add tests: 2-way split, 3-way split, unequal shares, shares not summing to 10000 panics
Acceptance Criteria

Funds split correctly per basis points

Invalid shares rejected

Tests pass
Complexity: Medium (150 pts)
