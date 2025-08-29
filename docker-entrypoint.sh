#!/bin/sh
set -e
update-ca-certificates
exec /usr/app/workspace_booking_system-cli start --environment production