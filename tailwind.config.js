/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "jit",
    content: [
        'index.html',
        'src/**/*.rs',
        'components/src/**/*.rs',
        'components/test/**/*.rs',
    ],
    // presets: [],
    theme: {
        extend: {},
    },
    safelist: [
        {
            pattern: /btn/,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /btm-nav/,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /navbar/,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /badge/,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /alert/,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /drawer/,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /text-./,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
        {
            pattern: /bg-./,
            variants: ['xs', 'sm', 'md', 'lg', 'xl'],
        },
    ],
    plugins: [
        require('daisyui'),
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
        require('@tailwindcss/aspect-ratio'),
        require('@tailwindcss/container-queries'),
    ],
    corePlugins: {
        preflight: true,
    },
    daisyui: {
        themes: ['autumn', 'night'],
        darkTheme: 'night',
        base: true,
        styled: true,
        utils: true,
        rtl: false,
        prefix: '',
        logs: true
    }
}