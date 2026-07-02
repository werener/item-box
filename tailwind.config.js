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
                    muted: '#c3e1ff',
                },
                surface: {
                    DEFAULT: '#f3f1ee',
                    muted: '#fffefd',
                    accented: {
                        DEFAULT: '#ffe7c3',
                        muted: '#fde3b9'
                    },
                },
                error: {
                    primary: {
                        DEFAULT: '#be3436',
                        hover: '#ff0400',
                        muted: '#ffa39e',
                    },
                    secondary: {
                        DEFAULT: '#ffffff',
                        hover: '#f8ffdf',
                        muted: '#272425',
                    }
                },
            },
        },
    },
    plugins: [],
}