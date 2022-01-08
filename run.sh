./web_build.sh ${@:1}
devserver --header Cross-Origin-Opener-Policy='same-origin' --header Cross-Origin-Embedder-Policy='require-corp' --path web_build