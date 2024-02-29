

globalThis.print = (value: any) => {
    if (value == null) {
        Deno.core.print("null\n");
    }
    else if (value.constructor == String) {
        Deno.core.print(value + "\n");
    }
    else {
        Deno.core.print(JSON.stringify(value) + "\n");
    }
}