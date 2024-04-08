import path from 'path';
import { fileURLToPath } from 'url';

export default {
    entry: path.resolve(path.dirname(fileURLToPath(import.meta.url)), './lib/app.js'),
    output: {
        path: path.resolve(path.dirname(fileURLToPath(import.meta.url)), 'dist'),
        filename: 'tasklist-glsp-server.js'
    },
    mode: 'development',
    target: 'node',
    resolve: {
        extensions: ['.ts', '.tsx', '.js']
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                use: ['source-map-loader'],
                enforce: 'pre'
            }
        ]
    }
};
