module.exports = {
    extends: [
        '@vue/typescript/recommended',
    ],
    rules: {
        // override/add rules settings here, such as:
        '@typescript-eslint/no-unused-vars': 'off'
    },
    ignorePatterns: ["rust/pkg/*"]
}
