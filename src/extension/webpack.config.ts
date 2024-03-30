import path from "path"
import HtmlWebpackPlugin from "html-webpack-plugin"
import ForkTsCheckerWebpackPlugin from "fork-ts-checker-webpack-plugin"
import CopyPlugin from "copy-webpack-plugin"
import { Configuration } from "webpack"

const config: Configuration = {
    entry: {
        popup_index: {import: "./app/popup_index.tsx", filename: "app/[name].js"},
        settings_index: {import: "./app/settings_index.tsx", filename: "app/[name].js"},
        background: {import: "./scripts/background.ts", filename: "scripts/[name].js"},
        findplaylists: {import: "./scripts/find-playlists.ts", filename: "scripts/[name].js"},
    },
    mode: "production",
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                include: path.resolve(__dirname, "."),
                use: [{
                    loader: "ts-loader",
                    options: {
                        compilerOptions: { noEmit: false },
                    }
                }],
            },
            {
                test: /\.css$/i,
                include: path.resolve(__dirname, "."),
                use: [
                    "style-loader",
                    "css-loader",
                    "postcss-loader"
                ]
            },
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            filename: `app/popup_index.html`,
            chunks: ["popup_index"],
        }),
        new HtmlWebpackPlugin({
            title: "/Playlist - Settings",
            filename: `app/settings_index.html`,
            chunks: ["settings_index"],
        }),
        new CopyPlugin({
            patterns: [
                { from: "manifest.json", to: "manifest.json" },
                { from: "icons", to: "icons" }
            ],
        }),
        new ForkTsCheckerWebpackPlugin(),
    ],
    watchOptions: {
        ignored: ["node_modules","dist"],
    },
    resolve: {
        extensions: [".tsx", ".ts", ".js"],
    },
    output: {
        path: path.join(__dirname, "../../dist/extension"),
        filename: "[name].js",
        clean: true
    },
};

export default config