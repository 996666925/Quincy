

globalThis.print = (value: any) => {
    if (value == null) {
        return;
    }
    else if (value.constructor == String) {
        Deno.core.print(value + "\n");
    }
    else {
        Deno.core.print(JSON.stringify(value) + "\n");
    }
}