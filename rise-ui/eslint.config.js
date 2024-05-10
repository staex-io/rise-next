import js from "@eslint/js";
import pluginVue from 'eslint-plugin-vue';
export default [
  js.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    rules: {
      "no-unused-vars": "warn"
    },
  },
  {
    ignores: [
      "src/assets/DIDContract.json",
      "src/assets/AgreementContract.json",
      "src/assets/DataProvingContract.json",
    ]
  }
];
