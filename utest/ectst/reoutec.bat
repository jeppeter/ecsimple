
set SCRIPTDIR=%~dp0
set ECKEYDIR=z:\eckeys

%SCRIPTDIR%\target\release\ectst.exe ecprivload -o %ECKEYDIR%\rust.sect163r1.ecpriv.named.der %ECKEYDIR%\sect163r1.ecpriv.named.pem --eccmprtype "hybrid" --ecparamenc ""
%SCRIPTDIR%\target\release\ectst.exe ecprivload -o %ECKEYDIR%\rust.sect163r1.ecpriv.der %ECKEYDIR%\sect163r1.ecpriv.pem --eccmprtype "compressed" --ecparamenc "explicit"

%SCRIPTDIR%\target\release\ectst.exe ecpubload -o %ECKEYDIR%\rust.sect163r1.ecpub.named.der %ECKEYDIR%\sect163r1.ecpub.named.pem --eccmprtype "hybrid" --ecparamenc ""
%SCRIPTDIR%\target\release\ectst.exe ecpubload -o %ECKEYDIR%\rust.sect163r1.ecpub.der %ECKEYDIR%\sect163r1.ecpub.pem --eccmprtype "compressed" --ecparamenc "explicit"


%SCRIPTDIR%\target\release\ectst.exe ecprivload -o %ECKEYDIR%\rust.secp224r1.ecpriv.named.der %ECKEYDIR%\secp224r1.ecpriv.named.pem --eccmprtype "hybrid" --ecparamenc ""
%SCRIPTDIR%\target\release\ectst.exe ecprivload -o %ECKEYDIR%\rust.secp224r1.ecpriv.der %ECKEYDIR%\secp224r1.ecpriv.pem --eccmprtype "compressed" --ecparamenc "explicit"

%SCRIPTDIR%\target\release\ectst.exe ecpubload -o %ECKEYDIR%\rust.secp224r1.ecpub.named.der %ECKEYDIR%\secp224r1.ecpub.named.pem --eccmprtype "hybrid" --ecparamenc ""
%SCRIPTDIR%\target\release\ectst.exe ecpubload -o %ECKEYDIR%\rust.secp224r1.ecpub.der %ECKEYDIR%\secp224r1.ecpub.pem --eccmprtype "compressed" --ecparamenc "explicit"
