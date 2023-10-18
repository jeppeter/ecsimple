echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
REM set ECSIMPLE_RANDOP=z:\rust_rand.bin
set ECPRIV_FILE=z:\ecpriv.bin
SET ECPUB_FILE=z:\ecpub.bin
set BIN_FILE=z:\bin.bin
set SIGN_BIN=z:\sign.bin
REM set ECTYPE=SECP112r1
REM pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe insertrand -i %ECSIMPLE_RANDOP2% -o %ECSIMPLE_RANDOP% 32 32  &&  popd
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecvfy --digesttype sm3 --ecpub %ECPUB_FILE% %BIN_FILE% %SIGN_BIN%  2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd
set ECSIMPLE_LEVEL=