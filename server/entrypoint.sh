#!/bin/sh
exec auditaserver --config /etc/auditaserver/config.toml | tee -a /var/log/audita.log
