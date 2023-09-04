echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set PUBBIN=z:\ecpub.bin
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
set PRIVNUMB=1152
set HASHNUM=7201
set SIGNBIN=z:\sign.bin
REM set ECTYPE=SECP112r1
set ECTYPE=PRIME192v2
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecpubload %ECTYPE% %PUBBIN% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd