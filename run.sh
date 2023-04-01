#!/bin/bash

maturin develop -r

tmux \
new-session \
'python 2022ex19_plain.py'\; \
split-window \
'python 2022ex19_pyo3.py'\; \
split-window -h \
'python 2022ex19_numba.py'\; \
select-layout even-horizontal\; \
select-pane -t 0 \; 