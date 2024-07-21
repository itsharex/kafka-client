import pluginVue from "eslint-plugin-vue";
import ts from "typescript-eslint"
import js from "@eslint/js"
import { parserOptions } from "eslint-plugin-vue/lib/configs/base";
export default [
    ...ts.config(
        js.configs.recommended,
        ...ts.configs.recommendedTypeChecked,
        {
            languageOptions: {
                parserOptions: {
                    project: true,
                },
            },
        },
        {
            files: ['**/*.js'],
            extends: [ts.configs.disableTypeChecked],
        },
    ),
    ...pluginVue.configs["flat/recommended"],
    {
        files: ['*.vue', '**/*.vue'],
        languageOptions: {
            parserOptions: {
                parser: '@typescript-eslint/parser',
            }
        }
    }
]