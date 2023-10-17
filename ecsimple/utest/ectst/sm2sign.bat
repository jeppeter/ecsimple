echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
set ECPRIV_FILE=z:\ecpriv.bin
set BIN_FILE=z:\bin.bin
set SIGN_BIN=z:\rust_sign.bin
REM set ECTYPE=SECP112r1
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecsign --digesttype sm3 --ecpriv %ECPRIV_FILE% %BIN_FILE% --output %SIGN_BIN%  2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd
set ECSIMPLE_LEVEL=