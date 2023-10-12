echo off

set CURDIR=%~dp0
set OUTFILE=z:\owngen.log
set SIMPLEOUT=z:\owngen_simple.log
set ECSIMPLE_LEVEL=50
REM set ECSIMPLE_RANDOP=z:\rust.rand.bin
set PRIVNUMB=0x13c5873c53d1046528aeed5cbe4b
REM set ECTYPE=SECP112r1
set ECTYPE=secp112r1
set ECPUBBIN=z:\own_ecpub.bin
set ECPRIVBIN=z:\own_ecpriv.bin
set HASHNUM=0x99bcf1bc2a70d552e85a3b7efe51
set HASHSIZE=14



pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecgenbase %ECTYPE% %PRIVNUMB% %HASHSIZE% --ecpriv %ECPRIVBIN%  --ecpub %ECPUBBIN% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd
SET ECSIMPLE_LEVEL=