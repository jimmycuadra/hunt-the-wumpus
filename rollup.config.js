import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
    input: './target/deploy/hunt-the-wumpus.js',
    output: {
        name: 'hunt-the-wumpus',
        file: './release/hunt-the-wumpus.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify
    ]
};
