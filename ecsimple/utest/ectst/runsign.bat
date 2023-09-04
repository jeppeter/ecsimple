echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
set PRIVNUMB=1152
REM set ECTYPE=SECP112r1
set ECTYPE=SECP384r1
set HASHNUM=7201
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecsignbase %ECTYPE% %PRIVNUMB% %HASHNUM% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd