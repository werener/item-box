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
                    DEFAULT: '#64748B',
                    hover: '#475569',
                    muted: '#5a79a8',
                },
                accent: {
                    DEFAULT: '#489dff',
                    hover: '#1b86ff',
                    muted: '#5c839e',
                },
                surface: {
                    DEFAULT: '#fdfbec',
                    muted: '#f5eacb',
                },
            },
        },
    },
    plugins: [],
}