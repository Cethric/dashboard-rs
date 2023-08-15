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