/// <reference types="vite/client" />


declare const globalThis: any;
declare function print(msg: any);

namespace Deno {

    declare class core {
        static ops: any;
        static print(msg: any);

    }

}



declare function getComponent();