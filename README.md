# pyo3_tests

## Init

```shell
python -m venv .env
source .env/bin/activate
pip install --upgrade pip
pip install maturin numba
```

## Command

Remove `-r` for a debug build

```shell

source .env/bin/activate

maturin develop -r && python 2022ex19_pyo3.py 

```