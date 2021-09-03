./web_build.sh ${@:1}
devserver --address 0.0.0.0:8080 --header Cross-Origin-Opener-Policy='same-origin' --header Cross-Origin-Embedder-Policy='require-corp' --path web_build