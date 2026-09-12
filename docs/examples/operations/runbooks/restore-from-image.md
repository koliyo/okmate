---
type: Runbook
title: Restore from image
description: Recover a failed host from the latest known-good backup image.
---

# Restore from image

## Trigger

The host does not boot, or a restore has been requested.

## Steps

1. Identify the latest good image on the [backup host](/services/backup-host.md).
2. Restore onto replacement hardware.
3. Confirm the service endpoint responds.

## Rollback

Keep the previous image until the restored host has passed the success
check. This is an operational response, not a product architecture note.
