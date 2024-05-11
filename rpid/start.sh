#!/usr/bin/env sh
systemd-run --user --working-directory=$(pwd) poetry run python main.py
