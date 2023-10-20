echo off

set CURDIR=%~dp0
set OUTFILE=z:\bnrust.log
set SIMPLEOUT=z:\bnrust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\bnrand.bin
set PRIVNUMB=1152
set HASHNUM=7201
set SIGNBIN=z:\sign.bin
set ANUM=0x543B60ADEEF534924A7A70030D1A404041D7DD115D93BA41AC6EB4A8
set PNUM=0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000001
REM set ECTYPE=SECP112r1
set ECTYPE=SECP224r1
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe bnmodsqrt %ANUM% %PNUM% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd