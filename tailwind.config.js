module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: '#000f0f',
                    hover: '#000000',
                    muted: '#333a3a',
                },
                secondary: {
                    DEFAULT: '#656565',
                    hover: '#a0b1bd',
                    muted: '#d6d6d6',
                },
                accent: {
                    DEFAULT: '#95B8F1',
                    hover: '#1b86ff',
                    muted: '#C3E1FF',
                },
                surface: {
                    DEFAULT: '#f3f1ee',
                    muted: '#fffefd',
                    accented: '#FFE7C3',
                },
            },
        },
    },
    plugins: [],
}