module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: '#000f0f',
                    hover: '#2563EB',
                    muted: '#1E3A8A',
                },
                secondary: {
                    DEFAULT: '#656565',
                    hover: '#475569',
                    muted: '#5a79a8',
                },
                accent: {
                    DEFAULT: '#95B8F1',
                    hover: '#1b86ff',
                    muted: '#C3E1FF',
                },
                surface: {
                    DEFAULT: '#fffefd',
                    muted: '#f3f3f3',
                    accented: '#FFE7C3',
                },
            },
        },
    },
    plugins: [],
}