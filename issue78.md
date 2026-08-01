Description
Once created, the beneficiary of an escrow cannot be changed. Add a transfer function allowing the depositor to reassign the beneficiary before release.

Tasks

Add transfer_beneficiary(env, depositor, escrow_id, new_beneficiary)

Only depositor can transfer

Escrow must be Active (not released/refunded/disputed)

Emit beneficiary_transferred event

Add tests: successful transfer, non-depositor attempt, transfer on non-active escrow
Acceptance Criteria

Beneficiary updated correctly

Only depositor can transfer

Tests pass
