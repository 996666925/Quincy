import { FilterPattern, PluginOption, createFilter, defineConfig } from 'vite'
// import swc from "@rollup/plugin-swc"

function swc(input: { include?: FilterPattern, exclude?: FilterPattern } = {}): PluginOption {
  const filter = createFilter(input.include, input.exclude);
  return {
    name: 'swc',
    transform(code, id) {
      if (!filter(id))
        return null;
      let clazz = code.match(/class\s+(.*?)\s+extends\s+Component\s+{/)![1];

      code = code.replace(/\s+extends\s+Component\s+{/, ` extends Component {
          static {
            this.typeName="${clazz}";
            globalThis.__${clazz}__=new ${clazz}();
          }`)
      console.log(code);
      return code;
    }


  }
}



export default defineConfig({
  build: {
    // lib: {
    //   entry: './lib/main.ts',
    //   name: 'OverLoad',
    //   fileName: 'overload',

    // },
    lib: {
      entry: './src/main.ts',
      name: 'OverLoad',
      fileName: 'overload',

    },

  },
  plugins: [swc({ include: "./src/**.ts", })],

})

