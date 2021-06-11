const path = require('path');

module.exports = {
    resolve: {
        fallback: {
            "buffer": require.resolve("buffer/"),
            "path": require.resolve("path-browserify"),
            "stream": require.resolve("stream-browserify"),
            "crypto": require.resolve("crypto-browserify"),
        }
    },
    entry: './src/main.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'dist'),
        library: {
            name: "main",
            type: "umd"
        },
    },
};