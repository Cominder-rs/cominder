#!/usr/bin/nu

export-env {
  let-env base64value = (base64 $"($env.HOME)/.docker_hub/auth.json" | tr -d '\n')
}


cat ../configs/dockerhub-cred.yaml | envsubst | kubectl apply -f -