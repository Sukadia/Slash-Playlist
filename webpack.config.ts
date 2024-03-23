import path from "path"
import HtmlWebpackPlugin from "html-webpack-plugin"
import ForkTsCheckerWebpackPlugin from "fork-ts-checker-webpack-plugin"
import CopyPlugin from "copy-webpack-plugin"
import { Configuration } from "webpack"

const config: Configuration = {
    entry: {
        index: {import: "./src/extension/app/index.tsx", filename: "app/[name].js"},
        background: {import: "./src/extension/scripts/background.ts", filename: "scripts/[name].js"},
        findplaylists: {import: "./src/extension/scripts/find-playlists.ts", filename: "scripts/[name].js"},
    },
    mode: "production",
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                include: path.resolve(__dirname, "src"),
                use: [{
                    loader: "ts-loader",
                    options: {
                        compilerOptions: { noEmit: false },
                    }
                }],
            },
            {
                test: /\.css$/i,
                include: path.resolve(__dirname, "src"),
                use: [
                    "style-loader",
                    "css-loader",
                    "postcss-loader"
                ]
            },
        ],
    },
    plugins: [
        new ForkTsCheckerWebpackPlugin(),
        new CopyPlugin({
            patterns: [
                { from: "src/extension/manifest.json", to: "manifest.json" },
                { from: "src/extension/icons", to: "icons" },
                { from: "node_modules/webextension-polyfill/dist/browser-polyfill.min.js", to: "scripts/browser-polyfill.js" },
            ],
        }),
        ...getHtmlPlugins(["index"])
    ],
    watchOptions: {
        ignored: ["node_modules","dist"],
    },
    resolve: {
        extensions: [".tsx", ".ts", ".js"],
    },
    output: {
        path: path.join(__dirname, "dist/extension"),
        filename: "[name].js",
        clean: true
    },
};

function getHtmlPlugins(chunks: string[]) {
    return chunks.map(
        (chunk) =>
            new HtmlWebpackPlugin({
                title: "React extension",
                filename: `app/${chunk}.html`,
                chunks: [chunk],
            })
    );
}

export default config