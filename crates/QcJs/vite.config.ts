import { FilterPattern, PluginOption, createFilter, defineConfig } from 'vite'
// import swc from "@rollup/plugin-swc"

function swc(input: { include?: FilterPattern, exclude?: FilterPattern } = {}): PluginOption {
  const filter = createFilter(input.include, input.exclude);
  return {
    name: 'swc',
    transform(code, id) {
      if (!filter(id))
        return null;

      let result = code.match(/class\s+(.*?)\s+extends\s+Component\s+{/);
      if (!result) {
        return;
      }
      let clazz = result[1];
      code = code.replace(/\s+extends\s+Component\s+{/, ` extends Component {
          static {
            this.typeName="${clazz}";
            globalThis.__${clazz}__=new ${clazz}();
            globalThis['##${clazz}##']=()=>new ${clazz}();
          }`)
      // console.log(code);
      return code;
    }


  }
}



export default defineConfig({
  build: {
    // lib: {
    //   entry: './lib/main.ts',
    //   name: 'Quincy',
    //   fileName: 'quincy',

    // },
    lib: {
      entry: './src/main.ts',
      name: 'Quincy',
      fileName: 'quincy',
      
    },
    target:"esnext"

  },
  
  plugins: [swc({ include: "./(src|lib)/**/**.ts" })],

})

