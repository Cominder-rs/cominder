#!/usr/bin/nu

try {
  let _ = $env.app_name
} catch {
  print 'You must specify `app_name` env variable'
  exit 1
}

cat ../configs/service-app.yaml | envsubst | kubectl apply -f -