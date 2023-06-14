#!/usr/bin/nu

kubectl create secret tls constellation-cert --key="../certs/constellation.key" --cert="../certs/constellation.crt" --namespace civilization
  